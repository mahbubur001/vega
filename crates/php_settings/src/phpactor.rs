//! Keeps phpactor's index in step with the PHP settings. phpactor indexes the
//! folders in `indexer.stub_paths` (our include paths) only when its index is
//! first built, so after they change VEGA restarts phpactor with the new
//! settings and asks it to reindex (`phpactor/indexer/reindex`).

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Entity};
use lsp::{LanguageServerName, LanguageServerSelector};
use project::Project;
use project::lsp_store::LspStoreEvent;
use util::ResultExt as _;

use crate::config::{PhpConfig, settings_path, settings_with_stub_keys};
use crate::store::{PhpSettingsStore, cached_config, project_root};
use crate::stubs::downloaded_stubs_dir;

const PHPACTOR: &str = "phpactor";

/// How long to wait after saving for the settings file to be reloaded
/// before phpactor restarts with it.
const SETTINGS_RELOAD_DELAY: Duration = Duration::from_millis(1500);

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
    format!("vega_phpactor_indexed_v2:{}", root.display())
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
    cx.subscribe(&lsp_store, move |lsp_store, event, cx| {
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
            cx.spawn(async move |cx| {
                cx.background_executor().timer(SETTINGS_RELOAD_DELAY).await;
                cx.update(|cx| restart(&project, cx));
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
        cx.background_executor().timer(SETTINGS_RELOAD_DELAY).await;
        cx.update(|cx| restart(&project, cx));
    })
    .detach();
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
