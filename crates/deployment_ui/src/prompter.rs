//! Answers the deployment engine's questions with VEGA's dialogs.
//!
//! The engine runs on Tokio and asks through the [`Prompter`] trait. Each
//! question is sent to the workspace window, which shows it and replies
//! through a oneshot channel.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use deployment::known_hosts::{HostKeyAnswer, HostKeyQuestion, HostKeyQuestionKind};
use deployment::prompt::{
    CertAnswer, ConflictDecision, ConflictQuestion, Level, Prompter, SecretAnswer, SecretKind,
    SecretRequest,
};
use futures::channel::{mpsc, oneshot};

pub(crate) enum Request {
    Secret {
        request: SecretRequest,
        reply: oneshot::Sender<Option<SecretAnswer>>,
    },
    /// A question answered by picking one of `answers`; replies with the index.
    Choice {
        level: Level,
        message: String,
        detail: String,
        answers: Vec<String>,
        reply: oneshot::Sender<usize>,
    },
    Diff {
        old: PathBuf,
        new: PathBuf,
        reply: oneshot::Sender<()>,
    },
    Notify {
        level: Level,
        message: String,
    },
}

pub(crate) struct WindowPrompter {
    requests: mpsc::UnboundedSender<Request>,
}

impl WindowPrompter {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<Request>) {
        let (requests, receiver) = mpsc::unbounded();
        (Self { requests }, receiver)
    }

    /// Asks a multiple-choice question. Returns `None` if the window is gone.
    async fn choose(
        &self,
        level: Level,
        message: String,
        detail: String,
        answers: &[&str],
    ) -> Option<usize> {
        let (reply, response) = oneshot::channel();
        self.requests
            .unbounded_send(Request::Choice {
                level,
                message,
                detail,
                answers: answers.iter().map(|answer| answer.to_string()).collect(),
                reply,
            })
            .ok()?;
        response.await.ok()
    }
}

#[async_trait]
impl Prompter for WindowPrompter {
    async fn secret(&self, request: &SecretRequest) -> Option<SecretAnswer> {
        let (reply, response) = oneshot::channel();
        self.requests
            .unbounded_send(Request::Secret {
                request: request.clone(),
                reply,
            })
            .ok()?;
        response.await.ok().flatten()
    }

    async fn host_key(&self, question: &HostKeyQuestion) -> HostKeyAnswer {
        let (message, level, answers): (String, Level, &[&str]) = match question.kind {
            HostKeyQuestionKind::Revoked => (
                format!("The host key of {} has been revoked.", question.host),
                Level::Error,
                &["Cancel"],
            ),
            HostKeyQuestionKind::Changed => (
                format!(
                    "The host key of {} has changed. Someone could be intercepting the connection.",
                    question.host
                ),
                Level::Error,
                &["Trust Once", "Cancel"],
            ),
            HostKeyQuestionKind::Unknown => (
                format!("{} is not a known host.", question.host),
                Level::Warning,
                &["Trust and Remember", "Trust Once", "Cancel"],
            ),
        };
        let detail = format!(
            "{} key fingerprint:\n{}",
            question.key_type, question.fingerprint
        );
        let choice = self.choose(level, message, detail, answers).await;
        match (question.kind, choice) {
            (HostKeyQuestionKind::Unknown, Some(0)) => HostKeyAnswer::Remember,
            (HostKeyQuestionKind::Unknown, Some(1)) | (HostKeyQuestionKind::Changed, Some(0)) => {
                HostKeyAnswer::Once
            }
            _ => HostKeyAnswer::Reject,
        }
    }

    async fn certificate(&self, host: &str, fingerprint: &str, reason: &str) -> CertAnswer {
        let message = format!("The TLS certificate of {host} isn't trusted: {reason}");
        let detail = format!("SHA-256 fingerprint:\n{fingerprint}");
        match self
            .choose(
                Level::Warning,
                message,
                detail,
                &["Trust and Pin", "Trust Once", "Cancel"],
            )
            .await
        {
            Some(0) => CertAnswer::Pin,
            Some(1) => CertAnswer::Once,
            _ => CertAnswer::Reject,
        }
    }

    async fn keyboard_interactive(
        &self,
        name: &str,
        instructions: &str,
        prompts: &[(String, bool)],
    ) -> Option<Vec<String>> {
        let mut answers = Vec::with_capacity(prompts.len());
        for (prompt, _echo) in prompts {
            let label = if instructions.is_empty() {
                prompt.clone()
            } else {
                format!("{instructions}\n{prompt}")
            };
            let answer = self
                .secret(&SecretRequest {
                    kind: SecretKind::Password,
                    server_name: name.to_string(),
                    target: label,
                    error: None,
                })
                .await?;
            answers.push(answer.value);
        }
        Some(answers)
    }

    async fn conflict(&self, question: &ConflictQuestion, allow_compare: bool) -> ConflictDecision {
        let message = format!(
            "{} on {} is newer than your local copy.",
            question.remote_path, question.server_name
        );
        let answers: &[&str] = if allow_compare {
            &["Overwrite", "Compare", "Skip", "Cancel"]
        } else {
            &["Overwrite", "Skip", "Cancel"]
        };
        let choice = self
            .choose(Level::Warning, message, question.reason.clone(), answers)
            .await;
        match (choice, allow_compare) {
            (Some(0), _) => ConflictDecision::Overwrite,
            (Some(1), true) => ConflictDecision::Compare,
            (Some(1), false) | (Some(2), true) => ConflictDecision::Skip,
            _ => ConflictDecision::Cancel,
        }
    }

    async fn confirm(&self, title: &str, message: &str, ok_label: &str) -> bool {
        self.choose(
            Level::Warning,
            title.to_string(),
            message.to_string(),
            &[ok_label, "Cancel"],
        )
        .await
            == Some(0)
    }

    async fn show_diff(&self, left: &Path, right: &Path, _title: &str) {
        let (reply, response) = oneshot::channel();
        let sent = self.requests.unbounded_send(Request::Diff {
            old: left.to_path_buf(),
            new: right.to_path_buf(),
            reply,
        });
        if sent.is_ok() {
            response.await.ok();
        }
    }

    fn notify(&self, level: Level, message: &str) {
        self.requests
            .unbounded_send(Request::Notify {
                level,
                message: message.to_string(),
            })
            .ok();
    }
}
