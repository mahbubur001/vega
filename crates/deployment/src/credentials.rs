//! Secrets: passwords and key passphrases. Never written to config files, logs or process arguments.
//!
//! VEGA stores them through a [`SecretStore`], normally VEGA's credentials provider (OS keychain).
//! iRemote kept them in the keychain under the `iremote` service; when VEGA's store has no entry for
//! an account, that entry is read once, copied into VEGA's store, and remembered as imported so it is
//! never consulted again for that account (a password later deleted for being wrong doesn't come
//! back). iRemote's own entry is left untouched. Without a store, secrets live for the session only.
//!
//! Accounts are keyed by connection identity, not server id, so the same login is shared across projects:
//! `password:sftp://deploy@example.com:22`, `passphrase:/Users/me/.ssh/id_ed25519`.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;

use crate::config::{Server, ServerType};

/// The keychain service iRemote stored its secrets under.
const IREMOTE_SERVICE: &str = "iremote";

pub type SecretStoreError = Box<dyn std::error::Error + Send + Sync>;

/// Where secrets are persisted.
#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn read(&self, account: &str) -> Result<Option<String>, SecretStoreError>;
    async fn write(&self, account: &str, secret: &str) -> Result<(), SecretStoreError>;
    async fn delete(&self, account: &str) -> Result<(), SecretStoreError>;
}

pub fn password_account(server: &Server) -> String {
    let scheme = match server.kind {
        ServerType::Sftp => "sftp",
        ServerType::Ftp => "ftp",
        ServerType::Ftps | ServerType::FtpsImplicit => "ftps",
        ServerType::Webdav => {
            return format!("password:webdav:{}@{}", server.username, server.host);
        }
        _ => "other",
    };
    format!(
        "password:{scheme}://{}@{}:{}",
        server.username,
        server.host,
        server.port_or_default()
    )
}

pub fn passphrase_account(key_path: &str) -> String {
    format!(
        "passphrase:{}",
        crate::dirs::expand_home(key_path).display()
    )
}

/// Accounts already imported from iRemote, persisted as a JSON list of account names (no secrets).
struct ImportedAccounts {
    path: Option<PathBuf>,
    accounts: Mutex<HashSet<String>>,
}

impl ImportedAccounts {
    fn load(path: Option<PathBuf>) -> Self {
        let accounts = path
            .as_ref()
            .and_then(|path| std::fs::read(path).ok())
            .and_then(|bytes| serde_json::from_slice::<HashSet<String>>(&bytes).ok())
            .unwrap_or_default();
        Self {
            path,
            accounts: Mutex::new(accounts),
        }
    }

    fn contains(&self, account: &str) -> bool {
        self.accounts
            .lock()
            .is_ok_and(|accounts| accounts.contains(account))
    }

    fn insert(&self, account: &str) {
        let Ok(mut accounts) = self.accounts.lock() else {
            return;
        };
        if !accounts.insert(account.to_string()) {
            return;
        }
        let Some(path) = &self.path else {
            return;
        };
        let mut sorted: Vec<_> = accounts.iter().collect();
        sorted.sort();
        let result = path
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| {
                std::fs::write(path, serde_json::to_vec_pretty(&sorted).unwrap_or_default())
            });
        if let Err(error) = result {
            log::warn!("Cannot record imported iRemote secrets in {path:?}: {error}");
        }
    }
}

pub struct Credentials {
    session: Mutex<HashMap<String, String>>,
    store: Option<Arc<dyn SecretStore>>,
    legacy: Option<Arc<dyn SecretStore>>,
    imported: ImportedAccounts,
}

impl Credentials {
    /// Persists to `store`, with a one-time fallback to iRemote's keychain entries. Set
    /// `IREMOTE_KEYRING=memory` to skip the fallback.
    pub fn new(store: Arc<dyn SecretStore>) -> Self {
        let legacy = KeyringStore::iremote().map(|store| Arc::new(store) as Arc<dyn SecretStore>);
        Self::with_stores(
            Some(store),
            legacy,
            Some(crate::dirs::state_dir().join("imported-iremote-secrets.json")),
        )
    }

    /// Session-only store (tests).
    pub fn memory() -> Self {
        Self::with_stores(None, None, None)
    }

    /// `imported_path` remembers which accounts were already read from `legacy`; `None` keeps that
    /// for the session only.
    pub fn with_stores(
        store: Option<Arc<dyn SecretStore>>,
        legacy: Option<Arc<dyn SecretStore>>,
        imported_path: Option<PathBuf>,
    ) -> Self {
        Self {
            session: Mutex::default(),
            store,
            legacy,
            imported: ImportedAccounts::load(imported_path),
        }
    }

    pub fn has_keychain(&self) -> bool {
        self.store.is_some()
    }

    pub async fn get(&self, account: &str) -> Option<String> {
        if let Some(secret) = self
            .session
            .lock()
            .ok()
            .and_then(|session| session.get(account).cloned())
        {
            return Some(secret);
        }
        let store = self.store.as_ref()?;
        match store.read(account).await {
            Ok(Some(secret)) => {
                self.remember(account, &secret);
                return Some(secret);
            }
            Ok(None) => {}
            Err(error) => {
                log::warn!("Cannot read from the keychain: {error}");
                return None;
            }
        }
        let secret = self.import_from_iremote(store, account).await?;
        self.remember(account, &secret);
        Some(secret)
    }

    /// Stores for the session, and persistently when `persist`.
    pub async fn set(&self, account: &str, secret: &str, persist: bool) {
        self.remember(account, secret);
        if persist
            && let Some(store) = &self.store
            && let Err(error) = store.write(account, secret).await
        {
            log::warn!("Cannot save to the keychain: {error}");
        }
    }

    pub async fn delete(&self, account: &str) {
        if let Ok(mut session) = self.session.lock() {
            session.remove(account);
        }
        if let Some(store) = &self.store
            && let Err(error) = store.delete(account).await
        {
            log::warn!("Cannot delete from the keychain: {error}");
        }
    }

    fn remember(&self, account: &str, secret: &str) {
        if let Ok(mut session) = self.session.lock() {
            session.insert(account.to_string(), secret.to_string());
        }
    }

    /// Reads `account` from iRemote's keychain entries the first time it is missing from `store`,
    /// and copies it over.
    async fn import_from_iremote(
        &self,
        store: &Arc<dyn SecretStore>,
        account: &str,
    ) -> Option<String> {
        let legacy = self.legacy.as_ref()?;
        if self.imported.contains(account) {
            return None;
        }
        let secret = match legacy.read(account).await {
            Ok(secret) => secret,
            Err(error) => {
                log::warn!("Cannot read iRemote's keychain entry: {error}");
                return None;
            }
        };
        self.imported.insert(account);
        let secret = secret?;
        if let Err(error) = store.write(account, &secret).await {
            log::warn!("Cannot copy iRemote's secret into VEGA's keychain: {error}");
        }
        Some(secret)
    }
}

/// The OS keychain via `keyring-core`: macOS Keychain, Windows Credential Manager, Secret Service on
/// Linux. Only used to read iRemote's entries.
pub struct KeyringStore {
    service: &'static str,
}

impl KeyringStore {
    /// iRemote's entries, or `None` without a system keychain or with `IREMOTE_KEYRING=memory`.
    pub fn iremote() -> Option<Self> {
        init_keyring().then_some(Self {
            service: IREMOTE_SERVICE,
        })
    }
}

#[async_trait]
impl SecretStore for KeyringStore {
    async fn read(&self, account: &str) -> Result<Option<String>, SecretStoreError> {
        let (service, account) = (self.service, account.to_string());
        let result = tokio::task::spawn_blocking(move || {
            keyring_core::Entry::new(service, &account).and_then(|entry| entry.get_password())
        })
        .await?;
        match result {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring_core::Error::NoEntry) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    async fn write(&self, account: &str, secret: &str) -> Result<(), SecretStoreError> {
        let (service, account, secret) = (self.service, account.to_string(), secret.to_string());
        tokio::task::spawn_blocking(move || {
            keyring_core::Entry::new(service, &account)
                .and_then(|entry| entry.set_password(&secret))
        })
        .await??;
        Ok(())
    }

    async fn delete(&self, account: &str) -> Result<(), SecretStoreError> {
        let (service, account) = (self.service, account.to_string());
        let result = tokio::task::spawn_blocking(move || {
            keyring_core::Entry::new(service, &account).and_then(|entry| entry.delete_credential())
        })
        .await?;
        match result {
            Ok(()) | Err(keyring_core::Error::NoEntry) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
}

static KEYRING_INIT: OnceLock<bool> = OnceLock::new();

fn init_keyring() -> bool {
    *KEYRING_INIT.get_or_init(|| {
        if std::env::var("IREMOTE_KEYRING").is_ok_and(|v| v == "memory") {
            return false;
        }
        #[cfg(target_os = "macos")]
        let store = apple_native_keyring_store::keychain::Store::new()
            .map(|s| s as std::sync::Arc<keyring_core::CredentialStore>);
        #[cfg(windows)]
        let store = windows_native_keyring_store::Store::new()
            .map(|s| s as std::sync::Arc<keyring_core::CredentialStore>);
        #[cfg(target_os = "linux")]
        let store = zbus_secret_service_keyring_store::Store::new()
            .map(|s| s as std::sync::Arc<keyring_core::CredentialStore>);
        #[cfg(not(any(target_os = "macos", windows, target_os = "linux")))]
        let store: Result<
            std::sync::Arc<keyring_core::CredentialStore>,
            keyring_core::Error,
        > = Err(keyring_core::Error::NoDefaultStore);
        match store {
            Ok(store) => {
                keyring_core::set_default_store(store);
                true
            }
            Err(e) => {
                log::warn!(
                    "No system keychain ({e}); iRemote's saved passwords can't be imported."
                );
                false
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An in-memory [`SecretStore`] that counts reads.
    #[derive(Default)]
    struct FakeStore {
        secrets: Mutex<HashMap<String, String>>,
        reads: Mutex<usize>,
    }

    impl FakeStore {
        fn with(account: &str, secret: &str) -> Arc<Self> {
            let store = Self::default();
            store
                .secrets
                .lock()
                .unwrap()
                .insert(account.into(), secret.into());
            Arc::new(store)
        }

        fn secret(&self, account: &str) -> Option<String> {
            self.secrets.lock().unwrap().get(account).cloned()
        }

        fn reads(&self) -> usize {
            *self.reads.lock().unwrap()
        }
    }

    #[async_trait]
    impl SecretStore for FakeStore {
        async fn read(&self, account: &str) -> Result<Option<String>, SecretStoreError> {
            *self.reads.lock().unwrap() += 1;
            Ok(self.secret(account))
        }

        async fn write(&self, account: &str, secret: &str) -> Result<(), SecretStoreError> {
            self.secrets
                .lock()
                .unwrap()
                .insert(account.into(), secret.into());
            Ok(())
        }

        async fn delete(&self, account: &str) -> Result<(), SecretStoreError> {
            self.secrets.lock().unwrap().remove(account);
            Ok(())
        }
    }

    #[tokio::test]
    async fn session_store() {
        let c = Credentials::memory();
        assert_eq!(c.get("a").await, None);
        c.set("a", "s3cret", true).await;
        assert_eq!(c.get("a").await.as_deref(), Some("s3cret"));
        c.delete("a").await;
        assert_eq!(c.get("a").await, None);
    }

    #[tokio::test]
    async fn persists_to_the_store() {
        let store = Arc::new(FakeStore::default());
        let c = Credentials::with_stores(Some(store.clone()), None, None);
        c.set("a", "s3cret", false).await;
        assert_eq!(store.secret("a"), None);
        c.set("a", "s3cret", true).await;
        assert_eq!(store.secret("a").as_deref(), Some("s3cret"));
        c.delete("a").await;
        assert_eq!(store.secret("a"), None);
    }

    #[tokio::test]
    async fn imports_from_iremote_once() {
        let dir = tempfile::tempdir().unwrap();
        let imported_path = dir.path().join("imported.json");
        let store = Arc::new(FakeStore::default());
        let legacy = FakeStore::with("a", "old");

        let c = Credentials::with_stores(
            Some(store.clone()),
            Some(legacy.clone()),
            Some(imported_path.clone()),
        );
        assert_eq!(c.get("a").await.as_deref(), Some("old"));
        assert_eq!(
            store.secret("a").as_deref(),
            Some("old"),
            "copied into VEGA's store"
        );
        assert_eq!(
            legacy.secret("a").as_deref(),
            Some("old"),
            "iRemote's entry is left alone"
        );

        // The password turns out to be wrong and is deleted; iRemote's copy must not come back,
        // even in a later session.
        c.delete("a").await;
        let later = Credentials::with_stores(
            Some(store.clone()),
            Some(legacy.clone()),
            Some(imported_path),
        );
        assert_eq!(later.get("a").await, None);
        assert_eq!(legacy.reads(), 1);
    }

    #[tokio::test]
    async fn vega_store_wins_over_iremote() {
        let store = FakeStore::with("a", "new");
        let legacy = FakeStore::with("a", "old");
        let c = Credentials::with_stores(Some(store), Some(legacy.clone()), None);
        assert_eq!(c.get("a").await.as_deref(), Some("new"));
        assert_eq!(legacy.reads(), 0);
    }

    #[test]
    fn accounts() {
        let mut s = Server::new("p", "P", ServerType::Sftp);
        s.host = "example.com".into();
        s.username = "deploy".into();
        assert_eq!(
            password_account(&s),
            "password:sftp://deploy@example.com:22"
        );
    }
}
