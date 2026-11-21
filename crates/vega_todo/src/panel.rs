//! The TODO tool window: TODO items grouped by file, for the whole project or
//! the current file (IntelliJ's "Project" and "Current File" tabs), and the
//! items a commit would add ("Changed Lines", shown after Check TODO).

use std::time::Duration;

use collections::HashSet;
use db::kvp::KeyValueStore;
use editor::Editor;
use futures::StreamExt as _;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render,
    Subscription, Task, UniformListScrollHandle, WeakEntity, Window, px, uniform_list,
};
use language::{Buffer, ParseStatus, Point};
use project::{
    Project, ProjectPath, SearchResults, WorktreeSettings,
    search::{SearchQuery, SearchResult},
};
use settings::{Settings as _, SettingsStore};
use ui::{HighlightedLabel, ListItem, Tooltip, prelude::*};
use util::ResultExt as _;
use util::paths::PathMatcher;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};

use crate::scan::Found;
use crate::{Refresh, TodoSettings, ToggleFocus, scan_buffer};

const DOCK_POSITION_KEY: &str = "vega_todo_dock_position";
/// Waits this long after files change before searching the project again.
const RESCAN_DELAY: Duration = Duration::from_millis(800);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scope {
    Project,
    CurrentFile,
    ChangedLines,
}

pub(crate) struct FileTodos {
    path: ProjectPath,
    /// Shown as the file's row: the file name, then its folder.
    pub(crate) name: SharedString,
    folder: SharedString,
    pub(crate) items: Vec<Found>,
}

#[derive(Clone, Copy)]
enum Row {
    File(usize),
    Item(usize, usize),
}

pub struct TodoPanel {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    position: DockPosition,
    scope: Scope,
    /// Project results, sorted by path.
    project_files: Vec<FileTodos>,
    /// The current file's results.
    current_file: Vec<FileTodos>,
    /// The items found by the last Check TODO before a commit.
    changed_lines: Vec<FileTodos>,
    collapsed: HashSet<ProjectPath>,
    rows: Vec<Row>,
    selected: Option<usize>,
    scanning: bool,
    limit_reached: bool,
    /// The project changed while the panel was hidden.
    stale: bool,
    visible: bool,
    scroll_handle: UniformListScrollHandle,
    scan_task: Task<()>,
    current_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl TodoPanel {
    pub(crate) fn new(
        workspace: &mut Workspace,
        workspace_entity: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = workspace.project().clone();
        let position = KeyValueStore::global(cx)
            .read_kvp(DOCK_POSITION_KEY)
            .log_err()
            .flatten()
            .and_then(|value| match value.as_str() {
                "left" => Some(DockPosition::Left),
                "right" => Some(DockPosition::Right),
                "bottom" => Some(DockPosition::Bottom),
                _ => None,
            })
            .unwrap_or(DockPosition::Bottom);

        let subscriptions = vec![
            cx.subscribe(&project, |this, _, event: &project::Event, cx| {
                if matches!(event, project::Event::WorktreeUpdatedEntries(..)) {
                    this.project_changed(cx);
                }
            }),
            cx.subscribe_in(
                &workspace_entity,
                window,
                |this, _, event: &workspace::Event, window, cx| {
                    if matches!(event, workspace::Event::ActiveItemChanged) {
                        this.refresh_current_file(window, cx);
                    }
                },
            ),
            cx.observe_global::<SettingsStore>(|this, cx| {
                this.stale = true;
                if this.visible {
                    this.refresh_project(cx);
                }
            }),
        ];

        Self {
            project,
            workspace: workspace.weak_handle(),
            focus_handle: cx.focus_handle(),
            position,
            scope: Scope::Project,
            project_files: Vec::new(),
            current_file: Vec::new(),
            changed_lines: Vec::new(),
            collapsed: HashSet::default(),
            rows: Vec::new(),
            selected: None,
            scanning: false,
            limit_reached: false,
            stale: true,
            visible: false,
            scroll_handle: UniformListScrollHandle::new(),
            scan_task: Task::ready(()),
            current_task: Task::ready(()),
            _subscriptions: subscriptions,
        }
    }

    fn files(&self) -> &[FileTodos] {
        match self.scope {
            Scope::Project => &self.project_files,
            Scope::CurrentFile => &self.current_file,
            Scope::ChangedLines => &self.changed_lines,
        }
    }

    /// Shows the items a commit was stopped for, in the "Changed Lines" tab.
    pub(crate) fn show_changed_lines(&mut self, files: Vec<FileTodos>, cx: &mut Context<Self>) {
        self.changed_lines = files;
        self.scope = Scope::ChangedLines;
        self.collapsed.clear();
        self.selected = None;
        self.rebuild_rows();
        cx.notify();
    }

    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();
        for (file_ix, file) in self.files().iter().enumerate() {
            rows.push(Row::File(file_ix));
            if !self.collapsed.contains(&file.path) {
                rows.extend((0..file.items.len()).map(|item_ix| Row::Item(file_ix, item_ix)));
            }
        }
        self.rows = rows;
        if self.selected.is_some_and(|ix| ix >= self.rows.len()) {
            self.selected = None;
        }
    }

    fn project_changed(&mut self, cx: &mut Context<Self>) {
        self.stale = true;
        if self.visible {
            self.refresh_project(cx);
        }
    }

    /// Searches the whole project (after a short delay, so a burst of file
    /// changes causes one search).
    fn refresh_project(&mut self, cx: &mut Context<Self>) {
        self.stale = false;
        self.scanning = true;
        cx.notify();
        let project = self.project.clone();
        self.scan_task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(RESCAN_DELAY).await;
            let (files, limit_reached) = search_project(project, cx).await;
            this.update(cx, |this, cx| {
                this.project_files = files;
                this.limit_reached = limit_reached;
                this.scanning = false;
                this.rebuild_rows();
                cx.notify();
            })
            .ok();
        });
    }

    /// Scans the active editor's file (the "Current File" view).
    fn refresh_current_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_item_as::<Editor>(cx));
        let Some(editor) = editor else {
            self.current_file.clear();
            self.rebuild_rows();
            cx.notify();
            return;
        };
        let reparsed = cx.subscribe_in(
            &editor,
            window,
            |this, _, event: &editor::EditorEvent, window, cx| {
                if matches!(event, editor::EditorEvent::Saved) {
                    this.refresh_current_file(window, cx);
                }
            },
        );
        let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() else {
            return;
        };
        self.current_task = cx.spawn(async move |this, cx| {
            let _reparsed = reparsed;
            let file = scan_file(buffer, cx).await;
            this.update(cx, |this, cx| {
                this.current_file = file.into_iter().collect();
                this.rebuild_rows();
                cx.notify();
            })
            .ok();
        });
    }

    fn toggle_file(&mut self, path: &ProjectPath, cx: &mut Context<Self>) {
        if !self.collapsed.remove(path) {
            self.collapsed.insert(path.clone());
        }
        self.rebuild_rows();
        cx.notify();
    }

    fn set_all_collapsed(&mut self, collapsed: bool, cx: &mut Context<Self>) {
        if collapsed {
            self.collapsed = self.files().iter().map(|file| file.path.clone()).collect();
        } else {
            self.collapsed.clear();
        }
        self.rebuild_rows();
        cx.notify();
    }

    fn open_row(&mut self, row_ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(row_ix);
        let Some(row) = self.rows.get(row_ix).copied() else {
            return;
        };
        let (file_ix, point) = match row {
            Row::File(file_ix) => {
                let path = self.files()[file_ix].path.clone();
                self.toggle_file(&path, cx);
                return;
            }
            Row::Item(file_ix, item_ix) => {
                let item = &self.files()[file_ix].items[item_ix];
                (file_ix, Point::new(item.row, item.column))
            }
        };
        let path = self.files()[file_ix].path.clone();
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let open = workspace.update(cx, |workspace, cx| {
            workspace.open_path(path, None, true, window, cx)
        });
        cx.spawn_in(window, async move |_, cx| {
            let item = open.await?;
            if let Some(editor) = item.downcast::<Editor>() {
                editor.update_in(cx, |editor, window, cx| {
                    editor.go_to_singleton_buffer_point(point, window, cx);
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        cx.notify();
    }

    fn summary(&self) -> String {
        let files = self.files();
        let items: usize = files.iter().map(|file| file.items.len()).sum();
        if self.scope == Scope::Project && self.scanning && files.is_empty() {
            return "Searching…".into();
        }
        let item_word = if items == 1 { "item" } else { "items" };
        let file_word = if files.len() == 1 { "file" } else { "files" };
        let mut summary = format!(
            "{items} TODO {item_word} found in {} {file_word}",
            files.len()
        );
        if self.scope == Scope::Project && self.limit_reached {
            summary.push_str(" (search limit reached)");
        }
        summary
    }

    fn render_row(&self, row_ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some(row) = self.rows.get(row_ix).copied() else {
            return div().into_any_element();
        };
        let selected = self.selected == Some(row_ix);
        let item = match row {
            Row::File(file_ix) => {
                let file = &self.files()[file_ix];
                let open = !self.collapsed.contains(&file.path);
                let count = file.items.len();
                ListItem::new(("todo-file", row_ix))
                    .toggle_state(selected)
                    .spacing(ui::ListItemSpacing::Dense)
                    .start_slot(
                        Icon::new(if open {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size(IconSize::Small)
                        .color(Color::Muted),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .min_w_0()
                            .child(Label::new(file.name.clone()))
                            .child(
                                Label::new(file.folder.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .truncate(),
                            )
                            .child(
                                Label::new(format!(
                                    "{count} {}",
                                    if count == 1 { "item" } else { "items" }
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            ),
                    )
            }
            Row::Item(file_ix, item_ix) => {
                let found = &self.files()[file_ix].items[item_ix];
                let keyword_len = found.keyword.len();
                ListItem::new(("todo-item", row_ix))
                    .toggle_state(selected)
                    .spacing(ui::ListItemSpacing::Dense)
                    .indent_level(1)
                    .indent_step_size(px(16.))
                    .start_slot(
                        Icon::new(IconName::ListTodo)
                            .size(IconSize::Small)
                            .color(Color::Info),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .min_w_0()
                            .child(
                                Label::new(format!("({}, {})", found.row + 1, found.column + 1))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                HighlightedLabel::new(
                                    found.text.clone(),
                                    (0..keyword_len)
                                        .filter(|ix| found.text.is_char_boundary(*ix))
                                        .collect(),
                                )
                                .color(Color::Info)
                                .truncate(),
                            ),
                    )
            }
        };
        item.on_click(cx.listener(move |this, _, window, cx| this.open_row(row_ix, window, cx)))
            .into_any_element()
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tab = |label: &'static str, scope: Scope, cx: &mut Context<Self>| {
            Button::new(label, label)
                .label_size(LabelSize::Small)
                .toggle_state(self.scope == scope)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.scope = scope;
                    if scope == Scope::CurrentFile {
                        this.refresh_current_file(window, cx);
                    }
                    this.rebuild_rows();
                    cx.notify();
                }))
        };
        h_flex()
            .px_2()
            .py_1()
            .gap_1()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(tab("Project", Scope::Project, cx))
            .child(tab("Current File", Scope::CurrentFile, cx))
            .when(
                !self.changed_lines.is_empty() || self.scope == Scope::ChangedLines,
                |toolbar| toolbar.child(tab("Changed Lines", Scope::ChangedLines, cx)),
            )
            .child(
                div().flex_1().min_w_0().pl_2().child(
                    Label::new(self.summary())
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .truncate(),
                ),
            )
            .child(
                IconButton::new("todo-refresh", IconName::RotateCw)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Refresh"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.refresh(window, cx);
                    })),
            )
            .child(
                IconButton::new("todo-expand", IconName::ChevronDown)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Expand All"))
                    .on_click(cx.listener(|this, _, _, cx| this.set_all_collapsed(false, cx))),
            )
            .child(
                IconButton::new("todo-collapse", IconName::ChevronUp)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Collapse All"))
                    .on_click(cx.listener(|this, _, _, cx| this.set_all_collapsed(true, cx))),
            )
    }

    fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.scope {
            Scope::Project => self.refresh_project(cx),
            Scope::CurrentFile => self.refresh_current_file(window, cx),
            Scope::ChangedLines => self.refresh_changed_lines(cx),
        }
    }

    /// Rescans the files of the "Changed Lines" tab and keeps the items on the
    /// rows they had (fixed items drop out).
    fn refresh_changed_lines(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().clone())
        else {
            return;
        };
        let previous: Vec<(ProjectPath, Vec<u32>)> = self
            .changed_lines
            .iter()
            .map(|file| {
                (
                    file.path.clone(),
                    file.items.iter().map(|item| item.row).collect(),
                )
            })
            .collect();
        self.current_task = cx.spawn(async move |this, cx| {
            let mut files = Vec::new();
            for (path, rows) in previous {
                let open = project.update(cx, |project, cx| project.open_buffer(path, cx));
                let Ok(buffer) = open.await else {
                    continue;
                };
                if let Some(mut file) = scan_file(buffer, cx).await {
                    file.items.retain(|item| rows.contains(&item.row));
                    if !file.items.is_empty() {
                        files.push(file);
                    }
                }
            }
            this.update(cx, |this, cx| {
                this.changed_lines = files;
                this.rebuild_rows();
                cx.notify();
            })
            .ok();
        });
    }

    fn select_next(&mut self, _: &menu::SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        if self.rows.is_empty() {
            return;
        }
        let next = self
            .selected
            .map_or(0, |ix| (ix + 1).min(self.rows.len() - 1));
        self.selected = Some(next);
        self.scroll_handle
            .scroll_to_item(next, gpui::ScrollStrategy::Center);
        cx.notify();
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.rows.is_empty() {
            return;
        }
        let previous = self.selected.map_or(0, |ix| ix.saturating_sub(1));
        self.selected = Some(previous);
        self.scroll_handle
            .scroll_to_item(previous, gpui::ScrollStrategy::Center);
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected {
            self.open_row(ix, window, cx);
        }
    }
}

/// Searches the project for the pattern words and scans each file that
/// matches. Returns the files with items, sorted by path, and whether the
/// search stopped at its limit.
async fn search_project(
    project: Entity<Project>,
    cx: &mut gpui::AsyncApp,
) -> (Vec<FileTodos>, bool) {
    let search = cx.update(|cx| {
        let settings = TodoSettings::get_global(cx);
        let pattern = crate::scan::words_pattern(&settings.patterns)?;
        let path_style = project.read(cx).path_style(cx);
        let worktree_settings = WorktreeSettings::get_global(cx);
        let exclude = PathMatcher::new(
            worktree_settings
                .file_scan_exclusions
                .sources()
                .chain(worktree_settings.private_files.sources()),
            path_style,
        )
        .log_err()?;
        let query = SearchQuery::regex(
            pattern,
            false,
            settings.case_sensitive,
            false,
            true,
            PathMatcher::default(),
            exclude,
            false,
            None,
        )
        .log_err()?;
        Some(project.update(cx, |project, cx| project.search(query, cx)))
    });
    let Some(SearchResults {
        rx,
        task_handle: _search,
    }) = search
    else {
        return (Vec::new(), false);
    };
    futures::pin_mut!(rx);
    let mut files = Vec::new();
    let mut limit_reached = false;
    while let Some(result) = rx.next().await {
        let buffer = match result {
            SearchResult::Buffer { buffer, .. } => buffer,
            SearchResult::LimitReached => {
                limit_reached = true;
                break;
            }
            SearchResult::WaitingForScan | SearchResult::Searching => continue,
        };
        if let Some(file) = scan_file(buffer, cx).await {
            files.push(file);
        }
    }
    files.sort_by(|a: &FileTodos, b: &FileTodos| {
        (a.folder.as_ref(), a.name.as_ref()).cmp(&(b.folder.as_ref(), b.name.as_ref()))
    });
    (files, limit_reached)
}

/// The TODO items of one buffer, once its syntax is parsed (comments are told
/// apart by the syntax tree). `None` when it has none or no file.
pub(crate) async fn scan_file(
    buffer: Entity<Buffer>,
    cx: &mut gpui::AsyncApp,
) -> Option<FileTodos> {
    let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status());
    while *parse_status.borrow() != ParseStatus::Idle {
        parse_status.changed().await.ok()?;
    }
    let (path, full_path, snapshot) = buffer.read_with(cx, |buffer, cx| {
        let file = buffer.file()?;
        Some((
            ProjectPath {
                worktree_id: file.worktree_id(cx),
                path: file.path().clone(),
            },
            file.full_path(cx),
            buffer.snapshot(),
        ))
    })?;
    let matcher = cx.update(|cx| TodoSettings::get_global(cx).matcher())?;
    let items = cx
        .background_spawn(async move { scan_buffer(&snapshot, &matcher) })
        .await;
    if items.is_empty() {
        return None;
    }
    let name = full_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let folder = full_path
        .parent()
        .map(|folder| folder.to_string_lossy().into_owned())
        .unwrap_or_default();
    Some(FileTodos {
        path,
        name: name.into(),
        folder: folder.into(),
        items,
    })
}

impl Focusable for TodoPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for TodoPanel {}

impl Render for TodoPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = if self.rows.is_empty() {
            let message = match (self.scope, self.scanning) {
                (Scope::Project, true) => "Searching for TODO items…",
                (Scope::Project, false) => "No TODO items found in the project.",
                (Scope::CurrentFile, _) => "No TODO items in the current file.",
                (Scope::ChangedLines, _) => "No TODO items left in the changed lines.",
            };
            v_flex()
                .flex_1()
                .p_3()
                .child(Label::new(message).color(Color::Muted))
                .into_any_element()
        } else {
            uniform_list(
                "todo-rows",
                self.rows.len(),
                cx.processor(|this, range: std::ops::Range<usize>, _, cx| {
                    range.map(|ix| this.render_row(ix, cx)).collect()
                }),
            )
            .flex_1()
            .size_full()
            .track_scroll(&self.scroll_handle)
            .into_any_element()
        };
        v_flex()
            .id("todo-panel")
            .key_context("TodoPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|this, _: &Refresh, window, cx| this.refresh(window, cx)))
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_toolbar(cx))
            .child(body)
    }
}

impl Panel for TodoPanel {
    fn persistent_name() -> &'static str {
        "TodoPanel"
    }

    fn panel_key() -> &'static str {
        "TodoPanel"
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        let value = match position {
            DockPosition::Left => "left",
            DockPosition::Right => "right",
            DockPosition::Bottom => "bottom",
        };
        let kvp = KeyValueStore::global(cx);
        db::write_and_log(cx, move || async move {
            kvp.write_kvp(DOCK_POSITION_KEY.into(), value.into()).await
        });
        cx.notify();
    }

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::ListTodo)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("TODO")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.visible = active;
        if active {
            if self.stale {
                self.refresh_project(cx);
            }
            // The workspace is mid-update when a panel is activated, so read
            // its active item once that update is done.
            cx.defer_in(window, |this, window, cx| {
                this.refresh_current_file(window, cx)
            });
        }
    }

    fn activation_priority(&self) -> u32 {
        22
    }
}
