//! The "Scratches and Consoles" node at the bottom of the project panel.
//!
//! The scratches folder is loaded as a hidden worktree of the project, so
//! scratch files get the project's language servers and renames carry their
//! open tabs along.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use db::kvp::KeyValueStore;
use file_icons::FileIcons;
use fs::{RemoveOptions, RenameOptions};
use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, Context, DismissEvent, Entity, EntityId,
    Focusable as _, MouseButton, MouseDownEvent, Pixels, Point, PromptLevel, Subscription, Task,
    WeakEntity, Window, anchored, deferred,
};
use project::{Project, ProjectPath, Worktree};
use project_panel::{ExtraRowState, ProjectPanel, ProjectPanelExtraRows};
use ui::{ContextMenu, prelude::*};
use util::ResultExt as _;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use vega_dialog::NameModal;
use workspace::{ItemHandle, SaveIntent, Workspace};
use zed_actions::scratches::{NewScratchBuffer, NewScratchFile};

use crate::list_picker::ListPicker;
use crate::naming::{extension_of, free_name, run_command, stem_of, with_extension};
use crate::{
    language_choices, language_entries, notify, open_scratch, recent_languages, remember_language,
    run_scratch, save_if_dirty, scratches_dir,
};

const EXPANDED_KEY: &str = "vega_scratches_node_expanded";

#[derive(Clone, Debug, PartialEq, Eq)]
enum RowId {
    /// "Scratches and Consoles".
    Consoles,
    /// A path in the scratches folder; the empty path is the "Scratches" node.
    Path(Arc<RelPath>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RowKind {
    Consoles,
    Scratches,
    Dir,
    File,
}

#[derive(Clone, Debug)]
struct Row {
    id: RowId,
    kind: RowKind,
    name: SharedString,
    depth: usize,
}

impl Row {
    fn rel_path(&self) -> Option<&Arc<RelPath>> {
        match &self.id {
            RowId::Path(path) => Some(path),
            RowId::Consoles => None,
        }
    }
}

pub(crate) struct ScratchRows {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    panel: WeakEntity<ProjectPanel>,
    /// The scratches folder as a hidden worktree (held so it stays loaded).
    worktree: Option<Entity<Worktree>>,
    consoles_expanded: bool,
    expanded: HashSet<Arc<RelPath>>,
    rows: Vec<Row>,
    selected: Option<RowId>,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, RowId, Subscription)>,
    /// Open scratch tabs, to delete scratches that are empty when closed.
    open_scratches: HashMap<EntityId, PathBuf>,
    _subscriptions: Vec<Subscription>,
    _load: Task<()>,
}

impl ScratchRows {
    pub(crate) fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        panel: WeakEntity<ProjectPanel>,
        cx: &mut Context<Self>,
    ) -> Self {
        let consoles_expanded = KeyValueStore::global(cx)
            .read_kvp(EXPANDED_KEY)
            .log_err()
            .flatten()
            .is_some_and(|value| value == "1");
        let mut subscriptions = Vec::new();
        if let Some(workspace) = workspace.upgrade() {
            subscriptions.push(cx.subscribe(&workspace, Self::on_workspace_event));
        }
        let fs = project.read(cx).fs().clone();
        let load_project = project.clone();
        let load =
            cx.spawn(async move |this, cx| {
                let root = scratches_dir();
                if let Err(error) = fs.create_dir(&root).await {
                    log::warn!("couldn't create the scratches folder: {error}");
                    return;
                }
                let task = load_project.update(cx, |project, cx| {
                    project.find_or_create_worktree(&root, false, cx)
                });
                let Some((worktree, _)) = task.await.log_err() else {
                    return;
                };
                this.update(cx, |this, cx| {
                    this._subscriptions
                        .push(cx.subscribe(&worktree, |this, _, event, cx| match event {
                            worktree::Event::UpdatedEntries(_)
                            | worktree::Event::DeletedEntry(_) => this.rebuild(cx),
                            _ => {}
                        }));
                    this.worktree = Some(worktree);
                    this.rebuild(cx);
                })
                .ok();
            });
        let mut this = Self {
            workspace,
            project,
            panel,
            worktree: None,
            consoles_expanded,
            expanded: HashSet::from_iter([RelPath::empty_arc()]),
            rows: Vec::new(),
            selected: None,
            context_menu: None,
            open_scratches: HashMap::default(),
            _subscriptions: subscriptions,
            _load: load,
        };
        this.rebuild(cx);
        this
    }

    fn rebuild(&mut self, cx: &mut Context<Self>) {
        let mut rows = vec![Row {
            id: RowId::Consoles,
            kind: RowKind::Consoles,
            name: "Scratches and Consoles".into(),
            depth: 0,
        }];
        if self.consoles_expanded {
            let root = RelPath::empty_arc();
            rows.push(Row {
                id: RowId::Path(root.clone()),
                kind: RowKind::Scratches,
                name: "Scratches".into(),
                depth: 1,
            });
            if self.expanded.contains(&root)
                && let Some(worktree) = &self.worktree
            {
                push_children(worktree.read(cx), &root, 2, &self.expanded, &mut rows);
            }
        }
        self.rows = rows;
        cx.notify();
    }

    fn abs_path(&self, row: &Row, cx: &App) -> PathBuf {
        let root = self
            .worktree
            .as_ref()
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .unwrap_or_else(scratches_dir);
        match row.rel_path() {
            Some(path) if !path.is_empty() => root.join(path.as_std_path()),
            _ => root,
        }
    }

    /// Names of the entries in `folder`.
    fn sibling_names(&self, folder: &RelPath, cx: &App) -> HashSet<String> {
        self.worktree
            .as_ref()
            .map(|worktree| {
                worktree
                    .read(cx)
                    .child_entries(folder)
                    .filter_map(|entry| entry.path.file_name().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn select(&mut self, id: RowId, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(id);
        if let Some(panel) = self.panel.upgrade() {
            panel.update(cx, |panel, cx| panel.clear_selection(cx));
            window.focus(&panel.focus_handle(cx), cx);
        }
        cx.notify();
    }

    fn click(
        &mut self,
        row: &Row,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select(row.id.clone(), window, cx);
        match row.kind {
            RowKind::Consoles => {
                self.consoles_expanded = !self.consoles_expanded;
                let kvp = KeyValueStore::global(cx);
                let value = if self.consoles_expanded { "1" } else { "0" };
                cx.background_spawn(async move {
                    kvp.write_kvp(EXPANDED_KEY.into(), value.into())
                        .await
                        .log_err();
                })
                .detach();
                self.rebuild(cx);
            }
            RowKind::Scratches | RowKind::Dir => {
                if let Some(path) = row.rel_path()
                    && !self.expanded.remove(path)
                {
                    self.expanded.insert(path.clone());
                }
                self.rebuild(cx);
            }
            RowKind::File => {
                let path = self.abs_path(row, cx);
                let focus = event.click_count() > 1;
                let workspace = self.workspace.clone();
                window
                    .spawn(cx, async move |cx| {
                        open_scratch(&workspace, path, focus, cx).await
                    })
                    .detach();
            }
        }
    }

    fn deploy_menu(
        &mut self,
        row: Row,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select(row.id.clone(), window, cx);
        let this = cx.entity().downgrade();
        let runnable =
            row.kind == RowKind::File && run_command(extension_of(&row.name), "php").is_some();
        let menu_row = row.clone();
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let row = menu_row;
            let act = |operation: fn(&mut Self, &Row, &mut Window, &mut Context<Self>)| {
                let this = this.clone();
                let row = row.clone();
                move |window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| operation(this, &row, window, cx))
                        .ok();
                }
            };
            let menu = match row.kind {
                RowKind::Consoles => menu
                    .action("New Scratch File…", Box::new(NewScratchFile))
                    .action("New Scratch Buffer", Box::new(NewScratchBuffer)),
                RowKind::Scratches => menu
                    .action("New Scratch File…", Box::new(NewScratchFile))
                    .entry("New File…", None, act(Self::new_file))
                    .entry("New Directory…", None, act(Self::new_directory)),
                RowKind::Dir => menu
                    .entry("New File…", None, act(Self::new_file))
                    .entry("New Directory…", None, act(Self::new_directory))
                    .separator()
                    .entry("Rename…", None, act(Self::rename))
                    .entry("Delete", None, act(Self::delete)),
                RowKind::File => menu
                    .when(runnable, |menu| {
                        menu.entry(format!("Run '{}'", row.name), None, act(Self::run))
                            .separator()
                    })
                    .entry("Change Language…", None, act(Self::change_language))
                    .entry("Rename…", None, act(Self::rename))
                    .entry("Copy…", None, act(Self::copy))
                    .entry("Move to Project…", None, act(Self::move_to_project))
                    .entry("Delete", None, act(Self::delete)),
            };
            menu.separator()
                .entry("Reveal in Finder", None, act(Self::reveal))
                .entry("Copy Path", None, act(Self::copy_path))
        });
        window.focus(&menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu = None;
            cx.notify();
        });
        self.context_menu = Some((menu, position, row.id, subscription));
        cx.notify();
    }

    fn run(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let path = self.abs_path(row, cx);
        self.workspace
            .update(cx, |workspace, cx| run_scratch(workspace, path, window, cx))
            .ok();
    }

    fn reveal(&mut self, row: &Row, _: &mut Window, cx: &mut Context<Self>) {
        cx.reveal_path(&self.abs_path(row, cx));
    }

    fn copy_path(&mut self, row: &Row, _: &mut Window, cx: &mut Context<Self>) {
        let path = self.abs_path(row, cx).to_string_lossy().into_owned();
        cx.write_to_clipboard(ClipboardItem::new_string(path));
    }

    /// Asks for a name with `NameModal`, then calls `then` with it.
    fn ask_name(
        &self,
        title: &'static str,
        initial: String,
        then: impl FnOnce(&mut Self, String, &mut Window, &mut Context<Self>) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let this = cx.entity().downgrade();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    NameModal::new(
                        title,
                        &initial,
                        move |name, window, cx| {
                            this.update(cx, |this, cx| then(this, name, window, cx))
                                .ok();
                        },
                        window,
                        cx,
                    )
                })
            })
            .ok();
    }

    fn rename(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = row.rel_path().cloned() else {
            return;
        };
        self.ask_name(
            "Rename",
            row.name.to_string(),
            move |this, name, _, cx| this.rename_entry(&path, &name, cx),
            window,
            cx,
        );
    }

    /// Renames `path` to `name` in the same folder; open tabs follow.
    fn rename_entry(&mut self, path: &Arc<RelPath>, name: &str, cx: &mut Context<Self>) {
        let Some(worktree) = &self.worktree else {
            return;
        };
        let (Some(entry), Some(parent), Ok(name_path)) = (
            worktree.read(cx).entry_for_path(path).map(|entry| entry.id),
            path.parent(),
            RelPath::new(Path::new(name), PathStyle::local()),
        ) else {
            return;
        };
        let new_path = parent.join(&name_path).as_rel_path().into_arc();
        let worktree_id = worktree.read(cx).id();
        let task = self.project.update(cx, |project, cx| {
            project.rename_entry(
                entry,
                ProjectPath {
                    worktree_id,
                    path: new_path.clone(),
                },
                cx,
            )
        });
        self.selected = Some(RowId::Path(new_path));
        self.report_failure(task, "Couldn't rename", cx);
    }

    fn report_failure<T: 'static>(
        &self,
        task: Task<anyhow::Result<T>>,
        what: &'static str,
        cx: &mut Context<Self>,
    ) {
        let workspace = self.workspace.clone();
        cx.spawn(async move |_, cx| {
            if let Err(error) = task.await {
                workspace
                    .update(cx, |workspace, cx| {
                        notify(workspace, format!("{what}: {error}"), cx)
                    })
                    .ok();
            }
        })
        .detach();
    }

    fn change_language(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let (Some(path), Some(parent)) = (
            row.rel_path().cloned(),
            row.rel_path().and_then(|path| path.parent()),
        ) else {
            return;
        };
        let siblings = self.sibling_names(parent, cx);
        let name = row.name.to_string();
        let registry = self.project.read(cx).languages().clone();
        let (languages, first_count) = language_choices(&registry, &recent_languages(cx), None);
        let current = extension_of(&name).to_lowercase();
        let selected = languages
            .iter()
            .position(|choice| choice.extension.to_lowercase() == current)
            .unwrap_or(0);
        let entries = language_entries(&languages, first_count, cx);
        let this = cx.entity().downgrade();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    ListPicker::new(
                        "Change Language…",
                        entries,
                        selected,
                        move |index, _, cx| {
                            let Some(choice) = languages.get(index) else {
                                return;
                            };
                            remember_language(&choice.name, cx);
                            let mut new_name = with_extension(&name, &choice.extension);
                            if new_name != name && siblings.contains(&new_name) {
                                new_name = free_name(stem_of(&name), &choice.extension, &siblings);
                            }
                            if new_name != name {
                                this.update(cx, |this, cx| this.rename_entry(&path, &new_name, cx))
                                    .ok();
                            }
                        },
                        window,
                        cx,
                    )
                })
            })
            .ok();
    }

    fn copy(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let (Some(path), Some(parent)) = (
            row.rel_path().cloned(),
            row.rel_path()
                .and_then(|path| path.parent())
                .map(RelPath::into_arc),
        ) else {
            return;
        };
        let initial = free_name(
            stem_of(&row.name),
            extension_of(&row.name),
            &self.sibling_names(&parent, cx),
        );
        self.ask_name(
            "Copy",
            initial,
            move |this, name, window, cx| {
                let (Some(worktree), Ok(name_path)) = (
                    this.worktree.clone(),
                    RelPath::new(Path::new(&name), PathStyle::local()),
                ) else {
                    return;
                };
                let Some(entry) = worktree
                    .read(cx)
                    .entry_for_path(&path)
                    .map(|entry| entry.id)
                else {
                    return;
                };
                let new_path = parent.join(&name_path).as_rel_path().into_arc();
                let worktree_id = worktree.read(cx).id();
                let target = worktree.read(cx).abs_path().join(new_path.as_std_path());
                let task = this.project.update(cx, |project, cx| {
                    project.copy_entry(
                        entry,
                        ProjectPath {
                            worktree_id,
                            path: new_path.clone(),
                        },
                        cx,
                    )
                });
                this.selected = Some(RowId::Path(new_path));
                let workspace = this.workspace.clone();
                window
                    .spawn(cx, async move |cx| match task.await {
                        Ok(_) => open_scratch(&workspace, target, false, cx).await,
                        Err(error) => {
                            crate::report(&workspace, format!("Couldn't copy: {error}"), cx)
                        }
                    })
                    .detach();
            },
            window,
            cx,
        );
    }

    /// Moves a scratch into the project (a path relative to its first folder).
    fn move_to_project(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let source = self.abs_path(row, cx);
        let Some(project_root) = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        else {
            self.workspace
                .update(cx, |workspace, cx| {
                    notify(workspace, "Open a project folder first.", cx)
                })
                .ok();
            return;
        };
        let this = cx.entity().downgrade();
        let initial = row.name.to_string();
        let project_name = project_root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    NameModal::new_path(
                        format!("Move to Project ({project_name})"),
                        "Path in project",
                        &initial,
                        move |relative, window, cx| {
                            let target = project_root.join(relative);
                            this.update(cx, |this, cx| this.move_file(source, target, window, cx))
                                .ok();
                        },
                        window,
                        cx,
                    )
                })
            })
            .ok();
    }

    fn move_file(
        &mut self,
        source: PathBuf,
        target: PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = self.project.read(cx).fs().clone();
        let save = save_if_dirty(&self.project, &source, cx);
        let workspace = self.workspace.clone();
        let this = cx.entity().downgrade();
        window
            .spawn(cx, async move |cx| {
                let moved = async {
                    save.await?;
                    fs.rename(
                        &source,
                        &target,
                        RenameOptions {
                            overwrite: false,
                            ignore_if_exists: false,
                            create_parents: true,
                        },
                    )
                    .await
                }
                .await;
                if let Err(error) = moved {
                    crate::report(&workspace, format!("Couldn't move: {error}"), cx);
                    return;
                }
                this.update_in(cx, |this, window, cx| {
                    this.close_tabs_under(&source, window, cx)
                })
                .ok();
                open_scratch(&workspace, target, true, cx).await;
            })
            .detach();
    }

    fn delete(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        let path = self.abs_path(row, cx);
        let is_dir = row.kind == RowKind::Dir;
        let fs = self.project.read(cx).fs().clone();
        let answer = window.prompt(
            PromptLevel::Warning,
            &format!("Delete \"{}\"?", row.name),
            Some("It moves to the Trash."),
            &["Move to Trash", "Cancel"],
            cx,
        );
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |this, cx| {
            if answer.await.ok() != Some(0) {
                return;
            }
            let options = RemoveOptions {
                recursive: is_dir,
                ignore_if_not_exists: true,
            };
            if let Err(error) = fs.trash(&path, options).await {
                crate::report(&workspace, format!("Couldn't delete: {error}"), cx);
                return;
            }
            this.update_in(cx, |this, window, cx| {
                this.close_tabs_under(&path, window, cx)
            })
            .ok();
        })
        .detach();
    }

    /// Closes the tabs of files at or under `path` (moved or deleted).
    fn close_tabs_under(&mut self, path: &Path, window: &mut Window, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                for pane in workspace.panes().to_vec() {
                    let items: Vec<EntityId> = pane
                        .read(cx)
                        .items()
                        .filter(|item| {
                            item_path(item.as_ref(), &project, cx)
                                .is_some_and(|item_path| item_path.starts_with(path))
                        })
                        .map(|item| item.item_id())
                        .collect();
                    for item in items {
                        pane.update(cx, |pane, cx| {
                            pane.close_item_by_id(item, SaveIntent::Skip, window, cx)
                        })
                        .detach_and_log_err(cx);
                    }
                }
            })
            .ok();
    }

    fn folder_of(row: &Row) -> Option<Arc<RelPath>> {
        match row.kind {
            RowKind::Scratches | RowKind::Dir => row.rel_path().cloned(),
            _ => None,
        }
    }

    fn new_file(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        self.create_in(row, false, window, cx);
    }

    fn new_directory(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        self.create_in(row, true, window, cx);
    }

    fn create_in(&mut self, row: &Row, is_dir: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(folder) = Self::folder_of(row) else {
            return;
        };
        let title = if is_dir { "New Directory" } else { "New File" };
        self.ask_name(
            title,
            String::new(),
            move |this, name, window, cx| {
                let (Some(worktree), Ok(name_path)) = (
                    this.worktree.clone(),
                    RelPath::new(Path::new(&name), PathStyle::local()),
                ) else {
                    return;
                };
                let path = folder.join(&name_path).as_rel_path().into_arc();
                let worktree_id = worktree.read(cx).id();
                let target = worktree.read(cx).abs_path().join(path.as_std_path());
                let task = this.project.update(cx, |project, cx| {
                    project.create_entry(
                        ProjectPath {
                            worktree_id,
                            path: path.clone(),
                        },
                        is_dir,
                        cx,
                    )
                });
                this.expanded.insert(folder.clone());
                this.selected = Some(RowId::Path(path));
                this.rebuild(cx);
                let workspace = this.workspace.clone();
                window
                    .spawn(cx, async move |cx| match task.await {
                        Ok(_) if !is_dir => open_scratch(&workspace, target, true, cx).await,
                        Ok(_) => {}
                        Err(error) => {
                            crate::report(&workspace, format!("Couldn't create: {error}"), cx)
                        }
                    })
                    .detach();
            },
            window,
            cx,
        );
    }

    fn on_workspace_event(
        &mut self,
        workspace: Entity<Workspace>,
        event: &workspace::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            workspace::Event::ItemAdded { item } => {
                if let Some(path) = item_path(item.as_ref(), &self.project, cx)
                    && path.starts_with(scratches_dir())
                {
                    self.open_scratches.insert(item.item_id(), path);
                }
            }
            workspace::Event::ItemRemoved { item_id } => {
                let Some(path) = self.open_scratches.remove(item_id) else {
                    return;
                };
                let still_open = workspace.read(cx).items(cx).any(|item| {
                    item_path(item.as_ref(), &self.project, cx).as_ref() == Some(&path)
                });
                if !still_open {
                    self.delete_if_empty(path, cx);
                }
            }
            workspace::Event::ActiveItemChanged => self.reveal_active(workspace, cx),
            _ => {}
        }
    }

    /// Closing an empty scratch deletes it, like PhpStorm.
    fn delete_if_empty(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let fs = self.project.read(cx).fs().clone();
        cx.background_spawn(async move {
            if let Ok(Some(metadata)) = fs.metadata(&path).await
                && !metadata.is_dir
                && metadata.len == 0
            {
                fs.remove_file(
                    &path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .log_err();
            }
        })
        .detach();
    }

    /// Selects the active editor's scratch in the tree (expanding its folders).
    fn reveal_active(&mut self, workspace: Entity<Workspace>, cx: &mut Context<Self>) {
        let Some(path) = workspace
            .read(cx)
            .active_item(cx)
            .and_then(|item| item_path(item.as_ref(), &self.project, cx))
        else {
            return;
        };
        let Some(relative) = path
            .strip_prefix(scratches_dir())
            .ok()
            .and_then(|relative| RelPath::new(relative, PathStyle::local()).ok())
            .map(|relative| relative.into_arc())
        else {
            return;
        };
        self.consoles_expanded = true;
        let mut ancestor = relative.parent();
        while let Some(folder) = ancestor {
            self.expanded.insert(folder.into_arc());
            ancestor = folder.parent();
        }
        self.selected = Some(RowId::Path(relative));
        if let Some(panel) = self.panel.upgrade() {
            panel.update(cx, |panel, cx| panel.clear_selection(cx));
        }
        self.rebuild(cx);
    }
}

fn item_path(item: &dyn ItemHandle, project: &Entity<Project>, cx: &App) -> Option<PathBuf> {
    let project_path = item.project_path(cx)?;
    project.read(cx).absolute_path(&project_path, cx)
}

/// Adds the entries of `folder` (folders first, then by name) and, for
/// expanded folders, their entries.
fn push_children(
    worktree: &Worktree,
    folder: &RelPath,
    depth: usize,
    expanded: &HashSet<Arc<RelPath>>,
    rows: &mut Vec<Row>,
) {
    let mut children: Vec<(Arc<RelPath>, String, bool)> = worktree
        .child_entries(folder)
        .filter_map(|entry| {
            let name = entry.path.file_name()?.to_string();
            (name != ".DS_Store").then(|| (entry.path.clone(), name, entry.is_dir()))
        })
        .collect();
    children.sort_by(|a, b| {
        b.2.cmp(&a.2)
            .then_with(|| a.1.to_lowercase().cmp(&b.1.to_lowercase()))
    });
    for (path, name, is_dir) in children {
        rows.push(Row {
            id: RowId::Path(path.clone()),
            kind: if is_dir { RowKind::Dir } else { RowKind::File },
            name: name.into(),
            depth,
        });
        if is_dir && expanded.contains(&path) {
            push_children(worktree, &path, depth + 1, expanded, rows);
        }
    }
}

impl ProjectPanelExtraRows for ScratchRows {
    fn row_count(&self, _: &App) -> usize {
        self.rows.len()
    }

    fn row_depth(&self, index: usize, _: &App) -> usize {
        self.rows.get(index).map_or(0, |row| row.depth)
    }

    fn render_row(
        &mut self,
        index: usize,
        state: ExtraRowState,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(row) = self.rows.get(index).cloned() else {
            return div().into_any_element();
        };
        let selected = !state.panel_has_selection && self.selected.as_ref() == Some(&row.id);
        let expanded = match (&row.kind, row.rel_path()) {
            (RowKind::Consoles, _) => self.consoles_expanded,
            (RowKind::Scratches | RowKind::Dir, Some(path)) => self.expanded.contains(path),
            _ => false,
        };
        let chevron = (row.kind != RowKind::File).then(|| {
            FileIcons::get_chevron_icon(expanded, cx)
                .map(|chevron| Icon::from_path(chevron).color(Color::Muted))
                .unwrap_or_else(|| {
                    Icon::new(if expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    })
                    .color(Color::Muted)
                })
        });
        let icon = match row.kind {
            RowKind::Consoles => Icon::new(IconName::Notepad).color(Color::Muted),
            RowKind::Scratches | RowKind::Dir => {
                FileIcons::get_folder_icon(expanded, Path::new(row.name.as_ref()), cx)
                    .map(Icon::from_path)
                    .unwrap_or_else(|| Icon::new(IconName::Folder))
                    .color(Color::Muted)
            }
            RowKind::File => FileIcons::get_icon(Path::new(row.name.as_ref()), cx)
                .map(Icon::from_path)
                .unwrap_or_else(|| Icon::new(IconName::File))
                .color(Color::Muted),
        };
        let menu = self
            .context_menu
            .as_ref()
            .filter(|(_, _, id, _)| *id == row.id)
            .map(|(menu, position, _, _)| (menu.clone(), *position));
        let click_row = row.clone();
        let menu_row = row.clone();
        project_panel::extra_row_container(
            SharedString::from(format!("scratch-row-{index}")),
            selected,
            state.panel_focused,
            cx,
        )
        .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
            this.click(&click_row, event, window, cx)
        }))
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                this.deploy_menu(menu_row.clone(), event.position, window, cx)
            }),
        )
        .child(
            project_panel::extra_row_item(
                SharedString::from(format!("scratch-item-{index}")),
                row.depth,
                cx,
            )
            .child(
                h_flex()
                    .gap_0p5()
                    .child(match chevron {
                        Some(chevron) => chevron.into_any_element(),
                        None => h_flex()
                            .size(IconSize::default().rems())
                            .flex_none()
                            .into_any_element(),
                    })
                    .child(icon),
            )
            .child(
                h_flex()
                    .h_6()
                    .min_w_0()
                    .child(Label::new(row.name).single_line().truncate()),
            ),
        )
        .children(menu.map(|(menu, position)| {
            deferred(
                anchored()
                    .position(position)
                    .anchor(gpui::Anchor::TopLeft)
                    .child(menu),
            )
            .with_priority(1)
        }))
        .into_any_element()
    }
}
