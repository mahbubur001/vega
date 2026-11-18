//! What we last knew about each file after a transfer, per server and remote path. It is the baseline for
//! "remote file changed since last upload" (warn when uploading over newer file), for incremental uploads
//! ("overwrite up-to-date files" off) and for sync classification without clock-skew problems.

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncRecord {
    /// Milliseconds since the epoch; 0 when the server can't report it.
    pub remote_mtime: i64,
    pub remote_size: u64,
    pub local_mtime: i64,
    pub local_size: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    pub at: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct FileShape {
    version: u32,
    servers: HashMap<String, BTreeMap<String, SyncRecord>>,
}

pub struct StateCache {
    file: Option<PathBuf>,
    data: Mutex<FileShape>,
}

pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn prefix(p: &str) -> String {
    if p.ends_with('/') {
        p.to_string()
    } else {
        format!("{p}/")
    }
}

impl StateCache {
    /// Loads from `file` (missing or corrupt files start empty).
    pub fn load(file: PathBuf) -> Self {
        let data = std::fs::read(&file)
            .ok()
            .and_then(|b| serde_json::from_slice::<FileShape>(&b).ok())
            .filter(|d| d.version == 1)
            .unwrap_or(FileShape {
                version: 1,
                servers: HashMap::new(),
            });
        Self {
            file: Some(file),
            data: Mutex::new(data),
        }
    }

    /// Not persisted (tests).
    pub fn in_memory() -> Self {
        Self {
            file: None,
            data: Mutex::new(FileShape {
                version: 1,
                servers: HashMap::new(),
            }),
        }
    }

    fn with<R>(&self, f: impl FnOnce(&mut FileShape) -> R) -> R {
        let mut guard = self.data.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut guard)
    }

    pub fn get(&self, server: &str, remote: &str) -> Option<SyncRecord> {
        self.with(|d| d.servers.get(server).and_then(|m| m.get(remote)).cloned())
    }

    pub fn set(&self, server: &str, remote: &str, mut record: SyncRecord) {
        record.at = now_ms();
        self.with(|d| {
            d.servers
                .entry(server.into())
                .or_default()
                .insert(remote.into(), record)
        });
    }

    pub fn delete(&self, server: &str, remote: &str, recursive: bool) {
        self.with(|d| {
            if let Some(m) = d.servers.get_mut(server) {
                m.remove(remote);
                if recursive {
                    let pre = prefix(remote);
                    m.retain(|k, _| !k.starts_with(&pre));
                }
            }
        });
    }

    pub fn rename(&self, server: &str, from: &str, to: &str) {
        self.with(|d| {
            let Some(m) = d.servers.get_mut(server) else {
                return;
            };
            let pre = prefix(from);
            let keys: Vec<String> = m
                .keys()
                .filter(|k| *k == from || k.starts_with(&pre))
                .cloned()
                .collect();
            for k in keys {
                if let Some(v) = m.remove(&k) {
                    m.insert(format!("{to}{}", &k[from.len()..]), v);
                }
            }
        });
    }

    pub fn forget_server(&self, server: &str) {
        self.with(|d| d.servers.remove(server));
    }

    /// Writes atomically. Called after each batch of transfers.
    pub fn flush(&self) -> std::io::Result<()> {
        let Some(file) = &self.file else {
            return Ok(());
        };
        let bytes = self
            .with(|d| serde_json::to_vec(d))
            .map_err(std::io::Error::other)?;
        crate::dirs::write_atomic(file, &bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(size: u64) -> SyncRecord {
        SyncRecord {
            remote_mtime: 1,
            remote_size: size,
            local_mtime: 1,
            local_size: size,
            sha1: None,
            at: 0,
        }
    }

    #[test]
    fn rename_and_delete_subtrees() {
        let c = StateCache::in_memory();
        c.set("s", "/a/x", rec(1));
        c.set("s", "/a/b/y", rec(2));
        c.set("s", "/ab", rec(3));
        c.rename("s", "/a", "/z");
        assert!(c.get("s", "/a/x").is_none());
        assert_eq!(c.get("s", "/z/b/y").unwrap().remote_size, 2);
        assert!(
            c.get("s", "/ab").is_some(),
            "sibling with common prefix untouched"
        );
        c.delete("s", "/z", true);
        assert!(c.get("s", "/z/x").is_none());
        assert!(c.get("s", "/ab").is_some());
    }

    #[test]
    fn persists_atomically() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("state.json");
        let c = StateCache::load(file.clone());
        c.set("s", "/f", rec(5));
        c.flush().unwrap();
        assert_eq!(
            StateCache::load(file).get("s", "/f").unwrap().remote_size,
            5
        );
    }
}
