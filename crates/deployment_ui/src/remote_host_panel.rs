//! Remote Host (D3): a dock panel that browses a deployment server's files,
//! PhpStorm-style. Opening a file edits a local copy whose saves are uploaded
//! back (the store routes those saves to the engine that opened the copy).

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use collections::{HashMap, HashSet};
use deployment::config::ServerType;
use deployment::engine::{Engine, RemoteItem};
use futures::FutureExt as _;
use futures::future::BoxFuture;
use gpui::{
    Action, AnyElement, App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, MouseDownEvent, PathPromptOptions, Pixels, Point, Render, Subscription,
    Task, UniformListScrollHandle, WeakEntity, Window, anchored, deferred, px, uniform_list,
};
use gpui_tokio::Tokio;
use ui::{ContextMenu, DropdownMenu, ListItem, ListItemSpacing, Tooltip, prelude::*};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{OpenOptions, Workspace};
use zed_actions::deployment::{OpenDeploymentServers, ToggleRemoteHost};

use crate::name_modal::NameModal;
use crate::store::DeploymentStore;

/// A server the panel can browse: the engine (project folder) it belongs to
/// and its id within that engine's config.
#[derive(Clone)]
struct ServerChoice {
    engine: Arc<Engine>,
    id: String,
    label: SharedString,
}

/// One visible row of the tree.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Row {
    pub path: String,
    pub name: String,
    pub depth: usize,
    pub is_dir: bool,
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
        let mut this = Self {
            store,
            workspace,
            focus_handle: cx.focus_handle(),
            position: DockPosition::Right,
            servers: Vec::new(),
            selected_server: None,
            root: None,
            children: HashMap::default(),
            expanded: HashSet::default(),
            loading: HashSet::default(),
            rows: Vec::new(),
            selected: None,
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

        let previous = self
            .selected_server
            .and_then(|index| self.servers.get(index))
            .map(|choice| (choice.engine.root().to_path_buf(), choice.id.clone()));
        self.servers = servers;
        let keep = previous.and_then(|(root, id)| {
            self.servers
                .iter()
                .position(|choice| choice.engine.root() == root && choice.id == id)
        });
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
        if row.is_dir {
            self.toggle_dir(row.path.clone(), cx);
        } else {
            self.open(row.path.clone(), false, window, cx);
        }
        cx.notify();
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

    /// Runs a remote file operation, then reloads `reload` (a folder).
    fn run(
        &mut self,
        reload: String,
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
                this.expanded.insert(reload.clone());
                this.load_dir(reload, cx);
            })
            .ok();
        });
        self._tasks.push(task);
    }

    /// The folder new items go into: the row itself for a folder, else its parent.
    fn target_dir(&self, row: &Row) -> String {
        if row.is_dir {
            row.path.clone()
        } else {
            remote_parent(&row.path)
        }
    }

    fn new_item(&mut self, dir: String, folder: bool, window: &mut Window, cx: &mut Context<Self>) {
        let this = cx.weak_entity();
        let title = if folder { "New Folder" } else { "New File" };
        self.prompt_name(title, "", window, cx, move |name, window, cx| {
            this.update(cx, |this, cx| {
                let path = remote_join(&dir, &name);
                if folder {
                    this.run(
                        dir,
                        move |engine, id| {
                            async move { engine.remote_mkdir(&id, &path).await.map(drop) }.boxed()
                        },
                        cx,
                    );
                } else {
                    let open_path = path.clone();
                    this.run(
                        dir,
                        move |engine, id| {
                            async move { engine.remote_new_file(&id, &path).await.map(drop) }
                                .boxed()
                        },
                        cx,
                    );
                    this.open(open_path, true, window, cx);
                }
            })
            .ok();
        });
    }

    fn rename(&mut self, row: Row, window: &mut Window, cx: &mut Context<Self>) {
        let this = cx.weak_entity();
        self.prompt_name("Rename", &row.name, window, cx, move |name, _, cx| {
            this.update(cx, |this, cx| {
                let parent = remote_parent(&row.path);
                let to = remote_join(&parent, &name);
                let from = row.path;
                this.run(
                    parent,
                    move |engine, id| {
                        async move { engine.remote_rename(&id, &from, &to).await.map(drop) }.boxed()
                    },
                    cx,
                );
            })
            .ok();
        });
    }

    fn prompt_name(
        &self,
        title: &'static str,
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

    fn upload_here(&mut self, dir: String, cx: &mut Context<Self>) {
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
            this.update(cx, |this, cx| {
                this.run(
                    dir.clone(),
                    move |engine, id| {
                        async move {
                            engine
                                .remote_upload_here(&id, &locals, &dir)
                                .await
                                .map(drop)
                        }
                        .boxed()
                    },
                    cx,
                );
            })
            .ok();
        });
        self._tasks.push(task);
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        row: Row,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected = Some(row.path.clone());
        let mapped = self
            .item(&row.path)
            .is_some_and(|item| item.local.is_some());
        let this = cx.weak_entity();
        let dir = self.target_dir(&row);
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let entry = |menu: ContextMenu,
                         label: &'static str,
                         handler: Box<
                dyn Fn(&mut RemoteHostPanel, &mut Window, &mut Context<RemoteHostPanel>),
            >| {
                let this = this.clone();
                menu.entry(label, None, move |window, cx| {
                    this.update(cx, |panel, cx| handler(panel, window, cx)).ok();
                })
            };
            let mut menu = menu;
            if !row.is_dir {
                let path = row.path.clone();
                menu = entry(
                    menu,
                    "Open",
                    Box::new(move |panel, window, cx| panel.open(path.clone(), false, window, cx)),
                );
                if mapped {
                    let path = row.path.clone();
                    menu = entry(
                        menu,
                        "Open Server Copy",
                        Box::new(move |panel, window, cx| {
                            panel.open(path.clone(), true, window, cx)
                        }),
                    );
                }
            }
            if mapped {
                let path = row.path.clone();
                let parent = remote_parent(&row.path);
                menu = entry(
                    menu,
                    "Download to Project",
                    Box::new(move |panel, _, cx| {
                        let paths = vec![path.clone()];
                        panel.run(
                            parent.clone(),
                            move |engine, id| {
                                async move { engine.remote_download(&id, &paths).await.map(drop) }
                                    .boxed()
                            },
                            cx,
                        )
                    }),
                );
            }
            {
                let dir = dir.clone();
                menu = entry(
                    menu,
                    "Upload Here…",
                    Box::new(move |panel, _, cx| panel.upload_here(dir.clone(), cx)),
                );
            }
            menu = menu.separator();
            {
                let dir = dir.clone();
                menu = entry(
                    menu,
                    "New File…",
                    Box::new(move |panel, window, cx| {
                        panel.new_item(dir.clone(), false, window, cx)
                    }),
                );
            }
            {
                let dir = dir.clone();
                menu = entry(
                    menu,
                    "New Folder…",
                    Box::new(move |panel, window, cx| {
                        panel.new_item(dir.clone(), true, window, cx)
                    }),
                );
            }
            {
                let row = row.clone();
                menu = entry(
                    menu,
                    "Rename…",
                    Box::new(move |panel, window, cx| panel.rename(row.clone(), window, cx)),
                );
            }
            {
                let path = row.path.clone();
                let parent = remote_parent(&row.path);
                menu = entry(
                    menu,
                    "Delete",
                    Box::new(move |panel, _, cx| {
                        let paths = vec![path.clone()];
                        panel.run(
                            parent.clone(),
                            move |engine, id| {
                                async move { engine.remote_delete(&id, &paths).await.map(drop) }
                                    .boxed()
                            },
                            cx,
                        )
                    }),
                );
            }
            menu = menu.separator();
            {
                let path = row.path.clone();
                menu = entry(
                    menu,
                    "Copy Path",
                    Box::new(move |_, _, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(path.clone()))
                    }),
                );
            }
            entry(menu, "Refresh", Box::new(|panel, _, cx| panel.refresh(cx)))
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
                let color = if excluded {
                    Color::Disabled
                } else {
                    Color::Default
                };
                let click_row = row.clone();
                let menu_row = row.clone();
                Some(
                    ListItem::new(("remote-host-row", index))
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
                                    menu_row.clone(),
                                    window,
                                    cx,
                                )
                            },
                        ))
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
}

/// Server types that have files to browse.
fn browsable(kind: ServerType) -> bool {
    kind.transfers() && kind != ServerType::Group
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
                .child(
                    uniform_list(
                        "remote-host-rows",
                        self.rows.len(),
                        cx.processor(|this, range: Range<usize>, window, cx| {
                            this.render_rows(range, window, cx)
                        }),
                    )
                    .flex_1()
                    .size_full()
                    .track_scroll(&self.scroll_handle),
                )
                .into_any_element()
        };

        v_flex()
            .key_context("RemoteHostPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
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
}
