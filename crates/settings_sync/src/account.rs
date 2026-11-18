//! The GitHub account used by Settings Sync.

use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use credentials_provider::CredentialsProvider;
use gpui::{App, AppContext as _, AsyncApp, Context, Entity, Global, SharedString, Task};
use http_client::HttpClient;

use crate::github::{self, DeviceFlow, GitHubUser};

/// Keychain entry holding the GitHub token. The username field stores the
/// GitHub login, so the account still shows while offline.
const CREDENTIALS_URL: &str = "https://github.com/vega-settings-sync";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountStatus {
    /// Still loading a previously stored token.
    Starting,
    SignedOut,
    /// `device_flow` is `None` while the code is being requested from GitHub.
    SigningIn {
        device_flow: Option<DeviceFlow>,
    },
    SignedIn(GitHubUser),
    Error(SharedString),
}

pub struct GitHubAccount {
    status: AccountStatus,
    token: Option<String>,
    http_client: Arc<dyn HttpClient>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    _load_task: Task<()>,
    sign_in_task: Option<Task<()>>,
}

struct GlobalGitHubAccount(Entity<GitHubAccount>);

impl Global for GlobalGitHubAccount {}

impl GitHubAccount {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalGitHubAccount>()
            .map(|account| account.0.clone())
    }

    pub(crate) fn init(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) {
        let account = cx.new(|cx| Self::new(http_client, credentials_provider, cx));
        cx.set_global(GlobalGitHubAccount(account));
    }

    fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut Context<Self>,
    ) -> Self {
        let load_task = cx.spawn({
            let http_client = http_client.clone();
            let credentials_provider = credentials_provider.clone();
            async move |this, cx| {
                let result =
                    Self::load_stored_account(&http_client, &credentials_provider, cx).await;
                this.update(cx, |this, cx| {
                    match result {
                        Ok(Some((user, token))) => {
                            this.token = Some(token);
                            this.status = AccountStatus::SignedIn(user);
                        }
                        Ok(None) => this.status = AccountStatus::SignedOut,
                        Err(error) => {
                            log::error!("loading GitHub account for Settings Sync: {error:#}");
                            this.status = AccountStatus::SignedOut;
                        }
                    }
                    cx.notify();
                })
                .ok();
            }
        });

        Self {
            status: AccountStatus::Starting,
            token: None,
            http_client,
            credentials_provider,
            _load_task: load_task,
            sign_in_task: None,
        }
    }

    async fn load_stored_account(
        http_client: &Arc<dyn HttpClient>,
        credentials_provider: &Arc<dyn CredentialsProvider>,
        cx: &AsyncApp,
    ) -> Result<Option<(GitHubUser, String)>> {
        let Some((login, token)) = credentials_provider
            .read_credentials(CREDENTIALS_URL, cx)
            .await?
        else {
            return Ok(None);
        };
        let token = String::from_utf8(token).context("GitHub token isn't valid UTF-8")?;

        match github::fetch_user(http_client, &token).await {
            Ok(Some(user)) => Ok(Some((user, token))),
            Ok(None) => {
                log::info!("GitHub rejected the stored Settings Sync token; signing out");
                credentials_provider
                    .delete_credentials(CREDENTIALS_URL, cx)
                    .await?;
                Ok(None)
            }
            // Offline or GitHub is unavailable: keep the account and show the
            // stored login.
            Err(error) => {
                log::warn!("couldn't reach GitHub to refresh the account: {error:#}");
                Ok(Some((offline_user(login), token)))
            }
        }
    }

    pub fn status(&self) -> &AccountStatus {
        &self.status
    }

    pub fn user(&self) -> Option<&GitHubUser> {
        match &self.status {
            AccountStatus::SignedIn(user) => Some(user),
            _ => None,
        }
    }

    /// The OAuth token, available while signed in.
    pub(crate) fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Begins the GitHub device flow. Progress is reported through
    /// [`Self::status`]; observe this entity to react to changes.
    pub fn sign_in(&mut self, cx: &mut Context<Self>) {
        if matches!(
            self.status,
            AccountStatus::SigningIn { .. } | AccountStatus::SignedIn(_)
        ) {
            return;
        }

        let Some(client_id) = github::client_id() else {
            self.status = AccountStatus::Error(
                "Settings Sync isn't configured in this build: no GitHub client ID.".into(),
            );
            cx.notify();
            return;
        };

        self.status = AccountStatus::SigningIn { device_flow: None };
        cx.notify();

        let http_client = self.http_client.clone();
        let credentials_provider = self.credentials_provider.clone();
        let executor = cx.background_executor().clone();
        self.sign_in_task = Some(cx.spawn(async move |this, cx| {
            let result = async {
                let device_flow = github::request_device_code(&http_client, &client_id).await?;
                this.update(cx, |this, cx| {
                    this.status = AccountStatus::SigningIn {
                        device_flow: Some(device_flow.clone()),
                    };
                    cx.notify();
                })?;

                let token = github::poll_for_access_token(
                    &http_client,
                    &client_id,
                    &device_flow,
                    &executor,
                )
                .await?;
                let user = github::fetch_user(&http_client, &token)
                    .await?
                    .ok_or_else(|| anyhow!("GitHub rejected the new token"))?;

                credentials_provider
                    .write_credentials(CREDENTIALS_URL, &user.login, token.as_bytes(), cx)
                    .await
                    .context("saving the GitHub token to the keychain")?;

                this.update(cx, |this, cx| {
                    this.token = Some(token);
                    this.status = AccountStatus::SignedIn(user);
                    cx.notify();
                })?;
                anyhow::Ok(())
            }
            .await;

            if let Err(error) = result {
                log::error!("GitHub sign-in failed: {error:#}");
                this.update(cx, |this, cx| {
                    this.status = AccountStatus::Error(error.to_string().into());
                    cx.notify();
                })
                .ok();
            }
        }));
    }

    /// Stops an in-progress sign-in, or clears an error.
    pub fn cancel_sign_in(&mut self, cx: &mut Context<Self>) {
        if matches!(
            self.status,
            AccountStatus::SigningIn { .. } | AccountStatus::Error(_)
        ) {
            self.sign_in_task = None;
            self.status = AccountStatus::SignedOut;
            cx.notify();
        }
    }

    /// Forgets the token on this machine. The OAuth grant itself can only be
    /// revoked by the user on github.com.
    pub fn sign_out(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.token = None;
        self.sign_in_task = None;
        self.status = AccountStatus::SignedOut;
        cx.notify();

        let credentials_provider = self.credentials_provider.clone();
        cx.spawn(async move |_, cx| {
            credentials_provider
                .delete_credentials(CREDENTIALS_URL, cx)
                .await
                .context("deleting the GitHub token from the keychain")
        })
    }
}

fn offline_user(login: String) -> GitHubUser {
    GitHubUser {
        avatar_url: format!("https://github.com/{login}.png"),
        login,
        name: None,
    }
}
