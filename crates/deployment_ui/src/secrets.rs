//! Persists deployment secrets through VEGA's credentials provider (the OS
//! keychain, or the development provider in dev builds).
//!
//! The engine runs on Tokio, but the provider needs the app, so each request is
//! sent to a foreground task that answers through a oneshot channel, like
//! [`crate::prompter`].

use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use credentials_provider::CredentialsProvider;
use deployment::credentials::{Credentials, SecretStore, SecretStoreError};
use futures::StreamExt as _;
use futures::channel::{mpsc, oneshot};
use gpui::{App, AsyncApp, Global};

/// Keychain entries are named `vega-deployment:<account>`, e.g.
/// `vega-deployment:password:sftp://deploy@example.com:22`.
const URL_PREFIX: &str = "vega-deployment:";
const USERNAME: &str = "deployment";

fn url(account: &str) -> String {
    format!("{URL_PREFIX}{account}")
}

enum Request {
    Read {
        account: String,
        reply: oneshot::Sender<anyhow::Result<Option<String>>>,
    },
    Write {
        account: String,
        secret: String,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Delete {
        account: String,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
}

struct ProviderSecretStore {
    requests: mpsc::UnboundedSender<Request>,
}

impl ProviderSecretStore {
    async fn send<T>(
        &self,
        request: impl FnOnce(oneshot::Sender<anyhow::Result<T>>) -> Request,
    ) -> Result<T, SecretStoreError> {
        let (reply, response) = oneshot::channel();
        self.requests
            .unbounded_send(request(reply))
            .map_err(|_| "VEGA's credentials provider has shut down")?;
        Ok(response
            .await
            .map_err(|_| "VEGA's credentials provider has shut down")??)
    }
}

#[async_trait]
impl SecretStore for ProviderSecretStore {
    async fn read(&self, account: &str) -> Result<Option<String>, SecretStoreError> {
        let account = account.to_string();
        self.send(|reply| Request::Read { account, reply }).await
    }

    async fn write(&self, account: &str, secret: &str) -> Result<(), SecretStoreError> {
        let (account, secret) = (account.to_string(), secret.to_string());
        self.send(|reply| Request::Write {
            account,
            secret,
            reply,
        })
        .await
    }

    async fn delete(&self, account: &str) -> Result<(), SecretStoreError> {
        let account = account.to_string();
        self.send(|reply| Request::Delete { account, reply }).await
    }
}

async fn handle(request: Request, provider: &dyn CredentialsProvider, cx: &AsyncApp) {
    match request {
        Request::Read { account, reply } => {
            let result = provider
                .read_credentials(&url(&account), cx)
                .await
                .and_then(|credentials| {
                    credentials
                        .map(|(_, secret)| {
                            String::from_utf8(secret).context("saved secret is not UTF-8")
                        })
                        .transpose()
                });
            reply.send(result).ok();
        }
        Request::Write {
            account,
            secret,
            reply,
        } => {
            let result = provider
                .write_credentials(&url(&account), USERNAME, secret.as_bytes(), cx)
                .await;
            reply.send(result).ok();
        }
        Request::Delete { account, reply } => {
            let result = provider.delete_credentials(&url(&account), cx).await;
            reply.send(result).ok();
        }
    }
}

/// A [`SecretStore`] that forwards to `provider` on the app's foreground thread.
fn provider_secret_store(
    provider: Arc<dyn CredentialsProvider>,
    cx: &mut App,
) -> ProviderSecretStore {
    let (requests, mut receiver) = mpsc::unbounded();
    cx.spawn(async move |cx| {
        while let Some(request) = receiver.next().await {
            handle(request, provider.as_ref(), cx).await;
        }
    })
    .detach();
    ProviderSecretStore { requests }
}

struct GlobalCredentials(Arc<Credentials>);

impl Global for GlobalCredentials {}

/// The app-wide deployment [`Credentials`], shared by every project so a
/// password entered once is remembered for the session.
pub(crate) fn credentials(cx: &mut App) -> Arc<Credentials> {
    if let Some(credentials) = cx.try_global::<GlobalCredentials>() {
        return credentials.0.clone();
    }
    let provider = zed_credentials_provider::global(cx);
    let store = provider_secret_store(provider, cx);
    let credentials = Arc::new(Credentials::new(Arc::new(store)));
    cx.set_global(GlobalCredentials(credentials.clone()));
    credentials
}

#[cfg(test)]
mod tests {
    use credentials_provider::test_support::FakeCredentialsProvider;
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    async fn round_trips_through_the_credentials_provider(cx: &mut TestAppContext) {
        let provider = Arc::new(FakeCredentialsProvider::default());
        let store = cx.update(|cx| provider_secret_store(provider.clone(), cx));
        let account = "password:sftp://deploy@example.com:22";

        assert_eq!(store.read(account).await.unwrap(), None);
        store.write(account, "s3cret").await.unwrap();
        assert!(
            provider
                .state
                .lock()
                .keys
                .contains_key("vega-deployment:password:sftp://deploy@example.com:22"),
            "stored under VEGA's own keychain name"
        );
        assert_eq!(
            store.read(account).await.unwrap().as_deref(),
            Some("s3cret")
        );
        store.delete(account).await.unwrap();
        assert_eq!(store.read(account).await.unwrap(), None);
    }
}
