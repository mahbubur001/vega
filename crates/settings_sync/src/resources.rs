//! What Settings Sync syncs, and where it keeps its own state on this machine.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use fs::Fs;
use gpui::SharedString;
use serde::{Deserialize, Serialize};

/// A file synced between machines.
pub(crate) struct Resource {
    /// The file name in the gist.
    pub gist_file: String,
    pub local_path: PathBuf,
    /// What an empty file means. GitHub rejects blank gist files.
    pub empty_contents: &'static str,
    /// Shown to the user.
    pub label: SharedString,
    /// Merged per top-level key with never-sync values kept local
    /// (`settings.json`); otherwise the whole file syncs.
    pub merge_keys: bool,
}

/// Single files that sync as a whole.
fn single_files() -> Vec<Resource> {
    vec![
        Resource {
            gist_file: "settings.json".into(),
            local_path: paths::settings_file().clone(),
            empty_contents: "{}\n",
            label: "Settings".into(),
            merge_keys: true,
        },
        // Keymaps differ between platforms (cmd vs ctrl), so each OS has its own.
        Resource {
            gist_file: format!("keymap.{}.json", std::env::consts::OS),
            local_path: paths::keymap_file().clone(),
            empty_contents: "[]\n",
            label: "Keymap".into(),
            merge_keys: false,
        },
        Resource {
            gist_file: "tasks.json".into(),
            local_path: paths::tasks_file().clone(),
            empty_contents: "[]\n",
            label: "Tasks".into(),
            merge_keys: false,
        },
        Resource {
            gist_file: "debug.json".into(),
            local_path: paths::debug_scenarios_file().clone(),
            empty_contents: "[]\n",
            label: "Debug scenarios".into(),
            merge_keys: false,
        },
        Resource {
            gist_file: "AGENTS.md".into(),
            local_path: paths::agents_file().clone(),
            empty_contents: "",
            label: "AGENTS.md".into(),
            merge_keys: false,
        },
    ]
}

/// Folders whose JSON files sync one by one. Gists are flat, so a file
/// `<dir>/<name>` is `<prefix>__<name>` in the gist.
fn folders() -> [(&'static str, &'static PathBuf, &'static str); 2] {
    [
        ("snippets", paths::snippets_dir(), "Snippets"),
        ("themes", paths::themes_dir(), "Themes"),
    ]
}

/// Everything that syncs: the single files, plus every file in the synced
/// folders that exists on this machine or on GitHub.
pub(crate) async fn resources(
    fs: &Arc<dyn Fs>,
    remote: &BTreeMap<String, String>,
) -> Result<Vec<Resource>> {
    let mut resources = single_files();
    for (prefix, dir, label) in folders() {
        let gist_prefix = format!("{prefix}__");
        let mut names = std::collections::BTreeSet::new();
        if fs.is_dir(dir).await {
            let mut entries = fs.read_dir(dir).await?;
            while let Some(entry) = futures::StreamExt::next(&mut entries).await {
                let path = entry?;
                if path
                    .extension()
                    .is_some_and(|extension| extension == "json")
                    && fs.is_file(&path).await
                    && let Some(name) = path.file_name().and_then(|name| name.to_str())
                {
                    names.insert(name.to_string());
                }
            }
        }
        for gist_file in remote.keys() {
            if let Some(name) = gist_file.strip_prefix(&gist_prefix)
                && is_plain_file_name(name)
            {
                names.insert(name.to_string());
            }
        }
        for name in names {
            resources.push(Resource {
                gist_file: format!("{gist_prefix}{name}"),
                local_path: dir.join(&name),
                empty_contents: "",
                label: format!("{label}: {name}").into(),
                merge_keys: false,
            });
        }
    }
    Ok(resources)
}

/// Paths to watch for local changes: the single files and the synced folders.
pub(crate) fn watched_paths() -> Vec<PathBuf> {
    single_files()
        .into_iter()
        .map(|resource| resource.local_path)
        .chain(folders().into_iter().map(|(_, dir, _)| dir.clone()))
        .collect()
}

/// A gist file name is only turned into a local path when it can't escape the folder.
fn is_plain_file_name(name: &str) -> bool {
    !name.is_empty()
        && name.ends_with(".json")
        && !name.contains(['/', '\\'])
        && name != "."
        && name != ".."
        && !name.starts_with('.')
}

pub(crate) fn state_dir() -> PathBuf {
    paths::data_dir().join("settings_sync")
}

/// Sync state for this machine. Never synced itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct SyncState {
    pub enabled: bool,
    /// The GitHub login the rest of the state belongs to.
    pub account: Option<String>,
    pub gist_id: Option<String>,
    pub etag: Option<String>,
    /// The gist commit `remote` came from.
    pub version: Option<String>,
    /// The gist's files as last seen.
    pub remote: BTreeMap<String, String>,
    /// Per gist file, the content both sides had after the last sync.
    pub bases: BTreeMap<String, String>,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            enabled: true,
            account: None,
            gist_id: None,
            etag: None,
            version: None,
            remote: BTreeMap::new(),
            bases: BTreeMap::new(),
        }
    }
}

impl SyncState {
    fn path() -> PathBuf {
        state_dir().join("state.json")
    }

    pub async fn load(fs: &Arc<dyn Fs>) -> Result<Self> {
        let path = Self::path();
        if !fs.is_file(&path).await {
            return Ok(Self::default());
        }
        let contents = fs.load(&path).await?;
        serde_json::from_str(&contents).context("parsing the Settings Sync state")
    }

    pub async fn save(&self, fs: &Arc<dyn Fs>) -> Result<()> {
        fs.create_dir(&state_dir()).await?;
        fs.atomic_write(Self::path(), serde_json::to_string_pretty(self)?)
            .await
    }

    /// Keeps only `enabled` when a different GitHub account signs in.
    pub fn for_account(mut self, login: &str) -> Self {
        if self.account.as_deref() != Some(login) {
            self = Self {
                enabled: self.enabled,
                account: Some(login.to_string()),
                ..Self::default()
            };
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gist_names_never_escape_the_folder() {
        assert!(is_plain_file_name("rust.json"));
        assert!(!is_plain_file_name("../settings.json"));
        assert!(!is_plain_file_name("a/b.json"));
        assert!(!is_plain_file_name("a\\b.json"));
        assert!(!is_plain_file_name(".hidden.json"));
        assert!(!is_plain_file_name("notes.txt"));
    }
}
