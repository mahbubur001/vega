//! Upload / download primitives on one connection, plus local and remote tree walking. The engine queues these;
//! they implement the deployment options: warn when uploading over a newer file (with Compare), temporary-file uploads,
//! preserved timestamps, permission overrides, original permissions (FTP) and empty directories.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha1::{Digest, Sha1};

use crate::config::{Options, OverwriteCheck, Server};
use crate::error::{ErrorCode, RemoteError, Result};
use crate::mapper::{PathMapper, join_remote, remote_name, remote_parent};
use crate::prompt::{ConflictDecision, ConflictQuestion, Prompter};
use crate::protocol::{Progress, RemoteClient, RemoteStat, no_progress, system_time_ms};
use crate::queue::TaskOutcome;
use crate::state::{StateCache, SyncRecord};

/// A server as used by one project: definition, path mapper and the project's options.
#[derive(Clone)]
pub struct Target {
    pub server: Server,
    pub mapper: PathMapper,
    pub options: Options,
}

/// Timestamps from FTP/SFTP have one-second precision.
pub fn same_time(a: i64, b: i64) -> bool {
    (a - b).abs() < 2000
}

pub fn sha1_bytes(b: &[u8]) -> String {
    hex::encode(Sha1::digest(b))
}

pub async fn sha1_file(path: &Path) -> std::io::Result<String> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        use std::io::Read;
        let mut f = std::fs::File::open(path)?;
        let mut h = Sha1::new();
        let mut buf = vec![0u8; 256 * 1024];
        loop {
            let n = f.read(&mut buf)?;
            if n == 0 {
                break;
            }
            h.update(&buf[..n]);
        }
        Ok(hex::encode(h.finalize()))
    })
    .await
    .map_err(std::io::Error::other)?
}

#[derive(Debug, Clone, Copy)]
pub struct LocalMeta {
    pub size: u64,
    pub mtime: i64,
}

pub async fn local_meta(path: &Path) -> std::io::Result<LocalMeta> {
    let m = tokio::fs::metadata(path).await?;
    Ok(LocalMeta {
        size: m.len(),
        mtime: m.modified().map(system_time_ms).unwrap_or(0),
    })
}

/// Creates missing remote directories top-down, applying folder permissions to the ones it creates.
pub async fn ensure_remote_dir(c: &mut dyn RemoteClient, t: &Target, dir: &str) -> Result<()> {
    let mut missing = Vec::new();
    let mut d = crate::mapper::norm_remote(dir);
    while d != "/" && c.stat(&d).await?.is_none() {
        missing.push(d.clone());
        d = remote_parent(&d);
    }
    let mode = t.options.folder_mode();
    for m in missing.into_iter().rev() {
        c.mkdirp(&m).await?;
        if let Some(mode) = mode.filter(|_| c.capabilities().chmod)
            && let Err(e) = c.chmod(&m, mode).await
        {
            log::warn!("chmod {m}: {e}");
        }
    }
    Ok(())
}

/// Records the post-transfer state so later conflict checks and syncs are accurate.
pub async fn remember(
    c: &mut dyn RemoteClient,
    cache: &StateCache,
    t: &Target,
    local: &Path,
    remote: &str,
    sha1: Option<String>,
) -> Result<()> {
    let Some(r) = c.stat(remote).await? else {
        return Ok(());
    };
    let l = local_meta(local).await?;
    let sha1 = match sha1 {
        Some(s) => Some(s),
        None => sha1_file(local).await.ok(),
    };
    cache.set(
        &t.server.id,
        remote,
        SyncRecord {
            remote_mtime: r.mtime,
            remote_size: r.size,
            local_mtime: l.mtime,
            local_size: l.size,
            sha1,
            at: 0,
        },
    );
    Ok(())
}

/// "Warn when uploading over newer file": did the remote copy change since we last transferred it?
pub async fn detect_conflict(
    c: &mut dyn RemoteClient,
    cache: &StateCache,
    t: &Target,
    local: &Path,
    remote: &str,
) -> Result<Option<(RemoteStat, String)>> {
    let mode = t.options.warn_on_newer_remote;
    if mode == OverwriteCheck::None {
        return Ok(None);
    }
    let Some(r) = c.stat(remote).await? else {
        return Ok(None);
    };
    if r.is_dir() {
        return Ok(None);
    }
    let record = cache.get(&t.server.id, remote);
    let reason = match &record {
        Some(rec) => {
            let time_known = r.mtime > 0 && rec.remote_mtime > 0;
            let changed =
                r.size != rec.remote_size || (time_known && !same_time(r.mtime, rec.remote_mtime));
            changed.then(|| {
                "The server copy was modified after it was last uploaded or downloaded.".to_string()
            })
        }
        None if r.mtime > 0 => {
            let l = local_meta(local).await?;
            (r.mtime > l.mtime + 2000)
                .then(|| "The server copy is newer than the local file.".to_string())
        }
        None => None,
    };
    let Some(reason) = reason else {
        return Ok(None);
    };
    if mode == OverwriteCheck::Content {
        let remote_data = c.read_file(remote).await?;
        let local_data = tokio::fs::read(local).await?;
        if remote_data == local_data {
            return Ok(None);
        }
        if record
            .and_then(|r| r.sha1)
            .is_some_and(|s| s == sha1_bytes(&remote_data))
        {
            return Ok(None);
        }
    }
    Ok(Some((r, reason)))
}

/// Downloads a remote file into the compare cache and returns its path.
pub async fn fetch_for_compare(
    c: &mut dyn RemoteClient,
    t: &Target,
    remote: &str,
) -> Result<PathBuf> {
    let dir = crate::dirs::cache_dir()
        .join("compare")
        .join(&t.server.id)
        .join(remote_parent(remote).trim_start_matches('/'));
    tokio::fs::create_dir_all(&dir).await?;
    let path = dir.join(format!("{} (remote)", remote_name(remote)));
    c.download(remote, &path, &no_progress).await?;
    Ok(path)
}

pub struct Uploader<'a> {
    pub cache: &'a StateCache,
    pub prompter: &'a Arc<dyn Prompter>,
}

impl Uploader<'_> {
    /// Uploads one file through a leased client. Returns `Skipped` when the user chose to keep the server copy.
    pub async fn upload(
        &self,
        c: &mut dyn RemoteClient,
        t: &Target,
        local: &Path,
        remote: &str,
        check_conflict: bool,
        progress: Progress<'_>,
    ) -> Result<TaskOutcome> {
        if check_conflict
            && let Some((_, reason)) = detect_conflict(c, self.cache, t, local, remote).await?
        {
            let q = ConflictQuestion {
                server_name: t.server.name.clone(),
                local_path: local.display().to_string(),
                remote_path: remote.to_string(),
                reason,
            };
            let mut allow_compare = true;
            loop {
                match self.prompter.conflict(&q, allow_compare).await {
                    ConflictDecision::Overwrite => break,
                    ConflictDecision::Skip => return Ok(TaskOutcome::Skipped),
                    ConflictDecision::Cancel => return Err(RemoteError::cancelled()),
                    ConflictDecision::Compare => {
                        let remote_copy = fetch_for_compare(c, t, remote).await?;
                        let title = format!("{} ↔ {}", remote_name(remote), t.server.name);
                        self.prompter.show_diff(&remote_copy, local, &title).await;
                        allow_compare = false;
                    }
                }
            }
        }
        self.put(c, t, local, remote, progress).await?;
        let local_m = tokio::fs::metadata(local).await?;
        if t.options.preserve_timestamps && c.capabilities().set_mtime {
            let mtime = local_m.modified().map(system_time_ms).unwrap_or(0);
            if let Err(e) = c.set_mtime(remote, mtime).await {
                log::warn!("Cannot preserve timestamp on {remote}: {e}");
            }
        }
        if c.capabilities().chmod {
            // "Preserve original file permissions" (FTP/FTPS on macOS/Linux).
            #[cfg(unix)]
            let original = {
                use std::os::unix::fs::PermissionsExt;
                (t.options.preserve_original_permissions && t.server.kind.is_ftp())
                    .then(|| local_m.permissions().mode() & 0o777)
            };
            #[cfg(not(unix))]
            let original: Option<u32> = None;
            let mode = t.options.file_mode().or(original);
            if let Some(mode) = mode
                && let Err(e) = c.chmod(remote, mode).await
            {
                log::warn!("Cannot chmod {remote}: {e}");
            }
        }
        remember(c, self.cache, t, local, remote, None).await?;
        Ok(TaskOutcome::Done)
    }

    /// Writes the bytes, creating missing parents. With "use a temporary file" the upload goes to a hidden name and
    /// is renamed into place, so the server never serves a half-written file.
    async fn put(
        &self,
        c: &mut dyn RemoteClient,
        t: &Target,
        local: &Path,
        remote: &str,
        progress: Progress<'_>,
    ) -> Result<()> {
        let dest = if t.options.use_temp_file {
            join_remote(
                &remote_parent(remote),
                &format!(".{}.iremote-upload", remote_name(remote)),
            )
        } else {
            remote.to_string()
        };
        if let Err(e) = c.upload(local, &dest, progress).await {
            // Servers report a missing parent as "no such file" or a generic failure; retry only when it's missing.
            if !matches!(e.code, ErrorCode::NotFound | ErrorCode::Unknown)
                || c.stat(&remote_parent(remote)).await?.is_some()
            {
                return Err(e);
            }
            ensure_remote_dir(c, t, &remote_parent(remote)).await?;
            c.upload(local, &dest, progress).await?;
        }
        if dest != remote
            && let Err(e) = c.rename(&dest, remote).await
        {
            let _ = c.delete_file(&dest).await;
            return Err(e);
        }
        Ok(())
    }
}

/// Downloads to a temporary sibling and renames, so a failed transfer never truncates the local file.
pub async fn download_to(
    c: &mut dyn RemoteClient,
    cache: &StateCache,
    t: &Target,
    remote: &str,
    local: &Path,
    progress: Progress<'_>,
) -> Result<()> {
    if let Some(parent) = local.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let tmp = local.with_file_name(format!(
        ".{}.iremote-download",
        local
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    ));
    if let Err(e) = c.download(remote, &tmp, progress).await {
        let _ = tokio::fs::remove_file(&tmp).await;
        return Err(e);
    }
    tokio::fs::rename(&tmp, local).await?;
    if t.options.preserve_timestamps
        && let Some(r) = c.stat(remote).await?.filter(|r| r.mtime > 0)
    {
        let ft = filetime::FileTime::from_unix_time(
            r.mtime.div_euclid(1000),
            (r.mtime.rem_euclid(1000) * 1_000_000) as u32,
        );
        let _ = filetime::set_file_mtime(local, ft);
    }
    remember(c, cache, t, local, remote, None).await
}

/// Deployable files (and directories, for empty-dir creation) under the given local paths.
pub fn collect_local(t: &Target, roots: &[PathBuf]) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut stack: Vec<PathBuf> = roots.iter().rev().cloned().collect();
    while let Some(p) = stack.pop() {
        if t.mapper.is_local_excluded(&p) || t.mapper.to_remote(&p).is_none() {
            continue;
        }
        let Ok(meta) = std::fs::metadata(&p) else {
            continue;
        };
        if meta.is_dir() {
            let real = std::fs::canonicalize(&p).unwrap_or_else(|_| p.clone());
            if !seen.insert(real) {
                continue;
            }
            dirs.push(p.clone());
            let mut children: Vec<PathBuf> = std::fs::read_dir(&p)
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.path())
                .collect();
            children.sort();
            stack.extend(children.into_iter().rev());
        } else if meta.is_file() {
            files.push(p);
        }
    }
    (files, dirs)
}

/// Remote files (with stats) and directories under `root`, honoring remote excludes. Doesn't follow directory
/// symlinks (loop risk).
pub async fn collect_remote(
    c: &mut dyn RemoteClient,
    t: &Target,
    root: &str,
) -> Result<(Vec<(String, RemoteStat)>, Vec<String>)> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let Some(st) = c.stat(root).await? else {
        return Ok((files, dirs));
    };
    let mut stack = vec![(root.to_string(), st)];
    while let Some((p, st)) = stack.pop() {
        if t.mapper.is_remote_excluded(&p) {
            continue;
        }
        if !st.is_dir() {
            files.push((p, st));
            continue;
        }
        if st.kind == crate::protocol::EntryType::Symlink {
            continue;
        }
        dirs.push(p.clone());
        for e in c.list(&p).await? {
            stack.push((join_remote(&p, &e.name), e.stat));
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((files, dirs))
}
