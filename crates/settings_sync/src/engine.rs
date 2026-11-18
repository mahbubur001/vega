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
use crate::gist::{self, Fetched, MARKER_FILE};
use crate::reconcile::{Outcome, is_untouched, reconcile};
use crate::resources::{SyncState, resources, state_dir};

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
        for resource in resources() {
            let fs = fs.clone();
            tasks.push(cx.spawn(async move |this, cx| {
                let (events, _watcher) = fs
                    .watch(&resource.local_path, Duration::from_millis(500))
                    .await;
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
                state.remote = found.files.clone();
                remote = Some(found.files);
            }
        }
    }

    let Some(remote) = remote else {
        // First machine: create the gist from this machine's files.
        let mut files = BTreeMap::new();
        for resource in resources() {
            let local = load_local(fs, &resource.local_path, resource.empty_contents).await?;
            if !is_untouched(&local) {
                state
                    .bases
                    .insert(resource.gist_file.clone(), local.clone());
                files.insert(resource.gist_file, local);
            }
        }
        files.insert(MARKER_FILE.to_string(), manifest());
        let created = gist::create_gist(http_client, token, &files).await?;
        state.gist_id = Some(created.id);
        state.remote = created.files;
        state.etag = None;
        state.save(fs).await?;
        return Ok(Vec::new());
    };

    let mut changes = BTreeMap::new();
    let mut conflicts = Vec::new();
    let mut backup_dir = None;
    for resource in resources() {
        let local = load_local(fs, &resource.local_path, resource.empty_contents).await?;
        let remote_file = remote.get(&resource.gist_file).map(String::as_str);
        let base = state.bases.get(&resource.gist_file).map(String::as_str);

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
            (Outcome::Push, _) => {
                changes.insert(resource.gist_file.clone(), Some(local.clone()));
                state.bases.insert(resource.gist_file, local);
            }
            (Outcome::Pull, Some(remote_file)) => {
                if !is_untouched(&local) {
                    let backup_dir = backup_dir.get_or_insert_with(new_backup_dir);
                    fs.create_dir(backup_dir).await?;
                    fs.atomic_write(backup_dir.join(&resource.gist_file), local)
                        .await
                        .context("backing up before applying synced changes")?;
                }
                fs.atomic_write(resource.local_path.clone(), remote_file.to_string())
                    .await
                    .with_context(|| format!("writing {}", resource.local_path.display()))?;
                state
                    .bases
                    .insert(resource.gist_file, remote_file.to_string());
            }
            (Outcome::Pull, None) => {}
            (Outcome::Conflict, _) => conflicts.push(resource.label.into()),
        }
    }

    if !changes.is_empty() {
        let gist_id = state.gist_id.clone().context("no sync gist to write to")?;
        changes.insert(MARKER_FILE.to_string(), Some(manifest()));
        let updated = gist::update_gist(http_client, token, &gist_id, &changes).await?;
        state.remote = updated.files;
        state.etag = None;
    }

    state.save(fs).await?;
    Ok(conflicts)
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

                    let gist_json = |id: &str, files: &BTreeMap<String, String>| {
                        let files: serde_json::Map<_, _> = files
                            .iter()
                            .map(|(name, content)| (name.clone(), json!({ "content": content })))
                            .collect();
                        json!({ "id": id, "public": false, "updated_at": "2026-11-18T00:00:00Z", "files": files })
                    };

                    let (status, response) = match (method.as_str(), path.as_str()) {
                        ("GET", "/gists") => {
                            let list: Vec<_> = github
                                .gists
                                .iter()
                                .map(|(id, files)| gist_json(id, files))
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
                            (201, gist_json(&id, &github.gists[&id]))
                        }
                        (method, path) => {
                            let id = path.trim_start_matches("/gists/").to_string();
                            match (method, github.gists.get_mut(&id)) {
                                ("GET", Some(files)) => (200, gist_json(&id, files)),
                                ("PATCH", Some(files)) => {
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
                                    (200, gist_json(&id, files))
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

    async fn settings(fs: &Arc<dyn Fs>) -> String {
        fs.load(paths::settings_file()).await.unwrap()
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
        assert_eq!(gist["settings.json"], "{ \"vim_mode\": true }");
        assert!(gist.contains_key(MARKER_FILE));

        // A fresh machine finds the gist and takes its settings.
        sync_once(&machine_b, &http_client, "token", "octocat", None)
            .await
            .unwrap();
        assert_eq!(settings(&machine_b).await, "{ \"vim_mode\": true }");

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
        assert_eq!(settings(&machine_a).await, "{ \"vim_mode\": false }");

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
        assert_eq!(conflicts, vec![SharedString::from("Settings")]);
        assert_eq!(settings(&machine_b).await, "{ \"theme\": \"B\" }");

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
        assert_eq!(settings(&machine_b).await, "{ \"theme\": \"A\" }");
        let backups = machine_b
            .read_dir(&state_dir().join("backups"))
            .await
            .unwrap()
            .count()
            .await;
        assert_eq!(backups, 1);
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
