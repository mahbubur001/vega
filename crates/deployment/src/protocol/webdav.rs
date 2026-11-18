//! WebDAV over HTTP(S). The endpoint URL (`server.host`) is the root; remote POSIX paths are appended to its path one
//! percent-encoded segment at a time. Metadata comes from PROPFIND, transfers are plain GET/PUT.

use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::header::{
    AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap, HeaderValue, WWW_AUTHENTICATE,
};
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use sha2::{Digest as _, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use url::{Position, Url};

use super::*;
use crate::config::WebdavAuth;
use crate::error::{ErrorCode, RemoteError, Result};
use crate::mapper::{norm_remote, remote_name, remote_parent};
use crate::tls::CertCapture;

const CHUNK: usize = 256 * 1024;

const PROPFIND_BODY: &str = concat!(
    r#"<?xml version="1.0" encoding="utf-8"?>"#,
    r#"<d:propfind xmlns:d="DAV:"><d:prop><d:resourcetype/><d:getcontentlength/><d:getlastmodified/></d:prop></d:propfind>"#
);

enum Auth {
    None,
    /// Sent preemptively: one round trip per request instead of two.
    Basic {
        user: String,
        pass: String,
    },
    /// Needs a server nonce, so the first request of a session is answered with a 401 challenge and retried.
    Digest {
        user: String,
        pass: String,
    },
}

pub struct WebDavClient {
    http: reqwest::Client,
    /// Endpoint URL; request URLs are built from it so its path prefix is kept.
    base: Url,
    /// Decoded, normalized endpoint path, used to recognise the hrefs PROPFIND returns.
    base_path: String,
    host: String,
    auth: Auth,
    digest: Mutex<Option<DigestChallenge>>,
    capture: Arc<CertCapture>,
    timeout: Duration,
    closed: bool,
}

/// Connects to `params.server.host` and verifies the endpoint and credentials with a `PROPFIND Depth: 0` on "/".
pub async fn connect(params: &ConnectParams) -> Result<WebDavClient> {
    let server = &params.server;
    let base = parse_endpoint(&server.host, server.port)?;
    let host = base.host_str().unwrap_or_default().to_string();
    let (config, capture) =
        crate::tls::client_config(params.accepted_certs.clone(), server.disable_tls13)?;
    // No global request timeout: it would cap whole transfers. Small requests get a per-request timeout and
    // transfers an inactivity timeout instead. Redirects are not followed because reqwest may turn them into GETs.
    let http = reqwest::Client::builder()
        .use_preconfigured_tls((*config).clone())
        .connect_timeout(params.timeout)
        .redirect_policy(reqwest::redirect::Policy::none())
        .user_agent(concat!("VEGA-Deployment/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| {
            RemoteError::new(
                ErrorCode::Config,
                format!("Cannot create the HTTP client: {}", error_chain(&e)),
            )
        })?;
    let user = server.username.clone();
    let pass = params.password.clone().unwrap_or_default();
    let auth = match server.webdav_auth.unwrap_or_default() {
        WebdavAuth::None => Auth::None,
        _ if user.is_empty() && params.password.is_none() => Auth::None,
        WebdavAuth::Basic => Auth::Basic { user, pass },
        WebdavAuth::Digest => Auth::Digest { user, pass },
    };
    let client = WebDavClient {
        http,
        base_path: norm_remote(&percent_decode(base.path())),
        base,
        host,
        auth,
        digest: Mutex::new(None),
        capture,
        timeout: params.timeout,
        closed: false,
    };
    match client.propfind("/", "0").await? {
        Some(_) => Ok(client),
        None => {
            Err(RemoteError::new(ErrorCode::NotFound, "WebDAV endpoint not found").with_path("/"))
        }
    }
}

/// Accepts `https://host/path` (or a bare `host/path`, assumed HTTPS); a configured port applies when the URL has none.
fn parse_endpoint(host: &str, port: Option<u16>) -> Result<Url> {
    let raw = host.trim();
    let raw = if raw.contains("://") {
        raw.to_string()
    } else {
        format!("https://{raw}")
    };
    let invalid = |why: String| {
        RemoteError::new(
            ErrorCode::Config,
            format!("Invalid WebDAV URL \"{host}\": {why}"),
        )
    };
    let mut url = Url::parse(&raw).map_err(|e| invalid(e.to_string()))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.cannot_be_a_base()
        || url.host_str().is_none()
    {
        return Err(invalid("expected an http:// or https:// URL".into()));
    }
    if let (None, Some(p)) = (url.port(), port.filter(|p| *p != 0)) {
        let _ = url.set_port(Some(p));
    }
    url.set_query(None);
    url.set_fragment(None);
    Ok(url)
}

fn dav_method(name: &'static str) -> Method {
    Method::from_bytes(name.as_bytes()).expect("static WebDAV method names are valid tokens")
}

impl WebDavClient {
    /// URL for a remote path. Collections get a trailing slash: some servers redirect or reject them without one.
    fn url(&self, remote: &str, dir: bool) -> Url {
        let mut url = self.base.clone();
        let norm = norm_remote(remote);
        if let Ok(mut segs) = url.path_segments_mut() {
            segs.pop_if_empty();
            segs.extend(norm.split('/').filter(|s| !s.is_empty()));
            if dir {
                segs.push("");
            }
        }
        url
    }

    /// Decoded server path of a remote path, comparable with [`href_path`] results.
    fn server_path(&self, remote: &str) -> String {
        norm_remote(&format!("{}/{}", self.base_path, norm_remote(remote)))
    }

    fn timed_out(&self, path: &str) -> RemoteError {
        RemoteError::connection(format!("Timed out after {}s", self.timeout.as_secs()))
            .with_path(path)
    }

    fn transport_error(&self, e: reqwest::Error, path: &str) -> RemoteError {
        let code = if e.is_timeout() || e.is_connect() || e.is_request() || e.is_body() {
            ErrorCode::Connection
        } else {
            ErrorCode::Unknown
        };
        let err = RemoteError::new(code, error_chain(&e));
        // A rejected certificate surfaces as a connect error; the capture knows the fingerprint to show the user.
        let err = if e.is_connect() {
            self.capture.error_or(&self.host, err)
        } else {
            err
        };
        err.with_path(path)
    }

    /// Sends a request built by `build`, adding credentials. A Digest 401 updates the challenge and retries once, so
    /// `build` must be able to recreate the body.
    async fn send<F>(&self, method: &Method, url: &Url, path: &str, build: F) -> Result<Response>
    where
        F: Fn(RequestBuilder) -> RequestBuilder + Send + Sync,
    {
        let mut retried = false;
        loop {
            let rb = build(self.http.request(method.clone(), url.clone()));
            let rb = match &self.auth {
                Auth::None => rb,
                Auth::Basic { user, pass } => rb.basic_auth(user, Some(pass)),
                Auth::Digest { user, pass } => match self.digest_header(user, pass, method, url) {
                    Some(v) => rb.header(AUTHORIZATION, v),
                    None => rb,
                },
            };
            let resp = rb.send().await.map_err(|e| self.transport_error(e, path))?;
            if resp.status() == StatusCode::UNAUTHORIZED
                && !retried
                && matches!(self.auth, Auth::Digest { .. })
                && let Some(ch) = DigestChallenge::from_headers(resp.headers())
            {
                *self.digest.lock().unwrap_or_else(|e| e.into_inner()) = Some(ch);
                retried = true;
                continue;
            }
            return Ok(resp);
        }
    }

    fn digest_header(
        &self,
        user: &str,
        pass: &str,
        method: &Method,
        url: &Url,
    ) -> Option<HeaderValue> {
        let mut guard = self.digest.lock().unwrap_or_else(|e| e.into_inner());
        let ch = guard.as_mut()?;
        ch.nc += 1;
        let uri = &url[Position::BeforePath..Position::AfterQuery];
        HeaderValue::from_str(&ch.authorization(user, pass, method.as_str(), uri, &cnonce())).ok()
    }

    /// Small request with a total timeout; returns the status and the body.
    async fn simple<F>(
        &self,
        method: &Method,
        url: &Url,
        path: &str,
        build: F,
    ) -> Result<(StatusCode, Bytes)>
    where
        F: Fn(RequestBuilder) -> RequestBuilder + Send + Sync,
    {
        let timeout = self.timeout;
        let resp = self
            .send(method, url, path, |rb| build(rb).timeout(timeout))
            .await?;
        let status = resp.status();
        let body = resp
            .bytes()
            .await
            .map_err(|e| self.transport_error(e, path))?;
        Ok((status, body))
    }

    /// PROPFIND; `None` when the path doesn't exist.
    async fn propfind(&self, path: &str, depth: &'static str) -> Result<Option<Vec<DavResponse>>> {
        let method = dav_method("PROPFIND");
        let build = |rb: RequestBuilder| {
            rb.header("Depth", depth)
                .header(CONTENT_TYPE, "application/xml; charset=utf-8")
                .body(PROPFIND_BODY)
        };
        let url = self.url(path, depth != "0");
        let (mut status, mut body) = self.simple(&method, &url, path, build).await?;
        if status.is_redirection() && !url.path().ends_with('/') {
            // A collection addressed without its trailing slash.
            (status, body) = self
                .simple(&method, &self.url(path, true), path, build)
                .await?;
        }
        match status {
            StatusCode::MULTI_STATUS => {
                let xml = String::from_utf8_lossy(&body);
                let list = parse_multistatus(&xml).map_err(|e| {
                    RemoteError::new(
                        ErrorCode::Unknown,
                        format!("Invalid PROPFIND response: {e}"),
                    )
                    .with_path(path)
                })?;
                Ok(Some(list))
            }
            StatusCode::NOT_FOUND | StatusCode::GONE => Ok(None),
            s if s.is_success() => Err(RemoteError::new(
                ErrorCode::Unknown,
                format!("The server does not speak WebDAV (PROPFIND returned {s})"),
            )
            .with_path(path)),
            s => Err(status_error(s, path)),
        }
    }

    async fn stat_inner(&self, path: &str) -> Result<Option<RemoteStat>> {
        let Some(list) = self.propfind(path, "0").await? else {
            return Ok(None);
        };
        let target = self.server_path(path);
        let found = list.iter().find(|r| r.path == target).or(list.first());
        Ok(found.and_then(|r| r.stat.clone()))
    }

    /// GET with a timeout on the response headers; the caller reads the body with [`Self::next_chunk`].
    async fn get(&self, path: &str) -> Result<Response> {
        let url = self.url(path, false);
        let resp = tokio::time::timeout(self.timeout, self.send(&Method::GET, &url, path, |rb| rb))
            .await
            .map_err(|_| self.timed_out(path))??;
        match resp.status() {
            s if s.is_success() => Ok(resp),
            s => Err(status_error(s, path)),
        }
    }

    /// Next body chunk; the timeout applies per chunk so long downloads only fail when the server stalls.
    async fn next_chunk(&self, resp: &mut Response, path: &str) -> Result<Option<Bytes>> {
        tokio::time::timeout(self.timeout, resp.chunk())
            .await
            .map_err(|_| self.timed_out(path))?
            .map_err(|e| self.transport_error(e, path))
    }

    /// Streams `src` with a PUT. The body reports bytes handed to the connection over a channel, which drives
    /// `progress` and an inactivity timeout here (the body itself must be `'static`, `progress` isn't).
    async fn put(&self, path: &str, src: Source, total: u64, progress: Progress<'_>) -> Result<()> {
        let url = self.url(path, false);
        let (tx, mut rx) = mpsc::unbounded_channel();
        let build = |rb: RequestBuilder| {
            rb.header(CONTENT_LENGTH, total)
                .body(reqwest::Body::wrap_stream(body_stream(
                    src.clone(),
                    tx.clone(),
                )))
        };
        let fut = self.send(&Method::PUT, &url, path, build);
        let resp = drive(fut, &mut rx, total, progress, self.timeout)
            .await
            .ok_or_else(|| self.timed_out(path))??;
        match resp.status() {
            s if s.is_success() => Ok(()),
            s => Err(status_error(s, path)),
        }
    }

    async fn mkcol(&self, path: &str) -> Result<()> {
        let (status, _) = self
            .simple(&dav_method("MKCOL"), &self.url(path, true), path, |rb| rb)
            .await?;
        if status.is_success() {
            return Ok(());
        }
        // 405 means "exists"; whatever the status, a directory being there now is success (also covers races).
        match self.stat_inner(path).await? {
            Some(st) if st.is_dir() => Ok(()),
            Some(_) => Err(
                RemoteError::new(ErrorCode::Exists, "A file with this name exists").with_path(path),
            ),
            None => Err(status_error(status, path)),
        }
    }

    async fn delete(&self, path: &str, dir: bool) -> Result<()> {
        let (status, _) = self
            .simple(&Method::DELETE, &self.url(path, dir), path, |rb| rb)
            .await?;
        match status {
            // 207 lists members that could not be deleted.
            StatusCode::MULTI_STATUS => Err(RemoteError::new(
                ErrorCode::Permission,
                "Some items could not be deleted",
            )
            .with_path(path)),
            s if s.is_success() => Ok(()),
            s => Err(status_error(s, path)),
        }
    }
}

#[async_trait]
impl RemoteClient for WebDavClient {
    fn capabilities(&self) -> Capabilities {
        // WebDAV has no portable way to set permissions or modification times (getlastmodified is read-only).
        Capabilities {
            chmod: false,
            set_mtime: false,
            exec: false,
            reliable_mtime: true,
        }
    }

    /// HTTP reconnects per request, so the client stays usable until closed.
    fn is_connected(&self) -> bool {
        !self.closed
    }

    async fn close(&mut self) {
        self.closed = true;
    }

    /// Paths are relative to the endpoint URL, which is the root.
    async fn home_dir(&mut self) -> Result<String> {
        Ok("/".into())
    }

    async fn list(&mut self, path: &str) -> Result<Vec<RemoteEntry>> {
        let list = self
            .propfind(path, "1")
            .await?
            .ok_or_else(|| RemoteError::not_found(path))?;
        Ok(list_entries(&self.server_path(path), list))
    }

    async fn stat(&mut self, path: &str) -> Result<Option<RemoteStat>> {
        self.stat_inner(path).await
    }

    async fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        let mut resp = self.get(path).await?;
        let mut out = Vec::with_capacity(resp.content_length().unwrap_or(0).min(64 << 20) as usize);
        while let Some(chunk) = self.next_chunk(&mut resp, path).await? {
            out.extend_from_slice(&chunk);
        }
        Ok(out)
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        self.put(
            path,
            Source::Memory(Bytes::copy_from_slice(data)),
            data.len() as u64,
            &no_progress,
        )
        .await
    }

    async fn upload(&mut self, local: &Path, remote: &str, progress: Progress<'_>) -> Result<()> {
        let local_err =
            |e: std::io::Error| RemoteError::from(e).with_path(local.display().to_string());
        let total = tokio::fs::metadata(local).await.map_err(local_err)?.len();
        self.put(remote, Source::File(local.to_path_buf()), total, progress)
            .await
    }

    async fn download(&mut self, remote: &str, local: &Path, progress: Progress<'_>) -> Result<()> {
        let local_err =
            |e: std::io::Error| RemoteError::from(e).with_path(local.display().to_string());
        // Request first so a failed download doesn't leave an empty local file behind.
        let mut resp = self.get(remote).await?;
        let total = resp.content_length().unwrap_or(0);
        let mut file = tokio::fs::File::create(local).await.map_err(local_err)?;
        let mut done = 0u64;
        while let Some(chunk) = self.next_chunk(&mut resp, remote).await? {
            file.write_all(&chunk).await.map_err(local_err)?;
            done += chunk.len() as u64;
            progress(done, total.max(done));
        }
        file.flush().await.map_err(local_err)?;
        Ok(())
    }

    /// MKCOL only creates one level, so find the deepest existing ancestor and create the rest top-down.
    async fn mkdirp(&mut self, path: &str) -> Result<()> {
        let mut missing = Vec::new();
        let mut cur = norm_remote(path);
        while cur != "/" {
            match self.stat_inner(&cur).await? {
                Some(st) if st.is_dir() => break,
                Some(_) => {
                    return Err(RemoteError::new(
                        ErrorCode::Exists,
                        "A file with this name exists",
                    )
                    .with_path(cur));
                }
                None => {
                    let parent = remote_parent(&cur);
                    missing.push(std::mem::replace(&mut cur, parent));
                }
            }
        }
        for dir in missing.iter().rev() {
            self.mkcol(dir).await?;
        }
        Ok(())
    }

    async fn delete_file(&mut self, path: &str) -> Result<()> {
        self.delete(path, false).await
    }

    /// DELETE on a collection is always recursive, so the non-recursive variant checks emptiness first. That check
    /// and the DELETE are not atomic: a file added in between is deleted too.
    async fn delete_dir(&mut self, path: &str, recursive: bool) -> Result<()> {
        if !recursive && !self.list(path).await?.is_empty() {
            return Err(RemoteError::new(ErrorCode::Unknown, "Directory not empty").with_path(path));
        }
        self.delete(path, true).await
    }

    async fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        let dest = self.url(to, false);
        let build = |rb: RequestBuilder| {
            rb.header("Destination", dest.as_str())
                .header("Overwrite", "T")
        };
        let (status, _) = self
            .simple(&dav_method("MOVE"), &self.url(from, false), from, build)
            .await?;
        match status {
            s if s.is_success() => Ok(()),
            s => Err(status_error(s, from)),
        }
    }

    async fn chmod(&mut self, path: &str, _mode: u32) -> Result<()> {
        Err(RemoteError::unsupported("WebDAV cannot change permissions").with_path(path))
    }

    async fn set_mtime(&mut self, path: &str, _mtime_ms: i64) -> Result<()> {
        Err(RemoteError::unsupported("WebDAV cannot set modification times").with_path(path))
    }

    async fn exec(&mut self, _command: &str) -> Result<ExecOutput> {
        Err(RemoteError::unsupported(
            "Shell commands are not available for WebDAV servers",
        ))
    }
}

fn status_error(status: StatusCode, path: &str) -> RemoteError {
    let (code, message) = match status.as_u16() {
        401 => (ErrorCode::Auth, "Authentication failed".to_string()),
        403 => (ErrorCode::Permission, "Permission denied".to_string()),
        423 => (ErrorCode::Permission, "The resource is locked".to_string()),
        404 | 410 => (ErrorCode::NotFound, "No such file or directory".to_string()),
        // MKCOL/PUT/MOVE answer 409 when the parent collection is missing.
        409 => (
            ErrorCode::NotFound,
            "The parent folder does not exist".to_string(),
        ),
        412 => (
            ErrorCode::Exists,
            "The destination already exists".to_string(),
        ),
        405 => (
            ErrorCode::Unknown,
            "The server does not allow this operation here".to_string(),
        ),
        507 => (
            ErrorCode::Unknown,
            "Insufficient storage on the server".to_string(),
        ),
        _ => (ErrorCode::Unknown, format!("The server returned {status}")),
    };
    RemoteError::new(code, message).with_path(path)
}

/// reqwest's own message is terse ("error sending request"); the cause chain says what actually failed.
fn error_chain(e: &dyn std::error::Error) -> String {
    let mut msg = e.to_string();
    let mut source = e.source();
    while let Some(s) = source {
        let part = s.to_string();
        if !msg.contains(&part) {
            msg.push_str(": ");
            msg.push_str(&part);
        }
        source = s.source();
    }
    msg
}

#[derive(Clone)]
enum Source {
    Memory(Bytes),
    File(PathBuf),
}

struct Chunks {
    src: Source,
    file: Option<tokio::fs::File>,
    done: u64,
    tx: mpsc::UnboundedSender<u64>,
}

/// Request body that reads `src` in chunks and reports the running byte count on `tx`.
fn body_stream(
    src: Source,
    tx: mpsc::UnboundedSender<u64>,
) -> impl Stream<Item = std::io::Result<Bytes>> + Send + 'static {
    futures::stream::try_unfold(
        Chunks {
            src,
            file: None,
            done: 0,
            tx,
        },
        |mut st| async move {
            let chunk = match &st.src {
                Source::Memory(data) => {
                    let start = st.done as usize;
                    if start >= data.len() {
                        return Ok(None);
                    }
                    data.slice(start..data.len().min(start + CHUNK))
                }
                Source::File(path) => {
                    let file = match &mut st.file {
                        Some(f) => f,
                        None => st.file.insert(tokio::fs::File::open(path).await?),
                    };
                    let mut buf = vec![0u8; CHUNK];
                    let n = file.read(&mut buf).await?;
                    if n == 0 {
                        return Ok(None);
                    }
                    buf.truncate(n);
                    Bytes::from(buf)
                }
            };
            st.done += chunk.len() as u64;
            let _ = st.tx.send(st.done);
            Ok(Some((chunk, st)))
        },
    )
}

/// Polls `fut` while forwarding progress; `None` when nothing moved for `timeout`.
async fn drive<F: Future>(
    fut: F,
    rx: &mut mpsc::UnboundedReceiver<u64>,
    total: u64,
    progress: Progress<'_>,
    timeout: Duration,
) -> Option<F::Output> {
    let mut fut = std::pin::pin!(fut);
    let mut idle = std::pin::pin!(tokio::time::sleep(timeout));
    let out = loop {
        tokio::select! {
            out = &mut fut => break out,
            Some(done) = rx.recv() => {
                progress(done, total);
                idle.as_mut().reset(tokio::time::Instant::now() + timeout);
            }
            () = &mut idle => return None,
        }
    };
    while let Ok(done) = rx.try_recv() {
        progress(done, total);
    }
    Some(out)
}

// ---- Digest authentication (RFC 7616) ----

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigestChallenge {
    realm: String,
    nonce: String,
    opaque: Option<String>,
    algorithm: String,
    /// "auth" when the server offers it; `None` for RFC 2069 servers without qop.
    qop: Option<String>,
    nc: u32,
}

impl DigestChallenge {
    fn from_headers(headers: &HeaderMap) -> Option<Self> {
        headers
            .get_all(WWW_AUTHENTICATE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find_map(Self::parse)
    }

    /// Parses the first supported Digest challenge in a `WWW-Authenticate` value (which may list several schemes).
    fn parse(header: &str) -> Option<Self> {
        let mut rest = header;
        loop {
            let (params, after) = parse_challenge(rest)?;
            if let Some(ch) = params.and_then(Self::from_params) {
                return Some(ch);
            }
            rest = after;
        }
    }

    fn from_params(params: Vec<(String, String)>) -> Option<Self> {
        let get = |k: &str| {
            params
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(k))
                .map(|(_, v)| v.clone())
        };
        let algorithm = get("algorithm").unwrap_or_else(|| "MD5".into());
        let supported = ["MD5", "MD5-sess", "SHA-256", "SHA-256-sess"];
        let algorithm = supported
            .iter()
            .find(|a| a.eq_ignore_ascii_case(&algorithm))?
            .to_string();
        let qop = match get("qop") {
            Some(q) => Some(
                q.split(',')
                    .map(str::trim)
                    .find(|q| q.eq_ignore_ascii_case("auth"))?
                    .to_string(),
            ),
            None => None,
        };
        Some(Self {
            realm: get("realm").unwrap_or_default(),
            nonce: get("nonce")?,
            opaque: get("opaque"),
            algorithm,
            qop,
            nc: 0,
        })
    }

    fn hash(&self, s: &str) -> String {
        if self.algorithm.starts_with("SHA-256") {
            hex::encode(Sha256::digest(s.as_bytes()))
        } else {
            format!("{:x}", <md5::Md5 as md5::Digest>::digest(s.as_bytes()))
        }
    }

    fn authorization(
        &self,
        user: &str,
        pass: &str,
        method: &str,
        uri: &str,
        cnonce: &str,
    ) -> String {
        let mut ha1 = self.hash(&format!("{user}:{}:{pass}", self.realm));
        if self.algorithm.ends_with("-sess") {
            ha1 = self.hash(&format!("{ha1}:{}:{cnonce}", self.nonce));
        }
        let ha2 = self.hash(&format!("{method}:{uri}"));
        let nc = format!("{:08x}", self.nc);
        let response = match &self.qop {
            Some(qop) => self.hash(&format!("{ha1}:{}:{nc}:{cnonce}:{qop}:{ha2}", self.nonce)),
            None => self.hash(&format!("{ha1}:{}:{ha2}", self.nonce)),
        };
        let mut out = format!(
            "Digest username=\"{}\", realm=\"{}\", nonce=\"{}\", uri=\"{}\", algorithm={}, response=\"{response}\"",
            quote(user),
            quote(&self.realm),
            quote(&self.nonce),
            quote(uri),
            self.algorithm
        );
        if let Some(qop) = &self.qop {
            out.push_str(&format!(", qop={qop}, nc={nc}, cnonce=\"{cnonce}\""));
        }
        if let Some(opaque) = &self.opaque {
            out.push_str(&format!(", opaque=\"{}\"", quote(opaque)));
        }
        out
    }
}

fn quote(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Client nonce; only needs to be unpredictable per request, which `RandomState`'s random keys provide.
fn cnonce() -> String {
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u128(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0),
    );
    format!("{:016x}", h.finish())
}

type ChallengeParams = Vec<(String, String)>;

/// Reads one challenge (`Scheme k=v, k="v", …`) from the start of `s`. Returns its params when the scheme is
/// Digest, and the rest of the header; `None` at the end.
fn parse_challenge(s: &str) -> Option<(Option<ChallengeParams>, &str)> {
    let s = s.trim_start_matches([' ', '\t', ',']);
    if s.is_empty() {
        return None;
    }
    let end = s.find([' ', '\t', ',']).unwrap_or(s.len());
    let scheme = &s[..end];
    let mut rest = &s[end..];
    let mut params = Vec::new();
    loop {
        let trimmed = rest.trim_start_matches([' ', '\t', ',']);
        let name_end = trimmed.find(['=', ' ', '\t', ',']).unwrap_or(trimmed.len());
        let name = &trimmed[..name_end];
        let after_name = trimmed[name_end..].trim_start();
        // A token not followed by '=' starts the next challenge.
        if name.is_empty() || !after_name.starts_with('=') {
            break;
        }
        let value_part = after_name[1..].trim_start();
        let (value, after) = match value_part.strip_prefix('"') {
            Some(quoted) => {
                let mut value = String::new();
                let mut end = quoted.len();
                let mut chars = quoted.char_indices();
                while let Some((i, c)) = chars.next() {
                    match c {
                        '\\' => value.extend(chars.next().map(|(_, n)| n)),
                        '"' => {
                            end = i + 1;
                            break;
                        }
                        c => value.push(c),
                    }
                }
                (value, &quoted[end..])
            }
            None => {
                let end = value_part
                    .find([',', ' ', '\t'])
                    .unwrap_or(value_part.len());
                (value_part[..end].to_string(), &value_part[end..])
            }
        };
        params.push((name.to_string(), value));
        rest = after;
    }
    Some((
        scheme.eq_ignore_ascii_case("Digest").then_some(params),
        rest,
    ))
}

// ---- PROPFIND multistatus ----

#[derive(Debug, Clone, PartialEq, Eq)]
struct DavResponse {
    /// Decoded, normalized server path of the href.
    path: String,
    /// `None` when the response carries a non-2xx status (e.g. a member that vanished).
    stat: Option<RemoteStat>,
}

#[derive(Default)]
struct Props {
    dir: bool,
    size: Option<u64>,
    mtime: Option<i64>,
    ok: bool,
}

/// Children of `dir` (a decoded server path). The entry for the directory itself is recognised by its href; when no
/// href matches (a proxy rewrote the prefix), the first response is taken as the directory, as servers list it first.
fn list_entries(dir: &str, list: Vec<DavResponse>) -> Vec<RemoteEntry> {
    let self_matched = list.iter().any(|r| r.path == dir);
    let mut out: Vec<RemoteEntry> = list
        .into_iter()
        .enumerate()
        .filter(|(i, r)| if self_matched { r.path != dir } else { *i != 0 })
        .filter_map(|(_, r)| {
            Some(RemoteEntry {
                name: remote_name(&r.path).to_string(),
                stat: r.stat?,
            })
        })
        .filter(|e| !e.name.is_empty())
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Parses a `207 Multi-Status` body. Elements are matched by local name so any namespace prefix works.
fn parse_multistatus(xml: &str) -> std::result::Result<Vec<DavResponse>, String> {
    let mut reader = Reader::from_str(xml);
    let mut stack: Vec<String> = Vec::new();
    let mut text = String::new();
    let mut out = Vec::new();
    let mut href: Option<String> = None;
    let mut response_ok = true;
    let mut props = Props::default();
    let mut merged = Props::default();

    loop {
        let event = reader
            .read_event()
            .map_err(|e| format!("{e} at {}", reader.error_position()))?;
        let (opened, closes) = match &event {
            Event::Start(e) => (
                Some(String::from_utf8_lossy(e.local_name().as_ref()).into_owned()),
                false,
            ),
            Event::Empty(e) => (
                Some(String::from_utf8_lossy(e.local_name().as_ref()).into_owned()),
                true,
            ),
            Event::End(_) => (None, true),
            Event::Text(t) => {
                text.push_str(&t.decode().map_err(|e| e.to_string())?);
                continue;
            }
            Event::CData(t) => {
                text.push_str(&t.decode().map_err(|e| e.to_string())?);
                continue;
            }
            // Entities arrive as separate events since quick-xml 0.38.
            Event::GeneralRef(r) => {
                match r.resolve_char_ref().map_err(|e| e.to_string())? {
                    Some(c) => text.push(c),
                    None => {
                        let name = r.decode().map_err(|e| e.to_string())?;
                        text.push_str(
                            quick_xml::escape::resolve_predefined_entity(&name).unwrap_or_default(),
                        );
                    }
                }
                continue;
            }
            Event::Eof => break,
            _ => continue,
        };
        if let Some(name) = opened {
            match name.as_str() {
                "response" => {
                    href = None;
                    response_ok = true;
                    merged = Props::default();
                }
                "propstat" => {
                    props = Props {
                        ok: true,
                        ..Props::default()
                    }
                }
                "collection" if stack.iter().any(|s| s == "resourcetype") => props.dir = true,
                _ => {}
            }
            stack.push(name);
            text.clear();
        }
        if !closes {
            continue;
        }
        let name = stack.pop().ok_or("unbalanced end tag")?;
        let parent = stack.last().map(String::as_str);
        let value = text.trim();
        match (name.as_str(), parent) {
            ("href", Some("response")) => href = Some(value.to_string()),
            ("status", Some("propstat")) => props.ok = status_ok(value),
            ("status", Some("response")) => response_ok = status_ok(value),
            ("getcontentlength", _) => props.size = value.parse().ok(),
            ("getlastmodified", _) => {
                props.mtime = httpdate::parse_http_date(value).ok().map(system_time_ms)
            }
            // Unknown properties come back in a 404 propstat; only 2xx ones count.
            ("propstat", _) if props.ok => {
                merged.dir |= props.dir;
                merged.size = merged.size.or(props.size);
                merged.mtime = merged.mtime.or(props.mtime);
            }
            ("response", _) => {
                if let Some(h) = href.take() {
                    let stat = response_ok.then(|| {
                        let mtime = merged.mtime.unwrap_or(0);
                        if merged.dir {
                            RemoteStat::dir(mtime)
                        } else {
                            RemoteStat::file(merged.size.unwrap_or(0), mtime)
                        }
                    });
                    out.push(DavResponse {
                        path: href_path(&h),
                        stat,
                    });
                }
            }
            _ => {}
        }
        text.clear();
    }
    Ok(out)
}

/// `HTTP/1.1 200 OK` → true for any 2xx.
fn status_ok(line: &str) -> bool {
    line.split_whitespace()
        .nth(1)
        .and_then(|c| c.parse::<u16>().ok())
        .is_some_and(|c| (200..300).contains(&c))
}

/// Decoded, normalized path of an href, which may be an absolute URL or a path, percent-encoded, with or without a
/// trailing slash.
fn href_path(href: &str) -> String {
    let path = match href.find("://") {
        Some(i) => {
            let rest = &href[i + 3..];
            rest.find('/').map_or("/", |j| &rest[j..])
        }
        None => href,
    };
    let path = path.split(['?', '#']).next().unwrap_or_default();
    norm_remote(&percent_decode(path))
}

fn percent_decode(s: &str) -> String {
    let hex = |b: u8| (b as char).to_digit(16).map(|d| d as u8);
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2]))
        {
            out.push(h << 4 | l);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<d:multistatus xmlns:d="DAV:" xmlns:s="http://sabredav.org/ns">
  <d:response>
    <d:href>/remote.php/dav/files/me/My%20Docs/</d:href>
    <d:propstat>
      <d:prop>
        <d:resourcetype><d:collection/></d:resourcetype>
        <d:getlastmodified>Tue, 15 Nov 1994 08:12:31 GMT</d:getlastmodified>
      </d:prop>
      <d:status>HTTP/1.1 200 OK</d:status>
    </d:propstat>
    <d:propstat>
      <d:prop><d:getcontentlength/></d:prop>
      <d:status>HTTP/1.1 404 Not Found</d:status>
    </d:propstat>
  </d:response>
  <d:response>
    <d:href>https://cloud.example.com/remote.php/dav/files/me/My%20Docs/r%C3%A9sum%C3%A9%20final.txt</d:href>
    <d:propstat>
      <d:prop>
        <d:resourcetype/>
        <d:getcontentlength>1234</d:getcontentlength>
        <d:getlastmodified>Tue, 15 Nov 1994 08:12:31 GMT</d:getlastmodified>
      </d:prop>
      <d:status>HTTP/1.1 200 OK</d:status>
    </d:propstat>
  </d:response>
  <d:response>
    <d:href>https://cloud.example.com/remote.php/dav/files/me/My%20Docs/sub%20dir</d:href>
    <d:propstat>
      <d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop>
      <d:status>HTTP/1.1 200 OK</d:status>
    </d:propstat>
  </d:response>
  <d:response>
    <d:href>/remote.php/dav/files/me/My%20Docs/gone.txt</d:href>
    <d:status>HTTP/1.1 404 Not Found</d:status>
  </d:response>
</d:multistatus>"#;

    #[test]
    fn webdav_parses_listing_and_skips_self() {
        let list = parse_multistatus(LISTING).unwrap();
        assert_eq!(list.len(), 4);
        assert_eq!(list[0].path, "/remote.php/dav/files/me/My Docs");
        assert_eq!(list[0].stat, Some(RemoteStat::dir(784_887_151_000)));
        assert_eq!(list[3].stat, None);

        let entries = list_entries("/remote.php/dav/files/me/My Docs", list);
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["résumé final.txt", "sub dir"]);
        assert_eq!(entries[0].stat, RemoteStat::file(1234, 784_887_151_000));
        // No getlastmodified / getcontentlength.
        assert_eq!(entries[1].stat, RemoteStat::dir(0));
    }

    #[test]
    fn webdav_default_namespace_and_entities() {
        let xml = r#"<multistatus xmlns="DAV:"><response><href>/a/b%26c&amp;d&#x2713;/</href><propstat><prop>
            <resourcetype/><getcontentlength>5</getcontentlength></prop><status>HTTP/1.1 200 OK</status></propstat>
            </response></multistatus>"#;
        let list = parse_multistatus(xml).unwrap();
        assert_eq!(
            list,
            [DavResponse {
                path: "/a/b&c&d✓".into(),
                stat: Some(RemoteStat::file(5, 0))
            }]
        );
    }

    #[test]
    fn webdav_self_entry_without_matching_prefix() {
        let xml = r#"<D:multistatus xmlns:D="DAV:">
            <D:response><D:href>/proxy/dir/</D:href><D:propstat><D:prop><D:resourcetype><D:collection/></D:resourcetype>
            </D:prop><D:status>HTTP/1.1 200 OK</D:status></D:propstat></D:response>
            <D:response><D:href>/proxy/dir/x</D:href><D:propstat><D:prop><D:resourcetype/></D:prop>
            <D:status>HTTP/1.1 200 OK</D:status></D:propstat></D:response></D:multistatus>"#;
        let entries = list_entries("/dir", parse_multistatus(xml).unwrap());
        assert_eq!(
            entries.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(),
            ["x"]
        );
    }

    #[test]
    fn webdav_href_forms() {
        assert_eq!(href_path("http://h:8080"), "/");
        assert_eq!(href_path("http://h/a%20b/"), "/a b");
        assert_eq!(href_path("/a/%E2%9C%93?x=1"), "/a/✓");
        assert_eq!(percent_decode("100%"), "100%");
        assert_eq!(percent_decode("%zz%4"), "%zz%4");
    }

    #[test]
    fn webdav_urls() {
        let base =
            parse_endpoint("https://dav.example.com/remote.php/dav/files/me/", None).unwrap();
        assert_eq!(base.path(), "/remote.php/dav/files/me/");
        let ep = parse_endpoint("127.0.0.1", Some(8443)).unwrap();
        assert_eq!(ep.as_str(), "https://127.0.0.1:8443/");
        assert!(parse_endpoint("ftp://x", None).is_err());
    }

    #[test]
    fn webdav_digest_rfc7616_example() {
        // RFC 7616 §3.9.1, MD5 variant.
        let ch = DigestChallenge::parse(concat!(
            r#"Basic realm="x", Digest realm="http-auth@example.org", qop="auth, auth-int", algorithm=MD5, "#,
            r#"nonce="7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v", opaque="FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS""#
        ))
        .unwrap();
        assert_eq!(ch.qop.as_deref(), Some("auth"));
        let ch = DigestChallenge { nc: 1, ..ch };
        let auth = ch.authorization(
            "Mufasa",
            "Circle of Life",
            "GET",
            "/dir/index.html",
            "f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ",
        );
        assert!(
            auth.contains(r#"response="8ca523f5e9506fed4657c9700eebdbec""#),
            "{auth}"
        );
        assert!(auth.contains("nc=00000001"));
        assert!(DigestChallenge::parse(r#"Basic realm="x""#).is_none());
        assert!(
            DigestChallenge::parse(r#"Digest realm="r", nonce="n", algorithm=SHA-512-256"#)
                .is_none()
        );
    }
}
