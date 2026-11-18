//! TLS for FTPS and WebDAV over HTTPS. Certificates are verified by the OS trust store before any credentials are
//! sent. An untrusted certificate is accepted only when its SHA-256 fingerprint matches one the user confirmed
//! (pinned in the server config, or trusted for this session).

use std::sync::{Arc, Mutex};

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};
use sha2::{Digest, Sha256};

use crate::error::{ErrorCode, RemoteError};

pub fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

/// `AA:BB:…` SHA-256 fingerprint of a DER certificate (the format browsers show).
pub fn cert_fingerprint(der: &[u8]) -> String {
    Sha256::digest(der)
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(":")
}

/// What the verifier saw on the last failed handshake, so the caller can ask the user and retry.
#[derive(Debug, Default)]
pub struct CertCapture {
    pub rejected: Mutex<Option<RejectedCert>>,
}

#[derive(Debug, Clone)]
pub struct RejectedCert {
    pub fingerprint: String,
    pub reason: String,
}

impl CertCapture {
    pub fn take(&self) -> Option<RejectedCert> {
        self.rejected.lock().ok().and_then(|mut r| r.take())
    }

    /// Turns a handshake failure into a `Certificate` error carrying the fingerprint, when the verifier rejected it.
    pub fn error_or(&self, host: &str, fallback: RemoteError) -> RemoteError {
        match self.rejected.lock().ok().and_then(|r| r.clone()) {
            Some(rej) => untrusted_error(host, &rej),
            None => fallback,
        }
    }
}

pub fn untrusted_error(host: &str, rej: &RejectedCert) -> RemoteError {
    RemoteError::new(
        ErrorCode::Certificate,
        format!(
            "Untrusted certificate for {host}: {} [{}]",
            rej.reason, rej.fingerprint
        ),
    )
}

/// Extracts the fingerprint from a `Certificate` error message created by [`untrusted_error`].
pub fn fingerprint_from_error(e: &RemoteError) -> Option<String> {
    if e.code != ErrorCode::Certificate {
        return None;
    }
    let start = e.message.rfind('[')? + 1;
    let end = e.message.rfind(']')?;
    (start < end).then(|| e.message[start..end].to_string())
}

#[derive(Debug)]
struct PinningVerifier {
    inner: Arc<dyn ServerCertVerifier>,
    accepted: Vec<String>,
    capture: Arc<CertCapture>,
}

impl ServerCertVerifier for PinningVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let fingerprint = cert_fingerprint(end_entity.as_ref());
        match self
            .inner
            .verify_server_cert(end_entity, intermediates, server_name, ocsp, now)
        {
            Ok(v) => Ok(v),
            Err(_)
                if self
                    .accepted
                    .iter()
                    .any(|a| a.eq_ignore_ascii_case(&fingerprint)) =>
            {
                Ok(ServerCertVerified::assertion())
            }
            Err(e) => {
                if let Ok(mut r) = self.capture.rejected.lock() {
                    *r = Some(RejectedCert {
                        fingerprint,
                        reason: e.to_string(),
                    });
                }
                Err(e)
            }
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls12_signature(message, cert, dss)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls13_signature(message, cert, dss)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner.supported_verify_schemes()
    }
}

/// Client config with OS verification plus the accepted fingerprints.
pub fn client_config(
    accepted: Vec<String>,
    disable_tls13: bool,
) -> Result<(Arc<ClientConfig>, Arc<CertCapture>), RemoteError> {
    let provider = provider();
    let inner = rustls_platform_verifier::Verifier::new(provider.clone()).map_err(|e| {
        RemoteError::new(
            ErrorCode::Certificate,
            format!("Cannot load the system trust store: {e}"),
        )
    })?;
    let capture = Arc::new(CertCapture::default());
    let verifier = Arc::new(PinningVerifier {
        inner: Arc::new(inner),
        accepted,
        capture: capture.clone(),
    });
    let versions: &[&rustls::SupportedProtocolVersion] = if disable_tls13 {
        &[&rustls::version::TLS12]
    } else {
        &[&rustls::version::TLS13, &rustls::version::TLS12]
    };
    let config = ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(versions)
        .map_err(|e| RemoteError::new(ErrorCode::Certificate, e.to_string()))?
        .dangerous()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();
    Ok((Arc::new(config), capture))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_round_trip() {
        let rej = RejectedCert {
            fingerprint: cert_fingerprint(b"x"),
            reason: "UnknownIssuer".into(),
        };
        let e = untrusted_error("h", &rej);
        assert_eq!(
            fingerprint_from_error(&e).as_deref(),
            Some(rej.fingerprint.as_str())
        );
        assert_eq!(rej.fingerprint.len(), 32 * 3 - 1);
    }

    #[test]
    fn builds_config() {
        let (cfg, _) = client_config(vec![], true).unwrap();
        assert!(cfg.alpn_protocols.is_empty());
    }
}
