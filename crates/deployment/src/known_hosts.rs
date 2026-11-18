//! OpenSSH known_hosts handling (hashed entries, wildcards, negation, `@revoked`) and iRemote's own trust store,
//! plus the host-key policy ("Strict host key checking": Ask / Accept new / Yes / No).

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::sync::Mutex;

use base64::Engine;
use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::config::HostKeyPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostKeyStatus {
    Trusted,
    Revoked,
    Mismatch,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marker {
    Revoked,
    CertAuthority,
}

#[derive(Debug, Clone)]
pub struct KnownHostEntry {
    marker: Option<Marker>,
    patterns: Vec<String>,
    key_type: String,
    key: String,
}

/// OpenSSH-style fingerprint: `SHA256:` + unpadded base64 of the SHA-256 of the key blob.
pub fn fingerprint_sha256(key_blob: &[u8]) -> String {
    format!(
        "SHA256:{}",
        STANDARD_NO_PAD.encode(Sha256::digest(key_blob))
    )
}

/// The host token OpenSSH writes: `host` for port 22, `[host]:port` otherwise.
pub fn host_token(host: &str, port: u16) -> String {
    if port == 22 {
        host.to_string()
    } else {
        format!("[{host}]:{port}")
    }
}

pub fn parse_known_hosts(text: &str) -> Vec<KnownHostEntry> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        let marker = match parts.first() {
            Some(&"@revoked") => Some(Marker::Revoked),
            Some(&"@cert-authority") => Some(Marker::CertAuthority),
            _ => None,
        };
        if marker.is_some() {
            parts.remove(0);
        }
        if parts.len() < 3 {
            continue;
        }
        out.push(KnownHostEntry {
            marker,
            patterns: parts[0].split(',').map(String::from).collect(),
            key_type: parts[1].to_string(),
            key: parts[2].to_string(),
        });
    }
    out
}

fn matches_hashed(pattern: &str, token: &str) -> bool {
    let mut it = pattern.split('|');
    let (_, kind, salt, hash) = (it.next(), it.next(), it.next(), it.next());
    let (Some("1"), Some(salt), Some(hash)) = (kind, salt, hash) else {
        return false;
    };
    let Ok(salt) = STANDARD.decode(salt) else {
        return false;
    };
    let Ok(mut mac) = <Hmac<Sha1> as Mac>::new_from_slice(&salt) else {
        return false;
    };
    mac.update(token.as_bytes());
    STANDARD.encode(mac.finalize().into_bytes()) == hash
}

/// OpenSSH patterns only know `*` and `?`; matching is case-insensitive.
fn wildcard_match(pattern: &str, value: &str) -> bool {
    fn go(p: &[u8], v: &[u8]) -> bool {
        match (p.first(), v.first()) {
            (None, None) => true,
            (Some(b'*'), _) => go(&p[1..], v) || (!v.is_empty() && go(p, &v[1..])),
            (Some(b'?'), Some(_)) => go(&p[1..], &v[1..]),
            (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => go(&p[1..], &v[1..]),
            _ => false,
        }
    }
    go(pattern.as_bytes(), value.as_bytes())
}

pub fn host_matches(patterns: &[String], host: &str, port: u16) -> bool {
    let token = host_token(host, port);
    let mut matched = false;
    for p in patterns {
        let (negated, pat) = match p.strip_prefix('!') {
            Some(rest) => (true, rest),
            None => (false, p.as_str()),
        };
        let hit = if pat.starts_with("|1|") {
            matches_hashed(pat, &token)
        } else {
            wildcard_match(pat, &token)
        };
        if hit && negated {
            return false;
        }
        matched |= hit;
    }
    matched
}

pub fn check_host_key(
    entries: &[KnownHostEntry],
    host: &str,
    port: u16,
    key_type: &str,
    key_blob: &[u8],
) -> HostKeyStatus {
    let key_b64 = STANDARD.encode(key_blob);
    for e in entries {
        if e.marker == Some(Marker::CertAuthority) || !host_matches(&e.patterns, host, port) {
            continue;
        }
        if e.key == key_b64 {
            return if e.marker == Some(Marker::Revoked) {
                HostKeyStatus::Revoked
            } else {
                HostKeyStatus::Trusted
            };
        }
    }
    let same_type = entries.iter().any(|e| {
        e.marker.is_none() && host_matches(&e.patterns, host, port) && e.key_type == key_type
    });
    if same_type {
        HostKeyStatus::Mismatch
    } else {
        HostKeyStatus::Unknown
    }
}

/// Revocations apply regardless of host.
pub fn is_revoked(entries: &[KnownHostEntry], key_blob: &[u8]) -> bool {
    let key_b64 = STANDARD.encode(key_blob);
    entries
        .iter()
        .any(|e| e.marker == Some(Marker::Revoked) && e.key == key_b64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostKeyQuestionKind {
    Revoked,
    Changed,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostKeyQuestion {
    pub kind: HostKeyQuestionKind,
    pub host: String,
    pub port: u16,
    pub key_type: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostKeyAnswer {
    /// Add to iRemote's trusted keys.
    Remember,
    /// Trust for this process only.
    Once,
    Reject,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TrustedKey {
    #[serde(rename = "type")]
    key_type: String,
    key: String,
}

/// known_hosts lookup plus iRemote's own trust store (`<config>/known_hosts.json`).
pub struct HostKeyTrust {
    store_file: PathBuf,
    extra_files: Vec<PathBuf>,
    session: Mutex<HashSet<String>>,
}

/// Result of the policy check before (or instead of) asking the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Accept,
    Reject(String),
    Ask(HostKeyQuestion),
}

impl HostKeyTrust {
    pub fn new(store_file: PathBuf, extra_files: Vec<PathBuf>) -> Self {
        Self {
            store_file,
            extra_files,
            session: Mutex::default(),
        }
    }

    fn load_store(&self) -> BTreeMap<String, Vec<TrustedKey>> {
        std::fs::read(&self.store_file)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    fn system_entries(&self) -> Vec<KnownHostEntry> {
        let mut files: Vec<PathBuf> = ::dirs::home_dir()
            .map(|h| vec![h.join(".ssh/known_hosts")])
            .unwrap_or_default();
        files.extend(self.extra_files.iter().cloned());
        files
            .iter()
            .filter_map(|f| std::fs::read_to_string(f).ok())
            .flat_map(|t| parse_known_hosts(&t))
            .collect()
    }

    /// Applies the policy. `Ask` must be answered with [`HostKeyTrust::apply_answer`].
    pub fn check(
        &self,
        policy: HostKeyPolicy,
        host: &str,
        port: u16,
        key_type: &str,
        key_blob: &[u8],
    ) -> Verdict {
        let fingerprint = fingerprint_sha256(key_blob);
        let question = |kind| HostKeyQuestion {
            kind,
            host: host.into(),
            port,
            key_type: key_type.into(),
            fingerprint: fingerprint.clone(),
        };
        let system = self.system_entries();
        if is_revoked(&system, key_blob) {
            return Verdict::Reject(format!(
                "The host key of {host} ({fingerprint}) is revoked in known_hosts."
            ));
        }
        if policy == HostKeyPolicy::No {
            return Verdict::Accept;
        }
        let status = check_host_key(&system, host, port, key_type, key_blob);
        if status == HostKeyStatus::Trusted {
            return Verdict::Accept;
        }
        let token = host_token(host, port);
        let key_b64 = STANDARD.encode(key_blob);
        if self
            .session
            .lock()
            .map(|s| s.contains(&format!("{token} {key_b64}")))
            .unwrap_or(false)
        {
            return Verdict::Accept;
        }
        let own = self.load_store().remove(&token).unwrap_or_default();
        let own_match = own.iter().find(|e| e.key_type == key_type);
        if own_match.is_some_and(|e| e.key == key_b64) {
            return Verdict::Accept;
        }
        let changed = status == HostKeyStatus::Mismatch || own_match.is_some();
        match (policy, changed) {
            (_, true) if policy == HostKeyPolicy::Yes => Verdict::Reject(format!(
                "HOST KEY CHANGED for {host} ({fingerprint}). Possible man-in-the-middle attack."
            )),
            (_, true) => Verdict::Ask(question(HostKeyQuestionKind::Changed)),
            (HostKeyPolicy::Yes, false) => Verdict::Reject(format!(
                "Unknown host key for {host} ({fingerprint}) and strict host key checking is on. Add it to known_hosts first."
            )),
            (HostKeyPolicy::AcceptNew, false) => {
                self.remember(host, port, key_type, key_blob);
                Verdict::Accept
            }
            _ => Verdict::Ask(question(HostKeyQuestionKind::Unknown)),
        }
    }

    /// Records the user's answer; returns whether the key is accepted. A changed key can't be trusted "once".
    pub fn apply_answer(
        &self,
        q: &HostKeyQuestion,
        key_blob: &[u8],
        answer: HostKeyAnswer,
    ) -> bool {
        match (answer, q.kind) {
            (
                HostKeyAnswer::Remember,
                HostKeyQuestionKind::Unknown | HostKeyQuestionKind::Changed,
            ) => {
                self.remember(&q.host, q.port, &q.key_type, key_blob);
                true
            }
            (HostKeyAnswer::Once, HostKeyQuestionKind::Unknown) => {
                let id = format!(
                    "{} {}",
                    host_token(&q.host, q.port),
                    STANDARD.encode(key_blob)
                );
                if let Ok(mut s) = self.session.lock() {
                    s.insert(id);
                }
                true
            }
            _ => false,
        }
    }

    fn remember(&self, host: &str, port: u16, key_type: &str, key_blob: &[u8]) {
        let mut all = self.load_store();
        let list = all.entry(host_token(host, port)).or_default();
        list.retain(|e| e.key_type != key_type);
        list.push(TrustedKey {
            key_type: key_type.into(),
            key: STANDARD.encode(key_blob),
        });
        if let Ok(bytes) = serde_json::to_vec_pretty(&all) {
            let _ = crate::dirs::write_atomic(&self.store_file, &bytes);
        }
    }

    pub fn forget(&self, host: &str, port: u16) {
        let mut all = self.load_store();
        all.remove(&host_token(host, port));
        if let Ok(bytes) = serde_json::to_vec_pretty(&all) {
            let _ = crate::dirs::write_atomic(&self.store_file, &bytes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = b"key-one";
    const OTHER: &[u8] = b"key-two";

    fn line(patterns: &str, key: &[u8]) -> String {
        format!("{patterns} ssh-ed25519 {}\n", STANDARD.encode(key))
    }

    fn hashed(token: &str) -> String {
        let salt = b"0123456789abcdefghij";
        let mut mac = <Hmac<Sha1> as Mac>::new_from_slice(salt).unwrap();
        mac.update(token.as_bytes());
        format!(
            "|1|{}|{}",
            STANDARD.encode(salt),
            STANDARD.encode(mac.finalize().into_bytes())
        )
    }

    #[test]
    fn matching_rules() {
        let text = [
            line("example.com,alias", KEY),
            line(&hashed("[hashed.example]:2222"), KEY),
            line("*.wild.test,!bad.wild.test", KEY),
            format!("@revoked * ssh-ed25519 {}\n", STANDARD.encode(OTHER)),
        ]
        .concat();
        let e = parse_known_hosts(&text);
        assert_eq!(
            check_host_key(&e, "example.com", 22, "ssh-ed25519", KEY),
            HostKeyStatus::Trusted
        );
        assert_eq!(
            check_host_key(&e, "EXAMPLE.com", 22, "ssh-ed25519", KEY),
            HostKeyStatus::Trusted
        );
        assert_eq!(
            check_host_key(&e, "example.com", 2200, "ssh-ed25519", KEY),
            HostKeyStatus::Unknown
        );
        assert_eq!(
            check_host_key(&e, "hashed.example", 2222, "ssh-ed25519", KEY),
            HostKeyStatus::Trusted
        );
        assert_eq!(
            check_host_key(&e, "a.wild.test", 22, "ssh-ed25519", KEY),
            HostKeyStatus::Trusted
        );
        assert_eq!(
            check_host_key(&e, "bad.wild.test", 22, "ssh-ed25519", KEY),
            HostKeyStatus::Unknown
        );
        assert_eq!(
            check_host_key(&e, "example.com", 22, "ssh-ed25519", b"different"),
            HostKeyStatus::Mismatch
        );
        assert!(is_revoked(&e, OTHER));
    }

    #[test]
    fn fingerprint_format() {
        assert_eq!(
            fingerprint_sha256(b""),
            "SHA256:47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU"
        );
    }

    #[test]
    fn policy_and_trust_store() {
        let dir = tempfile::tempdir().unwrap();
        let trust = HostKeyTrust::new(dir.path().join("trusted.json"), vec![]);
        let host = "policy.iremote.invalid";
        assert!(matches!(
            trust.check(HostKeyPolicy::Yes, host, 22, "ssh-ed25519", KEY),
            Verdict::Reject(_)
        ));
        assert_eq!(
            trust.check(HostKeyPolicy::No, host, 22, "ssh-ed25519", KEY),
            Verdict::Accept
        );
        let Verdict::Ask(q) = trust.check(HostKeyPolicy::Ask, host, 22, "ssh-ed25519", KEY) else {
            panic!()
        };
        assert_eq!(q.kind, HostKeyQuestionKind::Unknown);
        assert!(trust.apply_answer(&q, KEY, HostKeyAnswer::Once));
        assert_eq!(
            trust.check(HostKeyPolicy::Ask, host, 22, "ssh-ed25519", KEY),
            Verdict::Accept
        );
        assert!(trust.apply_answer(&q, KEY, HostKeyAnswer::Remember));
        // Same type, different key → changed.
        let Verdict::Ask(q2) = trust.check(HostKeyPolicy::Ask, host, 22, "ssh-ed25519", OTHER)
        else {
            panic!()
        };
        assert_eq!(q2.kind, HostKeyQuestionKind::Changed);
        assert!(
            !trust.apply_answer(&q2, OTHER, HostKeyAnswer::Once),
            "changed keys can't be trusted once"
        );
        assert!(matches!(
            trust.check(HostKeyPolicy::Yes, host, 22, "ssh-ed25519", OTHER),
            Verdict::Reject(_)
        ));
        // accept-new learns unknown hosts silently.
        assert_eq!(
            trust.check(
                HostKeyPolicy::AcceptNew,
                "new.iremote.invalid",
                22,
                "ssh-ed25519",
                KEY
            ),
            Verdict::Accept
        );
        assert_eq!(
            trust.check(
                HostKeyPolicy::Yes,
                "new.iremote.invalid",
                22,
                "ssh-ed25519",
                KEY
            ),
            Verdict::Accept
        );
    }
}
