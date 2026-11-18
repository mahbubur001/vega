//! Everything the engine needs to ask or tell the user. The language server, the app and the CLI each implement it
//! with their own UI (LSP message requests / GPUI dialogs / terminal).

use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub use crate::known_hosts::{HostKeyAnswer, HostKeyQuestion, HostKeyQuestionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecretKind {
    Password,
    Passphrase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretRequest {
    pub kind: SecretKind,
    pub server_name: String,
    /// `user@host` or the key file path.
    pub target: String,
    /// Set when a previous attempt failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretAnswer {
    pub value: String,
    /// Save in the OS keychain.
    pub save: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertAnswer {
    /// Pin the fingerprint in the server config.
    Pin,
    Once,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictQuestion {
    pub server_name: String,
    pub local_path: String,
    pub remote_path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConflictDecision {
    Overwrite,
    /// Show a diff, then ask again (without this option).
    Compare,
    Skip,
    /// Stop the whole operation.
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Info,
    Warning,
    Error,
}

#[async_trait]
pub trait Prompter: Send + Sync {
    async fn secret(&self, req: &SecretRequest) -> Option<SecretAnswer>;
    async fn host_key(&self, q: &HostKeyQuestion) -> HostKeyAnswer;
    async fn certificate(&self, host: &str, fingerprint: &str, reason: &str) -> CertAnswer;
    async fn keyboard_interactive(
        &self,
        name: &str,
        instructions: &str,
        prompts: &[(String, bool)],
    ) -> Option<Vec<String>>;
    /// `allow_compare` is false when asking again after a comparison.
    async fn conflict(&self, q: &ConflictQuestion, allow_compare: bool) -> ConflictDecision;
    async fn confirm(&self, title: &str, message: &str, ok_label: &str) -> bool;
    /// Shows a diff (left = remote/old, right = local/new).
    async fn show_diff(&self, left: &Path, right: &Path, title: &str);
    fn notify(&self, level: Level, message: &str);
}

/// Non-interactive prompter: refuses everything that needs a human (CI, tests, background jobs).
pub struct NoPrompt;

#[async_trait]
impl Prompter for NoPrompt {
    async fn secret(&self, _: &SecretRequest) -> Option<SecretAnswer> {
        None
    }
    async fn host_key(&self, _: &HostKeyQuestion) -> HostKeyAnswer {
        HostKeyAnswer::Reject
    }
    async fn certificate(&self, _: &str, _: &str, _: &str) -> CertAnswer {
        CertAnswer::Reject
    }
    async fn keyboard_interactive(
        &self,
        _: &str,
        _: &str,
        _: &[(String, bool)],
    ) -> Option<Vec<String>> {
        None
    }
    async fn conflict(&self, _: &ConflictQuestion, _: bool) -> ConflictDecision {
        ConflictDecision::Skip
    }
    async fn confirm(&self, _: &str, _: &str, _: &str) -> bool {
        false
    }
    async fn show_diff(&self, _: &Path, _: &Path, _: &str) {}
    fn notify(&self, level: Level, message: &str) {
        match level {
            Level::Error => log::error!("{message}"),
            Level::Warning => log::warn!("{message}"),
            Level::Info => log::info!("{message}"),
        }
    }
}
