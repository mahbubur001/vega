//! Sync with Deployed (D4): a folder comparison like PhpStorm's. A table lists
//! the files of both sides with an action each (→ ← × = ≠, click or Space to
//! change), filters by kind and name, and the selected file's diff below.

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use deployment::config::ServerType;
use deployment::engine::{Engine, Summary};
use deployment::sync::{CompareMode, SyncAction, SyncItem, SyncState, default_action};
use file_icons::FileIcons;
use git_ui_core::file_diff_view::{FileDiffView, build_buffer_diff};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Point,
    Render, ScrollStrategy, Subscription, Task, UniformListScrollHandle, WeakEntity, Window,
    actions, anchored, deferred, px, relative, uniform_list,
};
use gpui_tokio::Tokio;
use time::UtcOffset;
use ui::{
    ButtonLike, CommonAnimationExt as _, ContextMenu, ContextMenuEntry, DropdownMenu, IconPosition,
    KeyBinding, TintColor, Tooltip, prelude::*,
};
use ui_input::{ErasedEditorEvent, InputField};
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

use crate::servers_modal::server_type_icon;

actions!(
    deployment_sync,
    [
        /// Changes what happens to the selected file (upload, download, delete, nothing).
        CycleAction,
        /// Synchronizes the selected file.
        SynchronizeSelected,
        /// Runs every chosen action.
        SynchronizeAll,
    ]
);

/// Directions the toolbar can apply to every file at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bulk {
    /// What the comparison suggests (newer side wins, conflicts skipped).
    Suggested,
    /// Upload every file that differs or exists only in the project.
    Upload,
    /// Download every file that differs or exists only on the server.
    Download,
    /// Make the server match the project, deleting server-only files.
    MirrorToServer,
    /// Make the project match the server, deleting project-only files.
    MirrorToProject,
    /// Transfer nothing.
    Skip,
}

impl Bulk {
    const ALL: [Bulk; 6] = [
        Bulk::Suggested,
        Bulk::Upload,
        Bulk::Download,
        Bulk::MirrorToServer,
        Bulk::MirrorToProject,
        Bulk::Skip,
    ];

    fn label(self) -> &'static str {
        match self {
            Bulk::Suggested => "Suggested Directions",
            Bulk::Upload => "Upload All Differences",
            Bulk::Download => "Download All Differences",
            Bulk::MirrorToServer => "Mirror Project to Server",
            Bulk::MirrorToProject => "Mirror Server to Project",
            Bulk::Skip => "Skip All",
        }
    }
}

/// The action `bulk` gives a file in `state`.
pub(crate) fn bulk_action(state: SyncState, bulk: Bulk) -> SyncAction {
    use SyncState::*;
    match (bulk, state) {
        (_, Same) | (Bulk::Skip, _) => SyncAction::Skip,
        (Bulk::Suggested, state) => default_action(state),
        (Bulk::Upload, RemoteOnly) | (Bulk::Download, LocalOnly) => SyncAction::Skip,
        (Bulk::Upload | Bulk::MirrorToServer, _) if state != RemoteOnly => SyncAction::Upload,
        (Bulk::MirrorToServer, _) => SyncAction::DeleteRemote,
        (Bulk::Download | Bulk::MirrorToProject, _) if state != LocalOnly => SyncAction::Download,
        (Bulk::MirrorToProject, _) => SyncAction::DeleteLocal,
        (Bulk::Upload | Bulk::Download, _) => SyncAction::Skip,
    }
}

/// The actions a file in `state` can take, in the order a click cycles through them.
pub(crate) fn allowed_actions(state: SyncState) -> &'static [SyncAction] {
    match state {
        SyncState::LocalOnly => &[
            SyncAction::Upload,
            SyncAction::DeleteLocal,
            SyncAction::Skip,
        ],
        SyncState::RemoteOnly => &[
            SyncAction::Download,
            SyncAction::DeleteRemote,
            SyncAction::Skip,
        ],
        SyncState::Same => &[SyncAction::Skip, SyncAction::Upload, SyncAction::Download],
        SyncState::LocalNewer | SyncState::RemoteNewer | SyncState::Conflict => {
            &[SyncAction::Upload, SyncAction::Download, SyncAction::Skip]
        }
    }
}

/// The action after `current` for a file in `state`.
pub(crate) fn next_action(state: SyncState, current: SyncAction) -> SyncAction {
    let actions = allowed_actions(state);
    let index = actions.iter().position(|action| *action == current);
    match index {
        Some(index) => actions[(index + 1) % actions.len()],
        None => actions[0],
    }
}

fn action_label(action: SyncAction) -> &'static str {
    match action {
        SyncAction::Upload => "Upload →",
        SyncAction::Download => "← Download",
        SyncAction::DeleteRemote => "Delete on Server",
        SyncAction::DeleteLocal => "Delete in Project",
        SyncAction::Skip => "Skip",
    }
}

const MODES: [(CompareMode, &str); 5] = [
    (CompareMode::Smart, "Changes since last transfer"),
    (CompareMode::SizeAndTimestamp, "Size and timestamp"),
    (CompareMode::Size, "Size"),
    (CompareMode::Content, "Content"),
    (CompareMode::TextIgnoreEol, "Content, ignoring line endings"),
];

fn mode_label(mode: CompareMode) -> &'static str {
    MODES
        .iter()
        .find(|(candidate, _)| *candidate == mode)
        .map_or("", |(_, label)| label)
}

enum Status {
    Comparing,
    Ready,
    Synchronizing,
    Done(SharedString),
    Failed(SharedString),
}

/// The four groups the filter chips show and hide (PhpStorm's folder diff filters).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Category {
    NewInProject,
    Different,
    Equal,
    NewOnServer,
}

impl Category {
    const ALL: [Category; 4] = [
        Category::NewInProject,
        Category::Different,
        Category::Equal,
        Category::NewOnServer,
    ];

    pub(crate) fn of(state: SyncState) -> Self {
        match state {
            SyncState::LocalOnly => Category::NewInProject,
            SyncState::RemoteOnly => Category::NewOnServer,
            SyncState::Same => Category::Equal,
            SyncState::LocalNewer | SyncState::RemoteNewer | SyncState::Conflict => {
                Category::Different
            }
        }
    }

    fn label(self) -> &'static str {
        match self {
            Category::NewInProject => "New in project",
            Category::Different => "Different",
            Category::Equal => "Equal",
            Category::NewOnServer => "New on server",
        }
    }
}

/// Whether `path` matches the filter: words that must all appear, where `*`
/// stands for any characters (e.g. `*.php`).
pub(crate) fn matches_filter(path: &str, filter: &str) -> bool {
    let path = path.to_lowercase();
    filter.split_whitespace().all(|word| {
        let word = word.to_lowercase();
        let parts: Vec<&str> = word.split('*').collect();
        if parts.len() == 1 {
            return path.contains(&word);
        }
        let mut rest = path.as_str();
        for (index, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }
            let last = index == parts.len() - 1;
            if last {
                return rest.ends_with(part);
            }
            match rest.find(part) {
                Some(at) => rest = &rest[at + part.len()..],
                None => return false,
            }
        }
        true
    })
}

/// The action column's symbol: → upload, ← download, × delete, = equal, ≠ different.
fn action_symbol(state: SyncState, action: SyncAction) -> (&'static str, Color) {
    match action {
        SyncAction::Upload => ("→", Color::Created),
        SyncAction::Download => ("←", Color::Accent),
        SyncAction::DeleteRemote | SyncAction::DeleteLocal => ("×", Color::Error),
        SyncAction::Skip if state == SyncState::Same => ("=", Color::Muted),
        SyncAction::Skip => ("≠", Color::Warning),
    }
}

fn format_size(size: u64) -> String {
    crate::remote_host_panel::format_size(size)
}

pub(crate) struct SyncView {
    engine: Arc<Engine>,
    workspace: WeakEntity<Workspace>,
    roots: Vec<PathBuf>,
    title: SharedString,
    /// The server compared with; `None` until the first comparison picks the default.
    server_id: Option<String>,
    mode: CompareMode,
    items: Vec<SyncItem>,
    /// Indices into `items` that are shown.
    visible: Vec<usize>,
    shown: [bool; 4],
    filter: Entity<InputField>,
    selected: Option<usize>,
    /// The diff of the selected file: item index and the view (None while loading).
    diff: Option<(usize, Option<Entity<FileDiffView>>)>,
    diff_message: Option<SharedString>,
    local_offset: UtcOffset,
    status: Status,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
    _task: Option<Task<()>>,
    _diff_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl SyncView {
    /// Compares `roots` (all mapped folders when empty) with `server_id` or the default server.
    pub(crate) fn new(
        engine: Arc<Engine>,
        workspace: WeakEntity<Workspace>,
        roots: Vec<PathBuf>,
        server_id: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title = match roots.as_slice() {
            [root] => root
                .strip_prefix(engine.root())
                .ok()
                .filter(|relative| !relative.as_os_str().is_empty())
                .unwrap_or(
                    root.file_name()
                        .map_or(root.as_path(), std::path::Path::new),
                )
                .to_string_lossy()
                .into_owned(),
            _ => engine
                .root()
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
        };
        let server_id =
            server_id.or_else(|| engine.default_server().map(|server| server.server.id));
        let filter = cx.new(|cx| InputField::new(window, cx, "Filter, e.g. *.php"));
        let this_entity = cx.weak_entity();
        let editor = filter.read(cx).editor().clone();
        let filter_subscription = editor.subscribe(
            Box::new(move |event, _, cx| {
                if matches!(event, ErasedEditorEvent::BufferEdited) {
                    this_entity
                        .update(cx, |this, cx| {
                            this.update_visible(cx);
                            cx.notify();
                        })
                        .ok();
                }
            }),
            window,
            cx,
        );
        let mut this = Self {
            engine,
            workspace,
            roots,
            title: title.into(),
            server_id,
            mode: CompareMode::default(),
            items: Vec::new(),
            visible: Vec::new(),
            shown: [true, true, false, true],
            filter,
            selected: None,
            diff: None,
            diff_message: None,
            local_offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
            status: Status::Comparing,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            context_menu: None,
            _task: None,
            _diff_task: None,
            _subscriptions: vec![filter_subscription],
        };
        this.compare(window, cx);
        this
    }

    fn server_name(&self) -> SharedString {
        let id = self.server_id.as_deref();
        self.engine
            .servers()
            .into_iter()
            .find(|resolved| Some(resolved.server.id.as_str()) == id)
            .map(|resolved| resolved.server.name)
            .unwrap_or_else(|| "No server".into())
            .into()
    }

    fn update_visible(&mut self, cx: &App) {
        let filter = self.filter.read(cx).text(cx);
        self.visible = (0..self.items.len())
            .filter(|&index| {
                let item = &self.items[index];
                let category = Category::of(item.state);
                let shown = Category::ALL
                    .iter()
                    .position(|candidate| *candidate == category)
                    .is_some_and(|position| self.shown[position]);
                shown && matches_filter(&item.rel_path, &filter)
            })
            .collect();
    }

    fn count(&self, category: Category) -> usize {
        self.items
            .iter()
            .filter(|item| Category::of(item.state) == category)
            .count()
    }

    fn compare(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.status = Status::Comparing;
        let engine = self.engine.clone();
        let (server_id, roots, mode) = (self.server_id.clone(), self.roots.clone(), self.mode);
        let task = Tokio::spawn(cx, async move {
            engine
                .sync_compare(server_id.as_deref(), &roots, mode)
                .await
        });
        self._task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = task.await;
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(Ok((server_id, items))) => {
                        this.server_id = Some(server_id);
                        this.items = items;
                        this.status = Status::Ready;
                    }
                    Ok(Err(error)) => this.status = Status::Failed(error.to_string().into()),
                    Err(error) => this.status = Status::Failed(error.to_string().into()),
                }
                this.update_visible(cx);
                let first = this.visible.first().copied();
                this.select(first, window, cx);
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    /// Runs the chosen actions of `indices` (all items when None).
    fn synchronize(
        &mut self,
        indices: Option<Vec<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(server_id) = self.server_id.clone() else {
            return;
        };
        let items: Vec<SyncItem> = match indices {
            Some(indices) => indices
                .into_iter()
                .filter_map(|index| self.items.get(index))
                .filter(|item| item.action != SyncAction::Skip)
                .cloned()
                .collect(),
            None => self
                .items
                .iter()
                .filter(|item| item.action != SyncAction::Skip)
                .cloned()
                .collect(),
        };
        if items.is_empty() {
            return;
        }
        self.status = Status::Synchronizing;
        let engine = self.engine.clone();
        let task = Tokio::spawn(
            cx,
            async move { engine.sync_execute(&server_id, &items).await },
        );
        self._task = Some(cx.spawn_in(window, async move |this, cx| {
            let result = task.await;
            this.update_in(cx, |this, window, cx| {
                let server = this.server_name();
                match result {
                    Ok(Ok(summary)) if summary.cancelled && summary.transferred == 0 => {
                        this.status = Status::Ready;
                        cx.notify();
                    }
                    Ok(Ok(summary)) => {
                        let message = summary_message(&summary, &server);
                        this.compare(window, cx);
                        this.status = if summary.failed.is_empty() {
                            Status::Done(message.into())
                        } else {
                            Status::Failed(message.into())
                        };
                    }
                    Ok(Err(error)) => this.status = Status::Failed(error.to_string().into()),
                    Err(error) => this.status = Status::Failed(error.to_string().into()),
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn apply_bulk(&mut self, bulk: Bulk, cx: &mut Context<Self>) {
        for item in &mut self.items {
            item.action = bulk_action(item.state, bulk);
        }
        cx.notify();
    }

    fn cycle_action(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get_mut(index) {
            item.action = next_action(item.state, item.action);
            cx.notify();
        }
    }

    fn set_action(&mut self, index: usize, action: SyncAction, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get_mut(index)
            && allowed_actions(item.state).contains(&action)
        {
            item.action = action;
            cx.notify();
        }
    }

    fn busy(&self) -> bool {
        matches!(self.status, Status::Comparing | Status::Synchronizing)
    }

    /// Selects an item and shows its diff below the table.
    fn select(&mut self, index: Option<usize>, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = index;
        if let Some(row) = index.and_then(|index| self.visible.iter().position(|i| *i == index)) {
            self.scroll_handle
                .scroll_to_item(row, ScrollStrategy::Nearest);
        }
        self.load_diff(window, cx);
        cx.notify();
    }

    fn load_diff(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.diff_message = None;
        let Some(index) = self.selected else {
            self.diff = None;
            return;
        };
        if self.diff.as_ref().is_some_and(|(shown, _)| *shown == index) {
            return;
        }
        let Some(item) = self.items.get(index) else {
            return;
        };
        if item.local.is_none() || item.remote.is_none() {
            self.diff = None;
            self.diff_message = Some(
                if item.local.is_none() {
                    "Only on the server — choose ← to download it or × to delete it."
                } else {
                    "Only in the project — choose → to upload it or × to delete it."
                }
                .into(),
            );
            return;
        }
        self.diff = Some((index, None));
        let engine = self.engine.clone();
        let (server_id, local) = (self.server_id.clone(), item.local_path.clone());
        let fetch = Tokio::spawn(cx, {
            let local = local.clone();
            async move {
                engine
                    .fetch_deployed_copy(server_id.as_deref(), &local)
                    .await
            }
        });
        let workspace = self.workspace.clone();
        self._diff_task = Some(cx.spawn_in(window, async move |this, cx| {
            let remote = match fetch.await {
                Ok(Ok(remote)) => remote,
                Ok(Err(error)) => {
                    this.update(cx, |this, cx| {
                        this.diff = None;
                        this.diff_message = Some(error.to_string().into());
                        cx.notify();
                    })
                    .ok();
                    return;
                }
                Err(_) => return,
            };
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
            let old = project.update(cx, |project, cx| project.open_local_buffer(&remote, cx));
            let new = project.update(cx, |project, cx| project.open_local_buffer(&local, cx));
            let (Ok(old), Ok(new)) = (old.await, new.await) else {
                return;
            };
            let Ok(diff) = build_buffer_diff(&old, &new, cx).await else {
                return;
            };
            this.update_in(cx, |this, window, cx| {
                if this.selected != Some(index) {
                    return;
                }
                let view =
                    cx.new(|cx| FileDiffView::new(old, new, diff, project, workspace, window, cx));
                this.diff = Some((index, Some(view)));
                cx.notify();
            })
            .ok();
        }));
    }

    fn selected_row(&self) -> Option<usize> {
        let selected = self.selected?;
        self.visible.iter().position(|index| *index == selected)
    }

    fn select_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        let next = self.selected_row().map_or(0, |row| row + 1);
        if let Some(index) = self.visible.get(next).copied() {
            self.select(Some(index), window, cx);
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let previous = self.selected_row().map_or(0, |row| row.saturating_sub(1));
        if let Some(index) = self.visible.get(previous).copied() {
            self.select(Some(index), window, cx);
        }
    }

    fn cycle_selected(&mut self, _: &CycleAction, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected {
            self.cycle_action(index, cx);
        }
    }

    fn synchronize_selected(
        &mut self,
        _: &SynchronizeSelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.selected {
            self.synchronize(Some(vec![index]), window, cx);
        }
    }

    fn synchronize_all(&mut self, _: &SynchronizeAll, window: &mut Window, cx: &mut Context<Self>) {
        self.synchronize(None, window, cx);
    }

    fn deploy_row_menu(
        &mut self,
        index: usize,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select(Some(index), window, cx);
        let Some(state) = self.items.get(index).map(|item| item.state) else {
            return;
        };
        let this = cx.weak_entity();
        let menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for action in allowed_actions(state) {
                let this = this.clone();
                let action = *action;
                let label = match action {
                    SyncAction::Upload => "Copy to Server  →",
                    SyncAction::Download => "Copy to Project  ←",
                    SyncAction::DeleteRemote => "Delete on Server",
                    SyncAction::DeleteLocal => "Delete in Project",
                    SyncAction::Skip => "Do Nothing",
                };
                menu = menu.entry(label, None, move |_, cx| {
                    this.update(cx, |view, cx| view.set_action(index, action, cx))
                        .ok();
                });
            }
            menu.separator().entry("Synchronize This File", None, {
                let this = this.clone();
                move |window, cx| {
                    this.update(cx, |view, cx| {
                        view.synchronize(Some(vec![index]), window, cx)
                    })
                    .ok();
                }
            })
        });
        window.focus(&menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });
        self.context_menu = Some((menu, position, subscription));
        cx.notify();
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();
        let servers: Vec<(String, String, ServerType)> = self
            .engine
            .servers()
            .into_iter()
            .filter(|resolved| resolved.server.kind.transfers())
            .map(|resolved| {
                (
                    resolved.server.id,
                    resolved.server.name,
                    resolved.server.kind,
                )
            })
            .collect();
        let server_kind = servers
            .iter()
            .find(|(id, ..)| Some(id) == self.server_id.as_ref())
            .map(|(.., kind)| *kind);
        let server_menu = ContextMenu::build(window, cx, {
            let this = this.clone();
            move |mut menu, _, _| {
                for (id, name, kind) in servers.clone() {
                    let this = this.clone();
                    let (icon, color) = server_type_icon(kind);
                    menu = menu.item(
                        ContextMenuEntry::new(name)
                            .icon(icon)
                            .icon_color(color)
                            .icon_position(IconPosition::Start)
                            .handler(move |window, cx| {
                                this.update(cx, |view, cx| {
                                    view.server_id = Some(id.clone());
                                    view.diff = None;
                                    view.compare(window, cx);
                                })
                                .ok();
                            }),
                    );
                }
                menu
            }
        });
        let mode_menu = ContextMenu::build(window, cx, {
            let this = this.clone();
            move |mut menu, _, _| {
                for (mode, label) in MODES {
                    let this = this.clone();
                    menu = menu.entry(label, None, move |window, cx| {
                        this.update(cx, |view, cx| {
                            view.mode = mode;
                            view.compare(window, cx);
                        })
                        .ok();
                    });
                }
                menu
            }
        });
        let direction_menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for bulk in Bulk::ALL {
                let this = this.clone();
                menu = menu.entry(bulk.label(), None, move |_, cx| {
                    this.update(cx, |view, cx| view.apply_bulk(bulk, cx)).ok();
                });
            }
            menu
        });
        let pending = self
            .items
            .iter()
            .filter(|item| item.action != SyncAction::Skip)
            .count();
        let busy = self.busy();
        let focus = self.focus_handle.clone();

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .children(server_kind.map(|kind| {
                                let (icon, color) = server_type_icon(kind);
                                Icon::new(icon).size(IconSize::Small).color(color)
                            }))
                            .child(DropdownMenu::new(
                                "sync-server",
                                self.server_name(),
                                server_menu,
                            )),
                    )
                    .child(DropdownMenu::new(
                        "sync-mode",
                        format!("Compare: {}", mode_label(self.mode)),
                        mode_menu,
                    ))
                    .child(DropdownMenu::new(
                        "sync-directions",
                        "Set Directions",
                        direction_menu,
                    ))
                    .child(div().flex_1())
                    .child(
                        IconButton::new("sync-refresh", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Compare Again"))
                            .disabled(busy)
                            .on_click(cx.listener(|this, _, window, cx| this.compare(window, cx))),
                    )
                    .child(
                        Button::new("sync-selected", "Synchronize Selected")
                            .style(ButtonStyle::Outlined)
                            .disabled(busy || self.selected.is_none())
                            .key_binding(
                                KeyBinding::for_action_in(&SynchronizeSelected, &focus, cx)
                                    .map(|binding| binding.size(rems_from_px(12_f32))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.synchronize_selected(&SynchronizeSelected, window, cx)
                            })),
                    )
                    .child(
                        Button::new(
                            "sync-all",
                            if pending == 0 {
                                "Synchronize All".to_string()
                            } else {
                                format!("Synchronize All ({pending})")
                            },
                        )
                        .style(ButtonStyle::Tinted(TintColor::Accent))
                        .disabled(busy || pending == 0)
                        .key_binding(
                            KeyBinding::for_action_in(&SynchronizeAll, &focus, cx)
                                .map(|binding| binding.size(rems_from_px(12_f32))),
                        )
                        .on_click(
                            cx.listener(|this, _, window, cx| this.synchronize(None, window, cx)),
                        ),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .px_3()
                    .pb_2()
                    .children(
                        Category::ALL
                            .iter()
                            .enumerate()
                            .map(|(position, category)| {
                                let shown = self.shown[position];
                                Button::new(
                                    ("sync-filter", position),
                                    format!("{} · {}", category.label(), self.count(*category)),
                                )
                                .style(if shown {
                                    ButtonStyle::Tinted(TintColor::Accent)
                                } else {
                                    ButtonStyle::Subtle
                                })
                                .label_size(LabelSize::Small)
                                .toggle_state(shown)
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        this.shown[position] = !this.shown[position];
                                        this.update_visible(cx);
                                        cx.notify();
                                    },
                                ))
                            }),
                    )
                    .child(div().flex_1())
                    .child(div().w(rems(14.)).child(self.filter.clone())),
            )
    }

    fn render_status(&self) -> Option<impl IntoElement> {
        let (text, color): (SharedString, Color) = match &self.status {
            Status::Comparing => ("Comparing…".into(), Color::Muted),
            Status::Synchronizing => ("Synchronizing…".into(), Color::Muted),
            Status::Done(message) => (message.clone(), Color::Success),
            Status::Failed(message) => (message.clone(), Color::Error),
            Status::Ready if self.visible.is_empty() => {
                let text = if self.items.is_empty() {
                    "No files to compare."
                } else if self.count(Category::Equal) == self.items.len() {
                    "Everything is in sync."
                } else {
                    "Nothing matches the filters."
                };
                (text.into(), Color::Muted)
            }
            Status::Ready => return None,
        };
        Some(
            h_flex()
                .px_3()
                .py_1()
                .child(Label::new(text).size(LabelSize::Small).color(color)),
        )
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label =
            |text: &'static str| Label::new(text).size(LabelSize::Small).color(Color::Muted);
        h_flex()
            .px_3()
            .py_1()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(div().flex_1().child(label("Project")))
            .child(div().w(px(70.)).child(label("Size")))
            .child(div().w(px(120.)).child(label("Date")))
            .child(
                div()
                    .w(px(44.))
                    .flex()
                    .justify_center()
                    .child(label("Action")),
            )
            .child(div().w(px(70.)).child(label("Size")))
            .child(div().w(px(120.)).child(label("Date")))
            .child(div().flex_1().child(label("Server")))
    }

    fn render_rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let now = time::OffsetDateTime::now_utc();
        let offset = self.local_offset;
        let date = |mtime: i64| {
            if mtime > 0 {
                crate::remote_host_panel::format_mtime(mtime, now, offset)
            } else {
                String::new()
            }
        };
        range
            .filter_map(|row| {
                let index = *self.visible.get(row)?;
                let item = self.items.get(index)?;
                let (symbol, symbol_color) = action_symbol(item.state, item.action);
                let selected = self.selected == Some(index);
                let local_size = item.local.as_ref().map(|local| format_size(local.size));
                let local_date = item.local.as_ref().map(|local| date(local.mtime));
                let remote_size = item.remote.as_ref().map(|remote| format_size(remote.size));
                let remote_date = item.remote.as_ref().map(|remote| date(remote.mtime));
                let icon = FileIcons::get_icon(std::path::Path::new(&item.rel_path), cx)
                    .map(Icon::from_path)
                    .unwrap_or_else(|| Icon::new(IconName::File))
                    .size(IconSize::Small)
                    .color(Color::Muted);
                let name = |present: bool| {
                    if present {
                        h_flex()
                            .gap_1p5()
                            .min_w_0()
                            .child(icon.clone())
                            .child(Label::new(item.rel_path.clone()).truncate())
                            .into_any_element()
                    } else {
                        Label::new("—").color(Color::Disabled).into_any_element()
                    }
                };
                let small = |text: Option<String>| {
                    Label::new(text.unwrap_or_default())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .single_line()
                };
                Some(
                    h_flex()
                        .id(("sync-row", index))
                        .w_full()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .when(selected, |row| row.bg(cx.theme().colors().element_selected))
                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            window.focus(&this.focus_handle, cx);
                            this.select(Some(index), window, cx)
                        }))
                        .on_mouse_down(
                            gpui::MouseButton::Right,
                            cx.listener(move |this, event: &gpui::MouseDownEvent, window, cx| {
                                cx.stop_propagation();
                                this.deploy_row_menu(index, event.position, window, cx)
                            }),
                        )
                        .child(div().flex_1().min_w_0().child(name(item.local.is_some())))
                        .child(div().w(px(70.)).child(small(local_size)))
                        .child(div().w(px(120.)).child(small(local_date)))
                        .child(
                            div().w(px(44.)).flex().justify_center().child(
                                ButtonLike::new(("sync-action", index))
                                    .child(
                                        Label::new(symbol)
                                            .color(symbol_color)
                                            .weight(gpui::FontWeight::BOLD),
                                    )
                                    .tooltip(Tooltip::text(format!(
                                        "{} — click or Space to change",
                                        action_label(item.action)
                                    )))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.cycle_action(index, cx)
                                    })),
                            ),
                        )
                        .child(div().w(px(70.)).child(small(remote_size)))
                        .child(div().w(px(120.)).child(small(remote_date)))
                        .child(div().flex_1().min_w_0().child(name(item.remote.is_some())))
                        .into_any_element(),
                )
            })
            .collect()
    }

    fn render_diff(&self, cx: &mut Context<Self>) -> AnyElement {
        let body = match (&self.diff, &self.diff_message) {
            (Some((_, Some(view))), _) => view.clone().into_any_element(),
            (Some((_, None)), _) => h_flex()
                .p_3()
                .gap_2()
                .child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::Small)
                        .color(Color::Muted)
                        .with_rotate_animation(2),
                )
                .child(Label::new("Loading the server's version…").color(Color::Muted))
                .into_any_element(),
            (None, Some(message)) => h_flex()
                .p_3()
                .child(Label::new(message.clone()).color(Color::Muted))
                .into_any_element(),
            (None, None) => h_flex()
                .p_3()
                .child(Label::new("Select a file to see its differences.").color(Color::Muted))
                .into_any_element(),
        };
        let path = self
            .selected
            .and_then(|index| self.items.get(index))
            .map(|item| item.rel_path.clone())
            .unwrap_or_default();
        v_flex()
            .h(relative(0.45))
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .px_3()
                    .py_1()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Label::new(path).size(LabelSize::Small))
                    .child(div().flex_1())
                    .child(
                        Label::new(format!("Server ({}) ↔ Project", self.server_name()))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(div().flex_1().min_h_0().child(body))
            .into_any_element()
    }
}

fn summary_message(summary: &Summary, server: &str) -> String {
    let files = |n: u32| format!("{n} file{}", if n == 1 { "" } else { "s" });
    let mut message = format!("Synchronized {} with {server}", files(summary.transferred));
    if summary.skipped > 0 {
        message.push_str(&format!(", {} skipped", summary.skipped));
    }
    if let Some(failure) = summary.failed.first() {
        message.push_str(&format!(
            ", {} failed ({}: {})",
            summary.failed.len(),
            failure.path,
            failure.error
        ));
    }
    if summary.cancelled {
        message.push_str(", cancelled");
    }
    message
}

impl Focusable for SyncView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<ItemEvent> for SyncView {}

impl Render for SyncView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("DeploymentSync")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::cycle_selected))
            .on_action(cx.listener(Self::synchronize_selected))
            .on_action(cx.listener(Self::synchronize_all))
            .child(self.render_toolbar(window, cx))
            .children(self.render_status())
            .child(self.render_header(cx))
            .child(
                uniform_list(
                    "sync-rows",
                    self.visible.len(),
                    cx.processor(|this, range: Range<usize>, _, cx| this.render_rows(range, cx)),
                )
                .flex_1()
                .track_scroll(&self.scroll_handle),
            )
            .child(self.render_diff(cx))
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

impl Item for SyncView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Sync: {}", self.title).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ArrowRightLeft))
    }

    fn tab_tooltip_text(&self, _: &App) -> Option<SharedString> {
        Some(format!("Sync {} with {}", self.title, self.server_name()).into())
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }
}

/// Opens a Sync view for `roots` in the active pane.
pub(crate) fn open(
    workspace: &mut workspace::Workspace,
    engine: Arc<Engine>,
    roots: Vec<PathBuf>,
    server_id: Option<String>,
    window: &mut Window,
    cx: &mut Context<workspace::Workspace>,
) -> Entity<SyncView> {
    let weak = cx.weak_entity();
    let view = cx.new(|cx| SyncView::new(engine, weak, roots, server_id, window, cx));
    workspace.add_item_to_active_pane(Box::new(view.clone()), None, true, window, cx);
    view
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATES: [SyncState; 6] = [
        SyncState::Same,
        SyncState::LocalOnly,
        SyncState::RemoteOnly,
        SyncState::LocalNewer,
        SyncState::RemoteNewer,
        SyncState::Conflict,
    ];

    #[test]
    fn filters_by_words_and_wildcards() {
        assert!(matches_filter("includes/class-plugin.php", ""));
        assert!(matches_filter("includes/class-plugin.php", "*.php"));
        assert!(matches_filter(
            "includes/class-plugin.php",
            "INCLUDES plugin"
        ));
        assert!(matches_filter("includes/class-plugin.php", "inc*plugin*"));
        assert!(!matches_filter("assets/app.css", "*.php"));
        assert!(!matches_filter("includes/class-plugin.php", "*.php.bak"));
    }

    #[test]
    fn states_fall_into_the_filter_groups() {
        assert_eq!(Category::of(SyncState::LocalOnly), Category::NewInProject);
        assert_eq!(Category::of(SyncState::RemoteOnly), Category::NewOnServer);
        assert_eq!(Category::of(SyncState::Same), Category::Equal);
        assert_eq!(Category::of(SyncState::Conflict), Category::Different);
        assert_eq!(action_symbol(SyncState::Same, SyncAction::Skip).0, "=");
        assert_eq!(action_symbol(SyncState::Conflict, SyncAction::Skip).0, "≠");
    }

    #[test]
    fn bulk_directions() {
        let actions = |bulk| {
            STATES
                .iter()
                .map(|state| bulk_action(*state, bulk))
                .collect::<Vec<_>>()
        };
        use SyncAction::*;
        assert_eq!(
            actions(Bulk::Upload),
            [Skip, Upload, Skip, Upload, Upload, Upload]
        );
        assert_eq!(
            actions(Bulk::Download),
            [Skip, Skip, Download, Download, Download, Download]
        );
        assert_eq!(
            actions(Bulk::MirrorToServer),
            [Skip, Upload, DeleteRemote, Upload, Upload, Upload]
        );
        assert_eq!(
            actions(Bulk::MirrorToProject),
            [Skip, DeleteLocal, Download, Download, Download, Download]
        );
        assert_eq!(
            actions(Bulk::Suggested),
            [Skip, Upload, Download, Upload, Download, Skip]
        );
        assert_eq!(actions(Bulk::Skip), [Skip; 6]);
    }

    #[test]
    fn bulk_actions_are_allowed_for_their_state() {
        for state in STATES {
            for bulk in Bulk::ALL {
                assert!(
                    allowed_actions(state).contains(&bulk_action(state, bulk)),
                    "{bulk:?} gives {state:?} an action it can't cycle to"
                );
            }
        }
    }

    #[test]
    fn clicking_cycles_through_allowed_actions() {
        let state = SyncState::RemoteOnly;
        let mut action = SyncAction::Download;
        let mut seen = vec![action];
        for _ in 0..3 {
            action = next_action(state, action);
            seen.push(action);
        }
        assert_eq!(
            seen,
            [
                SyncAction::Download,
                SyncAction::DeleteRemote,
                SyncAction::Skip,
                SyncAction::Download
            ]
        );
        // An action the state doesn't allow restarts the cycle.
        assert_eq!(
            next_action(SyncState::LocalOnly, SyncAction::Download),
            SyncAction::Upload
        );
    }

    #[test]
    fn summary_messages() {
        let mut summary = Summary {
            transferred: 1,
            ..Default::default()
        };
        assert_eq!(
            summary_message(&summary, "Live"),
            "Synchronized 1 file with Live"
        );
        summary.transferred = 3;
        summary.skipped = 1;
        assert_eq!(
            summary_message(&summary, "Live"),
            "Synchronized 3 files with Live, 1 skipped"
        );
    }
}
