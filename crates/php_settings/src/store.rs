//! Loaded PHP settings per project folder. Views observe the store and
//! re-read when a project's settings change.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context as _;
use fs::Fs;
use gpui::{App, AppContext as _, Context, Entity, Global, Task};
use project::Project;

use crate::config::{
    PhpConfig, config_path, config_text, parse_config, settings_path, settings_with_lsp_keys,
};

pub struct PhpSettingsStore {
    configs: HashMap<PathBuf, PhpConfig>,
    loading: HashSet<PathBuf>,
}

struct GlobalStore(Entity<PhpSettingsStore>);

impl Global for GlobalStore {}

impl PhpSettingsStore {
    /// The app-wide store.
    pub fn global(cx: &mut App) -> Entity<PhpSettingsStore> {
        if let Some(store) = cx.try_global::<GlobalStore>() {
            return store.0.clone();
        }
        let store = cx.new(|_| PhpSettingsStore {
            configs: HashMap::default(),
            loading: HashSet::default(),
        });
        cx.set_global(GlobalStore(store.clone()));
        store
    }

    /// The settings of the project folder `root` (defaults until loaded; the
    /// store notifies once they are).
    pub fn config(&mut self, root: &Path, fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> PhpConfig {
        if let Some(config) = self.configs.get(root) {
            return config.clone();
        }
        if self.loading.insert(root.to_path_buf()) {
            let root = root.to_path_buf();
            cx.spawn(async move |this, cx| {
                let config = match fs.load(&config_path(&root)).await {
                    Ok(text) => parse_config(&text),
                    Err(_) => PhpConfig::default(),
                };
                this.update(cx, |this, cx| {
                    this.loading.remove(&root);
                    this.configs.insert(root, config);
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
        PhpConfig::default()
    }

    /// Saves `config` for `root` and updates the language-server settings.
    pub fn save(
        &mut self,
        root: PathBuf,
        config: PhpConfig,
        fs: Arc<dyn Fs>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.configs.insert(root.clone(), config.clone());
        cx.notify();
        cx.background_spawn(async move {
            let path = config_path(&root);
            if let Some(folder) = path.parent() {
                fs.create_dir(folder).await?;
            }
            fs.atomic_write(path, config_text(&config))
                .await
                .context("couldn't save .vega/php.json")?;
            let settings = settings_path(&root);
            let current = fs.load(&settings).await.unwrap_or_default();
            let updated = settings_with_lsp_keys(&current, &config);
            if updated != current {
                fs.atomic_write(settings, updated)
                    .await
                    .context("couldn't update .vega/settings.json")?;
            }
            Ok(())
        })
    }
}

/// The project's first folder, where its PHP settings live.
pub fn project_root(project: &Entity<Project>, cx: &App) -> Option<PathBuf> {
    project
        .read(cx)
        .visible_worktrees(cx)
        .next()
        .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
}

/// The PHP settings of `project` (defaults until loaded).
pub fn config_for_project(project: &Entity<Project>, cx: &mut App) -> PhpConfig {
    let Some(root) = project_root(project, cx) else {
        return PhpConfig::default();
    };
    let fs = project.read(cx).fs().clone();
    PhpSettingsStore::global(cx).update(cx, |store, cx| store.config(&root, fs, cx))
}

/// The PHP settings of `project` if they are loaded (no loading, for
/// read-only contexts).
pub fn cached_config(project: &Entity<Project>, cx: &App) -> Option<PhpConfig> {
    let root = project_root(project, cx)?;
    let store = cx.try_global::<GlobalStore>()?.0.clone();
    store.read(cx).configs.get(&root).cloned()
}
