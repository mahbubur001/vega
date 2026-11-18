//! FTP, FTPS (explicit `AUTH TLS`) and implicit FTPS on top of suppaftp's tokio client. Passive mode only and
//! always binary transfers, so file bytes are never rewritten.

use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};

use async_trait::async_trait;
use rustls::pki_types::ServerName;
use suppaftp::list::ListParser;
use suppaftp::tokio::{AsyncTlsConnector, ImplAsyncFtpStream, TokioTlsStream};
use suppaftp::types::FileType;
use suppaftp::{Encoding, FtpError, FtpResult, Mode, Status};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;

use super::*;
use crate::config::{DataProtection, ServerType};
use crate::error::{ErrorCode, RemoteError, Result};
use crate::tls::CertCapture;

type Ftp = ImplAsyncFtpStream<TlsOrPlain>;

const CHUNK: usize = 256 * 1024;

pub struct FtpClient {
    ftp: Ftp,
    host: String,
    /// PWD right after login. Cached because LIST mode changes the working directory.
    home: String,
    caps: Capabilities,
    /// MLST/MLSD advertised in FEAT and not disabled by `use_list`; cleared when the server rejects them anyway.
    mlsd: bool,
    mlst: bool,
    mdtm: bool,
    /// Present for FTPS: turns a rejected certificate into a `Certificate` error with the fingerprint.
    capture: Option<Arc<CertCapture>>,
    keep_alive_command: Option<String>,
    connected: AtomicBool,
}

/// Opens the control connection (TLS first for implicit FTPS, `AUTH TLS` for explicit), logs in and reads FEAT.
pub async fn connect(params: &ConnectParams) -> Result<FtpClient> {
    let server = &params.server;
    if !server.kind.is_ftp() {
        return Err(RemoteError::new(
            ErrorCode::Config,
            format!("{} is not an FTP server", server.name),
        ));
    }
    let host = server.host.trim().to_string();
    let port = server.port_or_default();
    let latin1 = server.encoding.as_deref().is_some_and(is_latin1);
    let clear_data = Arc::new(AtomicBool::new(false));
    let tls = match server.kind {
        ServerType::Ftps | ServerType::FtpsImplicit => {
            let (config, capture) =
                crate::tls::client_config(params.accepted_certs.clone(), server.disable_tls13)?;
            let connector = TlsConnector {
                inner: tokio_rustls::TlsConnector::from(config),
                clear_data: clear_data.clone(),
            };
            Some((connector, capture))
        }
        _ => None,
    };
    let secure = tls.is_some();
    let capture = tls.as_ref().map(|(_, c)| c.clone());
    let map = |e: FtpError| map_ftp_error(e, capture.as_deref(), &host);
    let timed_out = || RemoteError::connection(format!("Timed out connecting to {host}:{port}"));

    let open = async {
        let mut ftp = match tls {
            Some((connector, _)) if server.kind == ServerType::FtpsImplicit => {
                let mut ftp = Ftp::connect_secure_implicit((host.as_str(), port), connector, &host)
                    .await
                    .map_err(map)?;
                // Implicit FTPS skips AUTH TLS, so suppaftp never sends PBSZ/PROT.
                ftp.custom_command("PBSZ 0", &[Status::CommandOk])
                    .await
                    .map_err(map)?;
                ftp.custom_command("PROT P", &[Status::CommandOk])
                    .await
                    .map_err(map)?;
                ftp
            }
            Some((connector, _)) => {
                let ftp = Ftp::connect((host.as_str(), port)).await.map_err(map)?;
                // `into_secure` sends AUTH TLS, PBSZ 0 and PROT P.
                ftp.into_secure(connector, &host)
                    .await
                    .map_err(|e| match e {
                        FtpError::UnexpectedResponse(ref r)
                            if matches!(r.status.code(), 500..=504) =>
                        {
                            RemoteError::unsupported(format!(
                                "{host} does not support FTPS (AUTH TLS): {}",
                                reply_text(r)
                            ))
                        }
                        e => map(e),
                    })?
            }
            None => Ftp::connect((host.as_str(), port)).await.map_err(map)?,
        };
        if secure && server.data_protection == Some(DataProtection::Clear) {
            ftp.custom_command("PROT C", &[Status::CommandOk])
                .await
                .map_err(map)?;
            clear_data.store(true, Ordering::Release);
        }
        if latin1 {
            ftp.set_encoding(Latin1).await;
        }
        Ok::<_, RemoteError>(ftp)
    };
    let mut ftp = tokio::time::timeout(params.timeout, open)
        .await
        .map_err(|_| timed_out())??;

    let (user, pass) = if server.anonymous {
        ("anonymous".to_string(), "anonymous@".to_string())
    } else {
        (
            server.username.clone(),
            params.password.clone().unwrap_or_default(),
        )
    };
    let login = async {
        ftp.login(&user, &pass).await.map_err(|e| match e {
            FtpError::UnexpectedResponse(ref r) if matches!(r.status.code(), 430 | 530) => {
                RemoteError::new(
                    ErrorCode::Auth,
                    format!("Login failed for {user}@{host}: {}", reply_text(r)),
                )
            }
            e => map(e),
        })?;
        ftp.transfer_type(FileType::Binary).await.map_err(map)?;
        // Servers without FEAT get the most basic command set.
        let feat: std::collections::HashMap<String, Option<String>> = match ftp.feat().await {
            Ok(f) => f
                .into_iter()
                .map(|(k, v)| (k.to_ascii_uppercase(), v))
                .collect(),
            Err(FtpError::UnexpectedResponse(_)) => Default::default(),
            Err(e) => return Err(map(e)),
        };
        if !latin1 && feat.contains_key("UTF8") {
            // Some servers (IIS, older FileZilla Server) only send UTF-8 names after this; failure is harmless.
            let _ = ftp.opts("UTF8", Some("ON")).await;
        }
        let peer_is_v6 = ftp
            .get_ref()
            .await
            .peer_addr()
            .map(|a| a.is_ipv6())
            .unwrap_or(false);
        if feat.contains_key("EPSV") || peer_is_v6 {
            // EPSV only carries a port; the data connection goes to the control connection's address.
            ftp.set_mode(Mode::ExtendedPassive);
        } else {
            ftp.set_mode(Mode::Passive);
            // Servers behind NAT often announce their private address in the PASV reply.
            ftp.set_passive_nat_workaround(true);
        }
        let home = ftp.pwd().await.map_err(map)?;
        Ok((feat, home))
    };
    let (feat, home) = tokio::time::timeout(params.timeout, login)
        .await
        .map_err(|_| timed_out())??;

    let mlst = feat.contains_key("MLST") && !server.use_list;
    let mdtm = feat.contains_key("MDTM");
    let caps = Capabilities {
        // SITE CHMOD isn't advertised in FEAT; servers that lack it answer 500/502, reported as Unsupported.
        chmod: true,
        set_mtime: feat.contains_key("MFMT"),
        exec: false,
        reliable_mtime: mlst || mdtm,
    };
    Ok(FtpClient {
        ftp,
        host,
        home,
        caps,
        mlsd: mlst,
        mlst,
        mdtm,
        capture,
        keep_alive_command: server
            .keep_alive_command
            .clone()
            .filter(|c| !c.trim().is_empty()),
        connected: AtomicBool::new(true),
    })
}

impl FtpClient {
    /// The configured keep-alive command; `NOOP` is used when unset.
    pub fn keep_alive_command(&self) -> Option<&str> {
        self.keep_alive_command.as_deref()
    }

    /// Sends the keep-alive command so idle control connections aren't dropped by the server or a firewall.
    pub async fn keep_alive(&mut self) -> Result<()> {
        let cmd = self
            .keep_alive_command
            .clone()
            .unwrap_or_else(|| "NOOP".into());
        let ok = [
            Status::CommandOk,
            Status::CommandNotImplemented,
            Status::System,
            Status::Help,
            Status::PathCreated,
        ];
        let r = self
            .ftp
            .custom_command(cmd, &ok)
            .await
            .map(|_| ())
            .map_err(|e| self.err(e, ""));
        self.track(r)
    }

    fn err(&self, e: FtpError, path: &str) -> RemoteError {
        let e = map_ftp_error(e, self.capture.as_deref(), &self.host);
        if path.is_empty() {
            e
        } else {
            e.with_path(path)
        }
    }

    /// Marks the client dead after connection-level failures so the pool reconnects.
    fn track<T>(&self, r: Result<T>) -> Result<T> {
        if let Err(e) = &r
            && e.code == ErrorCode::Connection
        {
            self.connected.store(false, Ordering::Release);
        }
        r
    }

    async fn list_inner(&mut self, path: &str) -> Result<Vec<RemoteEntry>> {
        if self.mlsd {
            match self.ftp.mlsd(Some(path)).await {
                Ok(lines) => {
                    return Ok(lines
                        .iter()
                        .filter_map(|l| parse_mlsx(l))
                        .filter(|(name, ty, _)| !is_dot(name) && ty != "cdir" && ty != "pdir")
                        .map(|(name, _, stat)| RemoteEntry { name, stat })
                        .collect());
                }
                // Advertised but not implemented: remember and use LIST from now on.
                Err(FtpError::UnexpectedResponse(r)) if matches!(r.status.code(), 500 | 502) => {
                    self.mlsd = false
                }
                Err(e) => return Err(self.err(e, path)),
            }
        }
        // LIST output for a path argument is ambiguous (a file lists itself, spaces confuse some servers), so list
        // the working directory instead.
        self.ftp.cwd(path).await.map_err(|e| self.err(e, path))?;
        // `-a` shows dot files on vsftpd/ProFTPD/Pure-FTPd; servers that treat it as a file name reject it.
        let lines = match self.ftp.list(Some("-a")).await {
            Ok(lines) => lines,
            Err(FtpError::UnexpectedResponse(_)) => {
                self.ftp.list(None).await.map_err(|e| self.err(e, path))?
            }
            Err(e) => return Err(self.err(e, path)),
        };
        Ok(lines
            .iter()
            .filter_map(|l| parse_list_line(l))
            .filter(|e| !is_dot(&e.name))
            .collect())
    }

    async fn stat_inner(&mut self, path: &str) -> Result<Option<RemoteStat>> {
        if self.mlst {
            match self.ftp.mlst(Some(path)).await {
                Ok(line) => {
                    if let Some((_, _, stat)) = parse_mlsx(&line) {
                        return Ok(Some(stat));
                    }
                }
                Err(FtpError::UnexpectedResponse(r)) if r.status == Status::FileUnavailable => {
                    return Ok(None);
                }
                Err(FtpError::UnexpectedResponse(r)) if matches!(r.status.code(), 500 | 502) => {
                    self.mlst = false
                }
                Err(e) => return Err(self.err(e, path)),
            }
        }
        let name = crate::mapper::remote_name(path).to_string();
        if name.is_empty() {
            return Ok(Some(RemoteStat::dir(0)));
        }
        let parent = crate::mapper::remote_parent(path);
        match self.list_inner(&parent).await {
            Ok(entries) => {
                let Some(mut stat) = entries.into_iter().find(|e| e.name == name).map(|e| e.stat)
                else {
                    return Ok(None);
                };
                // LIST times have minute (or day) precision; MDTM has seconds.
                if stat.kind == EntryType::File
                    && self.mdtm
                    && let Ok(t) = self.ftp.mdtm(path).await
                {
                    stat.mtime = t.and_utc().timestamp_millis();
                }
                Ok(Some(stat))
            }
            Err(e) if e.code == ErrorCode::NotFound => Ok(None),
            // The parent may not be listable (permissions); probe the path itself.
            Err(e) if e.code == ErrorCode::Permission => self.probe(path, e).await,
            Err(e) => Err(e),
        }
    }

    /// Stat without a listing: SIZE answers for files, CWD for directories.
    async fn probe(&mut self, path: &str, fallback: RemoteError) -> Result<Option<RemoteStat>> {
        if let Ok(size) = self.ftp.size(path).await {
            let mtime = match self.ftp.mdtm(path).await {
                Ok(t) => t.and_utc().timestamp_millis(),
                Err(_) => 0,
            };
            return Ok(Some(RemoteStat::file(size as u64, mtime)));
        }
        match self.ftp.cwd(path).await {
            Ok(()) => Ok(Some(RemoteStat::dir(0))),
            Err(FtpError::UnexpectedResponse(r)) if r.status == Status::FileUnavailable => Ok(None),
            Err(_) => Err(fallback),
        }
    }

    async fn read_inner(&mut self, path: &str) -> Result<Vec<u8>> {
        let mut stream = self
            .ftp
            .retr_as_stream(path)
            .await
            .map_err(|e| self.err(e, path))?;
        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .await
            .map_err(|e| RemoteError::from(e).with_path(path))?;
        stream.finish().await.map_err(|e| self.err(e, path))?;
        Ok(buf)
    }

    async fn write_inner(&mut self, path: &str, data: &[u8]) -> Result<()> {
        let mut stream = self
            .ftp
            .put_with_stream(path)
            .await
            .map_err(|e| self.err(e, path))?;
        stream
            .write_all(data)
            .await
            .map_err(|e| RemoteError::from(e).with_path(path))?;
        stream.finish().await.map_err(|e| self.err(e, path))
    }

    async fn upload_inner(
        &mut self,
        local: &Path,
        remote: &str,
        progress: Progress<'_>,
    ) -> Result<()> {
        let local_err = |e: std::io::Error| RemoteError::from(e).with_path(local.to_string_lossy());
        let mut src = tokio::fs::File::open(local).await.map_err(local_err)?;
        let total = src.metadata().await.map_err(local_err)?.len();
        let mut stream = self
            .ftp
            .put_with_stream(remote)
            .await
            .map_err(|e| self.err(e, remote))?;
        let mut buf = vec![0u8; CHUNK];
        let mut done = 0u64;
        loop {
            let n = src.read(&mut buf).await.map_err(local_err)?;
            if n == 0 {
                break;
            }
            stream
                .write_all(&buf[..n])
                .await
                .map_err(|e| RemoteError::from(e).with_path(remote))?;
            done += n as u64;
            progress(done, total);
        }
        stream.finish().await.map_err(|e| self.err(e, remote))
    }

    async fn download_inner(
        &mut self,
        remote: &str,
        local: &Path,
        progress: Progress<'_>,
    ) -> Result<()> {
        // SIZE is optional; without it progress reports a total of 0.
        let total = self.ftp.size(remote).await.map(|s| s as u64).unwrap_or(0);
        let mut stream = self
            .ftp
            .retr_as_stream(remote)
            .await
            .map_err(|e| self.err(e, remote))?;
        // Created only after RETR succeeded, so a missing remote file leaves no empty local file.
        let local_err = |e: std::io::Error| RemoteError::from(e).with_path(local.to_string_lossy());
        let mut dst = tokio::fs::File::create(local).await.map_err(local_err)?;
        let mut buf = vec![0u8; CHUNK];
        let mut done = 0u64;
        loop {
            let n = stream
                .read(&mut buf)
                .await
                .map_err(|e| RemoteError::from(e).with_path(remote))?;
            if n == 0 {
                break;
            }
            dst.write_all(&buf[..n]).await.map_err(local_err)?;
            done += n as u64;
            progress(done, total.max(done));
        }
        dst.flush().await.map_err(local_err)?;
        stream.finish().await.map_err(|e| self.err(e, remote))
    }

    async fn mkdirp_inner(&mut self, path: &str) -> Result<()> {
        if self.ftp.cwd(path).await.is_ok() {
            return Ok(());
        }
        let norm = crate::mapper::norm_remote(path);
        let mut current = String::new();
        for part in norm.split('/').filter(|p| !p.is_empty()) {
            current.push('/');
            current.push_str(part);
            // CWD is the cheapest "is this an existing directory" check and needs no data connection.
            if self.ftp.cwd(&current).await.is_ok() {
                continue;
            }
            if let Err(e) = self.ftp.mkdir(&current).await {
                // Lost a race with another client, or the server refuses MKD on an existing dir.
                if self.ftp.cwd(&current).await.is_err() {
                    return Err(self.err(e, &current));
                }
            }
        }
        Ok(())
    }

    async fn set_mtime_inner(&mut self, path: &str, mtime_ms: i64) -> Result<()> {
        if !self.caps.set_mtime {
            return Err(
                RemoteError::unsupported("The server does not support MFMT").with_path(path)
            );
        }
        let cmd = format!("MFMT {} {path}", format_ftp_time(mtime_ms));
        self.ftp
            .custom_command(cmd, &[Status::File])
            .await
            .map(|_| ())
            .map_err(|e| self.err(e, path))
    }
}

#[async_trait]
impl RemoteClient for FtpClient {
    fn capabilities(&self) -> Capabilities {
        self.caps
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    async fn close(&mut self) {
        if self.is_connected() {
            let _ = self.ftp.quit().await;
        }
        self.connected.store(false, Ordering::Release);
    }

    async fn home_dir(&mut self) -> Result<String> {
        Ok(self.home.clone())
    }

    async fn list(&mut self, path: &str) -> Result<Vec<RemoteEntry>> {
        let r = self.list_inner(path).await;
        let mut entries = self.track(r)?;
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    async fn stat(&mut self, path: &str) -> Result<Option<RemoteStat>> {
        let r = self.stat_inner(path).await;
        self.track(r)
    }

    async fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        let r = self.read_inner(path).await;
        self.track(r)
    }

    async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        let r = self.write_inner(path, data).await;
        self.track(r)
    }

    async fn upload(&mut self, local: &Path, remote: &str, progress: Progress<'_>) -> Result<()> {
        let r = self.upload_inner(local, remote, progress).await;
        self.track(r)
    }

    async fn download(&mut self, remote: &str, local: &Path, progress: Progress<'_>) -> Result<()> {
        let r = self.download_inner(remote, local, progress).await;
        self.track(r)
    }

    async fn mkdirp(&mut self, path: &str) -> Result<()> {
        let r = self.mkdirp_inner(path).await;
        self.track(r)
    }

    async fn delete_file(&mut self, path: &str) -> Result<()> {
        let r = self.ftp.rm(path).await.map_err(|e| self.err(e, path));
        self.track(r)
    }

    async fn delete_dir(&mut self, path: &str, recursive: bool) -> Result<()> {
        if recursive {
            return delete_tree(self, path).await;
        }
        let r = self.ftp.rmdir(path).await.map_err(|e| self.err(e, path));
        self.track(r)
    }

    async fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        let r = self
            .ftp
            .rename(from, to)
            .await
            .map_err(|e| self.err(e, from));
        self.track(r)
    }

    async fn chmod(&mut self, path: &str, mode: u32) -> Result<()> {
        let r = self
            .ftp
            .site(format!("CHMOD {:o} {path}", mode & 0o7777))
            .await
            .map(|_| ())
            .map_err(|e| match e {
                FtpError::UnexpectedResponse(ref r)
                    if matches!(r.status.code(), 500 | 502 | 504) =>
                {
                    RemoteError::unsupported(format!(
                        "The server does not support SITE CHMOD: {}",
                        reply_text(r)
                    ))
                    .with_path(path)
                }
                e => self.err(e, path),
            });
        self.track(r)
    }

    async fn set_mtime(&mut self, path: &str, mtime_ms: i64) -> Result<()> {
        let r = self.set_mtime_inner(path, mtime_ms).await;
        self.track(r)
    }

    async fn exec(&mut self, _command: &str) -> Result<ExecOutput> {
        Err(RemoteError::unsupported(
            "Shell commands are not available over FTP",
        ))
    }
}

/// Reply text without the status code, for error messages.
fn reply_text(r: &suppaftp::types::Response) -> String {
    let text = String::from_utf8_lossy(&r.body);
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            if l.len() >= 4 && l.as_bytes()[..3].iter().all(u8::is_ascii_digit) {
                l[4..].trim()
            } else {
                l
            }
        })
        .filter(|l| !l.is_empty())
        .collect();
    if lines.is_empty() {
        format!("server replied {}", r.status.code())
    } else {
        lines.join(" ")
    }
}

fn map_ftp_error(e: FtpError, capture: Option<&CertCapture>, host: &str) -> RemoteError {
    match e {
        FtpError::UnexpectedResponse(r) => {
            let text = reply_text(&r);
            let lower = text.to_ascii_lowercase();
            let code = match r.status.code() {
                430 | 530 => ErrorCode::Auth,
                550 if lower.contains("permission") || lower.contains("denied") => {
                    ErrorCode::Permission
                }
                550 => ErrorCode::NotFound,
                553 | 532 => ErrorCode::Permission,
                500 | 502 | 504 => ErrorCode::Unsupported,
                421 | 425 | 426 => ErrorCode::Connection,
                _ => ErrorCode::Unknown,
            };
            RemoteError::new(code, text)
        }
        FtpError::SecureError(msg) => {
            let fallback = RemoteError::connection(format!("TLS error with {host}: {msg}"));
            match capture {
                Some(c) => c.error_or(host, fallback),
                None => fallback,
            }
        }
        FtpError::ConnectionError(io) if io.kind() == std::io::ErrorKind::InvalidInput => {
            // Names that can't be sent: CR/LF in a path, or characters outside Latin-1.
            RemoteError::new(ErrorCode::Unknown, io.to_string())
        }
        FtpError::ConnectionError(io) => {
            let mut e = RemoteError::from(io);
            e.code = ErrorCode::Connection;
            e
        }
        FtpError::InvalidAddress(a) => {
            RemoteError::connection(format!("Invalid passive address: {a}"))
        }
        FtpError::BadResponse => {
            RemoteError::new(ErrorCode::Unknown, "Unexpected reply from the FTP server")
        }
        FtpError::DataConnectionAlreadyOpen => {
            RemoteError::new(ErrorCode::Unknown, "A transfer is already running")
        }
    }
}

fn is_dot(name: &str) -> bool {
    name.is_empty() || name == "." || name == ".."
}

fn is_latin1(enc: &str) -> bool {
    matches!(
        enc.trim().to_ascii_lowercase().as_str(),
        "latin1" | "latin-1" | "iso-8859-1" | "iso8859-1"
    )
}

/// ISO-8859-1 names: every byte is the code point of the same value.
#[derive(Debug)]
struct Latin1;

impl Encoding for Latin1 {
    fn decode(&self, bytes: &[u8]) -> String {
        bytes.iter().map(|&b| char::from(b)).collect()
    }

    fn encode(&self, text: &str) -> std::io::Result<Vec<u8>> {
        text.chars()
            .map(|c| {
                u8::try_from(u32::from(c)).map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("{c:?} cannot be sent as Latin-1"),
                    )
                })
            })
            .collect()
    }
}

/// Parses an MLSD/MLST line (`fact=value;…; name`) into name, lowercased type and stat. Hand-rolled because servers
/// send types suppaftp rejects (`OS.unix=slink:…`).
fn parse_mlsx(line: &str) -> Option<(String, String, RemoteStat)> {
    let line = line.trim_end_matches(['\r', '\n']);
    let (facts, name) = line.split_once(' ')?;
    let mut kind = String::new();
    let mut stat = RemoteStat::file(0, 0);
    for fact in facts.split(';').filter(|f| !f.is_empty()) {
        let Some((key, value)) = fact.split_once('=') else {
            continue;
        };
        match key.to_ascii_lowercase().as_str() {
            "type" => kind = value.to_ascii_lowercase(),
            "size" | "sizd" => stat.size = value.parse().unwrap_or(0),
            "modify" => stat.mtime = parse_ftp_time(value).unwrap_or(0),
            "unix.mode" => stat.mode = u32::from_str_radix(value, 8).ok().map(|m| m & 0o7777),
            _ => {}
        }
    }
    stat.kind = match kind.as_str() {
        "dir" | "cdir" | "pdir" => EntryType::Dir,
        k if k.starts_with("os.unix=slink") || k.starts_with("os.unix=symlink") => {
            EntryType::Symlink
        }
        _ => EntryType::File,
    };
    if stat.kind == EntryType::Dir {
        stat.size = 0;
    }
    Some((name.to_string(), kind, stat))
}

/// Parses one LIST line (Unix `ls -l` or DOS style). "total N" and other noise yield `None`.
fn parse_list_line(line: &str) -> Option<RemoteEntry> {
    let (file, posix) = match ListParser::parse_posix(line) {
        Ok(f) => (f, true),
        Err(_) => (ListParser::parse_dos(line).ok()?, false),
    };
    let kind = if file.is_directory() {
        EntryType::Dir
    } else if file.is_symlink() {
        EntryType::Symlink
    } else {
        EntryType::File
    };
    // DOS listings carry no permissions; suppaftp reports them as rwx for everyone.
    let mode = posix.then(|| {
        use suppaftp::list::PosixPexQuery as Q;
        [(Q::Owner, 6), (Q::Group, 3), (Q::Others, 0)]
            .iter()
            .fold(0u32, |m, &(who, shift)| {
                let bits = (file.can_read(who) as u32) << 2
                    | (file.can_write(who) as u32) << 1
                    | file.can_execute(who) as u32;
                m | bits << shift
            })
    });
    let stat = RemoteStat {
        kind,
        target_kind: None,
        size: if kind == EntryType::Dir {
            0
        } else {
            file.size() as u64
        },
        mtime: system_time_ms(file.modified()),
        mode,
    };
    Some(RemoteEntry {
        name: file.name().to_string(),
        stat,
    })
}

/// `YYYYMMDDhhmmss[.sss]` (UTC, RFC 3659) → milliseconds since the epoch.
fn parse_ftp_time(s: &str) -> Option<i64> {
    let (main, frac) = s.split_once('.').unwrap_or((s, ""));
    if main.len() != 14 || !main.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n = |r: std::ops::Range<usize>| main[r].parse::<i64>().ok();
    let (y, mo, d, h, mi, se) = (
        n(0..4)?,
        n(4..6)?,
        n(6..8)?,
        n(8..10)?,
        n(10..12)?,
        n(12..14)?,
    );
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) {
        return None;
    }
    let frac = &frac[..frac.len().min(3)];
    let ms = if frac.bytes().all(|b| b.is_ascii_digit()) {
        format!("{frac:0<3}").parse().unwrap_or(0)
    } else {
        0
    };
    let days = days_from_civil(y, mo, d);
    Some((days * 86_400 + h * 3600 + mi * 60 + se) * 1000 + ms)
}

/// Milliseconds since the epoch → `YYYYMMDDhhmmss` (UTC), as MFMT expects.
fn format_ftp_time(ms: i64) -> String {
    let secs = ms.div_euclid(1000);
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}{m:02}{d:02}{:02}{:02}{:02}",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// TLS connector handed to suppaftp. It also opens the data connections, which stay plain TCP after `PROT C`.
struct TlsConnector {
    inner: tokio_rustls::TlsConnector,
    clear_data: Arc<AtomicBool>,
}

impl std::fmt::Debug for TlsConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsConnector").finish_non_exhaustive()
    }
}

#[async_trait]
impl AsyncTlsConnector for TlsConnector {
    type Stream = TlsOrPlain;

    async fn connect(&self, domain: &str, stream: TcpStream) -> FtpResult<TlsOrPlain> {
        if self.clear_data.load(Ordering::Acquire) {
            return Ok(TlsOrPlain::Plain(stream));
        }
        let name = ServerName::try_from(domain.to_owned())
            .map_err(|e| FtpError::SecureError(e.to_string()))?;
        let tls = self
            .inner
            .connect(name, stream)
            .await
            .map_err(|e| FtpError::SecureError(e.to_string()))?;
        Ok(TlsOrPlain::Tls(Box::new(tls)))
    }
}

/// Control and data stream type for every FTP flavour: TLS, or plain TCP for unprotected (`PROT C`) data.
#[derive(Debug)]
enum TlsOrPlain {
    Tls(Box<tokio_rustls::client::TlsStream<TcpStream>>),
    Plain(TcpStream),
}

impl AsyncRead for TlsOrPlain {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsOrPlain::Tls(s) => Pin::new(s.as_mut()).poll_read(cx, buf),
            TlsOrPlain::Plain(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsOrPlain {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            TlsOrPlain::Tls(s) => Pin::new(s.as_mut()).poll_write(cx, buf),
            TlsOrPlain::Plain(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsOrPlain::Tls(s) => Pin::new(s.as_mut()).poll_flush(cx),
            TlsOrPlain::Plain(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsOrPlain::Tls(s) => Pin::new(s.as_mut()).poll_shutdown(cx),
            TlsOrPlain::Plain(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

impl TokioTlsStream for TlsOrPlain {
    type InnerStream = Self;

    fn tcp_stream(self) -> FtpResult<TcpStream> {
        Ok(match self {
            TlsOrPlain::Tls(s) => s.into_inner().0,
            TlsOrPlain::Plain(s) => s,
        })
    }

    fn get_ref(&self) -> &TcpStream {
        match self {
            TlsOrPlain::Tls(s) => s.get_ref().0,
            TlsOrPlain::Plain(s) => s,
        }
    }

    fn mut_ref(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ftp_time_round_trip() {
        assert_eq!(parse_ftp_time("19700101000000"), Some(0));
        assert_eq!(parse_ftp_time("20200913122640.5"), Some(1_600_000_000_500));
        assert_eq!(format_ftp_time(1_600_000_000_999), "20200913122640");
        assert_eq!(format_ftp_time(-1000), "19691231235959");
        assert_eq!(parse_ftp_time("2020"), None);
    }

    #[test]
    fn mlsd_lines() {
        let (name, ty, st) =
            parse_mlsx("type=file;size=12;modify=20200913122640;UNIX.mode=0640; a b.txt").unwrap();
        assert_eq!((name.as_str(), ty.as_str()), ("a b.txt", "file"));
        assert_eq!(
            (st.kind, st.size, st.mtime, st.mode),
            (EntryType::File, 12, 1_600_000_000_000, Some(0o640))
        );
        let (_, ty, st) = parse_mlsx("type=cdir;sizd=4096;modify=20200913122640; .").unwrap();
        assert_eq!((ty.as_str(), st.kind, st.size), ("cdir", EntryType::Dir, 0));
        let (_, _, st) = parse_mlsx("type=OS.unix=slink:/etc;modify=20200913122640; etc").unwrap();
        assert_eq!(st.kind, EntryType::Symlink);
    }

    #[test]
    fn list_lines() {
        let e = parse_list_line("-rw-r----- 1 ftp ftp 14 Sep 13 2020 up file.txt").unwrap();
        assert_eq!(
            (e.name.as_str(), e.stat.kind, e.stat.size, e.stat.mode),
            ("up file.txt", EntryType::File, 14, Some(0o640))
        );
        let e = parse_list_line("drwxr-xr-x 2 ftp ftp 4096 Sep 13 12:00 dir").unwrap();
        assert_eq!((e.stat.kind, e.stat.mode), (EntryType::Dir, Some(0o755)));
        assert!(parse_list_line("total 8").is_none());
    }

    #[test]
    fn reply_text_strips_codes() {
        let r = suppaftp::types::Response::new(
            Status::FileUnavailable,
            b"550 /x: No such file or directory\r\n".to_vec(),
        );
        assert_eq!(reply_text(&r), "/x: No such file or directory");
        let e = map_ftp_error(FtpError::UnexpectedResponse(r), None, "h");
        assert_eq!(e.code, ErrorCode::NotFound);
        let r = suppaftp::types::Response::new(
            Status::FileUnavailable,
            b"550 x: Permission denied\r\n".to_vec(),
        );
        assert_eq!(
            map_ftp_error(FtpError::UnexpectedResponse(r), None, "h").code,
            ErrorCode::Permission
        );
    }

    #[test]
    fn latin1_names() {
        assert_eq!(Latin1.decode(&[0x63, 0x61, 0x66, 0xE9]), "café");
        assert_eq!(Latin1.encode("café").unwrap(), vec![0x63, 0x61, 0x66, 0xE9]);
        assert!(Latin1.encode("€").is_err());
    }
}
