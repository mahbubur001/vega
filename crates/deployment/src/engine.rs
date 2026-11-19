//! One engine per project: config, connections, the transfer queue and state, plus every deployment action. The
//! language server, the app and the CLI all drive it (directly or over IPC).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, oneshot};

use crate::config::{AutoUpload, ConfigStore, OverwriteCheck, ResolvedServer, Server, ServerType};
use crate::credentials::Credentials;
use crate::deploy::{self, Target, Uploader, collect_local, collect_remote, local_meta};
use crate::error::{ErrorCode, RemoteError, Result};
use crate::known_hosts::HostKeyTrust;
use crate::manager::{ConnectContext, ConnectionSettings, ServerManager, TestStep};
use crate::mapper::{PathMapper, join_remote, rebase_remote, remote_name, remote_parent};
use crate::prompt::{Level, Prompter};
use crate::protocol::{RemoteEntry, RemoteStat};
use crate::queue::{TaskKind, TaskOutcome, TaskSpec, TransferQueue};
use crate::state::StateCache;
use crate::sync::{CompareMode, CompareRequest, SyncAction, SyncItem};
use crate::with_client;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum OpenBehavior {
    /// Open the file directly from the server.
    #[default]
    Remote,
    /// Open the local mapped copy when it exists.
    LocalIfMapped,
}

/// Editor-level settings (Zed `lsp.iremote.settings`), not part of the project config.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EngineSettings {
    pub timeout_seconds: u64,
    pub idle_close_seconds: u64,
    pub known_hosts_files: Vec<String>,
    pub open_behavior: OpenBehavior,
    pub show_hidden_files: bool,
    pub offer_git_ignore: bool,
    /// Largest file downloaded just to compare content.
    pub max_compare_bytes: u64,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            timeout_seconds: 20,
            idle_close_seconds: 300,
            known_hosts_files: Vec::new(),
            open_behavior: OpenBehavior::Remote,
            show_hidden_files: true,
            offer_git_ignore: true,
            max_compare_bytes: 10 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub transferred: u32,
    pub skipped: u32,
    pub failed: Vec<Failure>,
    pub cancelled: bool,
}

impl Summary {
    fn absorb(&mut self, path: &str, r: Result<TaskOutcome>) {
        match r {
            Ok(TaskOutcome::Done) => self.transferred += 1,
            Ok(TaskOutcome::Skipped) => self.skipped += 1,
            Err(e) if e.code == ErrorCode::Cancelled => self.cancelled = true,
            Err(e) => self.failed.push(Failure {
                path: path.to_string(),
                error: e.to_string(),
            }),
        }
    }

    fn merge(&mut self, o: Summary) {
        self.transferred += o.transferred;
        self.skipped += o.skipped;
        self.failed.extend(o.failed);
        self.cancelled |= o.cancelled;
    }

    /// One-line result for notifications, e.g. "Uploaded 3 files to Production".
    pub fn describe(&self, verb: &str, server: &str) -> String {
        let mut s = match self.transferred {
            0 => format!("Nothing {} {server}", verb.to_lowercase()),
            1 => format!(
                "{verb} 1 file {} {server}",
                if verb == "Downloaded" { "from" } else { "to" }
            ),
            n => format!(
                "{verb} {n} files {} {server}",
                if verb == "Downloaded" { "from" } else { "to" }
            ),
        };
        if self.transferred == 0 && verb == "Downloaded" {
            s = format!("Nothing downloaded from {server}");
        } else if self.transferred == 0 {
            s = format!("Nothing uploaded to {server}");
        }
        if self.skipped > 0 {
            s.push_str(&format!(", {} skipped", self.skipped));
        }
        if !self.failed.is_empty() {
            s.push_str(&format!(
                ", {} failed: {}",
                self.failed.len(),
                self.failed[0].error
            ));
        }
        if self.cancelled {
            s.push_str(" (cancelled)");
        }
        s
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    pub at: i64,
    pub level: Level,
    pub message: String,
}

/// A remote file opened for editing: its local cache copy maps back here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteEdit {
    pub server_id: String,
    pub remote: String,
}

/// Remote Host listing entry with mapping information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteItem {
    pub name: String,
    pub path: String,
    #[serde(flatten)]
    pub stat: RemoteStat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<PathBuf>,
    pub local_exists: bool,
    pub excluded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEvent<'a> {
    Changed(&'a Path),
    Removed(&'a Path),
    Renamed(&'a Path, &'a Path),
}

pub struct Engine {
    root: PathBuf,
    config: RwLock<ConfigStore>,
    settings: RwLock<EngineSettings>,
    pub manager: ServerManager,
    pub queue: Arc<TransferQueue>,
    state: Arc<StateCache>,
    prompter: Arc<dyn Prompter>,
    own_writes: Mutex<HashMap<PathBuf, Instant>>,
    recent_saves: Mutex<HashMap<PathBuf, Instant>>,
    edits: Mutex<HashMap<PathBuf, RemoteEdit>>,
    log: broadcast::Sender<LogLine>,
}

fn edits_dir(root: &Path) -> PathBuf {
    crate::dirs::cache_dir()
        .join("edit")
        .join(crate::dirs::project_key(root))
}

impl Engine {
    pub fn new(
        root: &Path,
        prompter: Arc<dyn Prompter>,
        credentials: Arc<Credentials>,
    ) -> Arc<Self> {
        Self::with_parts(root, prompter, credentials, ConfigStore::load(root))
    }

    pub fn with_parts(
        root: &Path,
        prompter: Arc<dyn Prompter>,
        credentials: Arc<Credentials>,
        config: ConfigStore,
    ) -> Arc<Self> {
        let trust = Arc::new(HostKeyTrust::new(
            crate::dirs::trusted_hosts_file(),
            Vec::new(),
        ));
        let ctx = Arc::new(ConnectContext::new(credentials, prompter.clone(), trust));
        let state = Arc::new(StateCache::load(
            crate::dirs::state_dir().join(format!("{}.json", crate::dirs::project_key(root))),
        ));
        let edits = std::fs::read(edits_dir(root).join("index.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        let engine = Arc::new_cyclic(|weak: &Weak<Engine>| {
            let w = weak.clone();
            ctx.set_pin_handler(Arc::new(move |server_id: &str, fp: &str| {
                if let Some(e) = w.upgrade() {
                    e.pin_certificate(server_id, fp);
                }
            }));
            Engine {
                root: root.to_path_buf(),
                config: RwLock::new(config),
                settings: RwLock::new(EngineSettings::default()),
                manager: ServerManager::new(ctx.clone()),
                queue: TransferQueue::new(),
                state,
                prompter,
                own_writes: Mutex::default(),
                recent_saves: Mutex::default(),
                edits: Mutex::new(edits),
                log: broadcast::channel(512).0,
            }
        });
        let weak = Arc::downgrade(&engine);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(Duration::from_secs(30));
            loop {
                tick.tick().await;
                let Some(e) = weak.upgrade() else { break };
                e.manager.reap().await;
                let _ = e.state.flush();
            }
        });
        engine
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn prompter(&self) -> &Arc<dyn Prompter> {
        &self.prompter
    }

    // ---- config ---------------------------------------------------------------------------------------------

    pub fn config(&self) -> std::sync::RwLockReadGuard<'_, ConfigStore> {
        self.config.read().unwrap_or_else(|e| e.into_inner())
    }

    pub fn config_mut(&self) -> std::sync::RwLockWriteGuard<'_, ConfigStore> {
        self.config.write().unwrap_or_else(|e| e.into_inner())
    }

    pub fn reload_config(self: &Arc<Self>) {
        self.config_mut().reload();
    }

    pub fn settings(&self) -> EngineSettings {
        self.settings.read().map(|s| s.clone()).unwrap_or_default()
    }

    pub fn set_settings(&self, s: EngineSettings) {
        if let Ok(mut c) = self.manager.ctx.settings.write() {
            *c = ConnectionSettings {
                timeout: Duration::from_secs(s.timeout_seconds.clamp(3, 600)),
                idle_close: Duration::from_secs(s.idle_close_seconds.max(10)),
                known_hosts_files: s
                    .known_hosts_files
                    .iter()
                    .map(|p| crate::dirs::expand_home(p))
                    .collect(),
            };
        }
        if let Ok(mut cur) = self.settings.write() {
            *cur = s;
        }
    }

    fn pin_certificate(&self, server_id: &str, fingerprint: &str) {
        let mut cfg = self.config_mut();
        if let Some(r) = cfg.server(server_id) {
            let mut s = r.server;
            s.trusted_certificate_fingerprint = Some(fingerprint.to_string());
            if let Err(e) = cfg.upsert_server(s, r.shared) {
                self.log_line(Level::Error, format!("Cannot pin certificate: {e}"));
            }
        }
    }

    pub fn servers(&self) -> Vec<ResolvedServer> {
        self.config().servers()
    }

    pub fn default_server(&self) -> Option<ResolvedServer> {
        self.config().default_server()
    }

    pub fn target(&self, server: &Server) -> Target {
        let options = self.config().options().clone();
        Target {
            mapper: PathMapper::new(&self.root, server, &options.exclude_names),
            server: server.clone(),
            options,
        }
    }

    fn target_by_id(&self, id: &str) -> Result<Target> {
        let s = self.config().server(id).ok_or_else(|| {
            RemoteError::new(ErrorCode::Config, format!("Unknown server \"{id}\"."))
        })?;
        if !s.server.kind.transfers() {
            return Err(RemoteError::unsupported(format!(
                "\"{}\" ({}) does not transfer files.",
                s.server.name,
                s.server.kind.label()
            )));
        }
        Ok(self.target(&s.server))
    }

    /// The servers an action addresses: the given one (a group expands to its members) or the default server.
    pub fn targets(&self, server_id: Option<&str>) -> Result<Vec<Target>> {
        let cfg = self.config();
        let id = match server_id {
            Some(id) => id.to_string(),
            None => cfg.default_server_id().map(String::from).ok_or_else(|| {
                RemoteError::new(
                    ErrorCode::Config,
                    "No default server. Choose one in Deployment Settings.",
                )
            })?,
        };
        let servers = cfg.expand_targets(&id);
        drop(cfg);
        if servers.is_empty() {
            return Err(RemoteError::new(
                ErrorCode::Config,
                format!("\"{id}\" has no servers that transfer files."),
            ));
        }
        Ok(servers.iter().map(|r| self.target(&r.server)).collect())
    }

    // ---- logging ---------------------------------------------------------------------------------------------

    pub fn subscribe_log(&self) -> broadcast::Receiver<LogLine> {
        self.log.subscribe()
    }

    pub fn log_line(&self, level: Level, message: impl Into<String>) {
        let message = message.into();
        let configured = self.config().options().logging;
        let show = match level {
            Level::Error => true,
            Level::Warning | Level::Info => configured >= crate::config::LogLevel::Brief,
        };
        if show {
            let _ = self.log.send(LogLine {
                at: crate::state::now_ms(),
                level,
                message,
            });
        }
    }

    fn report(&self, summary: &Summary, verb: &str, server: &str) {
        let text = summary.describe(verb, server);
        if summary.failed.is_empty() {
            self.log_line(Level::Info, &text);
        } else {
            for f in &summary.failed {
                self.log_line(Level::Error, format!("{}: {}", f.path, f.error));
            }
            self.prompter.notify(Level::Error, &text);
        }
    }

    // ---- own writes ------------------------------------------------------------------------------------------

    fn mark_own_write(&self, path: &Path) {
        let mut w = self.own_writes.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        w.retain(|_, until| *until > now);
        w.insert(path.to_path_buf(), now + Duration::from_secs(5));
    }

    /// Recently written by a download; automatic upload must not echo it back.
    pub fn is_own_write(&self, path: &Path) -> bool {
        self.own_writes
            .lock()
            .map(|w| w.get(path).is_some_and(|until| *until > Instant::now()))
            .unwrap_or(false)
    }

    // ---- queued primitives -----------------------------------------------------------------------------------

    fn queue_upload(
        self: &Arc<Self>,
        t: Arc<Target>,
        local: PathBuf,
        remote: String,
        check: bool,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let engine = self.clone();
        let pool = self.manager.pool(&t.server);
        self.queue.enqueue(TaskSpec {
            kind: TaskKind::Upload,
            server_id: t.server.id.clone(),
            server_name: t.server.name.clone(),
            source: local.display().to_string(),
            target: remote.clone(),
            key: format!("{}:{remote}", t.server.id),
            concurrency: t.options.concurrency as usize,
            job: Box::new(move |progress| {
                Box::pin(async move {
                    let up = Uploader {
                        cache: &engine.state,
                        prompter: &engine.prompter,
                    };
                    let p = move |d, total| progress(d, total);
                    with_client!(pool, |c| up.upload(c, &t, &local, &remote, check, &p))
                })
            }),
        })
    }

    fn queue_download(
        self: &Arc<Self>,
        t: Arc<Target>,
        remote: String,
        local: PathBuf,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let engine = self.clone();
        let pool = self.manager.pool(&t.server);
        self.queue.enqueue(TaskSpec {
            kind: TaskKind::Download,
            server_id: t.server.id.clone(),
            server_name: t.server.name.clone(),
            source: remote.clone(),
            target: local.display().to_string(),
            key: format!("{}:{remote}", t.server.id),
            concurrency: t.options.concurrency as usize,
            job: Box::new(move |progress| {
                Box::pin(async move {
                    engine.mark_own_write(&local);
                    let p = move |d, total| progress(d, total);
                    let r = with_client!(pool, |c| deploy::download_to(
                        c,
                        &engine.state,
                        &t,
                        &remote,
                        &local,
                        &p
                    ));
                    engine.mark_own_write(&local);
                    r.map(|_| TaskOutcome::Done)
                })
            }),
        })
    }

    fn queue_simple(
        self: &Arc<Self>,
        t: &Target,
        kind: TaskKind,
        source: String,
        target: String,
        op: impl FnOnce(
            Arc<Engine>,
            crate::pool::ConnectionPool,
            Target,
        ) -> futures::future::BoxFuture<'static, Result<TaskOutcome>>
        + Send
        + 'static,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let engine = self.clone();
        let pool = self.manager.pool(&t.server);
        let tt = t.clone();
        self.queue.enqueue(TaskSpec {
            kind,
            server_id: t.server.id.clone(),
            server_name: t.server.name.clone(),
            key: format!("{}:{source}", t.server.id),
            source,
            target,
            concurrency: t.options.concurrency as usize,
            job: Box::new(move |_| op(engine, pool, tt)),
        })
    }

    async fn collect(receivers: Vec<(String, oneshot::Receiver<Result<TaskOutcome>>)>) -> Summary {
        let mut s = Summary::default();
        for (path, rx) in receivers {
            s.absorb(
                &path,
                rx.await.unwrap_or_else(|_| Err(RemoteError::cancelled())),
            );
        }
        s
    }

    // ---- upload / download -----------------------------------------------------------------------------------

    /// "Upload to…": files and folders, to a server (or group, or the default server).
    pub async fn upload(
        self: &Arc<Self>,
        server_id: Option<&str>,
        paths: &[PathBuf],
        explicit: bool,
    ) -> Result<Summary> {
        let mut total = Summary::default();
        for t in self.targets(server_id)? {
            total.merge(self.upload_to(Arc::new(t), paths, explicit).await?);
        }
        Ok(total)
    }

    async fn upload_to(
        self: &Arc<Self>,
        t: Arc<Target>,
        paths: &[PathBuf],
        explicit: bool,
    ) -> Result<Summary> {
        let (files, dirs) = collect_local(&t, paths);
        if files.is_empty() && dirs.is_empty() {
            let unmapped = paths.iter().find(|p| t.mapper.to_remote(p).is_none());
            return Err(RemoteError::new(
                ErrorCode::Config,
                match unmapped {
                    Some(p) => format!(
                        "{} is not mapped on {}. Add a mapping in Deployment Settings.",
                        p.display(),
                        t.server.name
                    ),
                    None => format!(
                        "Nothing to upload: the selection is excluded on {}.",
                        t.server.name
                    ),
                },
            ));
        }
        if explicit && t.options.confirm_upload {
            let msg = format!("Upload {} file(s) to {}?", files.len(), t.server.name);
            if !self.prompter.confirm("Upload", &msg, "Upload").await {
                return Ok(Summary {
                    cancelled: true,
                    ..Default::default()
                });
            }
        }
        let folder_upload = paths.iter().any(|p| p.is_dir());
        let check = t.options.warn_on_newer_remote != OverwriteCheck::None;
        let mut summary = Summary::default();
        let mut receivers = Vec::new();
        for f in &files {
            let Some(remote) = t.mapper.to_remote(f) else {
                continue;
            };
            if folder_upload
                && !t.options.overwrite_up_to_date
                && self.up_to_date(&t, f, &remote).await
            {
                summary.skipped += 1;
                continue;
            }
            receivers.push((
                f.display().to_string(),
                self.queue_upload(t.clone(), f.clone(), remote, check),
            ));
        }
        if t.options.create_empty_directories {
            let with_files: std::collections::HashSet<&Path> =
                files.iter().filter_map(|f| f.parent()).collect();
            for d in dirs.iter().filter(|d| !with_files.contains(d.as_path())) {
                if let Some(remote) = t.mapper.to_remote(d) {
                    receivers.push((d.display().to_string(), self.queue_mkdir(&t, remote)));
                }
            }
        }
        summary.merge(Self::collect(receivers).await);
        if folder_upload && t.options.delete_missing_target && !summary.cancelled {
            summary.merge(self.delete_missing_remote(&t, paths, &files).await?);
        }
        let _ = self.state.flush();
        self.report(&summary, "Uploaded", &t.server.name);
        Ok(summary)
    }

    /// Unchanged since the last transfer ("Overwrite up-to-date files" off).
    async fn up_to_date(&self, t: &Target, local: &Path, remote: &str) -> bool {
        let (Some(rec), Ok(m)) = (
            self.state.get(&t.server.id, remote),
            local_meta(local).await,
        ) else {
            return false;
        };
        rec.local_size == m.size && deploy::same_time(rec.local_mtime, m.mtime)
    }

    fn queue_mkdir(
        self: &Arc<Self>,
        t: &Target,
        remote: String,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let r2 = remote.clone();
        self.queue_simple(
            t,
            TaskKind::Mkdir,
            remote.clone(),
            remote,
            move |_, pool, t| {
                Box::pin(async move {
                    with_client!(pool, |c| deploy::ensure_remote_dir(c, &t, &r2))
                        .map(|_| TaskOutcome::Done)
                })
            },
        )
    }

    /// "Delete target items when source ones do not exist" for folder uploads.
    async fn delete_missing_remote(
        self: &Arc<Self>,
        t: &Arc<Target>,
        roots: &[PathBuf],
        local_files: &[PathBuf],
    ) -> Result<Summary> {
        let pool = self.manager.pool(&t.server);
        let local_remote: std::collections::HashSet<String> = local_files
            .iter()
            .filter_map(|f| t.mapper.to_remote(f))
            .collect();
        let mut extra = Vec::new();
        for root in roots.iter().filter(|r| r.is_dir()) {
            let Some(rr) = t.mapper.to_remote(root) else {
                continue;
            };
            let (files, _) = with_client!(pool, |c| collect_remote(c, t, &rr))?;
            extra.extend(
                files
                    .into_iter()
                    .map(|(p, _)| p)
                    .filter(|p| !local_remote.contains(p)),
            );
        }
        if extra.is_empty() {
            return Ok(Summary::default());
        }
        if t.options.confirm_remote_delete {
            let msg = format!(
                "Delete {} file(s) on {} that no longer exist locally?",
                extra.len(),
                t.server.name
            );
            if !self
                .prompter
                .confirm("Delete remote files", &msg, "Delete")
                .await
            {
                return Ok(Summary::default());
            }
        }
        let receivers = extra
            .into_iter()
            .map(|p| (p.clone(), self.queue_delete(t, p, false)))
            .collect();
        Ok(Self::collect(receivers).await)
    }

    /// "Download from…". Paths are local (files or folders); their mapped remote counterparts are fetched.
    pub async fn download(
        self: &Arc<Self>,
        server_id: Option<&str>,
        local_paths: &[PathBuf],
    ) -> Result<Summary> {
        let t = Arc::new(
            self.targets(server_id)?
                .into_iter()
                .next()
                .expect("targets is non-empty"),
        );
        let remotes: Vec<String> = local_paths
            .iter()
            .filter_map(|p| t.mapper.to_remote(p))
            .collect();
        if remotes.is_empty() {
            return Err(RemoteError::new(
                ErrorCode::Config,
                format!("The selection is not mapped on {}.", t.server.name),
            ));
        }
        self.download_remote(t, &remotes, true).await
    }

    /// Downloads remote files/folders to their mapped local paths.
    pub async fn download_remote(
        self: &Arc<Self>,
        t: Arc<Target>,
        remotes: &[String],
        explicit: bool,
    ) -> Result<Summary> {
        let pool = self.manager.pool(&t.server);
        let mut pairs = Vec::new();
        let mut empty_dirs = Vec::new();
        for r in remotes {
            let (files, dirs) = with_client!(pool, |c| collect_remote(c, &t, r))?;
            for (path, _) in &files {
                if let Some(local) = t
                    .mapper
                    .to_local(path)
                    .filter(|l| !t.mapper.is_local_excluded(l))
                {
                    pairs.push((path.clone(), local));
                }
            }
            if t.options.create_empty_directories {
                for d in dirs {
                    let prefix = format!("{d}/");
                    if !files.iter().any(|(f, _)| f.starts_with(&prefix))
                        && let Some(l) = t.mapper.to_local(&d)
                    {
                        empty_dirs.push(l);
                    }
                }
            }
        }
        if explicit && t.options.prompt_overwrite_local {
            let mut changed = 0;
            for (remote, local) in &pairs {
                if local.exists() && !self.up_to_date(&t, local, remote).await {
                    changed += 1;
                }
            }
            if changed > 0 {
                let msg = format!(
                    "Overwrite {changed} local file(s) with the version from {}?",
                    t.server.name
                );
                if !self.prompter.confirm("Download", &msg, "Overwrite").await {
                    return Ok(Summary {
                        cancelled: true,
                        ..Default::default()
                    });
                }
            }
        }
        for d in empty_dirs {
            self.mark_own_write(&d);
            let _ = tokio::fs::create_dir_all(&d).await;
        }
        let receivers = pairs
            .into_iter()
            .map(|(r, l)| (r.clone(), self.queue_download(t.clone(), r, l)))
            .collect();
        let summary = Self::collect(receivers).await;
        let _ = self.state.flush();
        self.report(&summary, "Downloaded", &t.server.name);
        Ok(summary)
    }

    // ---- compare ---------------------------------------------------------------------------------------------

    /// "Compare with Deployed Version": fetches the server copy and shows a diff.
    /// Downloads the server's version of `local` to a cache file (for diffs) and returns its path.
    pub async fn fetch_deployed_copy(
        self: &Arc<Self>,
        server_id: Option<&str>,
        local: &Path,
    ) -> Result<PathBuf> {
        let t = self
            .targets(server_id)?
            .into_iter()
            .next()
            .expect("targets is non-empty");
        let remote = t.mapper.to_remote(local).ok_or_else(|| {
            RemoteError::new(
                ErrorCode::Config,
                format!("{} is not mapped on {}.", local.display(), t.server.name),
            )
        })?;
        let pool = self.manager.pool(&t.server);
        with_client!(pool, |c| deploy::fetch_for_compare(c, &t, &remote))
    }

    pub async fn compare(
        self: &Arc<Self>,
        server_id: Option<&str>,
        local: &Path,
    ) -> Result<PathBuf> {
        let t = self
            .targets(server_id)?
            .into_iter()
            .next()
            .expect("targets is non-empty");
        let remote = t.mapper.to_remote(local).ok_or_else(|| {
            RemoteError::new(
                ErrorCode::Config,
                format!("{} is not mapped on {}.", local.display(), t.server.name),
            )
        })?;
        let copy = self.fetch_deployed_copy(server_id, local).await?;
        self.prompter
            .show_diff(
                &copy,
                local,
                &format!("{} ↔ {}", remote_name(&remote), t.server.name),
            )
            .await;
        Ok(copy)
    }

    pub fn web_url(&self, server_id: Option<&str>, local: &Path) -> Result<String> {
        let id = match server_id {
            Some(id) => id.to_string(),
            None => self
                .config()
                .default_server_id()
                .map(String::from)
                .ok_or_else(|| RemoteError::new(ErrorCode::Config, "No default server."))?,
        };
        let server = self.config().server(&id).ok_or_else(|| {
            RemoteError::new(ErrorCode::Config, format!("Unknown server \"{id}\"."))
        })?;
        let t = self.target(&server.server);
        if server.server.kind == ServerType::InPlace {
            let rel = local
                .strip_prefix(&self.root)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            return Ok(format!(
                "{}/{}",
                server.server.web_url.trim_end_matches('/'),
                rel
            ));
        }
        t.mapper.web_url_for_local(local).ok_or_else(|| {
            RemoteError::new(
                ErrorCode::Config,
                format!(
                    "{} has no web URL (set Web server URL and a web path).",
                    server.server.name
                ),
            )
        })
    }

    // ---- automatic upload --------------------------------------------------------------------------------------

    /// A file was saved in the editor. Uploads per "Upload changed files automatically"; edits of remote files go
    /// back to their server.
    pub async fn on_saved(self: &Arc<Self>, path: &Path, explicit: bool) {
        if let Some(edit) = self.remote_edit(path) {
            if let Err(e) = self.upload_edit(path, &edit).await {
                self.prompter.notify(
                    Level::Error,
                    &format!("Upload of {} failed: {e}", edit.remote),
                );
            }
            return;
        }
        let mode = self.config().options().auto_upload;
        let wanted = match mode {
            AutoUpload::Never => false,
            AutoUpload::OnExplicitSave => explicit,
            AutoUpload::Always => true,
        };
        if !wanted || self.is_own_write(path) {
            return;
        }
        if let Ok(mut s) = self.recent_saves.lock() {
            s.insert(path.to_path_buf(), Instant::now());
        }
        self.auto_upload(path).await;
    }

    async fn auto_upload(self: &Arc<Self>, path: &Path) {
        let Ok(targets) = self.targets(None) else {
            return;
        };
        for t in targets {
            if !t.mapper.is_deployable(path) {
                continue;
            }
            let Some(remote) = t.mapper.to_remote(path) else {
                continue;
            };
            let check = t.options.warn_on_newer_remote != OverwriteCheck::None;
            let name = t.server.name.clone();
            let r = self
                .queue_upload(Arc::new(t), path.to_path_buf(), remote.clone(), check)
                .await;
            match r.unwrap_or_else(|_| Err(RemoteError::cancelled())) {
                Ok(TaskOutcome::Done) => {
                    self.log_line(Level::Info, format!("Uploaded {remote} to {name}"))
                }
                Ok(TaskOutcome::Skipped) => self.log_line(
                    Level::Info,
                    format!("Skipped {remote} (kept the server copy)"),
                ),
                Err(e) if e.code == ErrorCode::Cancelled => {}
                Err(e) => {
                    self.log_line(Level::Error, format!("{remote}: {e}"));
                    self.prompter
                        .notify(Level::Error, &format!("Upload to {name} failed: {e}"));
                }
            }
        }
        let _ = self.state.flush();
    }

    /// File-system changes reported by the host (external changes, deletes, renames).
    pub async fn on_fs_event(self: &Arc<Self>, event: FsEvent<'_>) {
        let options = self.config().options().clone();
        if options.auto_upload == AutoUpload::Never {
            return;
        }
        match event {
            FsEvent::Changed(path) => {
                if options.auto_upload != AutoUpload::Always
                    || options.skip_external_changes
                    || !path.is_file()
                {
                    return;
                }
                let recently_saved = self
                    .recent_saves
                    .lock()
                    .map(|s| {
                        s.get(path)
                            .is_some_and(|t| t.elapsed() < Duration::from_secs(3))
                    })
                    .unwrap_or(false);
                if recently_saved || self.is_own_write(path) || self.remote_edit(path).is_some() {
                    return;
                }
                self.auto_upload(path).await;
            }
            FsEvent::Removed(path) => {
                if !options.delete_remote_on_local_delete || self.is_own_write(path) {
                    return;
                }
                let Ok(targets) = self.targets(None) else {
                    return;
                };
                for t in targets {
                    if t.mapper.is_local_excluded(path) {
                        continue;
                    }
                    let Some(remote) = t.mapper.to_remote(path) else {
                        continue;
                    };
                    if options.confirm_remote_delete {
                        let msg = format!(
                            "{} was deleted locally. Delete {remote} on {} too?",
                            remote_name(&remote),
                            t.server.name
                        );
                        if !self
                            .prompter
                            .confirm("Delete on server", &msg, "Delete")
                            .await
                        {
                            continue;
                        }
                    }
                    let r = self.queue_delete(&t, remote.clone(), true).await;
                    if let Ok(Err(e)) = r {
                        self.log_line(Level::Error, format!("Delete {remote}: {e}"));
                    }
                }
            }
            FsEvent::Renamed(from, to) => {
                let Ok(targets) = self.targets(None) else {
                    return;
                };
                for t in targets {
                    let (Some(rf), Some(rt)) = (t.mapper.to_remote(from), t.mapper.to_remote(to))
                    else {
                        continue;
                    };
                    if t.mapper.is_local_excluded(to) {
                        continue;
                    }
                    let tt = t.clone();
                    let r = self.queue_rename(&tt, rf.clone(), rt.clone()).await;
                    match r {
                        Ok(Ok(_)) => self.log_line(
                            Level::Info,
                            format!("Renamed {rf} → {rt} on {}", t.server.name),
                        ),
                        Ok(Err(e)) => self.log_line(Level::Error, format!("Rename {rf}: {e}")),
                        Err(_) => {}
                    }
                }
            }
        }
    }

    // ---- remote operations (Remote Host) ---------------------------------------------------------------------

    pub async fn remote_list(
        self: &Arc<Self>,
        server_id: &str,
        path: &str,
    ) -> Result<Vec<RemoteItem>> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        let entries: Vec<RemoteEntry> = with_client!(pool, |c| c.list(path))?;
        let show_hidden = self.settings().show_hidden_files;
        let mut items: Vec<RemoteItem> = entries
            .into_iter()
            .filter(|e| show_hidden || !e.name.starts_with('.'))
            .map(|e| {
                let full = join_remote(path, &e.name);
                let local = t.mapper.to_local(&full);
                RemoteItem {
                    local_exists: local.as_ref().is_some_and(|l| l.exists()),
                    excluded: t.mapper.is_remote_excluded(&full),
                    local,
                    path: full,
                    name: e.name,
                    stat: e.stat,
                }
            })
            .collect();
        items.sort_by(|a, b| {
            b.stat
                .is_dir()
                .cmp(&a.stat.is_dir())
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        Ok(items)
    }

    /// The directory Remote Host starts in: the first mapping's remote root, else the root path.
    pub fn remote_start(&self, server_id: &str) -> Result<String> {
        let t = self.target_by_id(server_id)?;
        Ok(t.mapper
            .remote_roots()
            .into_iter()
            .next()
            .unwrap_or_else(|| {
                if t.server.root_path.is_empty() {
                    "/".into()
                } else {
                    t.server.root_path.clone()
                }
            }))
    }

    pub async fn remote_stat(
        self: &Arc<Self>,
        server_id: &str,
        path: &str,
    ) -> Result<Option<RemoteStat>> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        with_client!(pool, |c| c.stat(path))
    }

    pub async fn remote_mkdir(self: &Arc<Self>, server_id: &str, path: &str) -> Result<()> {
        let t = self.target_by_id(server_id)?;
        self.queue_mkdir(&t, path.to_string())
            .await
            .unwrap_or_else(|_| Err(RemoteError::cancelled()))
            .map(|_| ())
    }

    pub async fn remote_new_file(self: &Arc<Self>, server_id: &str, path: &str) -> Result<()> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        with_client!(pool, |c| async {
            if c.stat(path).await?.is_some() {
                return Err(RemoteError::new(
                    ErrorCode::Exists,
                    "A file with this name already exists",
                )
                .with_path(path));
            }
            c.write_file(path, b"").await
        })
    }

    fn queue_delete(
        self: &Arc<Self>,
        t: &Target,
        remote: String,
        unknown_kind: bool,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let r = remote.clone();
        self.queue_simple(
            t,
            TaskKind::Delete,
            remote.clone(),
            remote,
            move |engine, pool, t| {
                Box::pin(async move {
                    let res = with_client!(pool, |c| async {
                        let is_dir = if unknown_kind {
                            c.stat(&r).await?.is_some_and(|s| s.is_dir())
                        } else {
                            false
                        };
                        let result = if is_dir {
                            c.delete_dir(&r, true).await
                        } else {
                            c.delete_file(&r).await
                        };
                        match result {
                            Err(e) if e.code == ErrorCode::NotFound => Ok(is_dir),
                            Err(e) => Err(e),
                            Ok(()) => Ok(is_dir),
                        }
                    });
                    let is_dir = res?;
                    engine.state.delete(&t.server.id, &r, is_dir);
                    Ok(TaskOutcome::Done)
                })
            },
        )
    }

    pub async fn remote_delete(
        self: &Arc<Self>,
        server_id: &str,
        paths: &[String],
    ) -> Result<Summary> {
        let t = self.target_by_id(server_id)?;
        if t.options.confirm_remote_delete {
            let msg = if paths.len() == 1 {
                format!(
                    "Delete {} on {}? This cannot be undone.",
                    paths[0], t.server.name
                )
            } else {
                format!(
                    "Delete {} items on {}? This cannot be undone.",
                    paths.len(),
                    t.server.name
                )
            };
            if !self
                .prompter
                .confirm("Delete on server", &msg, "Delete")
                .await
            {
                return Ok(Summary {
                    cancelled: true,
                    ..Default::default()
                });
            }
        }
        let receivers = paths
            .iter()
            .map(|p| (p.clone(), self.queue_delete(&t, p.clone(), true)))
            .collect();
        Ok(Self::collect(receivers).await)
    }

    fn queue_rename(
        self: &Arc<Self>,
        t: &Target,
        from: String,
        to: String,
    ) -> oneshot::Receiver<Result<TaskOutcome>> {
        let (f, tt) = (from.clone(), to.clone());
        self.queue_simple(
            t,
            TaskKind::Rename,
            from,
            to,
            move |engine, pool, target| {
                Box::pin(async move {
                    let res = with_client!(pool, |c| async {
                        match c.rename(&f, &tt).await {
                            Ok(()) => Ok(true),
                            Err(e)
                                if matches!(e.code, ErrorCode::NotFound | ErrorCode::Unknown) =>
                            {
                                if c.stat(&f).await?.is_none() {
                                    return Ok(false); // never deployed: nothing to rename
                                }
                                deploy::ensure_remote_dir(c, &target, &remote_parent(&tt)).await?;
                                c.rename(&f, &tt).await.map(|_| true)
                            }
                            Err(e) => Err(e),
                        }
                    })?;
                    if res {
                        engine.state.rename(&target.server.id, &f, &tt);
                    }
                    Ok(if res {
                        TaskOutcome::Done
                    } else {
                        TaskOutcome::Skipped
                    })
                })
            },
        )
    }

    pub async fn remote_rename(
        self: &Arc<Self>,
        server_id: &str,
        from: &str,
        to: &str,
    ) -> Result<()> {
        let t = self.target_by_id(server_id)?;
        self.queue_rename(&t, from.into(), to.into())
            .await
            .unwrap_or_else(|_| Err(RemoteError::cancelled()))
            .map(|_| ())
    }

    /// Copies remote files or folders on the same server (Remote Host Copy/Paste). Each pair is a
    /// source path and the path its copy gets; folders are copied with their contents.
    pub async fn remote_copy(
        self: &Arc<Self>,
        server_id: &str,
        copies: &[(String, String)],
    ) -> Result<Summary> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        let mut summary = Summary::default();
        for (src, dest_root) in copies {
            let (files, dirs) = with_client!(pool, |c| collect_remote(c, &t, src))?;
            for dir in dirs {
                let Some(dest) = rebase_remote(src, dest_root, &dir) else {
                    continue;
                };
                with_client!(pool, |c| deploy::ensure_remote_dir(c, &t, &dest))?;
            }
            for (f, _) in files {
                let Some(dest) = rebase_remote(src, dest_root, &f) else {
                    continue;
                };
                let r = with_client!(pool, |c| async {
                    let data = c.read_file(&f).await?;
                    deploy::ensure_remote_dir(c, &t, &remote_parent(&dest)).await?;
                    c.write_file(&dest, &data).await
                });
                summary.absorb(&f, r.map(|_| TaskOutcome::Done));
            }
        }
        Ok(summary)
    }

    pub async fn remote_chmod(
        self: &Arc<Self>,
        server_id: &str,
        path: &str,
        mode: u32,
        recursive: bool,
    ) -> Result<()> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        let targets: Vec<String> = if recursive {
            let (files, dirs) = with_client!(pool, |c| collect_remote(c, &t, path))?;
            dirs.into_iter()
                .chain(files.into_iter().map(|(f, _)| f))
                .collect()
        } else {
            vec![path.to_string()]
        };
        for p in targets {
            with_client!(pool, |c| c.chmod(&p, mode))?;
        }
        Ok(())
    }

    /// Changes permissions under `path` (itself included): folders get `dir_mode`, files get
    /// `file_mode`; `None` leaves that kind alone (Remote Host "Change Permissions…" on a folder).
    pub async fn remote_chmod_tree(
        self: &Arc<Self>,
        server_id: &str,
        path: &str,
        file_mode: Option<u32>,
        dir_mode: Option<u32>,
    ) -> Result<()> {
        let t = self.target_by_id(server_id)?;
        let pool = self.manager.pool(&t.server);
        let (files, dirs) = with_client!(pool, |c| collect_remote(c, &t, path))?;
        let changes = dirs
            .into_iter()
            .filter_map(|dir| Some((dir, dir_mode?)))
            .chain(
                files
                    .into_iter()
                    .filter_map(|(file, _)| Some((file, file_mode?))),
            );
        for (p, mode) in changes {
            with_client!(pool, |c| c.chmod(&p, mode))?;
        }
        Ok(())
    }

    /// Download from Remote Host to the mapped local paths.
    pub async fn remote_download(
        self: &Arc<Self>,
        server_id: &str,
        paths: &[String],
    ) -> Result<Summary> {
        let t = Arc::new(self.target_by_id(server_id)?);
        if let Some(p) = paths.iter().find(|p| t.mapper.to_local(p).is_none()) {
            return Err(RemoteError::new(
                ErrorCode::Config,
                format!("{p} is not mapped to a local path."),
            ));
        }
        self.download_remote(t, paths, true).await
    }

    /// Upload Here: local files/folders into a remote folder (no mapping needed).
    pub async fn remote_upload_here(
        self: &Arc<Self>,
        server_id: &str,
        locals: &[PathBuf],
        remote_dir: &str,
    ) -> Result<Summary> {
        let t = Arc::new(self.target_by_id(server_id)?);
        let mut receivers = Vec::new();
        for local in locals {
            let base = local.parent().unwrap_or(Path::new("/"));
            let files: Vec<PathBuf> = if local.is_dir() {
                walk_files(local)
            } else {
                vec![local.clone()]
            };
            for f in files {
                let rel = f
                    .strip_prefix(base)
                    .unwrap_or(&f)
                    .to_string_lossy()
                    .replace('\\', "/");
                let remote = join_remote(remote_dir, &rel);
                receivers.push((
                    f.display().to_string(),
                    self.queue_upload(t.clone(), f, remote, false),
                ));
            }
        }
        let summary = Self::collect(receivers).await;
        self.report(&summary, "Uploaded", &t.server.name);
        Ok(summary)
    }

    /// Adds a remote path to the server's excluded paths (Remote Host "Exclude Path").
    pub fn exclude_remote(&self, server_id: &str, path: &str, exclude: bool) -> Result<()> {
        let mut cfg = self.config_mut();
        let r = cfg
            .server(server_id)
            .ok_or_else(|| RemoteError::new(ErrorCode::Config, "Unknown server"))?;
        let mut s = r.server;
        s.excluded_remote.retain(|p| p != path);
        if exclude {
            s.excluded_remote.push(path.to_string());
        }
        cfg.upsert_server(s, r.shared)
    }

    // ---- remote editing --------------------------------------------------------------------------------------

    pub fn remote_edit(&self, local: &Path) -> Option<RemoteEdit> {
        self.edits.lock().ok().and_then(|e| e.get(local).cloned())
    }

    fn save_edits(&self) {
        if let Ok(e) = self.edits.lock()
            && let Ok(bytes) = serde_json::to_vec_pretty(&*e)
        {
            let _ = crate::dirs::write_atomic(&edits_dir(&self.root).join("index.json"), &bytes);
        }
    }

    /// Opens a remote file: the mapped local copy (when preferred and present) or a cached copy whose saves are
    /// uploaded back.
    pub async fn remote_open(
        self: &Arc<Self>,
        server_id: &str,
        remote: &str,
        force_remote: bool,
    ) -> Result<PathBuf> {
        let t = self.target_by_id(server_id)?;
        if !force_remote
            && self.settings().open_behavior == OpenBehavior::LocalIfMapped
            && let Some(local) = t.mapper.to_local(remote).filter(|l| l.is_file())
        {
            return Ok(local);
        }
        let local = edits_dir(&self.root)
            .join(&t.server.id)
            .join(remote.trim_start_matches('/'));
        let pool = self.manager.pool(&t.server);
        self.mark_own_write(&local);
        with_client!(pool, |c| deploy::download_to(
            c,
            &self.state,
            &t,
            remote,
            &local,
            &crate::protocol::no_progress
        ))?;
        self.mark_own_write(&local);
        if let Ok(mut e) = self.edits.lock() {
            e.insert(
                local.clone(),
                RemoteEdit {
                    server_id: t.server.id.clone(),
                    remote: remote.to_string(),
                },
            );
        }
        self.save_edits();
        let _ = self.state.flush();
        Ok(local)
    }

    async fn upload_edit(self: &Arc<Self>, local: &Path, edit: &RemoteEdit) -> Result<()> {
        let mut t = self.target_by_id(&edit.server_id)?;
        if t.options.warn_on_newer_remote == OverwriteCheck::None {
            t.options.warn_on_newer_remote = OverwriteCheck::TimestampAndSize;
        }
        let name = t.server.name.clone();
        let r = self
            .queue_upload(Arc::new(t), local.to_path_buf(), edit.remote.clone(), true)
            .await;
        match r.unwrap_or_else(|_| Err(RemoteError::cancelled()))? {
            TaskOutcome::Done => {
                self.log_line(Level::Info, format!("Saved {} on {name}", edit.remote))
            }
            TaskOutcome::Skipped => self.log_line(
                Level::Info,
                format!("Kept the server copy of {}", edit.remote),
            ),
        }
        let _ = self.state.flush();
        Ok(())
    }

    /// Remote-edit copies whose server version changed since they were downloaded (notify about remote
    /// changes).
    pub async fn changed_remote_edits(
        self: &Arc<Self>,
        only: Option<&Path>,
    ) -> Vec<(PathBuf, RemoteEdit)> {
        if !self.config().options().notify_remote_changes {
            return Vec::new();
        }
        let edits: Vec<(PathBuf, RemoteEdit)> = self
            .edits
            .lock()
            .map(|e| {
                e.iter()
                    .filter(|(p, _)| only.is_none_or(|o| o == p.as_path()))
                    .map(|(p, e)| (p.clone(), e.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let mut changed = Vec::new();
        for (local, edit) in edits {
            let Some(rec) = self.state.get(&edit.server_id, &edit.remote) else {
                continue;
            };
            if let Ok(Some(st)) = self.remote_stat(&edit.server_id, &edit.remote).await {
                let time_known = st.mtime > 0 && rec.remote_mtime > 0;
                if st.size != rec.remote_size
                    || (time_known && !deploy::same_time(st.mtime, rec.remote_mtime))
                {
                    changed.push((local, edit));
                }
            }
        }
        changed
    }

    /// Diff of a remote-edit copy against the current server version.
    /// Whether the default server's copy of the project file `local` changed since VEGA last
    /// uploaded or downloaded it ("Notify of remote changes"). Files never transferred are not
    /// checked, so opening them doesn't connect or ask for a password.
    pub async fn deployed_copy_changed(self: &Arc<Self>, local: &Path) -> Result<bool> {
        let options = self.config().options().clone();
        if !options.notify_remote_changes || options.warn_on_newer_remote == OverwriteCheck::None {
            return Ok(false);
        }
        let Some(t) = self.targets(None)?.into_iter().next() else {
            return Ok(false);
        };
        let Some(remote) = t.mapper.to_remote(local) else {
            return Ok(false);
        };
        let Some(record) = self.state.get(&t.server.id, &remote) else {
            return Ok(false);
        };
        let Some(stat) = self.remote_stat(&t.server.id, &remote).await? else {
            return Ok(false);
        };
        let time_known = stat.mtime > 0 && record.remote_mtime > 0;
        Ok(stat.size != record.remote_size
            || (time_known && !deploy::same_time(stat.mtime, record.remote_mtime)))
    }

    pub async fn compare_remote_edit(self: &Arc<Self>, local: &Path) -> Result<()> {
        let edit = self
            .remote_edit(local)
            .ok_or_else(|| RemoteError::new(ErrorCode::Config, "Not a remote file"))?;
        let t = self.target_by_id(&edit.server_id)?;
        let pool = self.manager.pool(&t.server);
        let copy = with_client!(pool, |c| deploy::fetch_for_compare(c, &t, &edit.remote))?;
        self.prompter
            .show_diff(
                &copy,
                local,
                &format!("{} ↔ {}", remote_name(&edit.remote), t.server.name),
            )
            .await;
        Ok(())
    }

    /// Re-downloads a remote-edit copy.
    pub async fn reload_remote_edit(self: &Arc<Self>, local: &Path) -> Result<()> {
        let edit = self
            .remote_edit(local)
            .ok_or_else(|| RemoteError::new(ErrorCode::Config, "Not a remote file"))?;
        self.remote_open(&edit.server_id, &edit.remote, true)
            .await
            .map(|_| ())
    }

    // ---- sync ------------------------------------------------------------------------------------------------

    pub async fn sync_compare(
        self: &Arc<Self>,
        server_id: Option<&str>,
        roots: &[PathBuf],
        mode: CompareMode,
    ) -> Result<(String, Vec<SyncItem>)> {
        let t = self
            .targets(server_id)?
            .into_iter()
            .next()
            .expect("targets is non-empty");
        let roots: Vec<PathBuf> = if roots.is_empty() {
            t.mapper.local_roots()
        } else {
            roots.to_vec()
        };
        let pool = self.manager.pool(&t.server);
        let max = self.settings().max_compare_bytes;
        let items = with_client!(pool, |c| crate::sync::compare(
            c,
            &self.state,
            CompareRequest {
                target: &t,
                project_root: &self.root,
                local_roots: &roots,
                mode,
                max_download_bytes: max
            }
        ))?;
        let _ = self.state.flush();
        Ok((t.server.id, items))
    }

    pub async fn sync_execute(
        self: &Arc<Self>,
        server_id: &str,
        items: &[SyncItem],
    ) -> Result<Summary> {
        let t = Arc::new(self.target_by_id(server_id)?);
        let deletes = items
            .iter()
            .filter(|i| matches!(i.action, SyncAction::DeleteRemote | SyncAction::DeleteLocal))
            .count();
        if deletes > 0 && t.options.confirm_remote_delete {
            let msg = format!("Synchronizing deletes {deletes} file(s). Continue?");
            if !self
                .prompter
                .confirm("Synchronize", &msg, "Synchronize")
                .await
            {
                return Ok(Summary {
                    cancelled: true,
                    ..Default::default()
                });
            }
        }
        let mut receivers = Vec::new();
        let mut summary = Summary::default();
        for item in items {
            match item.action {
                SyncAction::Upload => receivers.push((
                    item.rel_path.clone(),
                    self.queue_upload(
                        t.clone(),
                        item.local_path.clone(),
                        item.remote_path.clone(),
                        false,
                    ),
                )),
                SyncAction::Download => receivers.push((
                    item.rel_path.clone(),
                    self.queue_download(
                        t.clone(),
                        item.remote_path.clone(),
                        item.local_path.clone(),
                    ),
                )),
                SyncAction::DeleteRemote => receivers.push((
                    item.rel_path.clone(),
                    self.queue_delete(&t, item.remote_path.clone(), false),
                )),
                SyncAction::DeleteLocal => {
                    self.mark_own_write(&item.local_path);
                    match tokio::fs::remove_file(&item.local_path).await {
                        Ok(()) => {
                            self.state.delete(&t.server.id, &item.remote_path, false);
                            summary.transferred += 1;
                        }
                        Err(e) => summary.failed.push(Failure {
                            path: item.rel_path.clone(),
                            error: e.to_string(),
                        }),
                    }
                }
                SyncAction::Skip => {}
            }
        }
        summary.merge(Self::collect(receivers).await);
        let _ = self.state.flush();
        self.report(&summary, "Synchronized", &t.server.name);
        Ok(summary)
    }

    // ---- workflow triggers -----------------------------------------------------------------------------------

    /// Downloads every mapped folder from a server (create a project from existing files on the server).
    pub async fn download_project(self: &Arc<Self>, server_id: Option<&str>) -> Result<Summary> {
        let t = Arc::new(
            self.targets(server_id)?
                .into_iter()
                .next()
                .expect("targets is non-empty"),
        );
        let roots = t.mapper.remote_roots();
        if roots.is_empty() {
            return Err(RemoteError::new(
                ErrorCode::Config,
                format!("{} has no mappings.", t.server.name),
            ));
        }
        self.download_remote(t, &roots, true).await
    }

    /// Uploads files changed since their last transfer under the mapping roots (upload before run).
    pub async fn upload_changed(self: &Arc<Self>, server_id: Option<&str>) -> Result<Summary> {
        let mut total = Summary::default();
        for t in self.targets(server_id)? {
            let t = Arc::new(t);
            let (files, _) = collect_local(&t, &t.mapper.local_roots());
            let mut changed = Vec::new();
            for f in files {
                if let Some(remote) = t.mapper.to_remote(&f)
                    && !self.up_to_date(&t, &f, &remote).await
                {
                    changed.push(f);
                }
            }
            if !changed.is_empty() {
                total.merge(self.upload_to(t, &changed, false).await?);
            }
        }
        Ok(total)
    }

    /// Uploads the files a git commit added or modified, and propagates deletes/renames when enabled
    /// (upload after commit).
    pub async fn upload_commit(
        self: &Arc<Self>,
        server_id: Option<&str>,
        rev: &str,
    ) -> Result<Summary> {
        let out = tokio::process::Command::new("git")
            .args([
                "diff-tree",
                "--no-commit-id",
                "--name-status",
                "-r",
                "-M",
                rev,
            ])
            .current_dir(&self.root)
            .output()
            .await
            .map_err(|e| RemoteError::new(ErrorCode::Config, format!("Cannot run git: {e}")))?;
        if !out.status.success() {
            return Err(RemoteError::new(
                ErrorCode::Config,
                String::from_utf8_lossy(&out.stderr).trim().to_string(),
            ));
        }
        let top = tokio::process::Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(&self.root)
            .output()
            .await;
        let top = top
            .ok()
            .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
            .unwrap_or_else(|| self.root.clone());
        let mut uploads = Vec::new();
        let mut summary = Summary::default();
        let delete = self.config().options().delete_remote_on_local_delete;
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            match (parts.first().map(|s| &s[..1]), parts.as_slice()) {
                (Some("A" | "M" | "C"), [_, path, ..]) => uploads.push(top.join(path)),
                (Some("R"), [_, from, to]) => {
                    for t in self.targets(server_id)? {
                        if let (Some(rf), Some(rt)) = (
                            t.mapper.to_remote(&top.join(from)),
                            t.mapper.to_remote(&top.join(to)),
                        ) {
                            let r = self.queue_rename(&t, rf.clone(), rt).await;
                            summary
                                .absorb(&rf, r.unwrap_or_else(|_| Err(RemoteError::cancelled())));
                        }
                    }
                    uploads.push(top.join(to));
                }
                (Some("D"), [_, path]) if delete => {
                    for t in self.targets(server_id)? {
                        if let Some(r) = t.mapper.to_remote(&top.join(path)) {
                            let res = self.queue_delete(&t, r.clone(), false).await;
                            summary
                                .absorb(&r, res.unwrap_or_else(|_| Err(RemoteError::cancelled())));
                        }
                    }
                }
                _ => {}
            }
        }
        let uploads: Vec<PathBuf> = uploads.into_iter().filter(|p| p.is_file()).collect();
        if !uploads.is_empty() {
            summary.merge(self.upload(server_id, &uploads, false).await?);
        }
        Ok(summary)
    }

    // ---- misc ------------------------------------------------------------------------------------------------

    pub async fn test_connection(self: &Arc<Self>, server: &Server) -> Vec<TestStep> {
        crate::manager::test_connection(server, &self.manager.ctx).await
    }

    pub async fn detect_root(self: &Arc<Self>, server: &Server) -> Result<String> {
        crate::manager::detect_root(server, &self.manager.ctx).await
    }

    pub fn cancel(self: &Arc<Self>, server_id: Option<&str>) {
        self.queue.cancel(server_id);
    }

    pub async fn disconnect_all(&self) {
        self.manager.disconnect_all().await;
    }

    pub async fn forget_credentials(&self, server_id: &str) -> Result<()> {
        let s = self
            .config()
            .server(server_id)
            .ok_or_else(|| RemoteError::new(ErrorCode::Config, "Unknown server"))?;
        self.manager.forget_credentials(&s.server).await;
        Ok(())
    }

    /// Flushes state; call before exit.
    pub async fn shutdown(&self) {
        let _ = self.state.flush();
        self.manager.disconnect_all().await;
    }
}

fn walk_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.is_file() {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}
