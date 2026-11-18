//! FTP/FTPS against the Docker servers (`scripts/test-servers.sh up`): vsftpd on :2122 (LIST only, no MFMT) and
//! ProFTPD on :2121 (MLSD/MFMT/SITE CHMOD, explicit TLS with a self-signed certificate).
#![cfg(feature = "integration")]

use deployment::config::{DataProtection, Server, ServerType};
use deployment::protocol::{ConnectParams, RemoteClient, ftp, testing};
use deployment::{ErrorCode, tls};

fn params(kind: ServerType, port: u16) -> ConnectParams {
    let mut server = Server::new("test", "test", kind);
    server.host = "127.0.0.1".into();
    server.port = Some(port);
    server.username = "tester".into();
    let mut params = ConnectParams::new(server);
    params.password = Some("pass".into());
    params
}

async fn run_contract(params: &ConnectParams) {
    let mut client = ftp::connect(params).await.unwrap();
    let home = client.home_dir().await.unwrap();
    testing::contract(&mut client, &home).await;
    assert!(client.is_connected());
    client.close().await;
}

/// The first connection is refused with the certificate's fingerprint; trusting it lets the second one through.
async fn trusted_ftps_params(mut params: ConnectParams) -> ConnectParams {
    let err = ftp::connect(&params)
        .await
        .err()
        .expect("self-signed certificate must be rejected");
    assert_eq!(err.code, ErrorCode::Certificate, "{err}");
    params.accepted_certs =
        vec![tls::fingerprint_from_error(&err).expect("fingerprint in the error")];
    params
}

#[tokio::test]
async fn vsftpd_plain() {
    run_contract(&params(ServerType::Ftp, 2122)).await;
}

#[tokio::test]
async fn wrong_password_is_auth_error() {
    let mut p = params(ServerType::Ftp, 2122);
    p.password = Some("wrong".into());
    let err = ftp::connect(&p).await.err().unwrap();
    assert_eq!(err.code, ErrorCode::Auth, "{err}");
}

/// Plain, FTPS and FTPS with clear data run one after another: the contract uses a per-process directory name and
/// they share the ProFTPD home.
#[tokio::test]
async fn proftpd() {
    run_contract(&params(ServerType::Ftp, 2121)).await;

    let mut use_list = params(ServerType::Ftp, 2121);
    use_list.server.use_list = true;
    run_contract(&use_list).await;

    let ftps = trusted_ftps_params(params(ServerType::Ftps, 2121)).await;
    run_contract(&ftps).await;

    let mut clear = ftps.clone();
    clear.server.data_protection = Some(DataProtection::Clear);
    run_contract(&clear).await;
}
