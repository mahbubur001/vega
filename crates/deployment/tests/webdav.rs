//! WebDAV against the Docker test server (rclone, see tests/servers/compose.yml).
#![cfg(feature = "integration")]

use deployment::config::{Server, ServerType, WebdavAuth};
use deployment::error::ErrorCode;
use deployment::protocol::{ConnectParams, RemoteClient, webdav};

fn params(password: &str) -> ConnectParams {
    let mut server = Server::new("dav", "dav", ServerType::Webdav);
    server.host =
        std::env::var("IREMOTE_WEBDAV_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".into());
    server.username = "tester".into();
    server.webdav_auth = Some(WebdavAuth::Basic);
    let mut params = ConnectParams::new(server);
    params.password = Some(password.into());
    params
}

#[tokio::test]
async fn webdav_contract() {
    let mut client = webdav::connect(&params("pass")).await.unwrap();
    assert_eq!(client.home_dir().await.unwrap(), "/");
    deployment::protocol::testing::contract(&mut client, "/").await;
    client.close().await;
}

#[tokio::test]
async fn webdav_wrong_password() {
    let err = webdav::connect(&params("wrong"))
        .await
        .err()
        .expect("login must fail");
    assert_eq!(err.code, ErrorCode::Auth, "{err}");
}

#[tokio::test]
async fn webdav_encoded_names() {
    let mut client = webdav::connect(&params("pass")).await.unwrap();
    let root = format!("/names-{}", std::process::id());
    let dir = format!("{root}/a b%20ü#?");
    client.mkdirp(&dir).await.unwrap();
    client
        .write_file(&format!("{dir}/rés umé.txt"), b"x")
        .await
        .unwrap();
    let names: Vec<String> = client
        .list(&dir)
        .await
        .unwrap()
        .into_iter()
        .map(|e| e.name)
        .collect();
    assert_eq!(names, ["rés umé.txt"]);
    let parent: Vec<String> = client
        .list(&root)
        .await
        .unwrap()
        .into_iter()
        .map(|e| e.name)
        .collect();
    assert_eq!(parent, ["a b%20ü#?"]);
    let err = client
        .mkdirp(&format!("{dir}/rés umé.txt/x"))
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::Exists, "{err}");
    let err = client.delete_dir(&root, false).await.unwrap_err();
    assert_eq!(err.code, ErrorCode::Unknown, "{err}");
    client.delete_dir(&root, true).await.unwrap();
    assert!(client.stat(&root).await.unwrap().is_none());
}
