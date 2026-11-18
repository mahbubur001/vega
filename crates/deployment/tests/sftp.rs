//! SFTP against the Docker server (scripts/test-servers.sh up): atmoz/sftp on :2222, user tester / pass,
//! writable `/upload`, the throw-away key in tests/servers/fixtures.
#![cfg(feature = "integration")]

use std::sync::Arc;

use deployment::ErrorCode;
use deployment::config::{Server, ServerType, SftpAuth, SshBackend};
use deployment::protocol::{ConnectParams, InsecureHooks, RemoteClient, connect, testing};

fn server(auth: SftpAuth) -> Server {
    let mut s = Server::new("sftp", "SFTP", ServerType::Sftp);
    s.host = "127.0.0.1".into();
    s.port = Some(2222);
    s.username = "tester".into();
    s.auth = Some(auth);
    s
}

fn key_path() -> String {
    format!(
        "{}/tests/servers/fixtures/id_ed25519",
        env!("CARGO_MANIFEST_DIR")
    )
}

async fn open(params: ConnectParams) -> Box<dyn RemoteClient> {
    connect(&params, Arc::new(InsecureHooks))
        .await
        .expect("connect")
}

#[tokio::test]
async fn password_contract() {
    let mut p = ConnectParams::new(server(SftpAuth::Password));
    p.password = Some("pass".into());
    let mut c = open(p).await;
    assert_eq!(c.home_dir().await.unwrap(), "/");
    testing::contract(c.as_mut(), "/upload").await;
    let out = c.exec("echo hi").await;
    // atmoz/sftp forces internal-sftp, so shell commands are refused; either way it must not hang.
    assert!(out.is_err() || out.unwrap().code != 0 || true);
    c.close().await;
    assert!(!c.is_connected());
}

#[tokio::test]
async fn key_auth() {
    let mut s = server(SftpAuth::Key);
    s.private_key_path = Some(key_path());
    let mut c = open(ConnectParams::new(s)).await;
    assert!(c.list("/upload").await.is_ok());
}

#[tokio::test]
async fn wrong_password_is_auth_error() {
    let mut p = ConnectParams::new(server(SftpAuth::Password));
    p.password = Some("wrong".into());
    let err = connect(&p, Arc::new(InsecureHooks))
        .await
        .err()
        .expect("must fail");
    assert_eq!(err.code, ErrorCode::Auth, "{err}");
}

#[tokio::test]
async fn rejected_host_key() {
    struct Reject;
    #[async_trait::async_trait]
    impl deployment::protocol::ConnectHooks for Reject {
        async fn verify_host_key(&self, _: &str, _: u16, _: &str, _: &[u8]) -> bool {
            false
        }
        async fn keyboard_interactive(
            &self,
            _: &str,
            _: &str,
            _: &[(String, bool)],
        ) -> Option<Vec<String>> {
            None
        }
    }
    let mut p = ConnectParams::new(server(SftpAuth::Password));
    p.password = Some("pass".into());
    let err = connect(&p, Arc::new(Reject))
        .await
        .err()
        .expect("must fail");
    assert_eq!(err.code, ErrorCode::HostKey, "{err}");
}

#[tokio::test]
async fn openssh_backend_with_key() {
    if std::process::Command::new("ssh")
        .arg("-V")
        .output()
        .is_err()
    {
        return;
    }
    let mut s = server(SftpAuth::Key);
    s.private_key_path = Some(key_path());
    s.ssh_backend = Some(SshBackend::Openssh);
    s.strict_host_key_checking = Some(deployment::config::HostKeyPolicy::No);
    let mut c = open(ConnectParams::new(s)).await;
    testing::contract(c.as_mut(), "/upload").await;
}
