//! Remote Host (D3): a dock panel that browses a deployment server's files,
//! PhpStorm-style. Opening a file edits a local copy whose saves are uploaded
//! back (the store routes those saves to the engine that opened the copy).

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use collections::{HashMap, HashSet};
use db::kvp::KeyValueStore;
use deployment::config::ServerType;
use deployment::engine::{Engine, RemoteItem, Summary};
use deployment::mapper::remote_name;
use deployment::{ErrorCode, RemoteError};
use futures::FutureExt as _;
use futures::future::BoxFuture;
use gpui::{
    Action, AnyElement, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter,
    ExternalPaths, FocusHandle, Focusable, ListSizingBehavior, MouseButton, MouseDownEvent,
    PathPromptOptions, Pixels, Point, Render, ScrollStrategy, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, px, uniform_list,
};
use gpui_tokio::Tokio;
use ui::{ContextMenu, DropdownMenu, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{DraggedSelection, OpenOptions, Workspace};
use zed_actions::deployment::{OpenDeploymentServers, ToggleRemoteHost};

use crate::name_modal::NameModal;
use crate::store::DeploymentStore;

actions!(
    remote_host,
    [
        /// Opens the selected file, or expands or collapses the selected folder.
        Open,
        /// Opens the server's copy of the selected file, even when it is mapped to a project file.
        OpenServerCopy,
        /// Downloads the selected file or folder to its mapped project path.
        DownloadToProject,
        /// Uploads local files or folders into the selected folder.
        UploadHere,
        /// Creates a file in the selected folder.
        NewFile,
        /// Creates a folder in the selected folder.
        NewFolder,
        /// Renames the selected file or folder.
        Rename,
        /// Deletes the selected file or folder on the server.
        Delete,
        /// Marks the selected file or folder to be moved by the next paste.
        Cut,
        /// Marks the selected file or folder to be copied by the next paste.
        Copy,
        /// Copies or moves the cut or copied item into the selected folder.
        Paste,
        /// Changes the selected file's or folder's permissions (chmod).
        ChangePermissions,
        /// Excludes the selected path from uploads and downloads, or includes it again.
        ToggleExcluded,
        /// Copies the selected item's path on the server.
        CopyPath,
        /// Reloads the open folders.
        Refresh,
        /// Expands the selected folder, or selects its first child when it is already expanded.
        ExpandSelectedEntry,
        /// Collapses the selected folder, or selects its parent folder.
        CollapseSelectedEntry,
    ]
);

/// Key-value store entry that remembers which dock the panel is in.
const DOCK_POSITION_KEY: &str = "vega_remote_host_dock_position";

/// A server the panel can browse: the engine (project folder) it belongs to
/// and its id within that engine's config.
#[derive(Clone)]
struct ServerChoice {
    engine: Arc<Engine>,
    id: String,
    label: SharedString,
}

impl ServerChoice {
    fn key(&self) -> ServerKey {
        ServerKey {
            root: self.engine.root().to_path_buf(),
            id: self.id.clone(),
        }
    }
}

/// Identifies a server across config reloads (the engine may be replaced).
#[derive(Clone, Debug, PartialEq)]
struct ServerKey {
    root: PathBuf,
    id: String,
}

/// One visible row of the tree.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Row {
    pub path: String,
    pub name: String,
    pub depth: usize,
    pub is_dir: bool,
}

/// Items marked by Cut or Copy, waiting for Paste.
struct Clipboard {
    server: ServerKey,
    paths: Vec<String>,
    cut: bool,
}

/// A row being dragged within the panel (dropping it on a folder moves it there).
struct DraggedRemote {
    server: ServerKey,
    path: String,
}

struct DraggedRemoteView {
    name: SharedString,
    is_dir: bool,
    offset: Point<Pixels>,
}

impl Render for DraggedRemoteView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let icon = if self.is_dir {
            IconName::Folder
        } else {
            IconName::File
        };
        div()
            .pl(self.offset.x + px(12.))
            .pt(self.offset.y + px(12.))
            .child(
                h_flex()
                    .gap_1()
                    .py_1()
                    .px_2()
                    .rounded_lg()
                    .bg(cx.theme().colors().background)
                    .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    .child(Label::new(self.name.clone())),
            )
    }
}

pub struct RemoteHostPanel {
    store: Entity<DeploymentStore>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    position: DockPosition,
    servers: Vec<ServerChoice>,
    selected_server: Option<usize>,
    root: Option<String>,
    children: HashMap<String, Vec<RemoteItem>>,
    expanded: HashSet<String>,
    loading: HashSet<String>,
    rows: Vec<Row>,
    selected: Option<String>,
    clipboard: Option<Clipboard>,
    error: Option<SharedString>,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    _tasks: Vec<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl RemoteHostPanel {
    pub(crate) fn new(
        store: Entity<DeploymentStore>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&store, |this, _, cx| this.refresh_servers(cx));
        let position = KeyValueStore::global(cx)
            .read_kvp(DOCK_POSITION_KEY)
            .log_err()
            .flatten()
            .and_then(|value| dock_position_from_str(&value))
            .unwrap_or(DockPosition::Right);
        let mut this = Self {
            store,
            workspace,
            focus_handle: cx.focus_handle(),
            position,
            servers: Vec::new(),
            selected_server: None,
            root: None,
            children: HashMap::default(),
            expanded: HashSet::default(),
            loading: HashSet::default(),
            rows: Vec::new(),
            selected: None,
            clipboard: None,
            error: None,
            scroll_handle: UniformListScrollHandle::new(),
            context_menu: None,
            _tasks: Vec::new(),
            _subscriptions: vec![subscription],
        };
        this.refresh_servers(cx);
        this
    }

    /// Re-reads the servers after the project's deployment config changes,
    /// keeping the selected server when it still exists.
    fn refresh_servers(&mut self, cx: &mut Context<Self>) {
        let engines = self.store.read(cx).engines();
        let several_folders = engines.len() > 1;
        let servers: Vec<ServerChoice> = engines
            .iter()
            .flat_map(|engine| {
                let folder = engine
                    .root()
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default();
                engine
                    .servers()
                    .into_iter()
                    .filter(|resolved| browsable(resolved.server.kind))
                    .map(move |resolved| {
                        let label = if several_folders {
                            format!("{} ({folder})", resolved.server.name)
                        } else {
                            resolved.server.name.clone()
                        };
                        ServerChoice {
                            engine: engine.clone(),
                            id: resolved.server.id,
                            label: label.into(),
                        }
                    })
            })
            .collect();

        let previous = self.server().map(ServerChoice::key);
        self.servers = servers;
        let keep =
            previous.and_then(|key| self.servers.iter().position(|choice| choice.key() == key));
        match keep {
            Some(index) => self.selected_server = Some(index),
            None => {
                let default = self.servers.iter().position(|choice| {
                    choice
                        .engine
                        .default_server()
                        .is_some_and(|default| default.server.id == choice.id)
                });
                self.select_server(default.or((!self.servers.is_empty()).then_some(0)), cx);
            }
        }
        cx.notify();
    }

    fn select_server(&mut self, index: Option<usize>, cx: &mut Context<Self>) {
        self.selected_server = index;
        self.children.clear();
        self.expanded.clear();
        self.loading.clear();
        self.selected = None;
        self.error = None;
        self.root = self
            .server()
            .and_then(|choice| choice.engine.remote_start(&choice.id).ok());
        self.update_rows();
        if let Some(root) = self.root.clone() {
            self.load_dir(root, cx);
        }
        cx.notify();
    }

    fn server(&self) -> Option<&ServerChoice> {
        self.servers.get(self.selected_server?)
    }

    fn update_rows(&mut self) {
        self.rows = match &self.root {
            Some(root) => visible_rows(root, &self.children, &self.expanded),
            None => Vec::new(),
        };
    }

    fn item(&self, path: &str) -> Option<&RemoteItem> {
        self.children
            .get(&remote_parent(path))?
            .iter()
            .find(|item| item.path == path)
    }

    fn selected_index(&self) -> Option<usize> {
        let selected = self.selected.as_deref()?;
        self.rows.iter().position(|row| row.path == selected)
    }

    fn selected_row(&self) -> Option<Row> {
        self.rows.get(self.selected_index()?).cloned()
    }

    fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(row) = self.rows.get(index) {
            self.selected = Some(row.path.clone());
            self.scroll_handle
                .scroll_to_item(index, ScrollStrategy::Nearest);
            cx.notify();
        }
    }

    /// The folder new or pasted items go into: the selected folder, the selected
    /// file's folder, or the root when nothing is selected.
    fn target_dir(&self) -> Option<String> {
        match self.selected_row() {
            Some(row) => Some(dir_of(&row)),
            None => self.root.clone(),
        }
    }

    fn load_dir(&mut self, path: String, cx: &mut Context<Self>) {
        let Some(choice) = self.server().cloned() else {
            return;
        };
        self.loading.insert(path.clone());
        let list = Tokio::spawn(cx, {
            let path = path.clone();
            async move { choice.engine.remote_list(&choice.id, &path).await }
        });
        let task = cx.spawn(async move |this, cx| {
            let result = list.await;
            this.update(cx, |this, cx| {
                this.loading.remove(&path);
                match result {
                    Ok(Ok(items)) => {
                        this.error = None;
                        this.children.insert(path, items);
                    }
                    Ok(Err(error)) => this.error = Some(error.to_string().into()),
                    Err(error) => this.error = Some(error.to_string().into()),
                }
                this.update_rows();
                cx.notify();
            })
            .ok();
        });
        self._tasks.push(task);
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let mut dirs: Vec<String> = self.children.keys().cloned().collect();
        dirs.sort();
        for dir in dirs {
            self.load_dir(dir, cx);
        }
        if self.children.is_empty()
            && let Some(root) = self.root.clone()
        {
            self.load_dir(root, cx);
        }
    }

    fn toggle_dir(&mut self, path: String, cx: &mut Context<Self>) {
        if !self.expanded.remove(&path) {
            self.expanded.insert(path.clone());
            if !self.children.contains_key(&path) {
                self.load_dir(path, cx);
            }
        }
        self.update_rows();
        cx.notify();
    }

    fn click_row(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(row.path.clone());
        window.focus(&self.focus_handle, cx);
        self.activate(row, window, cx);
        cx.notify();
    }

    fn activate(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        if row.is_dir {
            self.toggle_dir(row.path.clone(), cx);
        } else {
            self.open(row.path.clone(), false, window, cx);
        }
    }

    /// Opens a remote file in the editor. `server_copy` opens the server's
    /// version even when the file is mapped to a local file.
    fn open(
        &mut self,
        path: String,
        server_copy: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(choice) = self.server().cloned() else {
            return;
        };
        let open = Tokio::spawn(cx, async move {
            choice
                .engine
                .remote_open(&choice.id, &path, server_copy)
                .await
        });
        let workspace = self.workspace.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let local: PathBuf = match open.await {
                Ok(Ok(local)) => local,
                Ok(Err(error)) => {
                    this.update(cx, |this, cx| this.show_error(error, cx)).ok();
                    return;
                }
                Err(error) => {
                    this.update(cx, |this, cx| this.show_error(error, cx)).ok();
                    return;
                }
            };
            let opened = workspace.update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(local, OpenOptions::default(), window, cx)
            });
            if let Ok(opened) = opened
                && let Err(error) = opened.await
            {
                this.update(cx, |this, cx| this.show_error(error, cx)).ok();
            }
        });
        self._tasks.push(task);
    }

    fn show_error(&mut self, error: impl std::fmt::Display, cx: &mut Context<Self>) {
        self.error = Some(error.to_string().into());
        cx.notify();
    }

    /// Runs a remote file operation, then reloads the `reload` folders.
    fn run(
        &mut self,
        reload: Vec<String>,
        operation: impl FnOnce(Arc<Engine>, String) -> BoxFuture<'static, deployment::Result<()>>
        + Send
        + 'static,
        cx: &mut Context<Self>,
    ) {
        let Some(choice) = self.server().cloned() else {
            return;
        };
        let task = Tokio::spawn(cx, async move { operation(choice.engine, choice.id).await });
        let task = cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(_)) => this.error = None,
                    Ok(Err(error)) => this.error = Some(error.to_string().into()),
                    Err(error) => this.error = Some(error.to_string().into()),
                }
                for dir in reload {
                    if this.root.as_ref() != Some(&dir) {
                        this.expanded.insert(dir.clone());
                    }
                    this.load_dir(dir, cx);
                }
            })
            .ok();
        });
        self._tasks.push(task);
    }

    fn new_item(&mut self, folder: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dir) = self.target_dir() else {
            return;
        };
        let this = cx.weak_entity();
        let title = if folder { "New Folder" } else { "New File" };
        self.prompt(
            NameModalKind::Name,
            title.into(),
            "",
            window,
            cx,
            move |name, window, cx| {
                this.update(cx, |this, cx| {
                    let path = remote_join(&dir, &name);
                    if folder {
                        this.run(
                            vec![dir],
                            move |engine, id| {
                                async move { engine.remote_mkdir(&id, &path).await }.boxed()
                            },
                            cx,
                        );
                    } else {
                        let open_path = path.clone();
                        this.run(
                            vec![dir],
                            move |engine, id| {
                                async move { engine.remote_new_file(&id, &path).await }.boxed()
                            },
                            cx,
                        );
                        this.open(open_path, true, window, cx);
                    }
                })
                .ok();
            },
        );
    }

    fn prompt(
        &self,
        kind: NameModalKind,
        title: SharedString,
        initial: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
    ) {
        let initial = initial.to_string();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| match kind {
                    NameModalKind::Name => NameModal::new(title, &initial, on_confirm, window, cx),
                    NameModalKind::Permissions => NameModal::with_validator(
                        title,
                        "Permissions, e.g. 644",
                        &initial,
                        permissions_problem,
                        on_confirm,
                        window,
                        cx,
                    ),
                });
            })
            .ok();
    }

    /// Uploads local files or folders into the remote folder `dir`.
    fn upload(&mut self, locals: Vec<PathBuf>, dir: String, cx: &mut Context<Self>) {
        if locals.is_empty() {
            return;
        }
        self.run(
            vec![dir.clone()],
            move |engine, id| {
                async move {
                    let summary = engine.remote_upload_here(&id, &locals, &dir).await?;
                    summary_result(summary)
                }
                .boxed()
            },
            cx,
        );
    }

    /// Copies (or, for `cut`, moves) remote items into the folder `dir`.
    fn paste_into(&mut self, sources: Vec<String>, dir: String, cut: bool, cx: &mut Context<Self>) {
        let mut reload = vec![dir.clone()];
        if cut {
            for source in &sources {
                let parent = remote_parent(source);
                if !reload.contains(&parent) {
                    reload.push(parent);
                }
            }
        }
        self.run(
            reload,
            move |engine, id| {
                async move {
                    let existing: Vec<String> = engine
                        .remote_list(&id, &dir)
                        .await?
                        .into_iter()
                        .map(|item| item.name)
                        .collect();
                    let plan = paste_plan(&sources, &dir, &existing, cut)
                        .map_err(|message| RemoteError::new(ErrorCode::Exists, message))?;
                    if cut {
                        for (from, to) in &plan {
                            engine.remote_rename(&id, from, to).await?;
                        }
                        Ok(())
                    } else {
                        summary_result(engine.remote_copy(&id, &plan).await?)
                    }
                }
                .boxed()
            },
            cx,
        );
    }

    /// Paths of the project panel entries in a drag.
    fn project_paths(&self, selection: &DraggedSelection, cx: &App) -> Vec<PathBuf> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Vec::new();
        };
        let project = workspace.read(cx).project().read(cx);
        selection
            .items()
            .filter_map(|entry| {
                let project_path = project.path_for_entry(entry.entry_id, cx)?;
                project.absolute_path(&project_path, cx)
            })
            .collect()
    }

    fn drop_remote(&mut self, dragged: &DraggedRemote, dir: String, cx: &mut Context<Self>) {
        if self.server().map(ServerChoice::key).as_ref() == Some(&dragged.server) {
            self.paste_into(vec![dragged.path.clone()], dir, true, cx);
        }
    }

    // ---- actions --------------------------------------------------------------------------------

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selected_index() {
            Some(index) => index + 1,
            None => 0,
        };
        self.select_index(next.min(self.rows.len().saturating_sub(1)), cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let previous = self
            .selected_index()
            .map_or(0, |index| index.saturating_sub(1));
        self.select_index(previous, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, _: &mut Window, cx: &mut Context<Self>) {
        self.select_index(0, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _: &mut Window, cx: &mut Context<Self>) {
        self.select_index(self.rows.len().saturating_sub(1), cx);
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(row) = self.selected_row() {
            self.activate(&row, window, cx);
        }
    }

    fn open_selected(&mut self, _: &Open, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(row) = self.selected_row() {
            self.activate(&row, window, cx);
        }
    }

    fn open_server_copy(
        &mut self,
        _: &OpenServerCopy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(row) = self.selected_row().filter(|row| !row.is_dir) {
            self.open(row.path, true, window, cx);
        }
    }

    fn expand_selected(&mut self, _: &ExpandSelectedEntry, _: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected_index() else {
            return;
        };
        let row = self.rows[index].clone();
        if !row.is_dir {
            return;
        }
        if !self.expanded.contains(&row.path) {
            self.toggle_dir(row.path, cx);
        } else if self
            .rows
            .get(index + 1)
            .is_some_and(|next| next.depth > row.depth)
        {
            self.select_index(index + 1, cx);
        }
    }

    fn collapse_selected(
        &mut self,
        _: &CollapseSelectedEntry,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row) = self.selected_row() else {
            return;
        };
        if row.is_dir && self.expanded.contains(&row.path) {
            self.toggle_dir(row.path, cx);
            return;
        }
        let parent = remote_parent(&row.path);
        if let Some(index) = self.rows.iter().position(|row| row.path == parent) {
            self.select_index(index, cx);
        }
    }

    fn download_to_project(
        &mut self,
        _: &DownloadToProject,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row) = self.selected_row() else {
            return;
        };
        let paths = vec![row.path.clone()];
        self.run(
            vec![remote_parent(&row.path)],
            move |engine, id| {
                async move { summary_result(engine.remote_download(&id, &paths).await?) }.boxed()
            },
            cx,
        );
    }

    fn upload_here(&mut self, _: &UploadHere, _: &mut Window, cx: &mut Context<Self>) {
        let Some(dir) = self.target_dir() else {
            return;
        };
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
            prompt: Some("Upload".into()),
        });
        let task = cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(locals))) = paths.await else {
                return;
            };
            this.update(cx, |this, cx| this.upload(locals, dir, cx))
                .ok();
        });
        self._tasks.push(task);
    }

    fn new_file(&mut self, _: &NewFile, window: &mut Window, cx: &mut Context<Self>) {
        self.new_item(false, window, cx);
    }

    fn new_folder(&mut self, _: &NewFolder, window: &mut Window, cx: &mut Context<Self>) {
        self.new_item(true, window, cx);
    }

    fn rename(&mut self, _: &Rename, window: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.selected_row() else {
            return;
        };
        let this = cx.weak_entity();
        self.prompt(
            NameModalKind::Name,
            "Rename".into(),
            &row.name,
            window,
            cx,
            move |name, _, cx| {
                this.update(cx, |this, cx| {
                    let parent = remote_parent(&row.path);
                    let to = remote_join(&parent, &name);
                    let from = row.path;
                    this.run(
                        vec![parent],
                        move |engine, id| {
                            async move { engine.remote_rename(&id, &from, &to).await }.boxed()
                        },
                        cx,
                    );
                })
                .ok();
            },
        );
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.selected_row() else {
            return;
        };
        let paths = vec![row.path.clone()];
        self.run(
            vec![remote_parent(&row.path)],
            move |engine, id| {
                async move { summary_result(engine.remote_delete(&id, &paths).await?) }.boxed()
            },
            cx,
        );
    }

    fn mark_for_paste(&mut self, cut: bool, cx: &mut Context<Self>) {
        let (Some(row), Some(server)) = (self.selected_row(), self.server().map(ServerChoice::key))
        else {
            return;
        };
        self.clipboard = Some(Clipboard {
            server,
            paths: vec![row.path],
            cut,
        });
        cx.notify();
    }

    fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        self.mark_for_paste(true, cx);
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        self.mark_for_paste(false, cx);
    }

    fn can_paste(&self) -> bool {
        self.clipboard.as_ref().is_some_and(|clipboard| {
            self.server().map(ServerChoice::key).as_ref() == Some(&clipboard.server)
        })
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if !self.can_paste() {
            return;
        }
        let Some(dir) = self.target_dir() else {
            return;
        };
        let Some(clipboard) = self.clipboard.take() else {
            return;
        };
        let (paths, cut) = (clipboard.paths.clone(), clipboard.cut);
        if !cut {
            self.clipboard = Some(clipboard);
        }
        self.paste_into(paths, dir, cut, cx);
    }

    fn change_permissions(
        &mut self,
        _: &ChangePermissions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(row) = self.selected_row() else {
            return;
        };
        let current = self
            .item(&row.path)
            .and_then(|item| item.stat.mode)
            .map(|mode| format!("{:o}", mode & 0o7777))
            .unwrap_or_default();
        let this = cx.weak_entity();
        self.prompt(
            NameModalKind::Permissions,
            format!("Permissions of {}", row.name).into(),
            &current,
            window,
            cx,
            move |value, _, cx| {
                let Some(mode) = parse_permissions(&value) else {
                    return;
                };
                this.update(cx, |this, cx| {
                    let path = row.path.clone();
                    this.run(
                        vec![remote_parent(&row.path)],
                        move |engine, id| {
                            async move { engine.remote_chmod(&id, &path, mode, false).await }
                                .boxed()
                        },
                        cx,
                    );
                })
                .ok();
            },
        );
    }

    /// Whether `path` itself is in the server's excluded paths (not only inside
    /// an excluded folder).
    fn explicitly_excluded(&self, path: &str) -> bool {
        let Some(choice) = self.server() else {
            return false;
        };
        choice
            .engine
            .servers()
            .into_iter()
            .find(|resolved| resolved.server.id == choice.id)
            .is_some_and(|resolved| resolved.server.excluded_remote.iter().any(|p| p == path))
    }

    fn toggle_excluded(&mut self, _: &ToggleExcluded, _: &mut Window, cx: &mut Context<Self>) {
        let (Some(row), Some(choice)) = (self.selected_row(), self.server().cloned()) else {
            return;
        };
        let exclude = !self.explicitly_excluded(&row.path);
        match choice.engine.exclude_remote(&choice.id, &row.path, exclude) {
            Ok(()) => self.refresh(cx),
            Err(error) => self.show_error(error, cx),
        }
    }

    fn copy_path(&mut self, _: &CopyPath, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = self.selected.clone() {
            cx.write_to_clipboard(ClipboardItem::new_string(path));
        }
    }

    fn refresh_action(&mut self, _: &Refresh, _: &mut Window, cx: &mut Context<Self>) {
        self.refresh(cx);
    }

    // ---- rendering ------------------------------------------------------------------------------

    /// Shows the context menu for `row`, or for the root folder when `row` is None.
    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        row: Option<Row>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected = row.as_ref().map(|row| row.path.clone());
        let item = row.as_ref().and_then(|row| self.item(&row.path));
        let mapped = item.is_some_and(|item| item.local.is_some());
        let excluded = item.is_some_and(|item| item.excluded);
        let explicitly_excluded = row
            .as_ref()
            .is_some_and(|row| self.explicitly_excluded(&row.path));
        let can_paste = self.can_paste();
        let focus_handle = self.focus_handle.clone();
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let menu = menu.context(focus_handle);
            let Some(row) = row else {
                return menu
                    .action("Upload Here…", Box::new(UploadHere))
                    .separator()
                    .action("New File…", Box::new(NewFile))
                    .action("New Folder…", Box::new(NewFolder))
                    .action_disabled_when(!can_paste, "Paste", Box::new(Paste))
                    .separator()
                    .action("Refresh", Box::new(Refresh));
            };
            menu.when(!row.is_dir, |menu| {
                menu.action("Open", Box::new(Open)).when(mapped, |menu| {
                    menu.action("Open Server Copy", Box::new(OpenServerCopy))
                })
            })
            .when(mapped, |menu| {
                menu.action("Download to Project", Box::new(DownloadToProject))
            })
            .action("Upload Here…", Box::new(UploadHere))
            .separator()
            .action("New File…", Box::new(NewFile))
            .action("New Folder…", Box::new(NewFolder))
            .separator()
            .action("Cut", Box::new(Cut))
            .action("Copy", Box::new(Copy))
            .action_disabled_when(!can_paste, "Paste", Box::new(Paste))
            .separator()
            .action("Rename…", Box::new(Rename))
            .action("Delete", Box::new(Delete))
            .action("Change Permissions…", Box::new(ChangePermissions))
            .map(|menu| {
                if explicitly_excluded {
                    menu.action("Include Path", Box::new(ToggleExcluded))
                } else if !excluded {
                    menu.action("Exclude Path", Box::new(ToggleExcluded))
                } else {
                    menu
                }
            })
            .separator()
            .action("Copy Path", Box::new(CopyPath))
            .action("Refresh", Box::new(Refresh))
        });
        window.focus(&menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((menu, position, subscription));
        cx.notify();
    }

    fn render_rows(
        &mut self,
        range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let server = self.server().map(ServerChoice::key);
        let cut_paths: Vec<String> = self
            .clipboard
            .as_ref()
            .filter(|clipboard| clipboard.cut && Some(&clipboard.server) == server.as_ref())
            .map(|clipboard| clipboard.paths.clone())
            .unwrap_or_default();
        let drop_background = cx.theme().colors().drop_target_background;
        range
            .filter_map(|index| {
                let row = self.rows.get(index)?.clone();
                let expanded = self.expanded.contains(&row.path);
                let loading = self.loading.contains(&row.path);
                let excluded = self.item(&row.path).is_some_and(|item| item.excluded);
                let icon = match (row.is_dir, expanded) {
                    (true, true) => IconName::FolderOpen,
                    (true, false) => IconName::Folder,
                    (false, _) => IconName::File,
                };
                let color = if excluded || cut_paths.contains(&row.path) {
                    Color::Disabled
                } else {
                    Color::Default
                };
                let dir = dir_of(&row);
                let click_row = row.clone();
                let menu_row = row.clone();
                let item = ListItem::new(("remote-host-row", index))
                    .indent_level(row.depth)
                    .indent_step_size(px(12.))
                    .spacing(ListItemSpacing::Dense)
                    .toggle_state(self.selected.as_deref() == Some(row.path.as_str()))
                    .when(row.is_dir, |item| item.toggle(Some(expanded)))
                    .start_slot(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    .child(Label::new(row.name.clone()).color(color).truncate())
                    .when(loading, |item| {
                        item.end_slot(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.click_row(&click_row, window, cx)
                    }))
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            this.deploy_context_menu(
                                event.position,
                                Some(menu_row.clone()),
                                window,
                                cx,
                            )
                        },
                    ));
                let dragged = server.clone().map(|server| DraggedRemote {
                    server,
                    path: row.path.clone(),
                });
                let (name, is_dir) = (SharedString::from(row.name.clone()), row.is_dir);
                Some(
                    div()
                        .id(("remote-host-entry", index))
                        .when_some(dragged, |div, dragged| {
                            div.on_drag(dragged, move |_, offset, _, cx| {
                                cx.new(|_| DraggedRemoteView {
                                    name: name.clone(),
                                    is_dir,
                                    offset,
                                })
                            })
                        })
                        .when(row.is_dir, |div| {
                            div.drag_over::<DraggedRemote>(move |style, _, _, _| {
                                style.bg(drop_background)
                            })
                            .drag_over::<ExternalPaths>(move |style, _, _, _| {
                                style.bg(drop_background)
                            })
                            .drag_over::<DraggedSelection>(move |style, _, _, _| {
                                style.bg(drop_background)
                            })
                        })
                        .on_drop(cx.listener({
                            let dir = dir.clone();
                            move |this, dragged: &DraggedRemote, _, cx| {
                                this.drop_remote(dragged, dir.clone(), cx)
                            }
                        }))
                        .on_drop(cx.listener({
                            let dir = dir.clone();
                            move |this, paths: &ExternalPaths, _, cx| {
                                this.upload(paths.paths().to_vec(), dir.clone(), cx)
                            }
                        }))
                        .on_drop(
                            cx.listener(move |this, selection: &DraggedSelection, _, cx| {
                                let locals = this.project_paths(selection, cx);
                                this.upload(locals, dir.clone(), cx)
                            }),
                        )
                        .child(item)
                        .into_any_element(),
                )
            })
            .collect()
    }

    fn render_header(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();
        let servers: Vec<(usize, SharedString)> = self
            .servers
            .iter()
            .enumerate()
            .map(|(index, choice)| (index, choice.label.clone()))
            .collect();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (index, label) in servers.clone() {
                let this = this.clone();
                menu = menu.entry(label, None, move |_, cx| {
                    this.update(cx, |this, cx| this.select_server(Some(index), cx))
                        .ok();
                });
            }
            menu.separator()
                .action("Deployment Servers…", OpenDeploymentServers.boxed_clone())
        });
        let label = self
            .server()
            .map(|choice| choice.label.clone())
            .unwrap_or_else(|| "No server".into());
        h_flex()
            .px_2()
            .py_1()
            .gap_1()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(DropdownMenu::new("remote-host-server", label, menu))
            .child(
                IconButton::new("remote-host-refresh", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Refresh"))
                    .disabled(self.server().is_none())
                    .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
            )
    }

    /// The tree, plus the space below it, which takes drops and opens the root
    /// folder's context menu.
    fn render_tree(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let root = self.root.clone().unwrap_or_default();
        let drop_background = cx.theme().colors().drop_target_background;
        v_flex()
            .id("remote-host-tree")
            .flex_1()
            .size_full()
            .child(
                uniform_list(
                    "remote-host-rows",
                    self.rows.len(),
                    cx.processor(|this, range: Range<usize>, window, cx| {
                        this.render_rows(range, window, cx)
                    }),
                )
                .with_sizing_behavior(ListSizingBehavior::Infer)
                .track_scroll(&self.scroll_handle),
            )
            .child(
                div()
                    .id("remote-host-blank-area")
                    .flex_1()
                    .min_h(px(24.))
                    .drag_over::<DraggedRemote>(move |style, _, _, _| style.bg(drop_background))
                    .drag_over::<ExternalPaths>(move |style, _, _, _| style.bg(drop_background))
                    .drag_over::<DraggedSelection>(move |style, _, _, _| style.bg(drop_background))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.selected = None;
                        window.focus(&this.focus_handle, cx);
                        cx.notify();
                    }))
                    .on_mouse_down(
                        MouseButton::Right,
                        cx.listener(|this, event: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            this.deploy_context_menu(event.position, None, window, cx)
                        }),
                    )
                    .on_drop(cx.listener({
                        let root = root.clone();
                        move |this, dragged: &DraggedRemote, _, cx| {
                            this.drop_remote(dragged, root.clone(), cx)
                        }
                    }))
                    .on_drop(cx.listener({
                        let root = root.clone();
                        move |this, paths: &ExternalPaths, _, cx| {
                            this.upload(paths.paths().to_vec(), root.clone(), cx)
                        }
                    }))
                    .on_drop(
                        cx.listener(move |this, selection: &DraggedSelection, _, cx| {
                            let locals = this.project_paths(selection, cx);
                            this.upload(locals, root.clone(), cx)
                        }),
                    ),
            )
    }
}

/// Which validation [`RemoteHostPanel::prompt`] uses.
enum NameModalKind {
    Name,
    Permissions,
}

/// Server types that have files to browse.
fn browsable(kind: ServerType) -> bool {
    kind.transfers() && kind != ServerType::Group
}

/// The folder a drop or paste on `row` goes into: the row itself for a folder, else its parent.
fn dir_of(row: &Row) -> String {
    if row.is_dir {
        row.path.clone()
    } else {
        remote_parent(&row.path)
    }
}

/// Turns failed transfers in a summary into an error, so the panel shows them.
fn summary_result(summary: Summary) -> deployment::Result<()> {
    match summary.failed.as_slice() {
        [] => Ok(()),
        [failure] => Err(RemoteError::new(
            ErrorCode::Unknown,
            format!("{}: {}", failure.path, failure.error),
        )),
        [failure, rest @ ..] => Err(RemoteError::new(
            ErrorCode::Unknown,
            format!(
                "{}: {} (and {} more failed)",
                failure.path,
                failure.error,
                rest.len()
            ),
        )),
    }
}

fn dock_position_to_str(position: DockPosition) -> &'static str {
    match position {
        DockPosition::Left => "left",
        DockPosition::Bottom => "bottom",
        DockPosition::Right => "right",
    }
}

fn dock_position_from_str(value: &str) -> Option<DockPosition> {
    match value {
        "left" => Some(DockPosition::Left),
        "bottom" => Some(DockPosition::Bottom),
        "right" => Some(DockPosition::Right),
        _ => None,
    }
}

/// Parses an octal permission mode such as `644` or `0755`.
pub(crate) fn parse_permissions(value: &str) -> Option<u32> {
    let value = value.trim();
    if value.is_empty() || value.len() > 4 {
        return None;
    }
    u32::from_str_radix(value, 8).ok()
}

fn permissions_problem(value: &str) -> Option<&'static str> {
    parse_permissions(value)
        .is_none()
        .then_some("Enter octal permissions, such as 644 or 755.")
}

/// Where each pasted item goes: `(source, destination)` pairs for pasting
/// `sources` into `dir`, which already holds `existing` names. Copies get a
/// free name ("name copy.ext"); moves keep their name and fail on a clash.
/// Items cut and pasted back into their own folder are left out.
pub(crate) fn paste_plan(
    sources: &[String],
    dir: &str,
    existing: &[String],
    cut: bool,
) -> Result<Vec<(String, String)>, String> {
    let mut taken: HashSet<String> = existing.iter().cloned().collect();
    let mut plan = Vec::new();
    for source in sources {
        if dir == source || dir.starts_with(&format!("{}/", source.trim_end_matches('/'))) {
            return Err(format!("{source} can't be pasted into itself."));
        }
        let name = remote_name(source);
        if cut {
            if remote_parent(source) == dir {
                continue;
            }
            if !taken.insert(name.to_string()) {
                return Err(format!("{} already exists.", remote_join(dir, name)));
            }
            plan.push((source.clone(), remote_join(dir, name)));
        } else {
            let free = copy_name(name, &taken);
            taken.insert(free.clone());
            plan.push((source.clone(), remote_join(dir, &free)));
        }
    }
    Ok(plan)
}

/// `name` when it's free, else "stem copy.ext", "stem copy 2.ext", …
fn copy_name(name: &str, taken: &HashSet<String>) -> String {
    if !taken.contains(name) {
        return name.to_string();
    }
    let (stem, extension) = match name.rfind('.') {
        Some(dot) if dot > 0 => name.split_at(dot),
        _ => (name, ""),
    };
    (1..)
        .map(|n| {
            if n == 1 {
                format!("{stem} copy{extension}")
            } else {
                format!("{stem} copy {n}{extension}")
            }
        })
        .find(|candidate| !taken.contains(candidate))
        .unwrap_or_default()
}

pub(crate) fn remote_join(dir: &str, name: &str) -> String {
    if dir.ends_with('/') {
        format!("{dir}{name}")
    } else {
        format!("{dir}/{name}")
    }
}

pub(crate) fn remote_parent(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    match trimmed.rsplit_once('/') {
        Some(("", _)) | None => "/".into(),
        Some((parent, _)) => parent.into(),
    }
}

/// The tree's visible rows: `root`'s loaded children, depth first, descending
/// into expanded folders whose children are loaded.
pub(crate) fn visible_rows(
    root: &str,
    children: &HashMap<String, Vec<RemoteItem>>,
    expanded: &HashSet<String>,
) -> Vec<Row> {
    fn walk(
        dir: &str,
        depth: usize,
        children: &HashMap<String, Vec<RemoteItem>>,
        expanded: &HashSet<String>,
        rows: &mut Vec<Row>,
    ) {
        let Some(items) = children.get(dir) else {
            return;
        };
        for item in items {
            let is_dir = item.stat.is_dir();
            rows.push(Row {
                path: item.path.clone(),
                name: item.name.clone(),
                depth,
                is_dir,
            });
            if is_dir && expanded.contains(&item.path) {
                walk(&item.path, depth + 1, children, expanded, rows);
            }
        }
    }
    let mut rows = Vec::new();
    walk(root, 0, children, expanded, &mut rows);
    rows
}

impl Focusable for RemoteHostPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for RemoteHostPanel {}

impl Render for RemoteHostPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = if self.servers.is_empty() {
            v_flex()
                .p_3()
                .gap_2()
                .child(
                    Label::new("This project has no deployment servers yet.").color(Color::Muted),
                )
                .child(
                    Button::new("remote-host-add-server", "Add a Server…")
                        .style(ButtonStyle::Filled)
                        .on_click(|_, window, cx| {
                            window.dispatch_action(OpenDeploymentServers.boxed_clone(), cx)
                        }),
                )
                .into_any_element()
        } else {
            let root = self.root.clone().unwrap_or_default();
            v_flex()
                .flex_1()
                .size_full()
                .child(
                    h_flex().px_2().py_1().child(
                        Label::new(root)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
                )
                .when_some(self.error.clone(), |column, error| {
                    column.child(
                        h_flex()
                            .px_2()
                            .child(Label::new(error).size(LabelSize::Small).color(Color::Error)),
                    )
                })
                .child(self.render_tree(cx))
                .into_any_element()
        };

        v_flex()
            .key_context("RemoteHostPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::open_selected))
            .on_action(cx.listener(Self::open_server_copy))
            .on_action(cx.listener(Self::expand_selected))
            .on_action(cx.listener(Self::collapse_selected))
            .on_action(cx.listener(Self::download_to_project))
            .on_action(cx.listener(Self::upload_here))
            .on_action(cx.listener(Self::new_file))
            .on_action(cx.listener(Self::new_folder))
            .on_action(cx.listener(Self::rename))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::change_permissions))
            .on_action(cx.listener(Self::toggle_excluded))
            .on_action(cx.listener(Self::copy_path))
            .on_action(cx.listener(Self::refresh_action))
            .when(!self.servers.is_empty(), |panel| {
                panel.child(self.render_header(window, cx))
            })
            .child(body)
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Anchor::TopLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }))
    }
}

impl Panel for RemoteHostPanel {
    fn persistent_name() -> &'static str {
        "RemoteHostPanel"
    }

    fn panel_key() -> &'static str {
        "RemoteHostPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        let kvp = KeyValueStore::global(cx);
        let value = dock_position_to_str(position).to_string();
        db::write_and_log(cx, move || async move {
            kvp.write_kvp(DOCK_POSITION_KEY.into(), value).await
        });
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(260.)
    }

    /// The dock button shows once the project has a deployment server.
    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        (!self.servers.is_empty()).then_some(IconName::Server)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Remote Host")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleRemoteHost)
    }

    fn activation_priority(&self) -> u32 {
        20
    }
}

#[cfg(test)]
mod tests {
    use deployment::protocol::{EntryType, RemoteStat};

    use super::*;

    fn item(dir: &str, name: &str, is_dir: bool) -> RemoteItem {
        RemoteItem {
            name: name.into(),
            path: remote_join(dir, name),
            stat: RemoteStat {
                kind: if is_dir {
                    EntryType::Dir
                } else {
                    EntryType::File
                },
                target_kind: None,
                size: 0,
                mtime: 0,
                mode: None,
            },
            local: None,
            local_exists: false,
            excluded: false,
        }
    }

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn remote_paths_join_and_split() {
        assert_eq!(remote_join("/var/www", "index.php"), "/var/www/index.php");
        assert_eq!(remote_join("/", "etc"), "/etc");
        assert_eq!(remote_parent("/var/www/index.php"), "/var/www");
        assert_eq!(remote_parent("/var/www/"), "/var");
        assert_eq!(remote_parent("/etc"), "/");
        assert_eq!(remote_parent("/"), "/");
    }

    #[test]
    fn rows_descend_into_expanded_loaded_folders() {
        let mut children = HashMap::default();
        children.insert(
            "/site".to_string(),
            vec![
                item("/site", "src", true),
                item("/site", "index.php", false),
            ],
        );
        children.insert(
            "/site/src".to_string(),
            vec![item("/site/src", "app.php", false)],
        );
        let mut expanded = HashSet::default();

        let names = |rows: Vec<Row>| {
            rows.into_iter()
                .map(|row| format!("{}{}", "  ".repeat(row.depth), row.name))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            names(visible_rows("/site", &children, &expanded)),
            ["src", "index.php"]
        );

        expanded.insert("/site/src".to_string());
        assert_eq!(
            names(visible_rows("/site", &children, &expanded)),
            ["src", "  app.php", "index.php"]
        );

        // Expanded but not loaded yet: no children shown.
        children.remove("/site/src");
        assert_eq!(
            names(visible_rows("/site", &children, &expanded)),
            ["src", "index.php"]
        );
    }

    #[test]
    fn copies_get_free_names() {
        let plan = paste_plan(
            &strings(&["/site/index.php", "/site/src", "/site/.env"]),
            "/site",
            &strings(&["index.php", "index copy.php", "src", ".env"]),
            false,
        )
        .unwrap();
        assert_eq!(
            plan,
            [
                ("/site/index.php".into(), "/site/index copy 2.php".into()),
                ("/site/src".into(), "/site/src copy".into()),
                ("/site/.env".into(), "/site/.env copy".into()),
            ]
        );

        let plan = paste_plan(&strings(&["/a/x.txt"]), "/b", &[], false).unwrap();
        assert_eq!(plan, [("/a/x.txt".into(), "/b/x.txt".into())]);
    }

    #[test]
    fn moves_keep_names_and_refuse_clashes() {
        let plan = paste_plan(
            &strings(&["/a/x.txt", "/b/y.txt"]),
            "/b",
            &strings(&["y.txt"]),
            true,
        )
        .unwrap();
        assert_eq!(plan, [("/a/x.txt".into(), "/b/x.txt".into())]);

        assert!(paste_plan(&strings(&["/a/x.txt"]), "/b", &strings(&["x.txt"]), true).is_err());
    }

    #[test]
    fn folders_cannot_be_pasted_into_themselves() {
        assert!(paste_plan(&strings(&["/a"]), "/a", &[], false).is_err());
        assert!(paste_plan(&strings(&["/a"]), "/a/b", &[], true).is_err());
        assert!(paste_plan(&strings(&["/a"]), "/ab", &[], true).is_ok());
    }

    #[test]
    fn permissions_are_octal() {
        assert_eq!(parse_permissions("644"), Some(0o644));
        assert_eq!(parse_permissions(" 0755 "), Some(0o755));
        assert_eq!(parse_permissions("4755"), Some(0o4755));
        assert_eq!(parse_permissions("888"), None);
        assert_eq!(parse_permissions("12345"), None);
        assert_eq!(parse_permissions(""), None);
    }

    #[test]
    fn dock_positions_round_trip() {
        for position in [
            DockPosition::Left,
            DockPosition::Bottom,
            DockPosition::Right,
        ] {
            assert_eq!(
                dock_position_from_str(dock_position_to_str(position)),
                Some(position)
            );
        }
        assert_eq!(dock_position_from_str("top"), None);
    }
}
