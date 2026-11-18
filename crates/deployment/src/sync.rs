//! Sync with Deployed: classifies every file under the chosen roots on both sides.
//!
//! The default mode uses the last-transfer records (no clock-skew problems); the explicit modes compare by
//! size and timestamp, size only, binary content, or text content ignoring line endings.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::deploy::{
    LocalMeta, Target, collect_local, collect_remote, local_meta, same_time, sha1_bytes, sha1_file,
};
use crate::error::Result;
use crate::protocol::{RemoteClient, RemoteStat, shell_quote};
use crate::state::{StateCache, SyncRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncState {
    Same,
    LocalOnly,
    RemoteOnly,
    LocalNewer,
    RemoteNewer,
    Conflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncAction {
    Upload,
    Download,
    DeleteRemote,
    DeleteLocal,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum CompareMode {
    /// Changes since the last transfer (falls back to size + time for files never transferred).
    #[default]
    Smart,
    SizeAndTimestamp,
    Size,
    /// Byte-for-byte (hashes over SSH when shell commands are allowed).
    Content,
    /// Text content ignoring line endings and a trailing newline.
    TextIgnoreEol,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalInfo {
    pub size: u64,
    pub mtime: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncItem {
    pub id: u32,
    pub remote_path: String,
    pub local_path: PathBuf,
    /// Project-relative display path.
    pub rel_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<LocalInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<RemoteStat>,
    pub state: SyncState,
    pub action: SyncAction,
    /// Transferred before (lets the UI explain "deleted on the other side").
    pub was_synced: bool,
}

pub fn default_action(state: SyncState) -> SyncAction {
    match state {
        SyncState::LocalOnly | SyncState::LocalNewer => SyncAction::Upload,
        SyncState::RemoteOnly | SyncState::RemoteNewer => SyncAction::Download,
        SyncState::Same | SyncState::Conflict => SyncAction::Skip,
    }
}

/// Pure classification of one file pair.
pub fn classify(
    local: Option<LocalMeta>,
    remote: Option<&RemoteStat>,
    record: Option<&SyncRecord>,
    mode: CompareMode,
) -> SyncState {
    let (l, r) = match (local, remote) {
        (Some(_), None) => return SyncState::LocalOnly,
        (None, Some(_)) => return SyncState::RemoteOnly,
        (None, None) => return SyncState::Same,
        (Some(l), Some(r)) => (l, r),
    };
    match mode {
        CompareMode::Size => {
            return if l.size == r.size {
                SyncState::Same
            } else {
                SyncState::Conflict
            };
        }
        CompareMode::SizeAndTimestamp | CompareMode::Content | CompareMode::TextIgnoreEol => {
            return by_time(l, r);
        }
        CompareMode::Smart => {}
    }
    if let Some(rec) = record {
        let local_changed = l.size != rec.local_size || !same_time(l.mtime, rec.local_mtime);
        let time_known = r.mtime > 0 && rec.remote_mtime > 0;
        let remote_changed =
            r.size != rec.remote_size || (time_known && !same_time(r.mtime, rec.remote_mtime));
        return match (local_changed, remote_changed) {
            (false, false) => SyncState::Same,
            (true, false) => SyncState::LocalNewer,
            (false, true) => SyncState::RemoteNewer,
            (true, true) => SyncState::Conflict,
        };
    }
    by_time(l, r)
}

fn by_time(l: LocalMeta, r: &RemoteStat) -> SyncState {
    if r.mtime == 0 {
        return if l.size == r.size {
            SyncState::Same
        } else {
            SyncState::Conflict
        };
    }
    if l.size == r.size && same_time(l.mtime, r.mtime) {
        SyncState::Same
    } else if l.mtime > r.mtime + 2000 {
        SyncState::LocalNewer
    } else if r.mtime > l.mtime + 2000 {
        SyncState::RemoteNewer
    } else {
        SyncState::Conflict
    }
}

fn normalize_eol(b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\r' {
            out.push(b'\n');
            if b.get(i + 1) == Some(&b'\n') {
                i += 1;
            }
        } else {
            out.push(b[i]);
        }
        i += 1;
    }
    while out.last() == Some(&b'\n') {
        out.pop();
    }
    out
}

pub struct CompareRequest<'a> {
    pub target: &'a Target,
    pub project_root: &'a std::path::Path,
    pub local_roots: &'a [PathBuf],
    pub mode: CompareMode,
    /// Upper bound for downloading a file just to compare it when hashing over SSH is unavailable.
    pub max_download_bytes: u64,
}

pub async fn compare(
    c: &mut dyn RemoteClient,
    cache: &StateCache,
    req: CompareRequest<'_>,
) -> Result<Vec<SyncItem>> {
    let t = req.target;
    let (local_files, _) = collect_local(t, req.local_roots);
    let mut by_remote: BTreeMap<String, (PathBuf, Option<LocalMeta>, Option<RemoteStat>)> =
        BTreeMap::new();
    for lp in local_files {
        let Some(rp) = t.mapper.to_remote(&lp) else {
            continue;
        };
        let meta = local_meta(&lp).await.ok();
        by_remote.insert(rp, (lp, meta, None));
    }
    let mut remote_roots: Vec<String> = req
        .local_roots
        .iter()
        .filter_map(|r| t.mapper.to_remote(r))
        .collect();
    remote_roots.sort();
    remote_roots.dedup();
    for rr in remote_roots {
        let (files, _) = collect_remote(c, t, &rr).await?;
        for (path, stat) in files {
            if let Some(e) = by_remote.get_mut(&path) {
                e.2 = Some(stat);
                continue;
            }
            let Some(lp) = t.mapper.to_local(&path) else {
                continue;
            };
            if t.mapper.is_local_excluded(&lp) {
                continue;
            }
            by_remote.insert(path, (lp, None, Some(stat)));
        }
    }
    let mut items = Vec::new();
    for (id, (remote_path, (local_path, local, remote))) in by_remote.into_iter().enumerate() {
        let record = cache.get(&t.server.id, &remote_path);
        let state = classify(local, remote.as_ref(), record.as_ref(), req.mode);
        let action = if state == SyncState::Same && t.options.overwrite_up_to_date {
            SyncAction::Upload
        } else {
            default_action(state)
        };
        let rel_path = local_path
            .strip_prefix(req.project_root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| remote_path.clone());
        items.push(SyncItem {
            id: id as u32,
            remote_path,
            local_path,
            rel_path,
            local: local.map(|l| LocalInfo {
                size: l.size,
                mtime: l.mtime,
            }),
            remote,
            state,
            action,
            was_synced: record.is_some(),
        });
    }
    if matches!(req.mode, CompareMode::Content | CompareMode::TextIgnoreEol) {
        refine_by_content(c, cache, t, &mut items, req.mode, req.max_download_bytes).await?;
    }
    items.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(items)
}

/// Files whose metadata differs but whose content is identical become "same" (and the record is refreshed).
async fn refine_by_content(
    c: &mut dyn RemoteClient,
    cache: &StateCache,
    t: &Target,
    items: &mut [SyncItem],
    mode: CompareMode,
    max_download: u64,
) -> Result<()> {
    let candidates: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, i)| i.local.is_some() && i.remote.is_some() && i.state != SyncState::Same)
        .filter(|(_, i)| {
            mode == CompareMode::TextIgnoreEol
                || i.local.as_ref().map(|l| l.size) == i.remote.as_ref().map(|r| r.size)
        })
        .map(|(n, _)| n)
        .collect();
    let mut remote_hashes = std::collections::HashMap::new();
    if mode == CompareMode::Content && c.capabilities().exec && t.server.allow_shell_commands {
        for chunk in candidates.chunks(50) {
            let paths: Vec<String> = chunk
                .iter()
                .map(|&n| shell_quote(&items[n].remote_path))
                .collect();
            let Ok(out) = c.exec(&format!("sha1sum -- {}", paths.join(" "))).await else {
                break;
            };
            if out.code != 0 {
                break;
            }
            for line in out.stdout.lines() {
                let line = line.trim().trim_start_matches('\\');
                if let Some((hash, path)) = line.split_once(' ')
                    && hash.len() == 40
                {
                    remote_hashes.insert(
                        path.trim_start_matches([' ', '*']).to_string(),
                        hash.to_string(),
                    );
                }
            }
        }
    }
    for n in candidates {
        let item = &mut items[n];
        let remote = item.remote.clone().expect("filtered");
        let local = item.local.clone().expect("filtered");
        let equal = if mode == CompareMode::TextIgnoreEol {
            if remote.size > max_download {
                continue;
            }
            let r = c.read_file(&item.remote_path).await?;
            let l = tokio::fs::read(&item.local_path).await?;
            normalize_eol(&r) == normalize_eol(&l)
        } else {
            let remote_hash = match remote_hashes.get(&item.remote_path) {
                Some(h) => h.clone(),
                None if remote.size <= max_download => {
                    sha1_bytes(&c.read_file(&item.remote_path).await?)
                }
                None => continue,
            };
            sha1_file(&item.local_path)
                .await
                .map(|h| h == remote_hash)
                .unwrap_or(false)
        };
        if equal {
            item.state = SyncState::Same;
            item.action = if t.options.overwrite_up_to_date {
                SyncAction::Upload
            } else {
                SyncAction::Skip
            };
            cache.set(
                &t.server.id,
                &item.remote_path,
                SyncRecord {
                    remote_mtime: remote.mtime,
                    remote_size: remote.size,
                    local_mtime: local.mtime,
                    local_size: local.size,
                    sha1: None,
                    at: 0,
                },
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(size: u64, mtime: i64) -> Option<LocalMeta> {
        Some(LocalMeta { size, mtime })
    }
    fn r(size: u64, mtime: i64) -> RemoteStat {
        RemoteStat::file(size, mtime)
    }
    fn rec(ls: u64, lm: i64, rs: u64, rm: i64) -> SyncRecord {
        SyncRecord {
            local_size: ls,
            local_mtime: lm,
            remote_size: rs,
            remote_mtime: rm,
            sha1: None,
            at: 0,
        }
    }

    #[test]
    fn classification_with_records() {
        let base = rec(10, 1_000_000, 10, 5_000_000);
        let smart = CompareMode::Smart;
        assert_eq!(
            classify(
                l(10, 1_000_000),
                Some(&r(10, 5_000_000)),
                Some(&base),
                smart
            ),
            SyncState::Same
        );
        assert_eq!(
            classify(
                l(11, 9_000_000),
                Some(&r(10, 5_000_000)),
                Some(&base),
                smart
            ),
            SyncState::LocalNewer
        );
        assert_eq!(
            classify(
                l(10, 1_000_000),
                Some(&r(12, 9_000_000)),
                Some(&base),
                smart
            ),
            SyncState::RemoteNewer
        );
        assert_eq!(
            classify(
                l(11, 2_000_000),
                Some(&r(12, 9_000_000)),
                Some(&base),
                smart
            ),
            SyncState::Conflict
        );
        // Clock skew doesn't matter with a record: remote clock far ahead, nothing changed.
        assert_eq!(
            classify(
                l(10, 1_000_000),
                Some(&r(10, 5_000_000)),
                Some(&base),
                smart
            ),
            SyncState::Same
        );
    }

    #[test]
    fn classification_without_records() {
        assert_eq!(
            classify(l(1, 0), None, None, CompareMode::Smart),
            SyncState::LocalOnly
        );
        assert_eq!(
            classify(None, Some(&r(1, 0)), None, CompareMode::Smart),
            SyncState::RemoteOnly
        );
        assert_eq!(
            classify(l(5, 10_000), Some(&r(5, 11_000)), None, CompareMode::Smart),
            SyncState::Same
        );
        assert_eq!(
            classify(l(5, 20_000), Some(&r(6, 10_000)), None, CompareMode::Smart),
            SyncState::LocalNewer
        );
        assert_eq!(
            classify(l(5, 0), Some(&r(5, 0)), None, CompareMode::Size),
            SyncState::Same
        );
        assert_eq!(
            classify(l(5, 0), Some(&r(6, 0)), None, CompareMode::Size),
            SyncState::Conflict
        );
    }

    #[test]
    fn eol_normalization() {
        assert_eq!(normalize_eol(b"a\r\nb\r\n"), normalize_eol(b"a\nb"));
        assert_eq!(normalize_eol(b"a\rb"), b"a\nb");
    }
}
