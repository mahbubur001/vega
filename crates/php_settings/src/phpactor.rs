//! Keeps phpactor's index in step with the PHP settings. phpactor indexes the
//! folders in `indexer.stub_paths` (our include paths) only when its index is
//! first built, so after they change VEGA restarts phpactor with the new
//! settings and asks it to reindex (`phpactor/indexer/reindex`).

use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, AsyncApp, Entity};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector};
use project::Project;
use project::lsp_command::file_path_to_lsp_url;
use project::lsp_store::{LspStore, LspStoreEvent, language_server_settings_for};
use proto::update_language_server::Variant;
use serde_json::Value;
use settings::SettingsLocation;
use util::ResultExt as _;
use util::rel_path::RelPath;

use crate::config::{PhpConfig, settings_path, settings_with_stub_keys};
use crate::store::{PhpSettingsStore, cached_config, project_root};
use crate::stubs::downloaded_stubs_dir;

const PHPACTOR: &str = "phpactor";

/// The title of phpactor's work-done progress while it builds its index
/// (`IndexerHandler`), at startup and after a reindex.
const INDEXING_TITLE: &str = "Indexing workspace";

/// How long to wait, at most, for a saved settings file to be reloaded
/// before phpactor restarts anyway.
const SETTINGS_RELOAD_TIMEOUT: Duration = Duration::from_secs(10);
const SETTINGS_RELOAD_POLL: Duration = Duration::from_millis(200);

enum Reindex {}

impl lsp::notification::Notification for Reindex {
    type Params = serde_json::Value;
    const METHOD: &'static str = "phpactor/indexer/reindex";
}

/// What phpactor's index depends on; a change means it must be rebuilt.
pub(crate) fn index_signature(config: &PhpConfig) -> String {
    let mut paths = config.include_paths.clone();
    paths.sort();
    paths.join("\n")
}

fn indexed_key(root: &Path) -> String {
    // v2: v1 markers could be saved without phpactor having reindexed.
    // v3: v2 markers could be saved for an index built before the new
    // include paths were loaded.
    format!("vega_phpactor_indexed_v3:{}", root.display())
}

fn indexed_signature(root: &Path, cx: &App) -> Option<String> {
    KeyValueStore::global(cx)
        .read_kvp(&indexed_key(root))
        .log_err()
        .flatten()
}

fn remember_indexed(root: &Path, signature: String, cx: &App) {
    let kvp = KeyValueStore::global(cx);
    let key = indexed_key(root);
    cx.background_spawn(async move { kvp.write_kvp(key, signature).await.log_err() })
        .detach();
}

/// Watches `project`: reindexes phpactor when it starts with include paths it
/// hasn't indexed, and restarts it when the include paths change.
pub(crate) fn watch(project: Entity<Project>, cx: &mut App) {
    let lsp_store = project.read(cx).lsp_store();
    let watched = project.clone();
    // phpactor's "Indexing workspace" progress, by server.
    let mut indexing: Vec<(LanguageServerId, proto::ProgressToken)> = Vec::new();
    cx.subscribe(&lsp_store, move |lsp_store, event, cx| {
        if let LspStoreEvent::LanguageServerUpdate {
            language_server_id,
            message,
            ..
        } = event
        {
            match message {
                Variant::WorkStart(start)
                    if start.title.as_deref() == Some(INDEXING_TITLE)
                        && is_phpactor(&lsp_store, *language_server_id, cx) =>
                {
                    if let Some(token) = start.token.clone() {
                        indexing.push((*language_server_id, token));
                    }
                }
                Variant::WorkEnd(end) => {
                    let before = indexing.len();
                    indexing.retain(|(id, token)| {
                        !(id == language_server_id && end.token.as_ref() == Some(token))
                    });
                    if indexing.len() != before {
                        recheck_open_files(&lsp_store, *language_server_id, cx);
                    }
                }
                _ => {}
            }
            return;
        }
        if let LspStoreEvent::LanguageServerAdded(id, name, _) = event
            && name.0.as_ref() == PHPACTOR
        {
            let Some(root) = project_root(&watched, cx) else {
                return;
            };
            let Some(config) = cached_config(&watched, cx) else {
                return;
            };
            let signature = index_signature(&config);
            if indexed_signature(&root, cx).unwrap_or_default() == signature {
                return;
            }
            if let Some(server) = lsp_store.read(cx).language_server_for_id(*id) {
                server.notify::<Reindex>(serde_json::json!({})).log_err();
                remember_indexed(&root, signature, cx);
            }
        }
    })
    .detach();

    // The stubs may have been downloaded before this project last opened.
    sync_stub_settings(&project, false, cx);

    let store = PhpSettingsStore::global(cx);
    // Load the settings now, so they are there when phpactor starts.
    crate::store::config_for_project(&project, cx);
    let mut last = cached_config(&project, cx).map(|config| index_signature(&config));
    cx.observe(&store, move |_, cx| {
        let Some(config) = cached_config(&project, cx) else {
            return;
        };
        let signature = index_signature(&config);
        let first_load = last.is_none();
        let changed = last.as_ref().is_some_and(|last| *last != signature);
        last = Some(signature.clone());
        if first_load {
            // phpactor may have started before the settings were loaded.
            let stale = project_root(&project, cx)
                .is_some_and(|root| indexed_signature(&root, cx).unwrap_or_default() != signature);
            if stale {
                reindex(&project, cx);
            }
        } else if changed {
            let project = project.clone();
            let wanted =
                (!config.include_paths.is_empty()).then(|| serde_json::json!(config.include_paths));
            cx.spawn(async move |cx| {
                restart_when_loaded(&project, "indexer.stub_paths", wanted, cx).await;
            })
            .detach();
        }
    })
    .detach();
}

/// Points phpactor at the downloaded stubs in the project's
/// `.vega/settings.json` (or back at its own when there are none) and restarts
/// it when that changed the file. Without `create`, a project that has no
/// settings file is left alone (opening any folder shouldn't add one).
pub(crate) fn sync_stub_settings(project: &Entity<Project>, create: bool, cx: &mut App) {
    let Some(root) = project_root(project, cx) else {
        return;
    };
    let fs = project.read(cx).fs().clone();
    let project = project.clone();
    cx.spawn(async move |cx| {
        let settings = settings_path(&root);
        let current = match fs.load(&settings).await {
            Ok(text) => text,
            Err(_) if create => String::new(),
            Err(_) => return,
        };
        let stub_dir = downloaded_stubs_dir(fs.as_ref()).await;
        let updated = settings_with_stub_keys(&current, stub_dir.as_deref());
        // A missing file stays missing when there is nothing to write.
        if updated == current || (current.is_empty() && stub_dir.is_none()) {
            return;
        }
        if let Some(folder) = settings.parent() {
            fs.create_dir(folder).await.log_err();
        }
        if fs.atomic_write(settings, updated).await.log_err().is_none() {
            return;
        }
        let wanted = stub_dir.map(|dir| serde_json::json!(dir.to_string_lossy()));
        restart_when_loaded(&project, "worse_reflection.stub_dir", wanted, cx).await;
    })
    .detach();
}

/// phpactor's initialization option `key` in the project's settings as they
/// are loaded now (`None` when it isn't set).
fn loaded_init_option(project: &Entity<Project>, key: &str, cx: &App) -> Option<Value> {
    let worktree_id = project.read(cx).visible_worktrees(cx).next()?.read(cx).id();
    let location = SettingsLocation {
        worktree_id,
        path: RelPath::empty(),
    };
    language_server_settings_for(location, &LanguageServerName::new_static(PHPACTOR), cx)?
        .initialization_options
        .as_ref()?
        .get(key)
        .cloned()
}

/// Restarts phpactor once the settings files have been reloaded with `key`
/// set to `wanted`, so it starts with the new settings. A restart that came
/// too early would build the index without the new include paths and then
/// count it as built.
async fn restart_when_loaded(
    project: &Entity<Project>,
    key: &str,
    wanted: Option<Value>,
    cx: &mut AsyncApp,
) {
    let deadline = Instant::now() + SETTINGS_RELOAD_TIMEOUT;
    while cx.update(|cx| loaded_init_option(project, key, cx)) != wanted {
        if Instant::now() >= deadline {
            log::warn!("phpactor settings ({key}) weren't reloaded in time; restarting anyway");
            break;
        }
        cx.background_executor().timer(SETTINGS_RELOAD_POLL).await;
    }
    cx.update(|cx| restart(project, cx));
}

fn is_phpactor(lsp_store: &Entity<LspStore>, id: LanguageServerId, cx: &App) -> bool {
    lsp_store
        .read(cx)
        .language_server_statuses()
        .any(|(server, status)| server == id && status.name.0.as_ref() == PHPACTOR)
}

/// Asks phpactor to check the open PHP files again. Their diagnostics were
/// made while the index was being built ("class not found"…) and otherwise
/// stay until the file changes. A `didSave` makes phpactor re-run its
/// diagnostics (`language_server.diagnostics_on_save`, on by default) without
/// changing the document.
fn recheck_open_files(lsp_store: &Entity<LspStore>, id: LanguageServerId, cx: &mut App) {
    let Some(server) = lsp_store.read(cx).language_server_for_id(id) else {
        return;
    };
    let buffers = lsp_store
        .read(cx)
        .buffer_store()
        .read(cx)
        .buffers()
        .collect::<Vec<_>>();
    for buffer in buffers {
        let uses_server = lsp_store.update(cx, |lsp_store, cx| {
            buffer.update(cx, |buffer, cx| {
                lsp_store
                    .language_servers_for_local_buffer(buffer, cx)
                    .contains(&id)
            })
        });
        if !uses_server {
            continue;
        }
        let Some(path) = buffer
            .read(cx)
            .file()
            .and_then(|file| file.as_local())
            .map(|file| file.abs_path(cx))
        else {
            continue;
        };
        let Some(uri) = file_path_to_lsp_url(&path).log_err() else {
            continue;
        };
        server
            .notify::<lsp::notification::DidSaveTextDocument>(lsp::DidSaveTextDocumentParams {
                text_document: lsp::TextDocumentIdentifier { uri },
                text: None,
            })
            .log_err();
    }
}

/// Restarts the project's phpactor (it reindexes when it comes back).
pub(crate) fn restart(project: &Entity<Project>, cx: &mut App) {
    let buffers = project
        .read(cx)
        .buffer_store()
        .read(cx)
        .buffers()
        .collect::<Vec<_>>();
    let only = HashSet::from_iter([LanguageServerSelector::Name(
        LanguageServerName::new_static(PHPACTOR),
    )]);
    project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
        lsp_store.restart_language_servers_for_buffers(buffers, only, cx)
    });
}

/// `php: reindex`: asks every phpactor of `project` to rebuild its index.
pub(crate) fn reindex(project: &Entity<Project>, cx: &mut App) -> usize {
    let lsp_store = project.read(cx).lsp_store();
    let servers: Vec<_> = lsp_store
        .read(cx)
        .language_server_statuses()
        .filter(|(_, status)| status.name.0.as_ref() == PHPACTOR)
        .filter_map(|(id, _)| lsp_store.read(cx).language_server_for_id(id))
        .collect();
    for server in &servers {
        server.notify::<Reindex>(serde_json::json!({})).log_err();
    }
    // Only a running phpactor can reindex; otherwise the next one to start does.
    if !servers.is_empty()
        && let (Some(root), Some(config)) = (project_root(project, cx), cached_config(project, cx))
    {
        remember_indexed(&root, index_signature(&config), cx);
    }
    servers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_index_depends_on_the_include_paths_only() {
        let config = |paths: &[&str]| PhpConfig {
            include_paths: paths.iter().map(|path| path.to_string()).collect(),
            ..Default::default()
        };
        assert_eq!(
            index_signature(&config(&["/b", "/a"])),
            index_signature(&config(&["/a", "/b"]))
        );
        assert_ne!(
            index_signature(&config(&["/a"])),
            index_signature(&config(&["/a", "/b"]))
        );
        let mut other = config(&["/a"]);
        other.language_level = Some("8.2".into());
        assert_eq!(index_signature(&other), index_signature(&config(&["/a"])));
    }
}
