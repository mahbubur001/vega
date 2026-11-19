//! One deployment engine per local worktree that has `.vega/deployment.json`.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use collections::HashMap;
use deployment::engine::{Engine, FsEvent};
use deployment::project::{CONFIG_PATH, locate_config};
use deployment::prompt::Prompter;
use deployment::queue::TaskStatus;
use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{App, AppContext as _, Context, Entity, SharedString, Task, WeakEntity};
use gpui_tokio::Tokio;
use language::BufferEvent;
use project::buffer_store::BufferStoreEvent;
use project::{PathChange, Project, WorktreeId};
use zed_actions::deployment::ConfiguredRoots;

use crate::prompter::WindowPrompter;

/// What the status bar shows about transfers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TransfersStatus {
    pub active: usize,
    pub current: Option<SharedString>,
}

pub struct DeploymentStore {
    project: WeakEntity<Project>,
    engines: HashMap<WorktreeId, Arc<Engine>>,
    prompter: Arc<WindowPrompter>,
    transfers: TransfersStatus,
    _tasks: Vec<Task<()>>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl DeploymentStore {
    /// The engine asks the user through `prompter`; the caller shows its
    /// requests in the workspace window.
    pub(crate) fn new(
        project: Entity<Project>,
        prompter: Arc<WindowPrompter>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = vec![cx.subscribe(&project, Self::on_project_event)];
        let buffer_store = project.read(cx).buffer_store().clone();
        subscriptions.push(cx.subscribe(&buffer_store, |this, _, event, cx| {
            if let BufferStoreEvent::BufferAdded(buffer) = event {
                this.watch_buffer(buffer, cx);
            }
        }));

        let mut this = Self {
            project: project.downgrade(),
            engines: HashMap::default(),
            prompter,
            transfers: TransfersStatus::default(),
            _tasks: Vec::new(),
            _subscriptions: subscriptions,
        };

        for buffer in buffer_store.read(cx).buffers().collect::<Vec<_>>() {
            this.watch_buffer(&buffer, cx);
        }
        let worktrees: Vec<_> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).id())
            .collect();
        for worktree_id in worktrees {
            this.load_engine(worktree_id, cx);
        }
        this
    }

    pub fn transfers(&self) -> &TransfersStatus {
        &self.transfers
    }

    /// The engine whose project root contains `path`.
    pub fn engine_for_path(&self, path: &Path) -> Option<Arc<Engine>> {
        self.engines
            .values()
            .filter(|engine| path.starts_with(engine.root()))
            .max_by_key(|engine| engine.root().as_os_str().len())
            .cloned()
    }

    /// The engine that opened `path` as a copy of a file on the server
    /// (Remote Host "edit on server"); such copies live outside the project.
    pub fn engine_for_remote_edit(&self, path: &Path) -> Option<Arc<Engine>> {
        self.engines
            .values()
            .find(|engine| engine.remote_edit(path).is_some())
            .cloned()
    }

    /// The project's deployment engines, one per configured local folder.
    pub fn engines(&self) -> Vec<Arc<Engine>> {
        let mut engines: Vec<_> = self.engines.values().cloned().collect();
        engines.sort_by(|a, b| a.root().cmp(b.root()));
        engines
    }

    fn worktree_root(&self, worktree_id: WorktreeId, cx: &App) -> Option<PathBuf> {
        let worktree = self
            .project
            .upgrade()?
            .read(cx)
            .worktree_for_id(worktree_id, cx)?;
        let worktree = worktree.read(cx);
        worktree
            .is_local()
            .then(|| worktree.abs_path().to_path_buf())
    }

    fn on_project_event(
        &mut self,
        _: Entity<Project>,
        event: &project::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            project::Event::WorktreeAdded(worktree_id) => self.load_engine(*worktree_id, cx),
            project::Event::WorktreeRemoved(worktree_id) => {
                if let Some(engine) = self.engines.remove(worktree_id) {
                    set_configured(engine.root(), false, cx);
                }
            }
            project::Event::WorktreeUpdatedEntries(worktree_id, entries) => {
                let Some(root) = self.worktree_root(*worktree_id, cx) else {
                    return;
                };
                let config_changed = entries
                    .iter()
                    .any(|(path, _, _)| path.as_std_path() == Path::new(CONFIG_PATH));
                if config_changed {
                    match self.engines.get(worktree_id) {
                        Some(engine) => {
                            let engine = engine.clone();
                            cx.background_spawn(async move { engine.reload_config() })
                                .detach();
                        }
                        None => self.load_engine(*worktree_id, cx),
                    }
                }

                // External changes and deletes; saves from VEGA come through
                // the buffer hook and the engine ignores the duplicate.
                let Some(engine) = self.engines.get(worktree_id).cloned() else {
                    return;
                };
                let changes: Vec<(PathBuf, bool)> = entries
                    .iter()
                    .filter_map(|(path, _, change)| {
                        let removed = match change {
                            PathChange::Removed => true,
                            PathChange::Added
                            | PathChange::Updated
                            | PathChange::AddedOrUpdated => false,
                            PathChange::Loaded => return None,
                        };
                        Some((root.join(path.as_std_path()), removed))
                    })
                    .collect();
                if !changes.is_empty() {
                    Tokio::spawn(cx, async move {
                        for (path, removed) in changes {
                            let event = if removed {
                                FsEvent::Removed(&path)
                            } else {
                                FsEvent::Changed(&path)
                            };
                            engine.on_fs_event(event).await;
                        }
                    })
                    .detach();
                }
            }
            _ => {}
        }
    }

    fn load_engine(&mut self, worktree_id: WorktreeId, cx: &mut Context<Self>) {
        if self.engines.contains_key(&worktree_id) {
            return;
        }
        let Some(root) = self.worktree_root(worktree_id, cx) else {
            return;
        };
        let prompter: Arc<dyn Prompter> = self.prompter.clone();
        let credentials = crate::secrets::credentials(cx);
        let load = Tokio::spawn(cx, async move {
            locate_config(&root)
                .exists
                .then(|| Engine::new(&root, prompter, credentials))
        });
        let task = cx.spawn(async move |this, cx| {
            let Some(engine) = load.await.ok().flatten() else {
                return;
            };
            this.update(cx, |this, cx| {
                set_configured(engine.root(), true, cx);
                this.watch_transfers(&engine, cx);
                this.engines.insert(worktree_id, engine);
                cx.notify();
            })
            .ok();
        });
        self._tasks.push(task);
    }

    fn watch_buffer(&mut self, buffer: &Entity<language::Buffer>, cx: &mut Context<Self>) {
        let subscription = cx.subscribe(buffer, |this, buffer, event, cx| {
            if !matches!(event, BufferEvent::Saved) {
                return;
            }
            let Some(path) = buffer
                .read(cx)
                .file()
                .and_then(|file| file.as_local())
                .map(|file| file.abs_path(cx))
            else {
                return;
            };
            if let Some(engine) = this
                .engine_for_path(&path)
                .or_else(|| this.engine_for_remote_edit(&path))
            {
                Tokio::spawn(cx, async move { engine.on_saved(&path, true).await }).detach();
            }
        });
        self._subscriptions.push(subscription);
    }

    /// Keeps [`TransfersStatus`] up to date from the engine's queue.
    fn watch_transfers(&mut self, engine: &Arc<Engine>, cx: &mut Context<Self>) {
        let (changed_tx, mut changed_rx) = mpsc::unbounded::<()>();
        let mut events = engine.queue.subscribe();
        let forward = Tokio::spawn(cx, async move {
            loop {
                match events.recv().await {
                    Ok(_) => {
                        if changed_tx.unbounded_send(()).is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });
        let update = cx.spawn(async move |this, cx| {
            let _forward = forward;
            while changed_rx.next().await.is_some() {
                let result = this.update(cx, |this, cx| {
                    this.refresh_transfers();
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        });
        self._tasks.push(update);
    }

    fn refresh_transfers(&mut self) {
        let tasks: Vec<_> = self
            .engines
            .values()
            .flat_map(|engine| engine.queue.tasks())
            .filter(|task| task.status.is_active())
            .collect();
        let current = tasks
            .iter()
            .find(|task| task.status == TaskStatus::Running)
            .or(tasks.first())
            .map(|task| {
                let name = Path::new(&task.source)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| task.source.clone());
                SharedString::from(format!("{name} → {}", task.server_name))
            });
        self.transfers = TransfersStatus {
            active: tasks.len(),
            current,
        };
    }

    /// What the Deployment Servers dialog needs for this project: the root of
    /// its first local folder, that folder's engine if it has a config, and
    /// the prompter for host-key and password questions.
    pub(crate) fn servers_dialog_context(
        &self,
        cx: &App,
    ) -> Option<(PathBuf, Option<Arc<Engine>>, Arc<dyn Prompter>)> {
        let project = self.project.upgrade()?;
        let worktree = project.read(cx).visible_worktrees(cx).next()?;
        let worktree = worktree.read(cx);
        if !worktree.is_local() {
            return None;
        }
        let root = worktree.abs_path().to_path_buf();
        let engine = self.engines.get(&worktree.id()).cloned();
        let prompter: Arc<dyn Prompter> = self.prompter.clone();
        Some((root, engine, prompter))
    }

    pub fn cancel_transfers(&self) {
        for engine in self.engines.values() {
            engine.cancel(None);
        }
    }
}

fn set_configured(root: &Path, configured: bool, cx: &mut App) {
    let roots = cx.default_global::<ConfiguredRoots>();
    if configured {
        roots.0.insert(root.to_path_buf());
    } else {
        roots.0.remove(root);
    }
}
