//! Where Deployment keeps its own files, inside VEGA's directories.
//! `VEGA_DEPLOYMENT_HOME` overrides everything (tests, portable setups).
//!
//! - config: `<VEGA config>/deployment` (shared servers, trusted host keys)
//! - state:  `<VEGA data>/deployment/state` (per-project transfer state)
//! - cache:  `<VEGA data>/deployment/cache` (remote files opened for editing, compare copies)

use std::path::{Path, PathBuf};

static HOME: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

/// Overrides every Deployment directory for this process (tests). First call wins.
pub fn set_home(path: PathBuf) {
    HOME.set(path).ok();
}

fn home_override() -> Option<PathBuf> {
    HOME.get().cloned().or_else(|| {
        std::env::var_os("VEGA_DEPLOYMENT_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}

pub fn config_dir() -> PathBuf {
    match home_override() {
        Some(home) => home.join("config"),
        None => paths::config_dir().join("deployment"),
    }
}

pub fn state_dir() -> PathBuf {
    match home_override() {
        Some(home) => home.join("state"),
        None => paths::data_dir().join("deployment").join("state"),
    }
}

pub fn cache_dir() -> PathBuf {
    match home_override() {
        Some(home) => home.join("cache"),
        None => paths::data_dir().join("deployment").join("cache"),
    }
}

pub fn global_servers_file() -> PathBuf {
    config_dir().join("servers.json")
}

pub fn trusted_hosts_file() -> PathBuf {
    config_dir().join("known_hosts.json")
}

/// Stable short id for a project root, used to name per-project state files and sockets.
pub fn project_key(root: &Path) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(root.to_string_lossy().as_bytes());
    let name = root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(24)
        .collect();
    format!("{name}-{}", &hex::encode(digest)[..12])
}

/// Expands a leading `~` to the home directory.
pub fn expand_home(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\"))
        && let Some(home) = ::dirs::home_dir()
    {
        return home.join(rest);
    }
    if path == "~"
        && let Some(home) = ::dirs::home_dir()
    {
        return home;
    }
    PathBuf::from(path)
}

/// Writes a file atomically: temp file in the same directory, then rename.
pub fn write_atomic(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default(),
        std::process::id()
    ));
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path).inspect_err(|_| {
        let _ = std::fs::remove_file(&tmp);
    })
}
