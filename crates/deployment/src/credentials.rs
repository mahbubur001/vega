//! Secrets in the OS keychain (service `iremote`): macOS Keychain, Windows Credential Manager, Secret Service on
//! Linux. Never written to config files, logs or process arguments. Without a keychain (or with
//! `IREMOTE_KEYRING=memory`) secrets live for the session only.
//!
//! Accounts are keyed by connection identity, not server id, so the same login is shared across projects:
//! `password:sftp://deploy@example.com:22`, `passphrase:/Users/me/.ssh/id_ed25519`.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::config::{Server, ServerType};

const SERVICE: &str = "iremote";

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

pub struct Credentials {
    session: Mutex<HashMap<String, String>>,
    keychain: bool,
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
                log::warn!("No system keychain ({e}); passwords are kept for this session only.");
                false
            }
        }
    })
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new()
    }
}

impl Credentials {
    pub fn new() -> Self {
        Self {
            session: Mutex::default(),
            keychain: init_keyring(),
        }
    }

    /// Session-only store (tests).
    pub fn memory() -> Self {
        Self {
            session: Mutex::default(),
            keychain: false,
        }
    }

    pub fn has_keychain(&self) -> bool {
        self.keychain
    }

    pub async fn get(&self, account: &str) -> Option<String> {
        if let Some(v) = self
            .session
            .lock()
            .ok()
            .and_then(|s| s.get(account).cloned())
        {
            return Some(v);
        }
        if !self.keychain {
            return None;
        }
        let account = account.to_string();
        tokio::task::spawn_blocking(move || {
            keyring_core::Entry::new(SERVICE, &account)
                .and_then(|e| e.get_password())
                .ok()
        })
        .await
        .ok()
        .flatten()
    }

    /// Stores for the session, and in the keychain when `persist`.
    pub async fn set(&self, account: &str, secret: &str, persist: bool) {
        if let Ok(mut s) = self.session.lock() {
            s.insert(account.to_string(), secret.to_string());
        }
        if persist && self.keychain {
            let (account, secret) = (account.to_string(), secret.to_string());
            let res = tokio::task::spawn_blocking(move || {
                keyring_core::Entry::new(SERVICE, &account).and_then(|e| e.set_password(&secret))
            })
            .await;
            if let Ok(Err(e)) = res {
                log::warn!("Cannot save to the keychain: {e}");
            }
        }
    }

    pub async fn delete(&self, account: &str) {
        if let Ok(mut s) = self.session.lock() {
            s.remove(account);
        }
        if self.keychain {
            let account = account.to_string();
            let _ = tokio::task::spawn_blocking(move || {
                keyring_core::Entry::new(SERVICE, &account).and_then(|e| e.delete_credential())
            })
            .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn session_store() {
        let c = Credentials::memory();
        assert_eq!(c.get("a").await, None);
        c.set("a", "s3cret", true).await;
        assert_eq!(c.get("a").await.as_deref(), Some("s3cret"));
        c.delete("a").await;
        assert_eq!(c.get("a").await, None);
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
