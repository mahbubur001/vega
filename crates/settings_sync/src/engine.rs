//! Runs Settings Sync: decides when to sync and applies the results.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context as _, Result};
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use http_client::HttpClient;
use serde_json::json;

use crate::account::{AccountStatus, GitHubAccount};
use crate::gist::{self, Fetched, GistVersion, MARKER_FILE};
use crate::merge::{Prefer, SettingsMerge, merge_settings};
use crate::reconcile::{Outcome, is_untouched, reconcile};
use crate::resources::{SyncState, resources, state_dir, watched_paths};

const LOCAL_CHANGE_DELAY: Duration = Duration::from_secs(3);
const POLL_INTERVAL: Duration = Duration::from_secs(5 * 60);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyncStatus {
    /// Not signed in, or sync is turned off.
    Off,
    Idle,
    Syncing,
    /// These files changed on this machine and on GitHub.
    Conflict(Vec<SharedString>),
    Error(SharedString),
}

/// How to settle a conflict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Resolution {
    UseLocal,
    UseRemote,
}

pub struct SettingsSync {
    account: Entity<GitHubAccount>,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    enabled: bool,
    status: SyncStatus,
    /// When this machine last finished a sync with GitHub.
    last_synced: Option<SystemTime>,
    sync_task: Option<Task<()>>,
    sync_again: bool,
    delayed_sync: Option<Task<()>>,
    _tasks: Vec<Task<()>>,
    _subscription: gpui::Subscription,
}

struct GlobalSettingsSync(Entity<SettingsSync>);

impl Global for GlobalSettingsSync {}

impl SettingsSync {
    pub fn global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalSettingsSync>()
            .map(|sync| sync.0.clone())
    }

    pub(crate) fn init(
        account: Entity<GitHubAccount>,
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut App,
    ) {
        let sync = cx.new(|cx| Self::new(account, fs, http_client, cx));
        cx.set_global(GlobalSettingsSync(sync));
    }

    fn new(
        account: Entity<GitHubAccount>,
        fs: Arc<dyn Fs>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut was_signed_in = false;
        let subscription = cx.observe(&account, move |this, account, cx| {
            let is_signed_in = matches!(account.read(cx).status(), AccountStatus::SignedIn(_));
            if is_signed_in && !was_signed_in {
                this.sync(cx);
            } else if !is_signed_in && was_signed_in {
                this.sync_task = None;
                this.status = SyncStatus::Off;
                cx.notify();
            }
            was_signed_in = is_signed_in;
        });

        let mut tasks = Vec::new();
        tasks.push(cx.spawn({
            let fs = fs.clone();
            async move |this, cx| {
                let enabled = SyncState::load(&fs)
                    .await
                    .map(|state| state.enabled)
                    .unwrap_or(true);
                this.update(cx, |this, cx| {
                    this.enabled = enabled;
                    this.sync(cx);
                })
                .ok();
            }
        }));
        tasks.push(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(POLL_INTERVAL).await;
                if this.update(cx, |this, cx| this.sync(cx)).is_err() {
                    break;
                }
            }
        }));
        for path in watched_paths() {
            let fs = fs.clone();
            tasks.push(cx.spawn(async move |this, cx| {
                let (events, _watcher) = fs.watch(&path, Duration::from_millis(500)).await;
                futures::pin_mut!(events);
                while events.next().await.is_some() {
                    let scheduled = this.update(cx, |this, cx| {
                        this.sync_after(LOCAL_CHANGE_DELAY, cx);
                    });
                    if scheduled.is_err() {
                        break;
                    }
                }
            }));
        }

        Self {
            account,
            fs,
            http_client,
            enabled: false,
            status: SyncStatus::Off,
            last_synced: None,
            sync_task: None,
            sync_again: false,
            delayed_sync: None,
            _tasks: tasks,
            _subscription: subscription,
        }
    }

    pub fn status(&self) -> &SyncStatus {
        &self.status
    }

    pub fn last_synced(&self) -> Option<SystemTime> {
        self.last_synced
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn sync_after(&mut self, delay: Duration, cx: &mut Context<Self>) {
        self.delayed_sync = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update(cx, |this, cx| this.sync(cx)).ok();
        }));
    }

    /// Syncs now, or right after the sync that is already running.
    pub fn sync(&mut self, cx: &mut Context<Self>) {
        self.run(None, cx);
    }

    pub(crate) fn resolve(&mut self, resolution: Resolution, cx: &mut Context<Self>) {
        self.run(Some(resolution), cx);
    }

    pub fn set_enabled(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.enabled = enabled;
        let fs = self.fs.clone();
        cx.spawn(async move |this, cx| {
            let mut state = SyncState::load(&fs).await.unwrap_or_default();
            state.enabled = enabled;
            if let Err(error) = state.save(&fs).await {
                log::error!("saving Settings Sync state: {error:#}");
            }
            this.update(cx, |this, cx| {
                if enabled {
                    this.sync(cx);
                } else {
                    this.sync_task = None;
                    this.status = SyncStatus::Off;
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// The signed-in token and login, if sync can run.
    fn credentials(&self, cx: &App) -> Option<(String, String)> {
        let account = self.account.read(cx);
        Some((account.token()?.to_string(), account.user()?.login.clone()))
    }

    /// The sync gist's saved versions, newest first.
    pub(crate) fn history(&self, cx: &App) -> Task<Result<Vec<GistVersion>>> {
        let Some((token, login)) = self.credentials(cx) else {
            return Task::ready(Err(anyhow::anyhow!(
                "Sign in with GitHub to see the sync history."
            )));
        };
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        cx.background_spawn(async move {
            let state = SyncState::load(&fs).await?.for_account(&login);
            let id = state.gist_id.context("Nothing has been synced yet.")?;
            gist::list_versions(&http_client, &token, &id).await
        })
    }

    /// Makes `version` the current settings on GitHub, then syncs, which
    /// applies it here (backing up this machine's files first).
    pub(crate) fn restore(&mut self, version: String, cx: &mut Context<Self>) {
        let Some((token, login)) = self.credentials(cx) else {
            return;
        };
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        self.status = SyncStatus::Syncing;
        cx.notify();
        cx.spawn(async move |this, cx| {
            let result = restore_version(&fs, &http_client, &token, &login, &version).await;
            this.update(cx, |this, cx| match result {
                Ok(()) => this.run(None, cx),
                Err(error) => {
                    log::error!("restoring synced settings failed: {error:#}");
                    this.status = SyncStatus::Error(error.to_string().into());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    fn run(&mut self, resolution: Option<Resolution>, cx: &mut Context<Self>) {
        if !self.enabled {
            return;
        }
        let account = self.account.read(cx);
        let (Some(token), Some(user)) = (account.token(), account.user()) else {
            return;
        };
        if self.sync_task.is_some() {
            self.sync_again = true;
            return;
        }

        let token = token.to_string();
        let login = user.login.clone();
        let fs = self.fs.clone();
        let http_client = self.http_client.clone();
        self.status = SyncStatus::Syncing;
        cx.notify();

        self.sync_task = Some(cx.spawn(async move |this, cx| {
            let result = sync_once(&fs, &http_client, &token, &login, resolution).await;
            this.update(cx, |this, cx| {
                this.sync_task = None;
                if result.is_ok() {
                    this.last_synced = Some(SystemTime::now());
                }
                this.status = match result {
                    Ok(conflicts) if conflicts.is_empty() => SyncStatus::Idle,
                    Ok(conflicts) => SyncStatus::Conflict(conflicts),
                    Err(error) => {
                        log::error!("Settings Sync failed: {error:#}");
                        SyncStatus::Error(error.to_string().into())
                    }
                };
                cx.notify();
                if std::mem::take(&mut this.sync_again) {
                    this.sync(cx);
                }
            })
            .ok();
        }));
    }
}

/// Runs one sync and returns the labels of files left in conflict.
async fn sync_once(
    fs: &Arc<dyn Fs>,
    http_client: &Arc<dyn HttpClient>,
    token: &str,
    login: &str,
    resolution: Option<Resolution>,
) -> Result<Vec<SharedString>> {
    let mut state = SyncState::load(fs).await?.for_account(login);

    let mut remote = None;
    if let Some(id) = state.gist_id.clone() {
        match gist::fetch_gist(http_client, token, &id, state.etag.as_deref()).await? {
            Fetched::NotModified => remote = Some(state.remote.clone()),
            Fetched::Found(found) => {
                state.etag = found.etag;
                state.version = found.version;
                state.remote = found.files.clone();
                remote = Some(found.files);
            }
            Fetched::Missing => {
                log::info!("the Settings Sync gist was deleted; looking for another one");
                state = SyncState::default().for_account(login);
            }
        }
    }

    if remote.is_none() {
        if let Some(id) = gist::find_sync_gist(http_client, token).await? {
            if let Fetched::Found(found) = gist::fetch_gist(http_client, token, &id, None).await? {
                state.gist_id = Some(found.id);
                state.etag = found.etag;
                state.version = found.version;
                state.remote = found.files.clone();
                remote = Some(found.files);
            }
        }
    }

    let Some(remote) = remote else {
        // First machine: create the gist from this machine's files.
        let mut files = BTreeMap::new();
        for resource in resources(fs, &BTreeMap::new()).await? {
            let local = load_local(fs, &resource.local_path, resource.empty_contents).await?;
            let upload = if resource.merge_keys {
                // Never-sync values are removed before anything is uploaded.
                match merge_settings(&local, None, None, None)? {
                    SettingsMerge::Merged { upload, .. } => upload,
                    SettingsMerge::Conflict(_) => None,
                }
            } else {
                (!is_untouched(&local)).then_some(local)
            };
            if let Some(upload) = upload {
                state
                    .bases
                    .insert(resource.gist_file.clone(), upload.clone());
                files.insert(resource.gist_file, upload);
            }
        }
        files.insert(MARKER_FILE.to_string(), manifest());
        let created = gist::create_gist(http_client, token, &files).await?;
        state.gist_id = Some(created.id);
        state.remote = created.files;
        state.version = created.version;
        state.etag = None;
        state.save(fs).await?;
        return Ok(Vec::new());
    };

    let bases_before = state.bases.clone();
    let mut changes = BTreeMap::new();
    let mut conflicts = Vec::new();
    let mut backup_dir = None;
    for resource in resources(fs, &remote).await? {
        let local = load_local(fs, &resource.local_path, resource.empty_contents).await?;
        let remote_file = remote.get(&resource.gist_file).map(String::as_str);
        let base = state.bases.get(&resource.gist_file).map(String::as_str);

        if resource.merge_keys {
            let prefer = resolution.map(|resolution| match resolution {
                Resolution::UseLocal => Prefer::Local,
                Resolution::UseRemote => Prefer::Remote,
            });
            match merge_settings(&local, remote_file, base, prefer)? {
                SettingsMerge::Conflict(keys) => {
                    conflicts.push(format!("{} ({})", resource.label, keys.join(", ")).into());
                }
                SettingsMerge::Merged {
                    local: new_local,
                    upload,
                    base,
                } => {
                    if let Some(new_local) = new_local {
                        if !is_untouched(&local) {
                            let backup_dir = backup_dir.get_or_insert_with(new_backup_dir);
                            fs.create_dir(backup_dir).await?;
                            fs.atomic_write(backup_dir.join(&resource.gist_file), local)
                                .await
                                .context("backing up before applying synced changes")?;
                        }
                        fs.atomic_write(resource.local_path.clone(), new_local)
                            .await
                            .with_context(|| {
                                format!("writing {}", resource.local_path.display())
                            })?;
                    }
                    if let Some(upload) = upload {
                        changes.insert(resource.gist_file.clone(), Some(upload));
                    }
                    state.bases.insert(resource.gist_file, base);
                }
            }
            continue;
        }

        let outcome = match (reconcile(&local, remote_file, base), resolution) {
            (Outcome::Conflict, Some(Resolution::UseLocal)) => Outcome::Push,
            (Outcome::Conflict, Some(Resolution::UseRemote)) => Outcome::Pull,
            (outcome, _) => outcome,
        };

        match (outcome, remote_file) {
            (Outcome::Unchanged, _) => {}
            (Outcome::Adopt, _) => {
                state.bases.insert(resource.gist_file, local);
            }
            (Outcome::Push, _) if local.trim().is_empty() => {
                // Deleted on this machine: delete it on GitHub too (gists reject blank files).
                changes.insert(resource.gist_file.clone(), None);
                state.bases.remove(&resource.gist_file);
            }
            (Outcome::Push, _) => {
                changes.insert(resource.gist_file.clone(), Some(local.clone()));
                state.bases.insert(resource.gist_file, local);
            }
            (Outcome::DeleteLocal, _) => {
                let backup_dir = backup_dir.get_or_insert_with(new_backup_dir);
                fs.create_dir(backup_dir).await?;
                fs.atomic_write(backup_dir.join(&resource.gist_file), local)
                    .await
                    .context("backing up before applying synced changes")?;
                fs.remove_file(&resource.local_path, Default::default())
                    .await
                    .with_context(|| format!("deleting {}", resource.local_path.display()))?;
                state.bases.remove(&resource.gist_file);
            }
            (Outcome::Pull, Some(remote_file)) => {
                if !is_untouched(&local) {
                    let backup_dir = backup_dir.get_or_insert_with(new_backup_dir);
                    fs.create_dir(backup_dir).await?;
                    fs.atomic_write(backup_dir.join(&resource.gist_file), local)
                        .await
                        .context("backing up before applying synced changes")?;
                }
                if let Some(parent) = resource.local_path.parent() {
                    fs.create_dir(parent).await?;
                }
                fs.atomic_write(resource.local_path.clone(), remote_file.to_string())
                    .await
                    .with_context(|| format!("writing {}", resource.local_path.display()))?;
                state
                    .bases
                    .insert(resource.gist_file, remote_file.to_string());
            }
            (Outcome::Pull, None) => {}
            (Outcome::Conflict, _) => conflicts.push(resource.label.clone()),
        }
    }

    if !changes.is_empty() {
        let gist_id = state.gist_id.clone().context("no sync gist to write to")?;
        changes.insert(MARKER_FILE.to_string(), Some(manifest()));
        let merged_against = state.version.clone();
        let mut updated = gist::update_gist(http_client, token, &gist_id, &changes).await?;

        // Gists can't refuse a write that's based on an old version, so check
        // afterwards whether another machine wrote between our read and write.
        if let (Some(expected), Some(previous)) =
            (merged_against.as_ref(), updated.previous_version.clone())
            && *expected != previous
        {
            log::info!("another machine wrote to the sync gist while this one synced; merging");
            let other = gist::fetch_gist_version(http_client, token, &gist_id, &previous).await?;
            let fixes = merge_race(
                fs,
                &changes,
                &other.files,
                &bases_before,
                &mut state,
                &mut conflicts,
                &mut backup_dir,
            )
            .await?;
            if !fixes.is_empty() {
                updated = gist::update_gist(http_client, token, &gist_id, &fixes).await?;
            }
        }

        state.remote = updated.files;
        state.version = updated.version;
        state.etag = None;
    }

    state.save(fs).await?;
    Ok(conflicts)
}

/// Writes the gist's files from `version` back as its newest version. Files
/// added since are removed; the marker file is kept.
async fn restore_version(
    fs: &Arc<dyn Fs>,
    http_client: &Arc<dyn HttpClient>,
    token: &str,
    login: &str,
    version: &str,
) -> Result<()> {
    let mut state = SyncState::load(fs).await?.for_account(login);
    let id = state
        .gist_id
        .clone()
        .context("Nothing has been synced yet.")?;
    let old = gist::fetch_gist_version(http_client, token, &id, version).await?;
    let Fetched::Found(current) = gist::fetch_gist(http_client, token, &id, None).await? else {
        anyhow::bail!("The sync gist is gone from GitHub.");
    };

    let mut changes = BTreeMap::new();
    for name in current.files.keys() {
        if name != MARKER_FILE && !old.files.contains_key(name) {
            changes.insert(name.clone(), None);
        }
    }
    for (name, content) in &old.files {
        if name != MARKER_FILE && current.files.get(name) != Some(content) {
            changes.insert(name.clone(), Some(content.clone()));
        }
    }
    if changes.is_empty() {
        return Ok(());
    }

    let updated = gist::update_gist(http_client, token, &id, &changes).await?;
    // The next sync sees GitHub changed while this machine didn't, so it pulls.
    state.remote = updated.files;
    state.version = updated.version;
    state.etag = None;
    state.save(fs).await
}

/// Another machine wrote `other` between this machine's read and its write of
/// `written`, so `written` replaced it. Merges each file both wrote against the
/// base this machine merged with, and returns the gist edits that make the gist
/// hold both machines' changes. Files both changed incompatibly get the other
/// machine's copy back and are reported as conflicts, so nothing is lost.
async fn merge_race(
    fs: &Arc<dyn Fs>,
    written: &BTreeMap<String, Option<String>>,
    other: &BTreeMap<String, String>,
    bases: &BTreeMap<String, String>,
    state: &mut SyncState,
    conflicts: &mut Vec<SharedString>,
    backup_dir: &mut Option<std::path::PathBuf>,
) -> Result<BTreeMap<String, Option<String>>> {
    let mut fixes = BTreeMap::new();
    for resource in resources(fs, other).await? {
        let Some(Some(ours)) = written.get(&resource.gist_file) else {
            continue;
        };
        let theirs = other.get(&resource.gist_file);
        let base = bases.get(&resource.gist_file);
        if theirs == base || theirs == Some(ours) {
            // The other machine didn't touch this file.
            continue;
        }
        let restore_theirs = |fixes: &mut BTreeMap<String, Option<String>>| {
            fixes.insert(resource.gist_file.clone(), theirs.cloned());
        };

        if resource.merge_keys {
            let local = load_local(fs, &resource.local_path, resource.empty_contents).await?;
            match merge_settings(
                &local,
                theirs.map(String::as_str),
                base.map(String::as_str),
                None,
            )? {
                SettingsMerge::Merged {
                    local: new_local,
                    upload,
                    base,
                } => {
                    if let Some(new_local) = new_local {
                        let backup_dir = backup_dir.get_or_insert_with(new_backup_dir);
                        fs.create_dir(backup_dir).await?;
                        fs.atomic_write(backup_dir.join(&resource.gist_file), local)
                            .await
                            .context("backing up before applying synced changes")?;
                        fs.atomic_write(resource.local_path.clone(), new_local)
                            .await
                            .with_context(|| {
                                format!("writing {}", resource.local_path.display())
                            })?;
                    }
                    // `upload` is relative to `theirs`; without it the gist must go back to it.
                    match upload {
                        Some(upload) => {
                            fixes.insert(resource.gist_file.clone(), Some(upload));
                        }
                        None => restore_theirs(&mut fixes),
                    }
                    state.bases.insert(resource.gist_file.clone(), base);
                }
                SettingsMerge::Conflict(keys) => {
                    restore_theirs(&mut fixes);
                    if let Some(base) = base {
                        state.bases.insert(resource.gist_file.clone(), base.clone());
                    }
                    conflicts.push(format!("{} ({})", resource.label, keys.join(", ")).into());
                }
            }
        } else {
            // Whole files can't be merged: give the other machine's copy back and ask.
            restore_theirs(&mut fixes);
            if let Some(base) = base {
                state.bases.insert(resource.gist_file.clone(), base.clone());
            }
            conflicts.push(resource.label.clone());
        }
    }
    Ok(fixes)
}

/// Reads a synced file, treating a missing or blank file as empty.
async fn load_local(
    fs: &Arc<dyn Fs>,
    path: &std::path::Path,
    empty_contents: &str,
) -> Result<String> {
    let contents = if fs.is_file(path).await {
        fs.load(path).await?
    } else {
        String::new()
    };
    Ok(if contents.trim().is_empty() {
        empty_contents.to_string()
    } else {
        contents
    })
}

fn new_backup_dir() -> std::path::PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    state_dir().join("backups").join(timestamp.to_string())
}

fn manifest() -> String {
    let updated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    let manifest = json!({
        "app": "vega-settings-sync",
        "schema": 1,
        "updated_at": updated_at,
        "updated_on": std::env::consts::OS,
    });
    format!("{manifest:#}\n")
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use fs::FakeFs;
    use futures::AsyncReadExt as _;
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, Response};

    use super::*;

    /// An in-memory stand-in for the GitHub Gist API.
    #[derive(Default)]
    struct FakeGitHub {
        gists: BTreeMap<String, BTreeMap<String, String>>,
        /// Each gist's versions, oldest first.
        history: BTreeMap<String, Vec<(String, BTreeMap<String, String>)>>,
        next_version: usize,
        /// Files another machine writes just before the next PATCH lands.
        concurrent_write: Option<BTreeMap<String, String>>,
    }

    impl FakeGitHub {
        fn commit(&mut self, id: &str) {
            self.next_version += 1;
            let version = format!("v{}", self.next_version);
            let files = self.gists[id].clone();
            self.history
                .entry(id.to_string())
                .or_default()
                .push((version, files));
        }

        fn gist_json(&self, id: &str, files: &BTreeMap<String, String>) -> serde_json::Value {
            let files: serde_json::Map<_, _> = files
                .iter()
                .map(|(name, content)| (name.clone(), json!({ "content": content })))
                .collect();
            let history: Vec<_> = self
                .history
                .get(id)
                .into_iter()
                .flatten()
                .rev()
                .map(|(version, _)| json!({ "version": version }))
                .collect();
            json!({
                "id": id,
                "public": false,
                "updated_at": "2026-11-18T00:00:00Z",
                "files": files,
                "history": history,
            })
        }
    }

    fn fake_github() -> (Arc<Mutex<FakeGitHub>>, Arc<dyn HttpClient>) {
        let github = Arc::new(Mutex::new(FakeGitHub::default()));
        let http_client = FakeHttpClient::create({
            let github = github.clone();
            move |request| {
                let github = github.clone();
                async move {
                    let method = request.method().clone();
                    let path = request.uri().path().to_string();
                    let mut body = Vec::new();
                    request.into_body().read_to_end(&mut body).await?;
                    let body: serde_json::Value = serde_json::from_slice(&body).unwrap_or_default();
                    let mut github = github.lock().unwrap();

                    let (status, response) = match (method.as_str(), path.as_str()) {
                        ("GET", "/gists") => {
                            let list: Vec<_> = github
                                .gists
                                .iter()
                                .map(|(id, files)| github.gist_json(id, files))
                                .collect();
                            (200, json!(list))
                        }
                        ("POST", "/gists") => {
                            assert_eq!(body["public"], json!(false));
                            let id = format!("gist{}", github.gists.len() + 1);
                            let files = body["files"]
                                .as_object()
                                .unwrap()
                                .iter()
                                .map(|(name, file)| {
                                    (name.clone(), file["content"].as_str().unwrap().to_string())
                                })
                                .collect();
                            github.gists.insert(id.clone(), files);
                            github.commit(&id);
                            (201, github.gist_json(&id, &github.gists[&id]))
                        }
                        (method, path) => {
                            let mut parts = path.trim_start_matches("/gists/").split('/');
                            let id = parts.next().unwrap_or_default().to_string();
                            let version = parts.next().map(str::to_string);
                            match (method, version, github.gists.contains_key(&id)) {
                                ("GET", Some(version), true) if version == "commits" => {
                                    let list: Vec<_> = github.history[&id]
                                        .iter()
                                        .rev()
                                        .map(|(version, _)| {
                                            json!({
                                                "version": version,
                                                "committed_at": "2026-11-19T08:03:12Z",
                                            })
                                        })
                                        .collect();
                                    (200, json!(list))
                                }
                                ("GET", Some(version), true) => {
                                    let files = github.history[&id]
                                        .iter()
                                        .find(|(v, _)| *v == version)
                                        .map(|(_, files)| files.clone())
                                        .unwrap();
                                    (200, github.gist_json(&id, &files))
                                }
                                ("GET", None, true) => {
                                    (200, github.gist_json(&id, &github.gists[&id]))
                                }
                                ("PATCH", None, true) => {
                                    if let Some(write) = github.concurrent_write.take() {
                                        github.gists.get_mut(&id).unwrap().extend(write);
                                        github.commit(&id);
                                    }
                                    let files = github.gists.get_mut(&id).unwrap();
                                    for (name, file) in body["files"].as_object().unwrap() {
                                        match file["content"].as_str() {
                                            Some(content) => {
                                                files.insert(name.clone(), content.to_string());
                                            }
                                            None => {
                                                files.remove(name);
                                            }
                                        }
                                    }
                                    github.commit(&id);
                                    (200, github.gist_json(&id, &github.gists[&id]))
                                }
                                _ => (404, json!({ "message": "Not Found" })),
                            }
                        }
                    };
                    Ok(Response::builder()
                        .status(status)
                        .body(serde_json::to_vec(&response)?.into())?)
                }
            }
        });
        (github, http_client)
    }

    async fn machine(cx: &TestAppContext, settings: &str) -> Arc<dyn Fs> {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(paths::config_dir()).await.unwrap();
        fs.insert_file(paths::settings_file(), settings.as_bytes().to_vec())
            .await;
        fs
    }

    /// This machine's settings, parsed, so formatting doesn't matter.
    async fn settings(fs: &Arc<dyn Fs>) -> serde_json::Value {
        settings_json::parse_json_with_comments(&fs.load(paths::settings_file()).await.unwrap())
            .unwrap()
    }

    fn parsed(text: &str) -> serde_json::Value {
        settings_json::parse_json_with_comments(text).unwrap()
    }

    #[gpui::test]
    async fn test_sync_between_two_machines(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{ \"vim_mode\": true }").await;
        let machine_b = machine(cx, "// Settings\n{\n}\n").await;

        // The first machine creates the gist from its settings.
        let conflicts = sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert!(conflicts.is_empty());
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert_eq!(parsed(&gist["settings.json"]), json!({ "vim_mode": true }));
        assert!(gist.contains_key(MARKER_FILE));

        // A fresh machine finds the gist and takes its settings.
        sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(settings(&machine_b).await, json!({ "vim_mode": true }));

        // A change on one machine reaches the other.
        machine_b
            .atomic_write(
                paths::settings_file().clone(),
                "{ \"vim_mode\": false }".into(),
            )
            .await
            .unwrap();
        sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(settings(&machine_a).await, json!({ "vim_mode": false }));

        // Changes on both machines are a conflict, and nothing is overwritten.
        machine_a
            .atomic_write(
                paths::settings_file().clone(),
                "{ \"theme\": \"A\" }".into(),
            )
            .await
            .unwrap();
        machine_b
            .atomic_write(
                paths::settings_file().clone(),
                "{ \"theme\": \"B\" }".into(),
            )
            .await
            .unwrap();
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        let conflicts = sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(conflicts, vec![SharedString::from("Settings (theme)")]);
        assert_eq!(settings(&machine_b).await, json!({ "theme": "B" }));

        // Taking GitHub's copy backs up this machine's settings first.
        let conflicts = sync_once(
            &machine_b,
            &http_client,
            "token",
            "octocat",
            Some(Resolution::UseRemote),
        )
        .await
        .unwrap();
        assert!(conflicts.is_empty());
        assert_eq!(settings(&machine_b).await, json!({ "theme": "A" }));
        let backups = machine_b
            .read_dir(&state_dir().join("backups"))
            .await
            .unwrap()
            .count()
            .await;
        assert_eq!(backups, 1);
    }

    #[gpui::test]
    async fn test_edits_to_different_settings_merge_and_secrets_stay_local(
        cx: &mut TestAppContext,
    ) {
        let (github, http_client) = fake_github();
        let machine_a = machine(
            cx,
            "// A\n{\n  \"tab_size\": 4,\n  \"vim_mode\": false,\n  \"proxy\": \"http://a\"\n}\n",
        )
        .await;
        let machine_b = machine(cx, "{}\n").await;
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert!(
            !gist["settings.json"].contains("proxy"),
            "never-sync values stay local"
        );

        // Each machine changes a different setting.
        machine_a
            .atomic_write(
                paths::settings_file().clone(),
                "// A\n{\n  \"tab_size\": 2,\n  \"vim_mode\": false,\n  \"proxy\": \"http://a\"\n}\n"
                    .into(),
            )
            .await
            .unwrap();
        machine_b
            .atomic_write(
                paths::settings_file().clone(),
                "{ \"tab_size\": 4, \"vim_mode\": true }".into(),
            )
            .await
            .unwrap();
        for machine in [&machine_a, &machine_b, &machine_a] {
            let conflicts = sync_once(machine, &http_client, "token", "octocat", None)
                .await
                .unwrap();
            assert!(conflicts.is_empty(), "{conflicts:?}");
        }

        assert_eq!(
            settings(&machine_a).await,
            json!({ "tab_size": 2, "vim_mode": true, "proxy": "http://a" })
        );
        assert_eq!(
            settings(&machine_b).await,
            json!({ "tab_size": 2, "vim_mode": true })
        );
        let text = machine_a.load(paths::settings_file()).await.unwrap();
        assert!(text.starts_with("// A\n"), "comments survive: {text}");
    }

    #[gpui::test]
    async fn test_concurrent_writes_from_two_machines_both_survive(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{ \"tab_size\": 4, \"vim_mode\": false }").await;
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();

        // Machine A changes tab_size. Between A's read and A's write, another
        // machine writes vim_mode, which A's write would otherwise replace.
        machine_a
            .atomic_write(
                paths::settings_file().clone(),
                "{ \"tab_size\": 2, \"vim_mode\": false }".into(),
            )
            .await
            .unwrap();
        github.lock().unwrap().concurrent_write = Some(BTreeMap::from([(
            "settings.json".to_string(),
            "{ \"tab_size\": 4, \"vim_mode\": true }".to_string(),
        )]));
        let conflicts = sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();

        assert!(conflicts.is_empty(), "{conflicts:?}");
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert_eq!(
            parsed(&gist["settings.json"]),
            json!({ "tab_size": 2, "vim_mode": true }),
            "the gist keeps both machines' changes"
        );
        assert_eq!(
            settings(&machine_a).await,
            json!({ "tab_size": 2, "vim_mode": true })
        );
    }

    #[gpui::test]
    async fn test_conflicting_concurrent_write_gives_the_other_machines_copy_back(
        cx: &mut TestAppContext,
    ) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{ \"tab_size\": 4 }").await;
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();

        machine_a
            .atomic_write(paths::settings_file().clone(), "{ \"tab_size\": 2 }".into())
            .await
            .unwrap();
        github.lock().unwrap().concurrent_write = Some(BTreeMap::from([(
            "settings.json".to_string(),
            "{ \"tab_size\": 8 }".to_string(),
        )]));
        let conflicts = sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();

        assert_eq!(conflicts, vec![SharedString::from("Settings (tab_size)")]);
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert_eq!(parsed(&gist["settings.json"]), json!({ "tab_size": 8 }));
        assert_eq!(settings(&machine_a).await, json!({ "tab_size": 2 }));

        // The next sync asks again, and nothing is lost on either side.
        let conflicts = sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(conflicts, vec![SharedString::from("Settings (tab_size)")]);
    }

    #[gpui::test]
    async fn test_snippets_and_agents_md_sync_and_deletions_follow(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{}\n").await;
        let snippet = paths::snippets_dir().join("rust.json");
        machine_a.create_dir(paths::snippets_dir()).await.unwrap();
        machine_a
            .atomic_write(
                snippet.clone(),
                "{ \"log\": { \"body\": \"dbg!($1)\" } }".into(),
            )
            .await
            .unwrap();
        machine_a
            .atomic_write(paths::agents_file().clone(), "Be brief.\n".into())
            .await
            .unwrap();
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        {
            let gist = &github.lock().unwrap().gists["gist1"];
            assert!(gist.contains_key("snippets__rust.json"));
            assert_eq!(gist["AGENTS.md"], "Be brief.\n");
        }

        // A fresh machine gets both.
        let machine_b = machine(cx, "{}\n").await;
        sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert!(machine_b.load(&snippet).await.unwrap().contains("dbg!"));
        assert_eq!(
            machine_b.load(paths::agents_file()).await.unwrap(),
            "Be brief.\n"
        );

        // Deleting the snippet on A deletes it on GitHub, then on B.
        machine_a
            .remove_file(&snippet, Default::default())
            .await
            .unwrap();
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert!(!github.lock().unwrap().gists["gist1"].contains_key("snippets__rust.json"));
        let conflicts = sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert!(conflicts.is_empty(), "{conflicts:?}");
        assert!(!machine_b.is_file(&snippet).await);
    }

    #[gpui::test]
    async fn test_gist_file_names_cannot_escape_the_synced_folder(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{ \"vim_mode\": true }").await;
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        github
            .lock()
            .unwrap()
            .gists
            .get_mut("gist1")
            .unwrap()
            .insert(
                "snippets__../../settings.json".into(),
                "{ \"hacked\": true }".into(),
            );
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(settings(&machine_a).await, json!({ "vim_mode": true }));
    }

    #[gpui::test]
    async fn test_restoring_an_earlier_version(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let machine_a = machine(cx, "{ \"tab_size\": 4 }").await;
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        let first_version = github.lock().unwrap().history["gist1"][0].0.clone();

        machine_a
            .atomic_write(paths::settings_file().clone(), "{ \"tab_size\": 2 }".into())
            .await
            .unwrap();
        machine_a
            .atomic_write(paths::tasks_file().clone(), "[{ \"label\": \"x\" }]".into())
            .await
            .unwrap();
        sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert!(github.lock().unwrap().gists["gist1"].contains_key("tasks.json"));

        let versions = gist::list_versions(&http_client, "token", "gist1")
            .await
            .unwrap();
        assert_eq!(versions.len(), 2, "newest first");
        assert_eq!(versions[1].version, first_version);

        restore_version(&machine_a, &http_client, "token", "octocat", &first_version)
            .await
            .unwrap();
        let conflicts = sync_once(&machine_a, &http_client, "token", "octocat", None)
            .await
            .unwrap();

        assert!(conflicts.is_empty(), "{conflicts:?}");
        assert_eq!(settings(&machine_a).await, json!({ "tab_size": 4 }));
        assert!(
            !machine_a.is_file(paths::tasks_file()).await,
            "added later, so removed"
        );
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert!(!gist.contains_key("tasks.json"));
        assert!(gist.contains_key(MARKER_FILE));
    }

    #[gpui::test]
    async fn test_untouched_machine_does_not_create_empty_files(cx: &mut TestAppContext) {
        let (github, http_client) = fake_github();
        let fresh = machine(cx, "{}\n").await;
        sync_once(&fresh, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        let gist = github.lock().unwrap().gists["gist1"].clone();
        assert_eq!(gist.keys().collect::<Vec<_>>(), vec![MARKER_FILE]);
    }
}
