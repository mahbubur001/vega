//! "Local or mounted folder": the server root is a local directory; remote paths are POSIX paths under it.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::*;
use crate::error::{ErrorCode, RemoteError, Result};

pub struct LocalClient {
    base: PathBuf,
}

impl LocalClient {
    /// Remote paths map onto the local filesystem directly (`root_path` already holds the local directory, so
    /// mapped remote paths are absolute local paths). `base` is "/" on Unix; on Windows remote paths look like
    /// `/C:/dir`.
    pub fn new() -> Self {
        Self {
            base: PathBuf::from("/"),
        }
    }

    fn path(&self, remote: &str) -> PathBuf {
        let p = crate::mapper::norm_remote(remote);
        if cfg!(windows) {
            // "/C:/x" → "C:/x"
            let trimmed = p.trim_start_matches('/');
            if trimmed.len() >= 2 && trimmed.as_bytes()[1] == b':' {
                return PathBuf::from(trimmed);
            }
        }
        self.base.join(p.trim_start_matches('/'))
    }
}

impl Default for LocalClient {
    fn default() -> Self {
        Self::new()
    }
}

fn map_err(e: std::io::Error, path: &str) -> RemoteError {
    RemoteError::from(e).with_path(path)
}

fn to_stat(m: &std::fs::Metadata, link: bool, target: Option<&std::fs::Metadata>) -> RemoteStat {
    let kind = if link {
        EntryType::Symlink
    } else if m.is_dir() {
        EntryType::Dir
    } else {
        EntryType::File
    };
    let target_kind = target.map(|t| {
        if t.is_dir() {
            EntryType::Dir
        } else {
            EntryType::File
        }
    });
    let shown = target.unwrap_or(m);
    #[cfg(unix)]
    let mode = {
        use std::os::unix::fs::PermissionsExt;
        Some(shown.permissions().mode() & 0o7777)
    };
    #[cfg(not(unix))]
    let mode = None;
    RemoteStat {
        kind,
        target_kind,
        size: if shown.is_dir() { 0 } else { shown.len() },
        mtime: shown.modified().map(system_time_ms).unwrap_or(0),
        mode,
    }
}

async fn stat_path(p: &Path) -> std::io::Result<RemoteStat> {
    let lm = tokio::fs::symlink_metadata(p).await?;
    if lm.file_type().is_symlink() {
        let target = tokio::fs::metadata(p).await.ok();
        Ok(to_stat(&lm, true, target.as_ref()))
    } else {
        Ok(to_stat(&lm, false, None))
    }
}

async fn copy_with_progress(from: &Path, to: &Path, progress: Progress<'_>) -> std::io::Result<()> {
    let mut src = tokio::fs::File::open(from).await?;
    let total = src.metadata().await?.len();
    let mut dst = tokio::fs::File::create(to).await?;
    let mut buf = vec![0u8; 256 * 1024];
    let mut done = 0u64;
    loop {
        let n = src.read(&mut buf).await?;
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

#[async_trait]
impl RemoteClient for LocalClient {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            chmod: cfg!(unix),
            set_mtime: true,
            exec: false,
            reliable_mtime: true,
        }
    }

    fn is_connected(&self) -> bool {
        true
    }

    async fn close(&mut self) {}

    async fn home_dir(&mut self) -> Result<String> {
        Ok("/".into())
    }

    async fn list(&mut self, path: &str) -> Result<Vec<RemoteEntry>> {
        let dir = self.path(path);
        let mut rd = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| map_err(e, path))?;
        let mut out = Vec::new();
        while let Some(entry) = rd.next_entry().await.map_err(|e| map_err(e, path))? {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Ok(stat) = stat_path(&entry.path()).await {
                out.push(RemoteEntry { name, stat });
            }
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    async fn stat(&mut self, path: &str) -> Result<Option<RemoteStat>> {
        match stat_path(&self.path(path)).await {
            Ok(s) => Ok(Some(s)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(map_err(e, path)),
        }
    }

    async fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        tokio::fs::read(self.path(path))
            .await
            .map_err(|e| map_err(e, path))
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        tokio::fs::write(self.path(path), data)
            .await
            .map_err(|e| map_err(e, path))
    }

    async fn upload(&mut self, local: &Path, remote: &str, progress: Progress<'_>) -> Result<()> {
        copy_with_progress(local, &self.path(remote), progress)
            .await
            .map_err(|e| map_err(e, remote))
    }

    async fn download(&mut self, remote: &str, local: &Path, progress: Progress<'_>) -> Result<()> {
        copy_with_progress(&self.path(remote), local, progress)
            .await
            .map_err(|e| map_err(e, remote))
    }

    async fn mkdirp(&mut self, path: &str) -> Result<()> {
        tokio::fs::create_dir_all(self.path(path))
            .await
            .map_err(|e| map_err(e, path))
    }

    async fn delete_file(&mut self, path: &str) -> Result<()> {
        tokio::fs::remove_file(self.path(path))
            .await
            .map_err(|e| map_err(e, path))
    }

    async fn delete_dir(&mut self, path: &str, recursive: bool) -> Result<()> {
        let p = self.path(path);
        let r = if recursive {
            tokio::fs::remove_dir_all(&p).await
        } else {
            tokio::fs::remove_dir(&p).await
        };
        r.map_err(|e| map_err(e, path))
    }

    async fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        tokio::fs::rename(self.path(from), self.path(to))
            .await
            .map_err(|e| map_err(e, from))
    }

    async fn chmod(&mut self, path: &str, mode: u32) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(self.path(path), std::fs::Permissions::from_mode(mode))
                .await
                .map_err(|e| map_err(e, path))
        }
        #[cfg(not(unix))]
        {
            let _ = (path, mode);
            Err(RemoteError::unsupported(
                "chmod is not supported on this platform",
            ))
        }
    }

    async fn set_mtime(&mut self, path: &str, mtime_ms: i64) -> Result<()> {
        let p = self.path(path);
        let t = filetime::FileTime::from_unix_time(
            mtime_ms.div_euclid(1000),
            (mtime_ms.rem_euclid(1000) * 1_000_000) as u32,
        );
        tokio::task::spawn_blocking(move || filetime::set_file_mtime(&p, t))
            .await
            .map_err(|e| RemoteError::new(ErrorCode::Unknown, e.to_string()))?
            .map_err(|e| map_err(e, path))
    }

    async fn exec(&mut self, _command: &str) -> Result<ExecOutput> {
        Err(RemoteError::unsupported(
            "Shell commands are not available for local servers",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn local_contract() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().to_string_lossy().replace('\\', "/");
        let root = if root.starts_with('/') {
            root
        } else {
            format!("/{root}")
        };
        crate::protocol::testing::contract(&mut LocalClient::new(), &root).await;
    }
}
