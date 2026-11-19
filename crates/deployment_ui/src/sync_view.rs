//! Sync with Deployed (D4): compares project files with a deployment server and
//! lets the user choose, file by file, which way each one goes before anything
//! is transferred. Compare opens the side-by-side diff for one file.

use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use deployment::engine::{Engine, Summary};
use deployment::sync::{CompareMode, SyncAction, SyncItem, SyncState, default_action};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, ListSizingBehavior, Render, Task,
    UniformListScrollHandle, Window, px, uniform_list,
};
use gpui_tokio::Tokio;
use ui::{Checkbox, ContextMenu, DropdownMenu, Tooltip, prelude::*};
use workspace::item::{Item, ItemEvent};

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

fn action_color(action: SyncAction) -> Color {
    match action {
        SyncAction::Upload | SyncAction::Download => Color::Accent,
        SyncAction::DeleteRemote | SyncAction::DeleteLocal => Color::Error,
        SyncAction::Skip => Color::Muted,
    }
}

fn state_label(state: SyncState) -> (&'static str, Color) {
    match state {
        SyncState::Same => ("Identical", Color::Muted),
        SyncState::LocalOnly => ("Only in project", Color::Created),
        SyncState::RemoteOnly => ("Only on server", Color::Created),
        SyncState::LocalNewer => ("Changed in project", Color::Modified),
        SyncState::RemoteNewer => ("Changed on server", Color::Modified),
        SyncState::Conflict => ("Changed on both sides", Color::Conflict),
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

pub(crate) struct SyncView {
    engine: Arc<Engine>,
    roots: Vec<PathBuf>,
    title: SharedString,
    /// The server compared with; `None` until the first comparison picks the default.
    server_id: Option<String>,
    mode: CompareMode,
    items: Vec<SyncItem>,
    /// Indices into `items` that are shown.
    visible: Vec<usize>,
    show_identical: bool,
    status: Status,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    _task: Option<Task<()>>,
}

impl SyncView {
    /// Compares `roots` (all mapped folders when empty) with the default server.
    pub(crate) fn new(
        engine: Arc<Engine>,
        roots: Vec<PathBuf>,
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
        let server_id = engine.default_server().map(|server| server.server.id);
        let mut this = Self {
            engine,
            roots,
            title: title.into(),
            server_id,
            mode: CompareMode::default(),
            items: Vec::new(),
            visible: Vec::new(),
            show_identical: false,
            status: Status::Comparing,
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            _task: None,
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

    fn update_visible(&mut self) {
        self.visible = (0..self.items.len())
            .filter(|&index| self.show_identical || self.items[index].state != SyncState::Same)
            .collect();
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
            this.update(cx, |this, cx| {
                match result {
                    Ok(Ok((server_id, items))) => {
                        this.server_id = Some(server_id);
                        this.items = items;
                        this.status = Status::Ready;
                    }
                    Ok(Err(error)) => this.status = Status::Failed(error.to_string().into()),
                    Err(error) => this.status = Status::Failed(error.to_string().into()),
                }
                this.update_visible();
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn synchronize(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(server_id) = self.server_id.clone() else {
            return;
        };
        let items: Vec<SyncItem> = self
            .items
            .iter()
            .filter(|item| item.action != SyncAction::Skip)
            .cloned()
            .collect();
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

    fn show_diff(&mut self, index: usize, cx: &mut Context<Self>) {
        let Some(item) = self.items.get(index) else {
            return;
        };
        let engine = self.engine.clone();
        let (server_id, local) = (self.server_id.clone(), item.local_path.clone());
        let task = Tokio::spawn(cx, async move {
            engine.compare(server_id.as_deref(), &local).await
        });
        cx.spawn(async move |this, cx| {
            let error = match task.await {
                Ok(Ok(_)) => return,
                Ok(Err(error)) => error.to_string(),
                Err(error) => error.to_string(),
            };
            this.update(cx, |this, cx| {
                this.status = Status::Failed(error.into());
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn busy(&self) -> bool {
        matches!(self.status, Status::Comparing | Status::Synchronizing)
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();
        let servers: Vec<(String, String)> = self
            .engine
            .servers()
            .into_iter()
            .filter(|resolved| resolved.server.kind.transfers())
            .map(|resolved| (resolved.server.id, resolved.server.name))
            .collect();
        let server_menu = ContextMenu::build(window, cx, {
            let this = this.clone();
            move |mut menu, _, _| {
                for (id, name) in servers.clone() {
                    let this = this.clone();
                    menu = menu.entry(name, None, move |window, cx| {
                        this.update(cx, |view, cx| {
                            view.server_id = Some(id.clone());
                            view.compare(window, cx);
                        })
                        .ok();
                    });
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

        h_flex()
            .flex_wrap()
            .gap_2()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(Label::new("Server").color(Color::Muted))
            .child(DropdownMenu::new(
                "sync-server",
                self.server_name(),
                server_menu,
            ))
            .child(Label::new("Compare by").color(Color::Muted))
            .child(DropdownMenu::new(
                "sync-mode",
                mode_label(self.mode),
                mode_menu,
            ))
            .child(DropdownMenu::new(
                "sync-directions",
                "Set Directions",
                direction_menu,
            ))
            .child(
                Checkbox::new("sync-show-identical", self.show_identical.into())
                    .label("Show identical")
                    .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                        this.show_identical = state.selected();
                        this.update_visible();
                        cx.notify();
                    })),
            )
            .child(div().flex_1())
            .child(
                IconButton::new("sync-refresh", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Compare Again"))
                    .disabled(busy)
                    .on_click(cx.listener(|this, _, window, cx| this.compare(window, cx))),
            )
            .child(
                Button::new(
                    "sync-run",
                    if pending == 0 {
                        "Synchronize".to_string()
                    } else {
                        format!(
                            "Synchronize {pending} File{}",
                            if pending == 1 { "" } else { "s" }
                        )
                    },
                )
                .style(ButtonStyle::Filled)
                .disabled(busy || pending == 0)
                .on_click(cx.listener(|this, _, window, cx| this.synchronize(window, cx))),
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
                } else {
                    "Everything is in sync."
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

    fn render_rows(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<AnyElement> {
        range
            .filter_map(|row| {
                let index = *self.visible.get(row)?;
                let item = self.items.get(index)?;
                let (state, state_color) = state_label(item.state);
                let both_sides = item.local.is_some() && item.remote.is_some();
                Some(
                    h_flex()
                        .id(("sync-row", index))
                        .w_full()
                        .gap_2()
                        .px_3()
                        .py_0p5()
                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                        .child(
                            div()
                                .w(px(150.))
                                .flex_none()
                                .child(Label::new(state).size(LabelSize::Small).color(state_color)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(Label::new(item.rel_path.clone()).truncate()),
                        )
                        .child(
                            IconButton::new(("sync-diff", index), IconName::Diff)
                                .icon_size(IconSize::Small)
                                .disabled(!both_sides)
                                .tooltip(Tooltip::text("Compare with Server"))
                                .on_click(
                                    cx.listener(move |this, _, _, cx| this.show_diff(index, cx)),
                                ),
                        )
                        .child(
                            div().w(px(140.)).flex_none().child(
                                Button::new(("sync-action", index), action_label(item.action))
                                    .label_size(LabelSize::Small)
                                    .color(action_color(item.action))
                                    .tooltip(Tooltip::text("Click to change"))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.cycle_action(index, cx)
                                    })),
                            ),
                        )
                        .into_any_element(),
                )
            })
            .collect()
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
            .child(self.render_toolbar(window, cx))
            .children(self.render_status())
            .child(
                uniform_list(
                    "sync-rows",
                    self.visible.len(),
                    cx.processor(|this, range: Range<usize>, _, cx| this.render_rows(range, cx)),
                )
                .with_sizing_behavior(ListSizingBehavior::Auto)
                .flex_1()
                .track_scroll(&self.scroll_handle),
            )
    }
}

impl Item for SyncView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Sync: {}", self.title).into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ArrowCircle))
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
    window: &mut Window,
    cx: &mut Context<workspace::Workspace>,
) -> Entity<SyncView> {
    let view = cx.new(|cx| SyncView::new(engine, roots, window, cx));
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
