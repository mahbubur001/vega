//! Owns one [`ConnectionPool`] per server and the connect flow around it: credentials from the keychain or the
//! user (with retry on a wrong password), host-key policy, untrusted-certificate confirmation and pinning. Pools are
//! rebuilt when connection-relevant settings change.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::config::{Server, ServerType, SftpAuth, WebdavAuth};
use crate::credentials::{Credentials, passphrase_account, password_account};
use crate::error::{ErrorCode, RemoteError, Result};
use crate::known_hosts::{HostKeyTrust, Verdict};
use crate::pool::{ClientFactory, ConnectionPool};
use crate::prompt::{CertAnswer, Prompter, SecretKind, SecretRequest};
use crate::protocol::{self, ConnectHooks, ConnectParams, RemoteClient};

#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    pub timeout: Duration,
    pub idle_close: Duration,
    pub known_hosts_files: Vec<PathBuf>,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(20),
            idle_close: Duration::from_secs(300),
            known_hosts_files: Vec::new(),
        }
    }
}

/// Called when the user pins a certificate: `(server id, fingerprint)`. The engine writes it to the config.
pub type PinCertificate = Arc<dyn Fn(&str, &str) + Send + Sync>;

pub struct ConnectContext {
    pub credentials: Arc<Credentials>,
    pub prompter: Arc<dyn Prompter>,
    pub trust: Arc<HostKeyTrust>,
    pub settings: RwLock<ConnectionSettings>,
    session_certs: Mutex<HashMap<String, Vec<String>>>,
    pin: RwLock<Option<PinCertificate>>,
}

impl ConnectContext {
    pub fn new(
        credentials: Arc<Credentials>,
        prompter: Arc<dyn Prompter>,
        trust: Arc<HostKeyTrust>,
    ) -> Self {
        Self {
            credentials,
            prompter,
            trust,
            settings: RwLock::new(ConnectionSettings::default()),
            session_certs: Mutex::default(),
            pin: RwLock::new(None),
        }
    }

    pub fn set_pin_handler(&self, f: PinCertificate) {
        if let Ok(mut p) = self.pin.write() {
            *p = Some(f);
        }
    }

    fn settings(&self) -> ConnectionSettings {
        self.settings.read().map(|s| s.clone()).unwrap_or_default()
    }
}

struct HostKeyHooks {
    ctx: Arc<ConnectContext>,
    policy: crate::config::HostKeyPolicy,
}

#[async_trait]
impl ConnectHooks for HostKeyHooks {
    async fn verify_host_key(
        &self,
        host: &str,
        port: u16,
        key_type: &str,
        key_blob: &[u8],
    ) -> bool {
        match self
            .ctx
            .trust
            .check(self.policy, host, port, key_type, key_blob)
        {
            Verdict::Accept => true,
            Verdict::Reject(msg) => {
                self.ctx.prompter.notify(crate::prompt::Level::Error, &msg);
                false
            }
            Verdict::Ask(q) => {
                let answer = self.ctx.prompter.host_key(&q).await;
                self.ctx.trust.apply_answer(&q, key_blob, answer)
            }
        }
    }

    async fn keyboard_interactive(
        &self,
        name: &str,
        instructions: &str,
        prompts: &[(String, bool)],
    ) -> Option<Vec<String>> {
        self.ctx
            .prompter
            .keyboard_interactive(name, instructions, prompts)
            .await
    }
}

fn needs_password_upfront(s: &Server) -> bool {
    match s.kind {
        ServerType::Sftp => s.auth.unwrap_or_default() == SftpAuth::Password,
        ServerType::Ftp | ServerType::Ftps | ServerType::FtpsImplicit => !s.anonymous,
        ServerType::Webdav => s.webdav_auth.unwrap_or_default() != WebdavAuth::None,
        _ => false,
    }
}

fn target_label(s: &Server) -> String {
    if s.kind == ServerType::Webdav {
        format!("{} at {}", s.username, s.host)
    } else {
        format!("{}@{}", s.username, s.host)
    }
}

/// Opens a connection, asking for whatever is missing. Shared by pools and "Test connection".
pub async fn connect_interactive(
    server: &Server,
    ctx: &Arc<ConnectContext>,
) -> Result<Box<dyn RemoteClient>> {
    let settings = ctx.settings();
    let pw_account = password_account(server);
    let pp_account = server.private_key_path.as_deref().map(passphrase_account);
    let mut password = ctx.credentials.get(&pw_account).await;
    let mut passphrase = match &pp_account {
        Some(a) => ctx.credentials.get(a).await,
        None => None,
    };
    let mut want_password = needs_password_upfront(server);
    let mut last_error: Option<String> = None;

    for _attempt in 0..4 {
        if want_password && password.is_none() {
            let req = SecretRequest {
                kind: SecretKind::Password,
                server_name: server.name.clone(),
                target: target_label(server),
                error: last_error.clone(),
            };
            let answer = ctx.prompter.secret(&req).await.ok_or_else(|| {
                RemoteError::new(
                    ErrorCode::Auth,
                    last_error.clone().unwrap_or_else(|| {
                        format!("A password is required for {}.", target_label(server))
                    }),
                )
            })?;
            ctx.credentials
                .set(&pw_account, &answer.value, answer.save)
                .await;
            password = Some(answer.value);
        }
        let mut params = ConnectParams::new(server.clone());
        params.timeout = settings.timeout;
        params.known_hosts_files = settings.known_hosts_files.clone();
        params.password = password.clone();
        params.passphrase = passphrase.clone();
        params.accepted_certs = server
            .trusted_certificate_fingerprint
            .iter()
            .cloned()
            .collect();
        if let Some(extra) = ctx
            .session_certs
            .lock()
            .ok()
            .and_then(|m| m.get(&server.host).cloned())
        {
            params.accepted_certs.extend(extra);
        }
        let hooks = Arc::new(HostKeyHooks {
            ctx: ctx.clone(),
            policy: server.strict_host_key_checking.unwrap_or_default(),
        });
        match protocol::connect(&params, hooks).await {
            Ok(c) => return Ok(c),
            Err(e) if e.code == ErrorCode::Auth => {
                // A stored password that no longer works is forgotten; ask again with the error shown.
                ctx.credentials.delete(&pw_account).await;
                password = None;
                want_password = true;
                last_error = Some(e.to_string());
            }
            Err(e) if e.code == ErrorCode::Passphrase => {
                let Some(account) = &pp_account else {
                    return Err(e);
                };
                ctx.credentials.delete(account).await;
                let req = SecretRequest {
                    kind: SecretKind::Passphrase,
                    server_name: server.name.clone(),
                    target: server.private_key_path.clone().unwrap_or_default(),
                    error: passphrase.is_some().then(|| e.to_string()),
                };
                let answer = ctx.prompter.secret(&req).await.ok_or(e)?;
                ctx.credentials
                    .set(account, &answer.value, answer.save)
                    .await;
                passphrase = Some(answer.value);
            }
            Err(e) if e.code == ErrorCode::Certificate => {
                let Some(fp) = crate::tls::fingerprint_from_error(&e) else {
                    return Err(e);
                };
                if !server.allow_untrusted_certificate {
                    return Err(RemoteError::new(
                        ErrorCode::Certificate,
                        format!(
                            "{e}. Enable \"Allow untrusted certificates\" for this server to review and trust it."
                        ),
                    ));
                }
                let reason = e
                    .message
                    .split(": ")
                    .nth(1)
                    .unwrap_or(&e.message)
                    .to_string();
                match ctx.prompter.certificate(&server.host, &fp, &reason).await {
                    CertAnswer::Reject => return Err(e),
                    answer => {
                        if let Ok(mut m) = ctx.session_certs.lock() {
                            m.entry(server.host.clone()).or_default().push(fp.clone());
                        }
                        if answer == CertAnswer::Pin
                            && let Some(pin) = ctx.pin.read().ok().and_then(|p| p.clone())
                        {
                            pin(&server.id, &fp);
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
    Err(RemoteError::new(
        ErrorCode::Auth,
        last_error.unwrap_or_else(|| "Too many failed attempts.".into()),
    ))
}

struct ServerFactory {
    server: Server,
    ctx: Arc<ConnectContext>,
}

#[async_trait]
impl ClientFactory for ServerFactory {
    async fn create(&self) -> Result<Box<dyn RemoteClient>> {
        connect_interactive(&self.server, &self.ctx).await
    }
}

/// Connection-relevant part of a server definition (mappings and display fields excluded).
fn connection_key(s: &Server) -> String {
    let mut c = s.clone();
    c.name.clear();
    c.mappings.clear();
    c.excluded_local.clear();
    c.excluded_remote.clear();
    c.web_url.clear();
    serde_json::to_string(&c).unwrap_or_default()
}

pub struct ServerManager {
    pub ctx: Arc<ConnectContext>,
    pools: Mutex<HashMap<String, (String, ConnectionPool)>>,
}

impl ServerManager {
    pub fn new(ctx: Arc<ConnectContext>) -> Self {
        Self {
            ctx,
            pools: Mutex::default(),
        }
    }

    pub fn pool(&self, server: &Server) -> ConnectionPool {
        let key = connection_key(server);
        let mut pools = self.pools.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((k, pool)) = pools.get(&server.id) {
            if *k == key {
                return pool.clone();
            }
            let old = pool.clone();
            tokio::spawn(async move { old.close_all().await });
        }
        let factory = Arc::new(ServerFactory {
            server: server.clone(),
            ctx: self.ctx.clone(),
        });
        let pool = ConnectionPool::new(
            factory,
            server.max_connections_or_default(),
            self.ctx.settings().idle_close,
        );
        pools.insert(server.id.clone(), (key, pool.clone()));
        pool
    }

    pub async fn disconnect_all(&self) {
        let pools: Vec<ConnectionPool> = self
            .pools
            .lock()
            .map(|p| p.values().map(|(_, p)| p.clone()).collect())
            .unwrap_or_default();
        for p in pools {
            p.close_all().await;
        }
    }

    pub async fn reap(&self) {
        let pools: Vec<ConnectionPool> = self
            .pools
            .lock()
            .map(|p| p.values().map(|(_, p)| p.clone()).collect())
            .unwrap_or_default();
        for p in pools {
            p.reap().await;
        }
    }

    /// Forgets saved password / passphrase and trusted host key for a server.
    pub async fn forget_credentials(&self, server: &Server) {
        self.ctx.credentials.delete(&password_account(server)).await;
        if let Some(k) = &server.private_key_path {
            self.ctx.credentials.delete(&passphrase_account(k)).await;
        }
        if server.kind == ServerType::Sftp {
            self.ctx
                .trust
                .forget(&server.host, server.port_or_default());
        }
        if let Ok(mut pools) = self.pools.lock() {
            pools.remove(&server.id);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StepState {
    Ok,
    Warning,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStep {
    pub label: String,
    pub state: StepState,
    pub detail: String,
    pub millis: u64,
}

/// "Test connection" with one line per step: resolve, TCP, login (host key + auth), root path readable/writable.
pub async fn test_connection(server: &Server, ctx: &Arc<ConnectContext>) -> Vec<TestStep> {
    let mut steps = Vec::new();
    let step = |label: &str, state, detail: String, started: Instant| TestStep {
        label: label.into(),
        state,
        detail,
        millis: started.elapsed().as_millis() as u64,
    };
    let timeout = ctx.settings().timeout;
    if server.kind.needs_host() {
        let (host, port) = if server.kind == ServerType::Webdav {
            match url::Url::parse(&server.host) {
                Ok(u) => (
                    u.host_str().unwrap_or_default().to_string(),
                    u.port_or_known_default().unwrap_or(80),
                ),
                Err(e) => {
                    steps.push(step(
                        "Parse URL",
                        StepState::Failed,
                        e.to_string(),
                        Instant::now(),
                    ));
                    return steps;
                }
            }
        } else {
            (server.host.clone(), server.port_or_default())
        };
        let skip_dns = server.kind == ServerType::Sftp && server.auth == Some(SftpAuth::Openssh);
        if !skip_dns {
            let t = Instant::now();
            match tokio::time::timeout(timeout, tokio::net::lookup_host((host.as_str(), port)))
                .await
            {
                Ok(Ok(mut addrs)) => {
                    let addr = addrs.next();
                    steps.push(step(
                        "Resolve host",
                        StepState::Ok,
                        format!(
                            "{host} → {}",
                            addr.map(|a| a.ip().to_string()).unwrap_or_default()
                        ),
                        t,
                    ));
                    let t = Instant::now();
                    if let Some(a) = addr {
                        match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(a)).await
                        {
                            Ok(Ok(_)) => steps.push(step(
                                "Connect",
                                StepState::Ok,
                                format!("TCP port {port} is open"),
                                t,
                            )),
                            Ok(Err(e)) => {
                                steps.push(step("Connect", StepState::Failed, e.to_string(), t));
                                return steps;
                            }
                            Err(_) => {
                                steps.push(step(
                                    "Connect",
                                    StepState::Failed,
                                    "Timed out".into(),
                                    t,
                                ));
                                return steps;
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    steps.push(step("Resolve host", StepState::Failed, e.to_string(), t));
                    return steps;
                }
                Err(_) => {
                    steps.push(step(
                        "Resolve host",
                        StepState::Failed,
                        "Timed out".into(),
                        t,
                    ));
                    return steps;
                }
            }
        }
    }
    let t = Instant::now();
    let mut client = match connect_interactive(server, ctx).await {
        Ok(c) => {
            let what = match server.kind {
                ServerType::Sftp => "Host key verified, logged in",
                ServerType::Local => "Folder is accessible",
                _ => "Logged in",
            };
            steps.push(step(
                "Log in",
                StepState::Ok,
                format!(
                    "{what} as {}",
                    if server.username.is_empty() {
                        "default user"
                    } else {
                        &server.username
                    }
                ),
                t,
            ));
            c
        }
        Err(e) => {
            steps.push(step("Log in", StepState::Failed, e.to_string(), t));
            return steps;
        }
    };
    let root = if server.root_path.is_empty() {
        "/".to_string()
    } else {
        server.root_path.clone()
    };
    let t = Instant::now();
    match client.list(&root).await {
        Ok(entries) => steps.push(step(
            "Root path",
            StepState::Ok,
            format!("{root} is readable ({} items)", entries.len()),
            t,
        )),
        Err(e) => {
            steps.push(step("Root path", StepState::Failed, e.to_string(), t));
            client.close().await;
            return steps;
        }
    }
    let t = Instant::now();
    let probe = crate::mapper::join_remote(
        &root,
        &format!(".iremote-write-test-{}", std::process::id()),
    );
    match client.write_file(&probe, b"iremote").await {
        Ok(()) => {
            let _ = client.delete_file(&probe).await;
            steps.push(step(
                "Write access",
                StepState::Ok,
                format!("{root} is writable"),
                t,
            ));
        }
        Err(e) => steps.push(step(
            "Write access",
            StepState::Warning,
            format!("{root} is not writable — uploads will fail ({e})"),
            t,
        )),
    }
    client.close().await;
    steps
}

/// Root path autodetection: the directory the server logs the user into.
pub async fn detect_root(server: &Server, ctx: &Arc<ConnectContext>) -> Result<String> {
    let mut client = connect_interactive(server, ctx).await?;
    let home = client.home_dir().await;
    client.close().await;
    home
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::NoPrompt;

    fn ctx() -> Arc<ConnectContext> {
        let dir = std::env::temp_dir().join(format!("iremote-mgr-{}", std::process::id()));
        Arc::new(ConnectContext::new(
            Arc::new(Credentials::memory()),
            Arc::new(NoPrompt),
            Arc::new(HostKeyTrust::new(dir.join("kh.json"), vec![])),
        ))
    }

    #[tokio::test]
    async fn local_server_test_connection() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Server::new("l", "Local", ServerType::Local);
        s.root_path = dir.path().to_string_lossy().replace('\\', "/");
        let steps = test_connection(&s, &ctx()).await;
        assert!(steps.iter().all(|s| s.state == StepState::Ok), "{steps:#?}");
        assert_eq!(steps.len(), 3);
    }

    #[tokio::test]
    async fn pools_are_rebuilt_on_connection_changes() {
        let m = ServerManager::new(ctx());
        let mut s = Server::new("l", "Local", ServerType::Local);
        let a = m.pool(&s);
        s.name = "Renamed".into();
        s.mappings.clear();
        let b = m.pool(&s);
        assert!(a.same(&b));
        s.root_path = "/other".into();
        let c = m.pool(&s);
        assert!(!a.same(&c));
    }
}
