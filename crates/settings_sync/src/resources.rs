//! What Settings Sync syncs, and where it keeps its own state on this machine.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use fs::Fs;
use serde::{Deserialize, Serialize};

/// A file synced between machines.
pub(crate) struct Resource {
    /// The file name in the gist.
    pub gist_file: String,
    pub local_path: PathBuf,
    /// What an empty file means. GitHub rejects blank gist files.
    pub empty_contents: &'static str,
    /// Shown to the user.
    pub label: &'static str,
    /// Merged per top-level key with never-sync values kept local
    /// (`settings.json`); otherwise the whole file syncs.
    pub merge_keys: bool,
}

pub(crate) fn resources() -> Vec<Resource> {
    vec![
        Resource {
            gist_file: "settings.json".into(),
            local_path: paths::settings_file().clone(),
            empty_contents: "{}\n",
            label: "Settings",
            merge_keys: true,
        },
        // Keymaps differ between platforms (cmd vs ctrl), so each OS has its own.
        Resource {
            gist_file: format!("keymap.{}.json", std::env::consts::OS),
            local_path: paths::keymap_file().clone(),
            empty_contents: "[]\n",
            label: "Keymap",
            merge_keys: false,
        },
    ]
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
