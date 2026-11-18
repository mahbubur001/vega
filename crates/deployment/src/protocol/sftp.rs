//! SFTP over SSH. Two backends:
//! - built-in (russh): host keys go through [`ConnectHooks`] (known_hosts + policy), auth by password / key /
//!   agent / `~/.ssh/config`, keyboard-interactive for 2FA, one jump host;
//! - system OpenSSH (`ssh -s host sftp`): ProxyJump chains, certificates, FIDO keys and Kerberos come for free.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use russh::client::{self, Handle};
use russh::keys::{HashAlg, PrivateKey, PrivateKeyWithHashAlg, PublicKeyOrCertificate};
use russh::{ChannelMsg, Disconnect, MethodKind};
use russh_sftp::client::SftpSession;
use russh_sftp::client::error::Error as SftpError;
use russh_sftp::protocol::{FileAttributes, StatusCode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::*;
use crate::config::{SftpAuth, SshBackend};
use crate::error::{ErrorCode, RemoteError, Result};

const CHUNK: usize = 256 * 1024;

pub struct SshHandler {
    hooks: Arc<dyn ConnectHooks>,
    host: String,
    port: u16,
    alive: Arc<AtomicBool>,
    host_key_rejected: Arc<AtomicBool>,
}

impl client::Handler for SshHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        key: &PublicKeyOrCertificate,
    ) -> std::result::Result<bool, Self::Error> {
        let public = match key {
            PublicKeyOrCertificate::PublicKey { key, .. } => key.clone(),
            PublicKeyOrCertificate::Certificate(cert) => {
                russh::keys::PublicKey::from(cert.public_key().clone())
            }
        };
        let blob = public.to_bytes().map_err(|_| russh::Error::UnknownKey)?;
        let ok = self
            .hooks
            .verify_host_key(&self.host, self.port, public.algorithm().as_str(), &blob)
            .await;
        if !ok {
            self.host_key_rejected.store(true, Ordering::SeqCst);
        }
        Ok(ok)
    }

    async fn disconnected(
        &mut self,
        reason: client::DisconnectReason<Self::Error>,
    ) -> std::result::Result<(), Self::Error> {
        self.alive.store(false, Ordering::SeqCst);
        match reason {
            client::DisconnectReason::ReceivedDisconnect(_) => Ok(()),
            client::DisconnectReason::Error(e) => Err(e),
        }
    }
}

pub fn map_ssh_error(e: russh::Error, host: &str) -> RemoteError {
    use russh::Error as E;
    let code = match &e {
        E::UnknownKey | E::WrongServerSig => ErrorCode::HostKey,
        E::NoAuthMethod | E::NotAuthenticated => ErrorCode::Auth,
        E::IO(io) => {
            return RemoteError::from(std::io::Error::new(io.kind(), format!("{host}: {io}")));
        }
        E::Disconnect
        | E::HUP
        | E::ConnectionTimeout
        | E::KeepaliveTimeout
        | E::InactivityTimeout
        | E::SendError => ErrorCode::Connection,
        _ => ErrorCode::Connection,
    };
    RemoteError::new(code, format!("{host}: {e}"))
}

fn map_sftp(e: SftpError, path: &str) -> RemoteError {
    match e {
        SftpError::Status(status) => {
            let code = match status.status_code {
                StatusCode::NoSuchFile => ErrorCode::NotFound,
                StatusCode::PermissionDenied => ErrorCode::Permission,
                StatusCode::OpUnsupported => ErrorCode::Unsupported,
                StatusCode::NoConnection | StatusCode::ConnectionLost => ErrorCode::Connection,
                _ => ErrorCode::Unknown,
            };
            let msg = if status.error_message.is_empty() {
                status.status_code.to_string()
            } else {
                status.error_message
            };
            RemoteError::new(code, msg).with_path(path)
        }
        SftpError::Timeout => RemoteError::connection("SFTP request timed out").with_path(path),
        other => RemoteError::new(ErrorCode::Connection, other.to_string()).with_path(path),
    }
}

/// Parses `user@host:port` (user and port optional).
pub fn parse_user_host_port(s: &str) -> (Option<String>, String, u16) {
    let (user, rest) = match s.rsplit_once('@') {
        Some((u, r)) => (Some(u.to_string()), r),
        None => (None, s),
    };
    let (host, port) = match rest.rsplit_once(':') {
        Some((h, p)) if !h.contains(':') || h.starts_with('[') => (
            h.trim_matches(['[', ']']).to_string(),
            p.parse().unwrap_or(22),
        ),
        _ => (rest.to_string(), 22),
    };
    (user, host, port)
}

/// Authentication material for one SSH login.
struct Login<'a> {
    user: String,
    auth: SftpAuth,
    password: Option<&'a str>,
    passphrase: Option<&'a str>,
    key_files: Vec<std::path::PathBuf>,
}

fn load_key(path: &Path, passphrase: Option<&str>) -> Result<PrivateKey> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        RemoteError::new(
            ErrorCode::Auth,
            format!("Cannot read private key {}: {e}", path.display()),
        )
    })?;
    match russh::keys::decode_secret_key(&text, passphrase) {
        Ok(k) => Ok(k),
        Err(russh::keys::Error::KeyIsEncrypted) => Err(RemoteError::new(
            ErrorCode::Passphrase,
            format!(
                "The private key {} is encrypted; a passphrase is required.",
                path.display()
            ),
        )),
        Err(e)
            if passphrase.is_some() && text.contains("ENCRYPTED")
                || text.contains("Encryption: aes") =>
        {
            Err(RemoteError::new(
                ErrorCode::Passphrase,
                format!("Wrong passphrase for {}: {e}", path.display()),
            ))
        }
        Err(e) => Err(RemoteError::new(
            ErrorCode::Auth,
            format!("Cannot load private key {}: {e}", path.display()),
        )),
    }
}

async fn auth_with_key(
    handle: &mut Handle<SshHandler>,
    user: &str,
    key: PrivateKey,
    host: &str,
) -> Result<bool> {
    let hash = if key.algorithm().is_rsa() {
        handle
            .best_supported_rsa_hash()
            .await
            .map_err(|e| map_ssh_error(e, host))?
            .flatten()
            .or(Some(HashAlg::Sha256))
    } else {
        None
    };
    let res = handle
        .authenticate_publickey(user, PrivateKeyWithHashAlg::new(Arc::new(key), hash))
        .await
        .map_err(|e| map_ssh_error(e, host))?;
    Ok(res.success())
}

#[cfg(unix)]
async fn auth_with_agent(handle: &mut Handle<SshHandler>, user: &str, host: &str) -> Result<bool> {
    let Ok(mut agent) = russh::keys::agent::client::AgentClient::connect_env().await else {
        return Ok(false);
    };
    auth_with_agent_client(handle, user, host, &mut agent).await
}

#[cfg(windows)]
async fn auth_with_agent(handle: &mut Handle<SshHandler>, user: &str, host: &str) -> Result<bool> {
    if let Ok(mut agent) =
        russh::keys::agent::client::AgentClient::connect_named_pipe(r"\\.\pipe\openssh-ssh-agent")
            .await
        && auth_with_agent_client(handle, user, host, &mut agent).await?
    {
        return Ok(true);
    }
    let Ok(mut agent) = russh::keys::agent::client::AgentClient::connect_pageant().await else {
        return Ok(false);
    };
    auth_with_agent_client(handle, user, host, &mut agent).await
}

async fn auth_with_agent_client<S>(
    handle: &mut Handle<SshHandler>,
    user: &str,
    host: &str,
    agent: &mut russh::keys::agent::client::AgentClient<S>,
) -> Result<bool>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let Ok(identities) = agent.request_identities().await else {
        return Ok(false);
    };
    for identity in identities {
        let public = identity.public_key().into_owned();
        let hash = if public.algorithm().is_rsa() {
            Some(HashAlg::Sha256)
        } else {
            None
        };
        match handle
            .authenticate_publickey_with(user, public, hash, agent)
            .await
        {
            Ok(r) if r.success() => return Ok(true),
            Ok(_) => continue,
            Err(e) => log::debug!("agent auth on {host} failed: {e:?}"),
        }
    }
    Ok(false)
}

/// Keyboard-interactive: prompts that ask for a password get the stored password, others go to the user (2FA).
async fn auth_keyboard_interactive(
    handle: &mut Handle<SshHandler>,
    user: &str,
    password: Option<&str>,
    hooks: &Arc<dyn ConnectHooks>,
    host: &str,
) -> Result<bool> {
    use russh::client::KeyboardInteractiveAuthResponse as R;
    let mut response = handle
        .authenticate_keyboard_interactive_start(user, None)
        .await
        .map_err(|e| map_ssh_error(e, host))?;
    let mut password_used = false;
    for _ in 0..8 {
        match response {
            R::Success => return Ok(true),
            R::Failure { .. } => return Ok(false),
            R::InfoRequest {
                name,
                instructions,
                prompts,
            } => {
                let answers = if prompts.is_empty() {
                    Vec::new()
                } else if let (Some(pw), false, true) = (
                    password,
                    password_used,
                    prompts.len() == 1
                        && !prompts[0].echo
                        && prompts[0].prompt.to_lowercase().contains("password"),
                ) {
                    password_used = true;
                    vec![pw.to_string()]
                } else {
                    let list: Vec<(String, bool)> =
                        prompts.iter().map(|p| (p.prompt.clone(), p.echo)).collect();
                    match hooks
                        .keyboard_interactive(&name, &instructions, &list)
                        .await
                    {
                        Some(a) => a,
                        None => return Err(RemoteError::cancelled()),
                    }
                };
                response = handle
                    .authenticate_keyboard_interactive_respond(answers)
                    .await
                    .map_err(|e| map_ssh_error(e, host))?;
            }
        }
    }
    Ok(false)
}

async fn authenticate(
    handle: &mut Handle<SshHandler>,
    login: &Login<'_>,
    hooks: &Arc<dyn ConnectHooks>,
    host: &str,
) -> Result<()> {
    let user = login.user.as_str();
    let mut ok: bool;
    let mut last_err: Option<RemoteError> = None;
    match login.auth {
        SftpAuth::Password => {
            if let Some(pw) = login.password {
                let res = handle
                    .authenticate_password(user, pw)
                    .await
                    .map_err(|e| map_ssh_error(e, host))?;
                ok = res.success();
                if !ok {
                    let remaining = match &res {
                        russh::client::AuthResult::Failure {
                            remaining_methods, ..
                        } => remaining_methods.contains(&MethodKind::KeyboardInteractive),
                        _ => false,
                    };
                    if remaining {
                        ok = auth_keyboard_interactive(handle, user, Some(pw), hooks, host).await?;
                    }
                }
            } else {
                ok = auth_keyboard_interactive(handle, user, None, hooks, host)
                    .await
                    .unwrap_or(false);
                if !ok {
                    return Err(RemoteError::new(
                        ErrorCode::Auth,
                        format!("A password is required for {user}@{host}."),
                    ));
                }
            }
        }
        SftpAuth::Key => {
            let path = login.key_files.first().ok_or_else(|| {
                RemoteError::new(ErrorCode::Config, "No private key file is set.")
            })?;
            let key = load_key(path, login.passphrase)?;
            ok = auth_with_key(handle, user, key, host).await?;
        }
        SftpAuth::Agent => ok = auth_with_agent(handle, user, host).await?,
        SftpAuth::Openssh => {
            ok = auth_with_agent(handle, user, host).await?;
            for path in &login.key_files {
                if ok {
                    break;
                }
                if !path.is_file() {
                    continue;
                }
                match load_key(path, login.passphrase) {
                    Ok(key) => ok = auth_with_key(handle, user, key, host).await?,
                    Err(e) => last_err = Some(e),
                }
            }
            if !ok && let Some(pw) = login.password {
                ok = handle
                    .authenticate_password(user, pw)
                    .await
                    .map_err(|e| map_ssh_error(e, host))?
                    .success();
            }
        }
    }
    if ok {
        return Ok(());
    }
    Err(last_err.unwrap_or_else(|| {
        RemoteError::new(
            ErrorCode::Auth,
            format!("Authentication failed for {user}@{host}."),
        )
    }))
}

fn ssh_config(params: &ConnectParams) -> Arc<client::Config> {
    let keepalive = params.server.keep_alive_seconds.unwrap_or(30);
    Arc::new(client::Config {
        keepalive_interval: (keepalive > 0).then(|| Duration::from_secs(keepalive as u64)),
        keepalive_max: 3,
        nodelay: true,
        ..Default::default()
    })
}

/// An authenticated SSH session (built-in backend), possibly through a jump host.
pub struct SshSession {
    pub handle: Handle<SshHandler>,
    jump: Option<Handle<SshHandler>>,
    pub alive: Arc<AtomicBool>,
}

impl SshSession {
    pub async fn close(&mut self) {
        let _ = self
            .handle
            .disconnect(Disconnect::ByApplication, "", "en")
            .await;
        if let Some(j) = &self.jump {
            let _ = j.disconnect(Disconnect::ByApplication, "", "en").await;
        }
        self.alive.store(false, Ordering::SeqCst);
    }
}

/// Target host, port and login after applying `~/.ssh/config` when `auth` is `openssh`.
pub struct SshTarget {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub key_files: Vec<std::path::PathBuf>,
    pub jump: Option<String>,
}

pub async fn resolve_target(params: &ConnectParams) -> SshTarget {
    let s = &params.server;
    let mut target = SshTarget {
        host: s.host.clone(),
        port: s.port_or_default(),
        user: s.username.clone(),
        key_files: s
            .private_key_path
            .iter()
            .map(|p| crate::dirs::expand_home(p))
            .collect(),
        jump: s.jump_host.clone().filter(|j| !j.trim().is_empty()),
    };
    if s.auth == Some(SftpAuth::Openssh) {
        let alias = s
            .ssh_config_host
            .clone()
            .filter(|a| !a.is_empty())
            .unwrap_or_else(|| s.host.clone());
        let r = crate::ssh_config::resolve(&alias).await;
        target.host = r.hostname;
        target.port = if s.port.is_some() && s.port != Some(22) {
            s.port_or_default()
        } else {
            r.port
        };
        if target.user.is_empty() {
            target.user = r.user.unwrap_or_default();
        }
        target.key_files.extend(r.identity_files);
        if target.jump.is_none() {
            target.jump = r
                .proxy_jump
                .map(|j| j.split(',').next().unwrap_or_default().to_string());
        }
    }
    if target.user.is_empty() {
        target.user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_default();
    }
    target
}

/// Connects and authenticates (built-in backend).
pub async fn open_session(
    params: &ConnectParams,
    hooks: Arc<dyn ConnectHooks>,
) -> Result<SshSession> {
    let target = resolve_target(params).await;
    let login = Login {
        user: target.user.clone(),
        auth: params.server.auth.unwrap_or_default(),
        password: params.password.as_deref(),
        passphrase: params.passphrase.as_deref(),
        key_files: target.key_files.clone(),
    };
    let config = ssh_config(params);
    let alive = Arc::new(AtomicBool::new(true));
    let rejected = Arc::new(AtomicBool::new(false));
    let handler = |host: &str, port: u16| SshHandler {
        hooks: hooks.clone(),
        host: host.to_string(),
        port,
        alive: alive.clone(),
        host_key_rejected: rejected.clone(),
    };
    let timeout = params.timeout;
    let host_key_error = |host: &str| {
        RemoteError::new(
            ErrorCode::HostKey,
            format!("The host key of {host} was not accepted."),
        )
    };

    let mut jump_handle = None;
    let mut handle = if let Some(jump) = &target.jump {
        let (juser, jhost, jport) = parse_user_host_port(jump);
        let mut jh = tokio::time::timeout(
            timeout,
            client::connect(
                config.clone(),
                (jhost.as_str(), jport),
                handler(&jhost, jport),
            ),
        )
        .await
        .map_err(|_| RemoteError::connection(format!("Timed out connecting to jump host {jhost}")))?
        .map_err(|e| {
            if rejected.load(Ordering::SeqCst) {
                host_key_error(&jhost)
            } else {
                map_ssh_error(e, &jhost)
            }
        })?;
        // Jump host login: agent and the same key files, then the same password.
        let jlogin = Login {
            user: juser.unwrap_or_else(|| target.user.clone()),
            auth: SftpAuth::Openssh,
            password: params.password.as_deref(),
            passphrase: params.passphrase.as_deref(),
            key_files: target.key_files.clone(),
        };
        authenticate(&mut jh, &jlogin, &hooks, &jhost).await?;
        let channel = jh
            .channel_open_direct_tcpip(target.host.clone(), target.port as u32, "127.0.0.1", 0)
            .await
            .map_err(|e| map_ssh_error(e, &jhost))?;
        let h = tokio::time::timeout(
            timeout,
            client::connect_stream(
                config.clone(),
                channel.into_stream(),
                handler(&target.host, target.port),
            ),
        )
        .await
        .map_err(|_| RemoteError::connection(format!("Timed out connecting to {}", target.host)))?
        .map_err(|e| {
            if rejected.load(Ordering::SeqCst) {
                host_key_error(&target.host)
            } else {
                map_ssh_error(e, &target.host)
            }
        })?;
        jump_handle = Some(jh);
        h
    } else {
        tokio::time::timeout(
            timeout,
            client::connect(
                config,
                (target.host.as_str(), target.port),
                handler(&target.host, target.port),
            ),
        )
        .await
        .map_err(|_| {
            RemoteError::connection(format!(
                "Timed out connecting to {}:{}",
                target.host, target.port
            ))
        })?
        .map_err(|e| {
            if rejected.load(Ordering::SeqCst) {
                host_key_error(&target.host)
            } else {
                map_ssh_error(e, &target.host)
            }
        })?
    };
    authenticate(&mut handle, &login, &hooks, &target.host).await?;
    Ok(SshSession {
        handle,
        jump: jump_handle,
        alive,
    })
}

/// Runs a command over the built-in session.
pub async fn exec_on(handle: &Handle<SshHandler>, command: &str, host: &str) -> Result<ExecOutput> {
    let mut channel = handle
        .channel_open_session()
        .await
        .map_err(|e| map_ssh_error(e, host))?;
    channel
        .exec(true, command)
        .await
        .map_err(|e| map_ssh_error(e, host))?;
    let mut out = ExecOutput::default();
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
            ChannelMsg::ExtendedData { data, .. } => stderr.extend_from_slice(&data),
            ChannelMsg::ExitStatus { exit_status } => out.code = exit_status,
            _ => {}
        }
    }
    out.stdout = String::from_utf8_lossy(&stdout).into_owned();
    out.stderr = String::from_utf8_lossy(&stderr).into_owned();
    Ok(out)
}

/// Arguments for the system `ssh` matching a server (OpenSSH backend, SSH sessions, rsync).
pub fn openssh_args(params: &ConnectParams) -> Vec<String> {
    let s = &params.server;
    let mut args = vec!["-o".to_string(), "BatchMode=no".into()];
    let policy = match s.strict_host_key_checking.unwrap_or_default() {
        crate::config::HostKeyPolicy::Yes => "yes",
        crate::config::HostKeyPolicy::No => "no",
        // Without a terminal `ask` can't prompt; new keys are learned, changed keys still fail.
        _ => "accept-new",
    };
    args.extend(["-o".into(), format!("StrictHostKeyChecking={policy}")]);
    if let Some(k) = s.keep_alive_seconds.filter(|k| *k > 0) {
        args.extend(["-o".into(), format!("ServerAliveInterval={k}")]);
    }
    args.extend([
        "-o".into(),
        format!("ConnectTimeout={}", params.timeout.as_secs().max(1)),
    ]);
    for f in &params.known_hosts_files {
        args.extend([
            "-o".into(),
            format!("UserKnownHostsFile=~/.ssh/known_hosts {}", f.display()),
        ]);
    }
    if s.auth == Some(SftpAuth::Openssh) {
        args.push(
            s.ssh_config_host
                .clone()
                .filter(|h| !h.is_empty())
                .unwrap_or_else(|| s.host.clone()),
        );
        return args;
    }
    if let Some(port) = s.port.filter(|p| *p != 22) {
        args.extend(["-p".into(), port.to_string()]);
    }
    if let (Some(SftpAuth::Key), Some(key)) = (s.auth, &s.private_key_path) {
        args.extend([
            "-i".into(),
            crate::dirs::expand_home(key).display().to_string(),
            "-o".into(),
            "IdentitiesOnly=yes".into(),
        ]);
    }
    if let Some(j) = s.jump_host.as_ref().filter(|j| !j.is_empty()) {
        args.extend(["-J".into(), j.clone()]);
    }
    let dest = if s.username.is_empty() {
        s.host.clone()
    } else {
        format!("{}@{}", s.username, s.host)
    };
    args.push(dest);
    args
}

/// Environment that lets `ssh` read a password / passphrase without a terminal: `SSH_ASKPASS` points at
/// `iremote askpass`, which prints `IREMOTE_ASKPASS_SECRET`.
pub fn askpass_env(secret: Option<&str>) -> Vec<(String, String)> {
    let Some(secret) = secret else {
        return Vec::new();
    };
    let Ok(exe) = std::env::current_exe() else {
        return Vec::new();
    };
    vec![
        ("SSH_ASKPASS".into(), exe.display().to_string()),
        ("SSH_ASKPASS_REQUIRE".into(), "force".into()),
        ("IREMOTE_ASKPASS".into(), "1".into()),
        ("IREMOTE_ASKPASS_SECRET".into(), secret.to_string()),
        (
            "DISPLAY".into(),
            std::env::var("DISPLAY").unwrap_or_else(|_| ":0".into()),
        ),
    ]
}

enum Transport {
    Builtin(SshSession),
    Openssh {
        child: tokio::process::Child,
        args: Vec<String>,
        env: Vec<(String, String)>,
    },
}

pub struct SftpClient {
    sftp: SftpSession,
    transport: Transport,
    host: String,
    alive: Arc<AtomicBool>,
}

impl SftpClient {
    pub async fn connect(params: &ConnectParams, hooks: Arc<dyn ConnectHooks>) -> Result<Self> {
        match params.server.ssh_backend.unwrap_or_default() {
            SshBackend::Builtin => {
                let session = open_session(params, hooks).await?;
                let host = params.server.host.clone();
                let channel = session
                    .handle
                    .channel_open_session()
                    .await
                    .map_err(|e| map_ssh_error(e, &host))?;
                channel
                    .request_subsystem(true, "sftp")
                    .await
                    .map_err(|e| map_ssh_error(e, &host))?;
                let sftp =
                    tokio::time::timeout(params.timeout, SftpSession::new(channel.into_stream()))
                        .await
                        .map_err(|_| {
                            RemoteError::connection("Timed out starting the SFTP subsystem")
                        })?
                        .map_err(|e| map_sftp(e, "/"))?;
                let alive = session.alive.clone();
                Ok(Self {
                    sftp,
                    transport: Transport::Builtin(session),
                    host,
                    alive,
                })
            }
            SshBackend::Openssh => Self::connect_openssh(params).await,
        }
    }

    async fn connect_openssh(params: &ConnectParams) -> Result<Self> {
        let args = openssh_args(params);
        let env = askpass_env(params.password.as_deref().or(params.passphrase.as_deref()));
        let mut cmd = tokio::process::Command::new("ssh");
        cmd.args(["-s"])
            .args(&args[..args.len() - 1])
            .arg(&args[args.len() - 1])
            .arg("sftp");
        cmd.envs(env.iter().cloned())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped()).kill_on_drop(true);
        let mut child = cmd
            .spawn()
            .map_err(|e| RemoteError::connection(format!("Cannot start ssh: {e}")))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| RemoteError::connection("ssh has no stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| RemoteError::connection("ssh has no stdout"))?;
        let stream = tokio::io::join(stdout, stdin);
        match tokio::time::timeout(params.timeout * 3, SftpSession::new(stream)).await {
            Ok(Ok(sftp)) => Ok(Self {
                sftp,
                transport: Transport::Openssh { child, args, env },
                host: params.server.host.clone(),
                alive: Arc::new(AtomicBool::new(true)),
            }),
            other => {
                let _ = child.start_kill();
                let mut err = String::new();
                if let Some(mut e) = child.stderr.take() {
                    let _ =
                        tokio::time::timeout(Duration::from_secs(2), e.read_to_string(&mut err))
                            .await;
                }
                let err = err.trim();
                let code = if err.contains("Permission denied") {
                    ErrorCode::Auth
                } else if err.contains("Host key verification failed")
                    || err.contains("HOST IDENTIFICATION HAS CHANGED")
                {
                    ErrorCode::HostKey
                } else {
                    ErrorCode::Connection
                };
                let detail = match other {
                    Ok(Err(e)) => e.to_string(),
                    _ => "timed out".into(),
                };
                Err(RemoteError::new(
                    code,
                    if err.is_empty() {
                        format!("ssh: {detail}")
                    } else {
                        err.to_string()
                    },
                ))
            }
        }
    }

    fn mark(&self, e: RemoteError) -> RemoteError {
        if e.code == ErrorCode::Connection {
            self.alive.store(false, Ordering::SeqCst);
        }
        e
    }

    async fn lstat(&self, path: &str) -> Result<Option<RemoteStat>> {
        let meta = match self.sftp.symlink_metadata(path).await {
            Ok(m) => m,
            Err(e) => {
                let e = map_sftp(e, path);
                return if e.code == ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(self.mark(e))
                };
            }
        };
        let mut stat = to_stat(&meta);
        if meta.is_symlink()
            && let Ok(target) = self.sftp.metadata(path).await
        {
            let t = to_stat(&target);
            stat.target_kind = Some(t.kind);
            stat.size = t.size;
            stat.mtime = t.mtime;
        }
        Ok(Some(stat))
    }
}

fn to_stat(m: &FileAttributes) -> RemoteStat {
    let kind = if m.is_symlink() {
        EntryType::Symlink
    } else if m.is_dir() {
        EntryType::Dir
    } else {
        EntryType::File
    };
    RemoteStat {
        kind,
        target_kind: None,
        size: if kind == EntryType::Dir {
            0
        } else {
            m.size.unwrap_or(0)
        },
        mtime: m.mtime.map(|t| t as i64 * 1000).unwrap_or(0),
        mode: m.permissions.map(|p| p & 0o7777),
    }
}

#[async_trait]
impl RemoteClient for SftpClient {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            chmod: true,
            set_mtime: true,
            exec: true,
            reliable_mtime: true,
        }
    }

    fn is_connected(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
            && match &self.transport {
                Transport::Builtin(s) => !s.handle.is_closed(),
                Transport::Openssh { .. } => true,
            }
    }

    async fn close(&mut self) {
        let _ = self.sftp.close().await;
        match &mut self.transport {
            Transport::Builtin(s) => s.close().await,
            Transport::Openssh { child, .. } => {
                let _ = child.start_kill();
            }
        }
        self.alive.store(false, Ordering::SeqCst);
    }

    async fn home_dir(&mut self) -> Result<String> {
        self.sftp
            .canonicalize(".")
            .await
            .map_err(|e| self.mark(map_sftp(e, ".")))
    }

    async fn list(&mut self, path: &str) -> Result<Vec<RemoteEntry>> {
        let rd = self
            .sftp
            .read_dir(path)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))?;
        let mut out = Vec::new();
        for entry in rd {
            let name = entry.file_name();
            if name == "." || name == ".." {
                continue;
            }
            let meta = entry.metadata();
            let mut stat = to_stat(&meta);
            if meta.is_symlink()
                && let Ok(t) = self
                    .sftp
                    .metadata(crate::mapper::join_remote(path, &name))
                    .await
            {
                let t = to_stat(&t);
                stat.target_kind = Some(t.kind);
                stat.size = t.size;
            }
            out.push(RemoteEntry { name, stat });
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    async fn stat(&mut self, path: &str) -> Result<Option<RemoteStat>> {
        self.lstat(path).await
    }

    async fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        self.sftp
            .read(path)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        let mut f = self
            .sftp
            .create(path)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))?;
        f.write_all(data)
            .await
            .map_err(|e| self.mark(RemoteError::from(e).with_path(path)))?;
        f.shutdown()
            .await
            .map_err(|e| self.mark(RemoteError::from(e).with_path(path)))?;
        Ok(())
    }

    async fn upload(&mut self, local: &Path, remote: &str, progress: Progress<'_>) -> Result<()> {
        let mut src = tokio::fs::File::open(local)
            .await
            .map_err(|e| RemoteError::from(e).with_path(local.display().to_string()))?;
        let total = src.metadata().await.map(|m| m.len()).unwrap_or(0);
        let mut dst = self
            .sftp
            .create(remote)
            .await
            .map_err(|e| self.mark(map_sftp(e, remote)))?;
        let mut buf = vec![0u8; CHUNK];
        let mut done = 0u64;
        loop {
            let n = src
                .read(&mut buf)
                .await
                .map_err(|e| RemoteError::from(e).with_path(local.display().to_string()))?;
            if n == 0 {
                break;
            }
            dst.write_all(&buf[..n])
                .await
                .map_err(|e| self.mark(RemoteError::from(e).with_path(remote)))?;
            done += n as u64;
            progress(done, total);
        }
        dst.shutdown()
            .await
            .map_err(|e| self.mark(RemoteError::from(e).with_path(remote)))?;
        Ok(())
    }

    async fn download(&mut self, remote: &str, local: &Path, progress: Progress<'_>) -> Result<()> {
        let total = self.lstat(remote).await?.map(|s| s.size).unwrap_or(0);
        let mut src = self
            .sftp
            .open(remote)
            .await
            .map_err(|e| self.mark(map_sftp(e, remote)))?;
        let mut dst = tokio::fs::File::create(local)
            .await
            .map_err(|e| RemoteError::from(e).with_path(local.display().to_string()))?;
        let mut buf = vec![0u8; CHUNK];
        let mut done = 0u64;
        loop {
            let n = src
                .read(&mut buf)
                .await
                .map_err(|e| self.mark(RemoteError::from(e).with_path(remote)))?;
            if n == 0 {
                break;
            }
            dst.write_all(&buf[..n]).await?;
            done += n as u64;
            progress(done, total);
        }
        dst.flush().await?;
        Ok(())
    }

    async fn mkdirp(&mut self, path: &str) -> Result<()> {
        let norm = crate::mapper::norm_remote(path);
        let mut missing = Vec::new();
        let mut cur = norm;
        loop {
            match self.lstat(&cur).await? {
                Some(s) if s.is_dir() => break,
                Some(_) => {
                    return Err(RemoteError::new(
                        ErrorCode::Exists,
                        "A file with this name exists",
                    )
                    .with_path(cur));
                }
                None => {
                    missing.push(cur.clone());
                    if cur == "/" {
                        break;
                    }
                    cur = crate::mapper::remote_parent(&cur);
                }
            }
        }
        for dir in missing.into_iter().rev() {
            if let Err(e) = self.sftp.create_dir(dir.clone()).await {
                // Another connection may have created it meanwhile.
                if !self.lstat(&dir).await?.is_some_and(|s| s.is_dir()) {
                    return Err(self.mark(map_sftp(e, &dir)));
                }
            }
        }
        Ok(())
    }

    async fn delete_file(&mut self, path: &str) -> Result<()> {
        self.sftp
            .remove_file(path)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))
    }

    async fn delete_dir(&mut self, path: &str, recursive: bool) -> Result<()> {
        if recursive {
            return delete_tree(self, path).await;
        }
        self.sftp
            .remove_dir(path)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))
    }

    async fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        if let Err(e) = self.sftp.rename(from, to).await {
            // SFTP v3 rename fails when the target exists; replace it like other protocols do.
            if self.lstat(from).await?.is_some()
                && self.lstat(to).await?.is_some_and(|s| !s.is_dir())
            {
                self.delete_file(to).await?;
                return self
                    .sftp
                    .rename(from, to)
                    .await
                    .map_err(|e| self.mark(map_sftp(e, from)));
            }
            return Err(self.mark(map_sftp(e, from)));
        }
        Ok(())
    }

    async fn chmod(&mut self, path: &str, mode: u32) -> Result<()> {
        let attrs = FileAttributes {
            permissions: Some(mode),
            ..FileAttributes::empty()
        };
        self.sftp
            .set_metadata(path, attrs)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))
    }

    async fn set_mtime(&mut self, path: &str, mtime_ms: i64) -> Result<()> {
        let secs = (mtime_ms.max(0) / 1000) as u32;
        let attrs = FileAttributes {
            atime: Some(secs),
            mtime: Some(secs),
            ..FileAttributes::empty()
        };
        self.sftp
            .set_metadata(path, attrs)
            .await
            .map_err(|e| self.mark(map_sftp(e, path)))
    }

    async fn exec(&mut self, command: &str) -> Result<ExecOutput> {
        match &self.transport {
            Transport::Builtin(s) => exec_on(&s.handle, command, &self.host)
                .await
                .map_err(|e| self.mark(e)),
            Transport::Openssh { args, env, .. } => {
                let out = tokio::process::Command::new("ssh")
                    .args(args)
                    .arg(command)
                    .envs(env.iter().cloned())
                    .stdin(std::process::Stdio::null())
                    .output()
                    .await
                    .map_err(|e| RemoteError::connection(format!("Cannot start ssh: {e}")))?;
                Ok(ExecOutput {
                    code: out.status.code().unwrap_or(255) as u32,
                    stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_jump_hosts() {
        assert_eq!(
            parse_user_host_port("deploy@bastion:2200"),
            (Some("deploy".into()), "bastion".into(), 2200)
        );
        assert_eq!(
            parse_user_host_port("bastion"),
            (None, "bastion".into(), 22)
        );
        assert_eq!(
            parse_user_host_port("u@[::1]:22"),
            (Some("u".into()), "::1".into(), 22)
        );
    }

    #[test]
    fn openssh_args_follow_config() {
        let mut s = crate::config::Server::new("p", "P", crate::config::ServerType::Sftp);
        s.host = "example.com".into();
        s.username = "deploy".into();
        s.port = Some(2222);
        s.auth = Some(SftpAuth::Key);
        s.private_key_path = Some("/k/id".into());
        s.jump_host = Some("bastion".into());
        let args = openssh_args(&ConnectParams::new(s));
        let joined = args.join(" ");
        assert!(joined.contains("-p 2222"));
        assert!(joined.contains("-i /k/id"));
        assert!(joined.contains("-J bastion"));
        assert!(joined.contains("StrictHostKeyChecking=accept-new"));
        assert_eq!(args.last().unwrap(), "deploy@example.com");
    }
}
