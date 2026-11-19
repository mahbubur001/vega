//! Remote Host (D3): a dock panel that browses a deployment server's files,
//! PhpStorm-style. Opening a file edits a local copy whose saves are uploaded
//! back (the store routes those saves to the engine that opened the copy).

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use collections::{HashMap, HashSet};
use db::kvp::KeyValueStore;
use deployment::config::ServerType;
use deployment::engine::{Engine, RemoteItem, Summary};
use deployment::mapper::remote_name;
use deployment::protocol::{EntryType, RemoteStat};
use deployment::{ErrorCode, RemoteError};
use file_icons::FileIcons;
use futures::FutureExt as _;
use futures::future::BoxFuture;
use gpui::{
    Action, AnyElement, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter,
    ExternalPaths, FocusHandle, Focusable, ListSizingBehavior, MouseButton, MouseDownEvent,
    PathPromptOptions, Pixels, Point, Render, ScrollStrategy, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, actions, anchored, deferred, px, uniform_list,
};
use gpui_tokio::Tokio;
use project_panel::project_panel_settings::ProjectPanelSettings;
use settings::{ProjectPanelEntrySpacing, Settings as _, ShowIndentGuides};
use time::{OffsetDateTime, UtcOffset};
use ui::{
    ContextMenu, ContextMenuEntry, DropdownMenu, IconPosition, IndentGuideColors, ListItem,
    ListItemSpacing, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{DraggedSelection, OpenOptions, Workspace};
use zed_actions::deployment::{OpenDeploymentServers, ToggleRemoteHost};

use crate::name_modal::NameModal;
use crate::permissions_modal::{PermissionsChoice, PermissionsModal};
use crate::store::DeploymentStore;

actions!(
    remote_host,
    [
        /// Opens the server's copy of the selected file (saving uploads it), or
        /// expands or collapses the selected folder.
        EditRemoteFile,
        /// Opens the project file the selected server file is mapped to.
        EditLocalFile,
        /// Downloads the selected files or folders to their mapped project paths.
        DownloadToProject,
        /// Compares the selected server file with its project version.
        CompareWithLocal,
        /// Compares the selected server folder with its project folder before synchronizing.
        SyncWithLocal,
        /// Opens the selected item's web URL in the browser.
        OpenInBrowser,
        /// Opens an SSH session in the terminal, in the selected folder (SFTP servers).
        StartSshSession,
        /// Reveals the file open in the editor in the tree.
        SelectOpenedFile,
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
        /// Copies the selected item's absolute path on the server.
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

/// Key-value store entry that remembers the panel's view options.
const VIEW_OPTIONS_KEY: &str = "vega_remote_host_view_options";

/// What the tree shows next to each name (PhpStorm's Remote Host options).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ViewOptions {
    pub show_size: bool,
    pub show_date: bool,
    pub show_permissions: bool,
    pub highlight_symlinks: bool,
    pub highlight_mappings: bool,
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self {
            show_size: true,
            show_date: true,
            show_permissions: false,
            highlight_symlinks: true,
            highlight_mappings: true,
        }
    }
}

impl ViewOptions {
    const FLAGS: [&'static str; 5] = ["size", "date", "permissions", "symlinks", "mappings"];

    fn flags(&self) -> [bool; 5] {
        [
            self.show_size,
            self.show_date,
            self.show_permissions,
            self.highlight_symlinks,
            self.highlight_mappings,
        ]
    }

    /// Stored as the enabled flags, comma-separated.
    pub(crate) fn to_value(self) -> String {
        Self::FLAGS
            .iter()
            .zip(self.flags())
            .filter(|(_, on)| *on)
            .map(|(flag, _)| *flag)
            .collect::<Vec<_>>()
            .join(",")
    }

    pub(crate) fn from_value(value: &str) -> Self {
        let on: HashSet<&str> = value.split(',').map(str::trim).collect();
        Self {
            show_size: on.contains("size"),
            show_date: on.contains("date"),
            show_permissions: on.contains("permissions"),
            highlight_symlinks: on.contains("symlinks"),
            highlight_mappings: on.contains("mappings"),
        }
    }
}

/// A server the panel can browse: the engine (project folder) it belongs to
/// and its id within that engine's config.
#[derive(Clone)]
struct ServerChoice {
    engine: Arc<Engine>,
    id: String,
    label: SharedString,
    /// Shown on the root node, e.g. "Live (example.com)".
    address: SharedString,
    kind: ServerType,
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
    /// Extra rows picked with ⌘-click or ⇧-click (multi-select).
    marked: Vec<String>,
    clipboard: Option<Clipboard>,
    options: ViewOptions,
    local_offset: UtcOffset,
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
        let options = KeyValueStore::global(cx)
            .read_kvp(VIEW_OPTIONS_KEY)
            .log_err()
            .flatten()
            .map(|value| ViewOptions::from_value(&value))
            .unwrap_or_default();
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
            marked: Vec::new(),
            clipboard: None,
            options,
            local_offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
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
                        let address = match resolved.server.kind {
                            ServerType::Local => resolved.server.root_path.clone(),
                            _ => resolved.server.host.clone(),
                        };
                        ServerChoice {
                            engine: engine.clone(),
                            id: resolved.server.id,
                            label: label.into(),
                            address: address.into(),
                            kind: resolved.server.kind,
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
        if let Some(root) = self.root.clone() {
            self.expanded.insert(root.clone());
            self.load_dir(root, cx);
        }
        self.update_rows();
        cx.notify();
    }

    fn server(&self) -> Option<&ServerChoice> {
        self.servers.get(self.selected_server?)
    }

    /// The root node ("Server (host)") followed by the expanded tree.
    fn update_rows(&mut self) {
        let (Some(root), Some(choice)) = (&self.root, self.server()) else {
            self.rows = Vec::new();
            return;
        };
        let mut rows = vec![Row {
            path: root.clone(),
            name: choice.label.to_string(),
            depth: 0,
            is_dir: true,
        }];
        if self.expanded.contains(root) {
            rows.extend(
                visible_rows(root, &self.children, &self.expanded)
                    .into_iter()
                    .map(|row| Row {
                        depth: row.depth + 1,
                        ..row
                    }),
            );
        }
        self.rows = rows;
    }

    fn is_root(&self, path: &str) -> bool {
        self.root.as_deref() == Some(path)
    }

    /// The selected row unless it is the root node, which can't be renamed,
    /// deleted, copied and so on.
    fn selected_entry(&self) -> Option<Row> {
        self.selected_row().filter(|row| !self.is_root(&row.path))
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

    fn click_row(
        &mut self,
        row: &Row,
        modifiers: gpui::Modifiers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        if self.is_root(&row.path) {
            self.marked.clear();
            self.selected = Some(row.path.clone());
            self.activate(row, window, cx);
        } else if modifiers.secondary(cx) {
            if let Some(selected) = self.selected.clone().filter(|_| self.marked.is_empty())
                && !self.is_root(&selected)
            {
                self.marked.push(selected);
            }
            match self.marked.iter().position(|path| *path == row.path) {
                Some(index) => {
                    self.marked.remove(index);
                }
                None => self.marked.push(row.path.clone()),
            }
            self.selected = Some(row.path.clone());
        } else if modifiers.shift
            && let (Some(from), Some(to)) = (
                self.selected_index(),
                self.rows
                    .iter()
                    .position(|candidate| candidate.path == row.path),
            )
        {
            let (start, end) = if from <= to { (from, to) } else { (to, from) };
            self.marked = self.rows[start..=end]
                .iter()
                .filter(|candidate| !self.is_root(&candidate.path))
                .map(|candidate| candidate.path.clone())
                .collect();
        } else {
            self.marked.clear();
            self.selected = Some(row.path.clone());
            self.activate(row, window, cx);
        }
        cx.notify();
    }

    /// The rows an action applies to: the multi-selection when the selected row
    /// is part of it, else the selected row (never the root node).
    fn selected_paths(&self) -> Vec<String> {
        let selected = self.selected_entry().map(|row| row.path);
        match &selected {
            Some(path) if self.marked.contains(path) => self
                .rows
                .iter()
                .filter(|row| self.marked.contains(&row.path))
                .map(|row| row.path.clone())
                .collect(),
            _ => selected.into_iter().collect(),
        }
    }

    /// The parent folders of `paths`, each once (to reload after a change).
    fn parents_of(paths: &[String]) -> Vec<String> {
        let mut parents = Vec::new();
        for path in paths {
            let parent = remote_parent(path);
            if !parents.contains(&parent) {
                parents.push(parent);
            }
        }
        parents
    }

    fn activate(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        if row.is_dir {
            self.toggle_dir(row.path.clone(), cx);
        } else {
            self.open(row.path.clone(), true, window, cx);
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
        self.prompt(title.into(), "", window, cx, move |name, window, cx| {
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
        });
    }

    fn prompt(
        &self,
        title: SharedString,
        initial: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
    ) {
        let initial = initial.to_string();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    NameModal::new(title, &initial, on_confirm, window, cx)
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

    fn edit_remote_file(
        &mut self,
        _: &EditRemoteFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(row) = self.selected_row() {
            self.activate(&row, window, cx);
        }
    }

    fn edit_local_file(&mut self, _: &EditLocalFile, window: &mut Window, cx: &mut Context<Self>) {
        let Some(local) = self
            .selected_entry()
            .and_then(|row| self.item(&row.path))
            .and_then(|item| item.local.clone())
        else {
            return;
        };
        if !local.is_file() {
            self.show_error(
                format!(
                    "{} isn't in the project yet. Download it first.",
                    local.display()
                ),
                cx,
            );
            return;
        }
        self.workspace
            .update(cx, |workspace, cx| {
                workspace
                    .open_abs_path(local, OpenOptions::default(), window, cx)
                    .detach_and_log_err(cx);
            })
            .ok();
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
        let paths = self.selected_paths();
        if paths.is_empty() {
            return;
        }
        self.run(
            Self::parents_of(&paths),
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
        let Some(row) = self.selected_entry() else {
            return;
        };
        let this = cx.weak_entity();
        self.prompt(
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
        let paths = self.selected_paths();
        if paths.is_empty() {
            return;
        }
        self.marked.clear();
        self.run(
            Self::parents_of(&paths),
            move |engine, id| {
                async move { summary_result(engine.remote_delete(&id, &paths).await?) }.boxed()
            },
            cx,
        );
    }

    fn mark_for_paste(&mut self, cut: bool, cx: &mut Context<Self>) {
        let paths = self.selected_paths();
        let Some(server) = self
            .server()
            .map(ServerChoice::key)
            .filter(|_| !paths.is_empty())
        else {
            return;
        };
        self.clipboard = Some(Clipboard { server, paths, cut });
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
        let Some(row) = self.selected_entry() else {
            return;
        };
        let current = self
            .item(&row.path)
            .and_then(|item| item.stat.mode)
            .map(|mode| mode & 0o7777);
        let (file_mode, dir_mode) = if row.is_dir {
            (0o644, current.unwrap_or(0o755))
        } else {
            (current.unwrap_or(0o644), 0o755)
        };
        let title = row.path.clone();
        let this = cx.weak_entity();
        let on_confirm = move |choice: PermissionsChoice, _: &mut Window, cx: &mut App| {
            this.update(cx, |this, cx| {
                let path = row.path.clone();
                let mut reload = vec![remote_parent(&row.path)];
                if row.is_dir {
                    reload.push(row.path.clone());
                }
                this.run(
                    reload,
                    move |engine, id| {
                        async move {
                            if choice.recursive {
                                engine
                                    .remote_chmod_tree(
                                        &id,
                                        &path,
                                        choice.file_mode,
                                        choice.dir_mode,
                                    )
                                    .await
                            } else {
                                match choice.dir_mode.or(choice.file_mode) {
                                    Some(mode) => {
                                        engine.remote_chmod(&id, &path, mode, false).await
                                    }
                                    None => Ok(()),
                                }
                            }
                        }
                        .boxed()
                    },
                    cx,
                );
            })
            .ok();
        };
        let is_dir = row.is_dir;
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    PermissionsModal::new(
                        title, is_dir, file_mode, dir_mode, on_confirm, window, cx,
                    )
                });
            })
            .ok();
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
        let (Some(row), Some(choice)) = (self.selected_entry(), self.server().cloned()) else {
            return;
        };
        let exclude = !self.explicitly_excluded(&row.path);
        match choice.engine.exclude_remote(&choice.id, &row.path, exclude) {
            Ok(()) => self.refresh(cx),
            Err(error) => self.show_error(error, cx),
        }
    }

    /// The mapped project path of the selected entry, with the server it is on.
    fn selected_local(&self) -> Option<(Arc<Engine>, String, PathBuf, Row)> {
        let row = self.selected_entry()?;
        let local = self.item(&row.path)?.local.clone()?;
        let choice = self.server()?;
        Some((choice.engine.clone(), choice.id.clone(), local, row))
    }

    fn compare_with_local(&mut self, _: &CompareWithLocal, _: &mut Window, cx: &mut Context<Self>) {
        let Some((engine, id, local, row)) = self.selected_local().filter(|(.., row)| !row.is_dir)
        else {
            return;
        };
        if !local.is_file() {
            self.show_error(format!("{} has no project copy yet.", row.name), cx);
            return;
        }
        let compare = Tokio::spawn(cx, async move { engine.compare(Some(&id), &local).await });
        self._tasks.push(cx.spawn(async move |this, cx| {
            if let Ok(Err(error)) = compare.await {
                this.update(cx, |this, cx| this.show_error(error, cx)).ok();
            }
        }));
    }

    fn sync_with_local(&mut self, _: &SyncWithLocal, window: &mut Window, cx: &mut Context<Self>) {
        let Some((engine, id, local, _)) = self.selected_local() else {
            return;
        };
        self.workspace
            .update(cx, |workspace, cx| {
                crate::sync_view::open(workspace, engine, vec![local], Some(id), window, cx);
            })
            .ok();
    }

    fn open_in_browser(&mut self, _: &OpenInBrowser, _: &mut Window, cx: &mut Context<Self>) {
        let Some((engine, id, local, _)) = self.selected_local() else {
            return;
        };
        match engine.web_url(Some(&id), &local) {
            Ok(url) => cx.open_url(&url),
            Err(error) => self.show_error(error, cx),
        }
    }

    /// Opens `ssh` in a terminal tab and changes to the selected folder.
    fn start_ssh_session(
        &mut self,
        _: &StartSshSession,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(choice) = self.server() else {
            return;
        };
        let Some(server) = choice
            .engine
            .servers()
            .into_iter()
            .map(|resolved| resolved.server)
            .find(|server| server.id == choice.id)
        else {
            return;
        };
        let dir = self
            .selected_row()
            .map(|row| dir_of(&row))
            .or_else(|| self.root.clone())
            .unwrap_or_else(|| "~".into());
        let args = ssh_args(&server, &dir);
        let label = format!("SSH: {}", server.name);
        let command_label = format!("ssh {}", args.join(" "));
        let spawn = task::SpawnInTerminal {
            id: task::TaskId(format!("vega-ssh-{}", server.id)),
            full_label: label.clone(),
            label,
            command: Some("ssh".into()),
            args,
            command_label,
            use_new_terminal: true,
            allow_concurrent_runs: true,
            show_command: true,
            reveal: task::RevealStrategy::Always,
            ..Default::default()
        };
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.spawn_in_terminal(spawn, window, cx).detach();
            })
            .ok();
    }

    /// Reveals the file open in the editor (when it is mapped to this server).
    fn select_opened_file(&mut self, _: &SelectOpenedFile, _: &mut Window, cx: &mut Context<Self>) {
        let Some(choice) = self.server().cloned() else {
            return;
        };
        let local = self.workspace.upgrade().and_then(|workspace| {
            let workspace = workspace.read(cx);
            let project_path = workspace.active_item(cx)?.project_path(cx)?;
            workspace
                .project()
                .read(cx)
                .absolute_path(&project_path, cx)
        });
        let remote = local.and_then(|local| {
            let targets = choice.engine.targets(Some(&choice.id)).ok()?;
            targets.first()?.mapper.to_remote(&local)
        });
        let (Some(remote), Some(root)) = (remote, self.root.clone()) else {
            self.show_error("The open file isn't mapped to this server.", cx);
            return;
        };
        let folders = folders_between(&root, &remote);
        let missing: Vec<String> = folders
            .iter()
            .filter(|folder| !self.children.contains_key(*folder))
            .cloned()
            .collect();
        let list = Tokio::spawn(cx, async move {
            let mut listed = Vec::new();
            for folder in missing {
                let items = choice.engine.remote_list(&choice.id, &folder).await?;
                listed.push((folder, items));
            }
            Ok::<_, RemoteError>(listed)
        });
        self._tasks.push(cx.spawn(async move |this, cx| {
            let result = list.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(listed)) => {
                        this.children.extend(listed);
                        this.expanded.extend(folders);
                        this.update_rows();
                        this.marked.clear();
                        this.selected = Some(remote.clone());
                        if let Some(index) = this.rows.iter().position(|row| row.path == remote) {
                            this.scroll_handle
                                .scroll_to_item(index, ScrollStrategy::Center);
                        }
                    }
                    Ok(Err(error)) => this.show_error(error, cx),
                    Err(error) => this.show_error(error, cx),
                }
                cx.notify();
            })
            .ok();
        }));
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
        if row
            .as_ref()
            .is_none_or(|row| !self.marked.contains(&row.path))
        {
            self.marked.clear();
        }
        let item = row.as_ref().and_then(|row| self.item(&row.path));
        let mapped = item.is_some_and(|item| item.local.is_some());
        let is_sftp = self
            .server()
            .is_some_and(|choice| choice.kind == ServerType::Sftp);
        let excluded = item.is_some_and(|item| item.excluded);
        let explicitly_excluded = row
            .as_ref()
            .is_some_and(|row| self.explicitly_excluded(&row.path));
        let can_paste = self.can_paste();
        let focus_handle = self.focus_handle.clone();
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let focus = focus_handle.clone();
            let entry =
                move |label: &'static str, icon: Option<IconName>, action: Box<dyn Action>| {
                    menu_entry(label, icon, action, focus.clone())
                };
            let new_submenu = {
                let focus = focus_handle.clone();
                move |menu: ContextMenu| {
                    let focus = focus.clone();
                    menu.submenu_with_icon("New", IconName::Plus, move |menu, _, _| {
                        menu.context(focus.clone())
                            .item(menu_entry(
                                "File…",
                                Some(IconName::File),
                                Box::new(NewFile),
                                focus.clone(),
                            ))
                            .item(menu_entry(
                                "Folder…",
                                Some(IconName::Folder),
                                Box::new(NewFolder),
                                focus.clone(),
                            ))
                    })
                }
            };
            let menu = menu.context(focus_handle.clone());
            let Some(row) = row else {
                return new_submenu(menu)
                    .item(entry("Upload Here…", None, Box::new(UploadHere)))
                    .separator()
                    .item(entry("Paste", None, Box::new(Paste)).disabled(!can_paste))
                    .separator()
                    .item(entry(
                        "Refresh",
                        Some(IconName::RotateCw),
                        Box::new(Refresh),
                    ));
            };
            let menu = menu
                .when(!row.is_dir, |menu| {
                    menu.item(entry(
                        "Edit Remote File",
                        Some(IconName::Pencil),
                        Box::new(EditRemoteFile),
                    ))
                    .when(mapped, |menu| {
                        menu.item(entry("Edit Local File", None, Box::new(EditLocalFile)))
                            .item(entry(
                                "Compare with Local Version",
                                Some(IconName::Diff),
                                Box::new(CompareWithLocal),
                            ))
                    })
                })
                .when(row.is_dir && mapped, |menu| {
                    menu.item(entry(
                        "Sync with Local…",
                        Some(IconName::ArrowRightLeft),
                        Box::new(SyncWithLocal),
                    ))
                })
                .separator()
                .when(mapped, |menu| {
                    menu.item(entry(
                        "Download from Here",
                        Some(IconName::Download),
                        Box::new(DownloadToProject),
                    ))
                })
                .item(entry("Upload Here…", None, Box::new(UploadHere)))
                .separator();
            new_submenu(menu)
                .item(entry(
                    "Change Permissions…",
                    Some(IconName::Lock),
                    Box::new(ChangePermissions),
                ))
                .item(entry("Rename…", None, Box::new(Rename)))
                .item(entry("Delete…", Some(IconName::Trash), Box::new(Delete)))
                .separator()
                .item(entry("Cut", Some(IconName::Scissors), Box::new(Cut)))
                .item(entry("Copy", Some(IconName::Copy), Box::new(Copy)))
                .item(entry("Paste", None, Box::new(Paste)).disabled(!can_paste))
                .item(entry("Copy Absolute Path", None, Box::new(CopyPath)))
                .separator()
                .when(mapped, |menu| {
                    menu.item(entry(
                        "Open in Browser",
                        Some(IconName::ArrowUpRight),
                        Box::new(OpenInBrowser),
                    ))
                })
                .when(is_sftp, |menu| {
                    menu.item(entry(
                        "Start SSH Session Here",
                        Some(IconName::Terminal),
                        Box::new(StartSshSession),
                    ))
                })
                .map(|menu| {
                    if explicitly_excluded {
                        menu.item(entry("Include Path", None, Box::new(ToggleExcluded)))
                    } else if !excluded {
                        menu.item(entry("Exclude Path", None, Box::new(ToggleExcluded)))
                    } else {
                        menu
                    }
                })
                .item(entry(
                    "Refresh",
                    Some(IconName::RotateCw),
                    Box::new(Refresh),
                ))
        });
        self.show_menu(menu, position, window, cx);
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
        let options = self.options;
        let now = OffsetDateTime::now_utc().to_offset(self.local_offset);
        let server_kind = self.server().map(|choice| choice.kind);
        let server_address = self
            .server()
            .map(|choice| choice.address.clone())
            .unwrap_or_default();
        let mapping_roots = self
            .server()
            .and_then(|choice| {
                let targets = choice.engine.targets(Some(&choice.id)).ok()?;
                Some(targets.first()?.mapper.remote_roots())
            })
            .unwrap_or_default();
        let panel_settings = ProjectPanelSettings::get_global(cx);
        let spacing = match panel_settings.entry_spacing {
            ProjectPanelEntrySpacing::Comfortable => ListItemSpacing::Dense,
            ProjectPanelEntrySpacing::Standard => ListItemSpacing::ExtraDense,
        };
        range
            .filter_map(|index| {
                let row = self.rows.get(index)?.clone();
                let is_root = self.is_root(&row.path);
                let expanded = self.expanded.contains(&row.path);
                let loading = self.loading.contains(&row.path);
                let item = self.item(&row.path);
                let excluded = item.is_some_and(|item| item.excluded);
                let is_symlink = item.is_some_and(|item| item.stat.kind == EntryType::Symlink);
                let mapped = item.is_some_and(|item| item.local.is_some());
                let icon = if is_root {
                    server_icon(server_kind)
                } else if row.is_dir {
                    FileIcons::get_folder_icon(expanded, Path::new(&row.name), cx)
                        .map(Icon::from_path)
                        .unwrap_or_else(|| {
                            Icon::new(if expanded {
                                IconName::FolderOpen
                            } else {
                                IconName::Folder
                            })
                        })
                        .color(Color::Muted)
                } else {
                    FileIcons::get_icon(Path::new(&row.name), cx)
                        .map(Icon::from_path)
                        .unwrap_or_else(|| Icon::new(IconName::File))
                        .color(Color::Muted)
                };
                // Like the project panel: a chevron for folders, the same space left empty
                // for files so their icons line up.
                let chevron = row.is_dir.then(|| {
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
                let color = if excluded || cut_paths.contains(&row.path) {
                    Color::Disabled
                } else if is_symlink && options.highlight_symlinks {
                    Color::Accent
                } else if mapped && options.highlight_mappings && mapping_roots.contains(&row.path)
                {
                    Color::Created
                } else {
                    Color::Default
                };
                let details = item
                    .map(|item| entry_details(&item.stat, options, now, self.local_offset))
                    .unwrap_or_default();
                let dir = dir_of(&row);
                let click_row = row.clone();
                let menu_row = (!is_root).then(|| row.clone());
                let item = ListItem::new(("remote-host-row", index))
                    .indent_level(row.depth)
                    .indent_step_size(px(panel_settings.indent_size))
                    .spacing(spacing)
                    .toggle_state(
                        self.selected.as_deref() == Some(row.path.as_str())
                            || self.marked.contains(&row.path),
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
                            .gap_1p5()
                            .min_w_0()
                            .child(Label::new(row.name.clone()).color(color).truncate())
                            .when(is_root, |row| {
                                row.child(
                                    Label::new(server_address.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .single_line(),
                                )
                            }),
                    )
                    .when(loading || !details.is_empty(), |item| {
                        item.end_slot(
                            h_flex()
                                .gap_2()
                                .flex_none()
                                .pr_2()
                                .children(details.into_iter().map(|(text, _)| {
                                    Label::new(text)
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted)
                                        .single_line()
                                }))
                                .when(loading, |row| {
                                    row.child(
                                        Icon::new(IconName::ArrowCircle)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                    })
                    .on_click(
                        cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                            this.click_row(&click_row, event.modifiers(), window, cx)
                        }),
                    )
                    .on_secondary_mouse_down(cx.listener(
                        move |this, event: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            this.deploy_context_menu(event.position, menu_row.clone(), window, cx)
                        },
                    ));
                let dragged = server
                    .clone()
                    .filter(|_| !is_root)
                    .map(|server| DraggedRemote {
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
        let servers: Vec<(usize, SharedString, ServerType)> = self
            .servers
            .iter()
            .enumerate()
            .map(|(index, choice)| (index, choice.label.clone(), choice.kind))
            .collect();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (index, label, kind) in servers.clone() {
                let this = this.clone();
                menu = menu.custom_entry(
                    move |_, _| {
                        h_flex()
                            .gap_1p5()
                            .child(server_icon(Some(kind)).size(IconSize::Small))
                            .child(Label::new(label.clone()))
                            .into_any_element()
                    },
                    move |_, cx| {
                        this.update(cx, |this, cx| this.select_server(Some(index), cx))
                            .ok();
                    },
                );
            }
            menu
        });
        let label = self
            .server()
            .map(|choice| choice.label.clone())
            .unwrap_or_else(|| "<None>".into());
        let has_server = self.server().is_some();
        h_flex()
            .px_2()
            .py_1()
            .gap_1()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(DropdownMenu::new("remote-host-server", label, menu).full_width(true)),
            )
            .child(
                Button::new("remote-host-servers", "…")
                    .style(ButtonStyle::Outlined)
                    .tooltip(Tooltip::text("Deployment Servers…"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(OpenDeploymentServers.boxed_clone(), cx)
                    }),
            )
            .child(
                IconButton::new("remote-host-locate", IconName::Crosshair)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Select Opened File"))
                    .disabled(!has_server)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.select_opened_file(&SelectOpenedFile, window, cx)
                    })),
            )
            .child(
                IconButton::new("remote-host-collapse", IconName::ListCollapse)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Collapse All"))
                    .disabled(!has_server)
                    .on_click(cx.listener(|this, _, _, cx| this.collapse_all(cx))),
            )
            .child(
                IconButton::new("remote-host-refresh", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Refresh"))
                    .disabled(!has_server)
                    .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
            )
            .child(
                IconButton::new("remote-host-disconnect", IconName::Close)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Error)
                    .tooltip(Tooltip::text("Disconnect"))
                    .disabled(!has_server)
                    .on_click(cx.listener(|this, _, _, cx| this.disconnect(cx))),
            )
            .child(
                IconButton::new("remote-host-options", IconName::Ellipsis)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Options"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let position = window.mouse_position();
                        this.deploy_options_menu(position, window, cx)
                    })),
            )
    }

    fn collapse_all(&mut self, cx: &mut Context<Self>) {
        self.expanded.clear();
        if let Some(root) = self.root.clone() {
            self.expanded.insert(root);
        }
        self.update_rows();
        cx.notify();
    }

    /// Closes the connections to this project's servers; the next action reconnects.
    fn disconnect(&mut self, cx: &mut Context<Self>) {
        let Some(choice) = self.server().cloned() else {
            return;
        };
        let task = Tokio::spawn(cx, async move { choice.engine.disconnect_all().await });
        self._tasks.push(cx.spawn(async move |this, cx| {
            task.await.ok();
            this.update(cx, |this, cx| {
                this.error = Some("Disconnected.".into());
                cx.notify();
            })
            .ok();
        }));
    }

    fn set_options(&mut self, options: ViewOptions, cx: &mut Context<Self>) {
        self.options = options;
        let kvp = KeyValueStore::global(cx);
        let value = options.to_value();
        db::write_and_log(cx, move || async move {
            kvp.write_kvp(VIEW_OPTIONS_KEY.into(), value).await
        });
        cx.notify();
    }

    fn deploy_options_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let this = cx.weak_entity();
        let options = self.options;
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let toggle = |menu: ContextMenu,
                          label: &'static str,
                          on: bool,
                          set: fn(&mut ViewOptions, bool)| {
                let this = this.clone();
                menu.toggleable_entry(label, on, IconPosition::Start, None, move |_, cx| {
                    this.update(cx, |panel, cx| {
                        let mut options = panel.options;
                        set(&mut options, !on);
                        panel.set_options(options, cx);
                    })
                    .ok();
                })
            };
            let menu = toggle(menu, "Show Size", options.show_size, |o, v| o.show_size = v);
            let menu = toggle(menu, "Show Date", options.show_date, |o, v| o.show_date = v);
            let menu = toggle(
                menu,
                "Show Permissions",
                options.show_permissions,
                |o, v| o.show_permissions = v,
            );
            let menu = toggle(
                menu,
                "Highlight Symlinks",
                options.highlight_symlinks,
                |o, v| o.highlight_symlinks = v,
            );
            toggle(
                menu,
                "Highlight Mappings",
                options.highlight_mappings,
                |o, v| o.highlight_mappings = v,
            )
        });
        self.show_menu(menu, position, window, cx);
    }

    fn show_menu(
        &mut self,
        menu: Entity<ContextMenu>,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((menu, position, subscription));
        cx.notify();
    }

    /// The tree, plus the space below it, which takes drops and opens the root
    /// folder's context menu.
    fn render_tree(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let panel_settings = ProjectPanelSettings::get_global(cx);
        let indent_size = panel_settings.indent_size;
        let show_indent_guides = panel_settings.indent_guides.show == ShowIndentGuides::Always;
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
                .when(show_indent_guides, |list| {
                    list.with_decoration(
                        ui::indent_guides(px(indent_size), IndentGuideColors::panel(cx))
                            .with_compute_indents_fn(cx.entity(), |this, range, _, _| {
                                this.rows
                                    .get(range)
                                    .map(|rows| rows.iter().map(|row| row.depth).collect())
                                    .unwrap_or_default()
                            }),
                    )
                })
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

/// A context menu entry that runs `action` on the panel and shows its key binding.
fn menu_entry(
    label: &'static str,
    icon: Option<IconName>,
    action: Box<dyn Action>,
    focus: FocusHandle,
) -> ContextMenuEntry {
    let dispatch = action.boxed_clone();
    let entry = ContextMenuEntry::new(label)
        .action(action)
        .handler(move |window, cx| {
            window.focus(&focus, cx);
            window.dispatch_action(dispatch.boxed_clone(), cx);
        });
    match icon {
        Some(icon) => entry.icon(icon).icon_position(IconPosition::Start),
        None => entry,
    }
}

fn server_icon(kind: Option<ServerType>) -> Icon {
    match kind {
        Some(kind) => {
            let (icon, color) = crate::servers_modal::server_type_icon(kind);
            Icon::new(icon).color(color)
        }
        None => Icon::new(IconName::Server),
    }
}

/// The size, date and permissions shown after a name, per `options`.
/// Folders get permissions only, like PhpStorm.
fn entry_details(
    stat: &RemoteStat,
    options: ViewOptions,
    now: OffsetDateTime,
    offset: UtcOffset,
) -> Vec<(String, Color)> {
    let mut details = Vec::new();
    if !stat.is_dir() {
        if options.show_size {
            details.push((format_size(stat.size), Color::Muted));
        }
        if options.show_date && stat.mtime > 0 {
            details.push((format_mtime(stat.mtime, now, offset), Color::Default));
        }
    }
    if options.show_permissions
        && let Some(mode) = stat.mode
    {
        details.push((format_permissions(mode), Color::Muted));
    }
    details
}

/// PhpStorm-style sizes: "40 byte", "8.6 kB", "1.2 MB" (powers of 1000).
pub(crate) fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["kB", "MB", "GB", "TB"];
    if bytes < 1000 {
        return format!("{bytes} byte");
    }
    let mut value = bytes as f64 / 1000.0;
    let mut unit = 0;
    while value >= 1000.0 && unit < UNITS.len() - 1 {
        value /= 1000.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

/// "d/m/yy, h:mm AM" in local time, like PhpStorm's Remote Host.
pub(crate) fn format_mtime(mtime_ms: i64, _now: OffsetDateTime, offset: UtcOffset) -> String {
    let Ok(time) = OffsetDateTime::from_unix_timestamp_nanos(i128::from(mtime_ms) * 1_000_000)
    else {
        return String::new();
    };
    let time = time.to_offset(offset);
    let (hour, am_pm) = match time.hour() {
        0 => (12, "AM"),
        hour @ 1..=11 => (hour, "AM"),
        12 => (12, "PM"),
        hour => (hour - 12, "PM"),
    };
    format!(
        "{}/{}/{:02}, {}:{:02} {}",
        time.day(),
        u8::from(time.month()),
        time.year().rem_euclid(100),
        hour,
        time.minute(),
        am_pm
    )
}

/// "rwxr-xr-x" for the permission bits of `mode`.
pub(crate) fn format_permissions(mode: u32) -> String {
    (0..9)
        .map(|bit| {
            if mode & (1 << (8 - bit)) == 0 {
                '-'
            } else {
                ['r', 'w', 'x'][bit % 3]
            }
        })
        .collect()
}

/// Server types that have files to browse.
fn browsable(kind: ServerType) -> bool {
    kind.transfers() && kind != ServerType::Group
}

/// The folders from `root` down to the parent of `path`, which must be inside `root`.
pub(crate) fn folders_between(root: &str, path: &str) -> Vec<String> {
    let mut folders = Vec::new();
    let mut dir = remote_parent(path);
    while dir.len() >= root.len() && dir.starts_with(root.trim_end_matches('/')) {
        folders.push(dir.clone());
        if dir == root || dir == "/" {
            break;
        }
        dir = remote_parent(&dir);
    }
    folders.reverse();
    folders
}

/// `ssh` arguments that log in to `server` and open a shell in `dir`.
pub(crate) fn ssh_args(server: &deployment::config::Server, dir: &str) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(port) = server.port.filter(|port| *port != 22) {
        args.extend(["-p".to_string(), port.to_string()]);
    }
    if let Some(key) = server
        .private_key_path
        .as_ref()
        .filter(|key| !key.is_empty())
    {
        args.extend(["-i".to_string(), key.clone()]);
    }
    if let Some(jump) = server.jump_host.as_ref().filter(|jump| !jump.is_empty()) {
        args.extend(["-J".to_string(), jump.clone()]);
    }
    args.push("-t".into());
    args.push(if server.username.is_empty() {
        server.host.clone()
    } else {
        format!("{}@{}", server.username, server.host)
    });
    let quoted = dir.replace('\'', "'\\''");
    args.push(format!("cd '{quoted}' && exec $SHELL -l"));
    args
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
            v_flex()
                .flex_1()
                .size_full()
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
            .on_action(cx.listener(Self::edit_remote_file))
            .on_action(cx.listener(Self::edit_local_file))
            .on_action(cx.listener(Self::compare_with_local))
            .on_action(cx.listener(Self::sync_with_local))
            .on_action(cx.listener(Self::open_in_browser))
            .on_action(cx.listener(Self::start_ssh_session))
            .on_action(cx.listener(Self::select_opened_file))
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
    fn details_are_formatted_like_phpstorm() {
        assert_eq!(format_size(40), "40 byte");
        assert_eq!(format_size(8_600), "8.6 kB");
        assert_eq!(format_size(654_100), "654.1 kB");
        assert_eq!(format_size(2_500_000), "2.5 MB");
        assert_eq!(format_permissions(0o755), "rwxr-xr-x");
        assert_eq!(format_permissions(0o100600), "rw-------");
        let now = OffsetDateTime::now_utc();
        // 2025-01-08 13:05 UTC
        assert_eq!(
            format_mtime(1_736_341_500_000, now, UtcOffset::UTC),
            "8/1/25, 1:05 PM"
        );
        assert_eq!(
            format_mtime(1_736_294_400_000, now, UtcOffset::UTC),
            "8/1/25, 12:00 AM"
        );
    }

    #[test]
    fn view_options_round_trip() {
        let options = ViewOptions {
            show_permissions: true,
            highlight_mappings: false,
            ..ViewOptions::default()
        };
        assert_eq!(ViewOptions::from_value(&options.to_value()), options);
        assert_eq!(
            ViewOptions::from_value(""),
            ViewOptions {
                show_size: false,
                show_date: false,
                show_permissions: false,
                highlight_symlinks: false,
                highlight_mappings: false,
            }
        );
    }

    #[test]
    fn folders_to_reveal_a_file() {
        assert_eq!(
            folders_between("/site", "/site/a/b/c.php"),
            ["/site", "/site/a", "/site/a/b"]
        );
        assert_eq!(folders_between("/site", "/site/c.php"), ["/site"]);
        assert_eq!(folders_between("/", "/etc/hosts"), ["/", "/etc"]);
    }

    #[test]
    fn ssh_session_arguments() {
        let mut server = deployment::config::Server::new("s", "S", ServerType::Sftp);
        server.host = "example.com".into();
        server.username = "deploy".into();
        server.port = Some(2222);
        assert_eq!(
            ssh_args(&server, "/var/www/it's"),
            [
                "-p",
                "2222",
                "-t",
                "deploy@example.com",
                "cd '/var/www/it'\\''s' && exec $SHELL -l"
            ]
        );
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
