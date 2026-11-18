//! One `RemoteClient` contract for every server type. Remote paths are POSIX; bytes are never re-encoded.

use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::Result;

pub mod ftp;
pub mod local;
pub mod sftp;
pub mod webdav;

#[cfg(any(test, feature = "integration"))]
pub mod testing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntryType {
    File,
    Dir,
    Symlink,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStat {
    pub kind: EntryType,
    /// For symlinks: the target's type when it could be resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_kind: Option<EntryType>,
    pub size: u64,
    /// Milliseconds since the epoch; 0 when the server can't report it.
    pub mtime: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<u32>,
}

impl RemoteStat {
    pub fn is_dir(&self) -> bool {
        self.kind == EntryType::Dir
            || (self.kind == EntryType::Symlink && self.target_kind == Some(EntryType::Dir))
    }

    pub fn file(size: u64, mtime: i64) -> Self {
        Self {
            kind: EntryType::File,
            target_kind: None,
            size,
            mtime,
            mode: None,
        }
    }

    pub fn dir(mtime: i64) -> Self {
        Self {
            kind: EntryType::Dir,
            target_kind: None,
            size: 0,
            mtime,
            mode: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteEntry {
    pub name: String,
    #[serde(flatten)]
    pub stat: RemoteStat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub chmod: bool,
    pub set_mtime: bool,
    /// Remote shell commands (SFTP over SSH).
    pub exec: bool,
    /// `mtime` values are reliable enough for change detection.
    pub reliable_mtime: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ExecOutput {
    pub code: u32,
    pub stdout: String,
    pub stderr: String,
}

/// Everything a client needs to connect. Credentials are resolved (keychain / prompts) before this is built.
#[derive(Debug, Clone)]
pub struct ConnectParams {
    pub server: crate::config::Server,
    /// Password for password auth (SFTP, FTP/FTPS, WebDAV). `None` for anonymous / key / agent / no auth.
    pub password: Option<String>,
    pub timeout: std::time::Duration,
    /// Certificate fingerprints accepted although OS verification failed (pinned in config or trusted for the
    /// session). See [`crate::tls`].
    pub accepted_certs: Vec<String>,
    /// Passphrase for an encrypted private key (SFTP).
    pub passphrase: Option<String>,
    /// Extra known_hosts files (`ssh.knownHostsFiles`).
    pub known_hosts_files: Vec<std::path::PathBuf>,
}

impl ConnectParams {
    pub fn new(server: crate::config::Server) -> Self {
        Self {
            server,
            password: None,
            timeout: std::time::Duration::from_secs(20),
            accepted_certs: Vec::new(),
            passphrase: None,
            known_hosts_files: Vec::new(),
        }
    }
}

/// Interactive decisions needed while connecting (SSH). Implemented by the engine, which asks the user.
#[async_trait]
pub trait ConnectHooks: Send + Sync {
    /// Called with the server's host key blob (SSH wire format); must enforce the host-key policy.
    async fn verify_host_key(&self, host: &str, port: u16, key_type: &str, key_blob: &[u8])
    -> bool;
    /// Keyboard-interactive prompts (2FA etc.): `(prompt, echo)` pairs. `None` cancels.
    async fn keyboard_interactive(
        &self,
        name: &str,
        instructions: &str,
        prompts: &[(String, bool)],
    ) -> Option<Vec<String>>;
}

/// Accepts every host key and answers no prompts (tests against throw-away servers only).
pub struct InsecureHooks;

#[async_trait]
impl ConnectHooks for InsecureHooks {
    async fn verify_host_key(&self, _: &str, _: u16, _: &str, _: &[u8]) -> bool {
        true
    }

    async fn keyboard_interactive(
        &self,
        _: &str,
        _: &str,
        _: &[(String, bool)],
    ) -> Option<Vec<String>> {
        None
    }
}

/// Opens a client for any transferring server type.
pub async fn connect(
    params: &ConnectParams,
    hooks: std::sync::Arc<dyn ConnectHooks>,
) -> Result<Box<dyn RemoteClient>> {
    use crate::config::ServerType as T;
    let fut = async {
        Ok::<Box<dyn RemoteClient>, crate::error::RemoteError>(match params.server.kind {
            T::Sftp => Box::new(sftp::SftpClient::connect(params, hooks).await?),
            T::Ftp | T::Ftps | T::FtpsImplicit => Box::new(ftp::connect(params).await?),
            T::Webdav => Box::new(webdav::connect(params).await?),
            T::Local => Box::new(local::LocalClient::new()),
            T::InPlace | T::Group => {
                return Err(crate::error::RemoteError::unsupported(format!(
                    "\"{}\" ({}) does not transfer files",
                    params.server.name,
                    params.server.kind.label()
                )));
            }
        })
    };
    // SSH prompts (host key, 2FA) wait for the user, so the overall timeout is generous; protocol clients apply
    // `params.timeout` to each network step themselves.
    fut.await
}

/// `(transferred, total)` byte counts.
pub type Progress<'a> = &'a (dyn Fn(u64, u64) + Send + Sync);

pub fn no_progress(_: u64, _: u64) {}

#[async_trait]
pub trait RemoteClient: Send + Sync {
    fn capabilities(&self) -> Capabilities;
    /// False once the connection dropped; the pool discards such clients.
    fn is_connected(&self) -> bool;
    async fn close(&mut self);
    /// Directory the server places the user in after login.
    async fn home_dir(&mut self) -> Result<String>;
    async fn list(&mut self, path: &str) -> Result<Vec<RemoteEntry>>;
    /// `None` when the path does not exist.
    async fn stat(&mut self, path: &str) -> Result<Option<RemoteStat>>;
    async fn read_file(&mut self, path: &str) -> Result<Vec<u8>>;
    async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()>;
    async fn upload(&mut self, local: &Path, remote: &str, progress: Progress<'_>) -> Result<()>;
    async fn download(&mut self, remote: &str, local: &Path, progress: Progress<'_>) -> Result<()>;
    /// Creates the directory and missing parents; succeeds when it exists.
    async fn mkdirp(&mut self, path: &str) -> Result<()>;
    async fn delete_file(&mut self, path: &str) -> Result<()>;
    async fn delete_dir(&mut self, path: &str, recursive: bool) -> Result<()>;
    async fn rename(&mut self, from: &str, to: &str) -> Result<()>;
    async fn chmod(&mut self, path: &str, mode: u32) -> Result<()>;
    async fn set_mtime(&mut self, path: &str, mtime_ms: i64) -> Result<()>;
    async fn exec(&mut self, command: &str) -> Result<ExecOutput>;
}

/// Recursive delete built on list/delete for protocols without a native one.
pub async fn delete_tree(client: &mut dyn RemoteClient, path: &str) -> Result<()> {
    let mut stack = vec![(path.to_string(), false)];
    while let Some((dir, expanded)) = stack.pop() {
        if expanded {
            client.delete_dir(&dir, false).await?;
            continue;
        }
        stack.push((dir.clone(), true));
        for e in client.list(&dir).await? {
            let child = crate::mapper::join_remote(&dir, &e.name);
            if e.stat.kind == EntryType::Dir {
                stack.push((child, false));
            } else {
                client.delete_file(&child).await?;
            }
        }
    }
    Ok(())
}

pub fn system_time_ms(t: std::time::SystemTime) -> i64 {
    match t.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_millis() as i64,
        Err(e) => -(e.duration().as_millis() as i64),
    }
}

/// Single-quotes a string for POSIX shells.
pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}
