//! VEGA: an Editor / Editor and Preview / Preview switcher for Markdown files.
//!
//! The editor and the preview stay separate tabs. A layout is an arrangement of
//! those tabs across panes, so the current layout is derived from what is open
//! instead of being stored.

use std::sync::Arc;

use collections::{HashMap, HashSet};
use editor::{
    Bias, Editor, EditorEvent,
    display_map::{DisplayPoint, DisplayRow, ToDisplayPoint as _},
};
use gpui::{
    App, Context, Empty, Entity, EntityId, EventEmitter, Global, IntoElement, ParentElement as _,
    Render, Subscription, WeakEntity, Window, actions,
};
use language::{Buffer, BufferEvent, LanguageRegistry};
use markdown::MarkdownScrolled;
use markdown_preview::markdown_preview_view::MarkdownPreviewView;
use settings::{MarkdownDefaultLayout, RegisterSetting, Settings};
use ui::{ButtonStyle, IconButton, IconName, IconSize, Tooltip, prelude::*};
use workspace::{
    Item as _, Pane, SaveIntent, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    item::ItemHandle,
};

actions!(
    markdown_layout,
    [
        /// Shows only the Markdown source editor and closes its previews.
        ShowEditorOnly,
        /// Shows the Markdown source editor with its preview in the pane beside it.
        ShowEditorAndPreview,
        /// Shows only the Markdown preview in place of the source editor.
        ShowPreviewOnly,
    ]
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkdownLayout {
    EditorOnly,
    EditorAndPreview,
    PreviewOnly,
}

impl MarkdownLayout {
    const ALL: [Self; 3] = [Self::EditorOnly, Self::EditorAndPreview, Self::PreviewOnly];

    fn label(self) -> &'static str {
        match self {
            Self::EditorOnly => "Editor Only",
            Self::EditorAndPreview => "Editor and Preview",
            Self::PreviewOnly => "Preview Only",
        }
    }

    fn icon(self) -> IconName {
        match self {
            Self::EditorOnly => IconName::Code,
            Self::EditorAndPreview => IconName::Split,
            Self::PreviewOnly => IconName::FileTextOutlined,
        }
    }

    fn button_id(self) -> &'static str {
        match self {
            Self::EditorOnly => "markdown-layout-editor-only",
            Self::EditorAndPreview => "markdown-layout-editor-and-preview",
            Self::PreviewOnly => "markdown-layout-preview-only",
        }
    }

    fn action(self) -> Box<dyn gpui::Action> {
        match self {
            Self::EditorOnly => Box::new(ShowEditorOnly),
            Self::EditorAndPreview => Box::new(ShowEditorAndPreview),
            Self::PreviewOnly => Box::new(ShowPreviewOnly),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
struct MarkdownLayoutSettings {
    default_layout: MarkdownDefaultLayout,
    scroll_preview_with_editor: bool,
    scroll_editor_with_preview: bool,
}

impl Settings for MarkdownLayoutSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let markdown_preview = content.markdown_preview.as_ref();
        Self {
            default_layout: markdown_preview
                .and_then(|markdown_preview| markdown_preview.default_layout)
                .unwrap_or_default(),
            scroll_preview_with_editor: markdown_preview
                .and_then(|markdown_preview| markdown_preview.scroll_preview_with_editor)
                .unwrap_or(true),
            scroll_editor_with_preview: markdown_preview
                .and_then(|markdown_preview| markdown_preview.scroll_editor_with_preview)
                .unwrap_or(true),
        }
    }
}

/// Editors and previews that were already set up. A tab is "added" again when
/// it moves between panes or comes back from Preview Only; the default layout
/// must not override the layout the user chose since, and scroll sync must not
/// be subscribed twice.
#[derive(Default)]
struct KnownMarkdownTabs(HashSet<EntityId>);

impl Global for KnownMarkdownTabs {}

fn mark_known_markdown_tab<T: 'static>(tab: &Entity<T>, cx: &mut App) -> bool {
    let tab_id = tab.entity_id();
    if !cx.default_global::<KnownMarkdownTabs>().0.insert(tab_id) {
        return false;
    }
    cx.observe_release(tab, move |_, cx| {
        cx.default_global::<KnownMarkdownTabs>().0.remove(&tab_id);
    })
    .detach();
    true
}

fn apply_default_layout(
    workspace: &mut Workspace,
    buffer: &Entity<Buffer>,
    layout: MarkdownLayout,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let tabs = open_tabs(workspace, buffer, cx);
    // A second editor for the same file is a split made by the user.
    if tabs.editors.len() > 1 {
        return;
    }
    // The file shows as its editor, or as the preview that
    // `open_markdown_files_in_preview` has just put in the editor's place.
    let editor_tabs = tabs
        .editors
        .iter()
        .map(|(pane, editor)| (pane, editor.entity_id()));
    let preview_tabs = tabs
        .previews
        .iter()
        .map(|(pane, preview)| (pane, preview.entity_id()));
    let Some(pane) = editor_tabs
        .chain(preview_tabs)
        .find(|(pane, item_id)| is_active_in_pane(pane, *item_id, cx))
        .map(|(pane, _)| pane.clone())
    else {
        return;
    };
    set_layout(workspace, &pane, layout, window, cx);
}

struct EditorScrollTop {
    row: f64,
    source_index: usize,
}

fn editor_scroll_top(editor: &mut Editor, cx: &mut Context<Editor>) -> Option<EditorScrollTop> {
    let row = editor.scroll_position(cx).y.max(0.);
    let snapshot = editor.display_snapshot(cx);
    let point = snapshot.clip_point(DisplayPoint::new(DisplayRow(row as u32), 0), Bias::Left);
    let (_, buffer_offset) = snapshot
        .buffer_snapshot()
        .point_to_buffer_offset(point.to_offset(&snapshot, Bias::Left))?;
    Some(EditorScrollTop {
        row,
        source_index: buffer_offset.0,
    })
}

/// Rows an editor was scrolled to because its preview scrolled. The editor
/// reports that scroll like any other; it must not be sent back to the preview,
/// which would snap the preview to a line start under the user's hand.
#[derive(Default)]
struct EditorRowsSetByPreview(HashMap<EntityId, f64>);

impl Global for EditorRowsSetByPreview {}

/// Keeps the previews of `editor`'s file aligned with the top of the editor.
fn scroll_previews_with_editor(
    editor: &Entity<Editor>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let mut previous_row = None;
    cx.subscribe_in(
        editor,
        window,
        move |workspace, editor, event: &EditorEvent, _, cx| {
            let EditorEvent::ScrollPositionChanged { .. } = event else {
                return;
            };
            let Some(top) = editor.update(cx, editor_scroll_top) else {
                return;
            };
            let unchanged = previous_row == Some(top.row);
            previous_row = Some(top.row);
            let set_by_preview = cx
                .default_global::<EditorRowsSetByPreview>()
                .0
                .remove(&editor.entity_id())
                == Some(top.row);
            if unchanged
                || set_by_preview
                || !MarkdownLayoutSettings::get_global(cx).scroll_preview_with_editor
            {
                return;
            }
            let Some(buffer) = singleton_buffer(editor, cx) else {
                return;
            };
            let line_fraction = top.row.fract() as f32;
            for (_, preview) in open_tabs(workspace, &buffer, cx).previews {
                let markdown = preview.read(cx).markdown().clone();
                markdown.update(cx, |markdown, cx| {
                    markdown.scroll_source_index_to_top(top.source_index, line_fraction, cx);
                });
            }
        },
    )
    .detach();
}

/// Scrolls the source editor of `preview` when the preview is scrolled by hand.
fn scroll_editor_with_preview(
    preview: &Entity<MarkdownPreviewView>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let markdown = preview.read(cx).markdown().clone();
    let preview = preview.downgrade();
    cx.subscribe_in(
        &markdown,
        window,
        move |_, _, event: &MarkdownScrolled, window, cx| {
            if !MarkdownLayoutSettings::get_global(cx).scroll_editor_with_preview {
                return;
            }
            let Some(preview) = preview.upgrade() else {
                return;
            };
            let preview: &dyn ItemHandle = &preview;
            let Some(editor) = preview.act_as::<Editor>(cx) else {
                return;
            };
            let Some(buffer) = singleton_buffer(&editor, cx) else {
                return;
            };
            let top_point = {
                let buffer = buffer.read(cx);
                buffer.offset_to_point(event.top_source_index.min(buffer.len()))
            };
            let new_row = editor.update(cx, |editor, cx| {
                let snapshot = editor.display_snapshot(cx);
                let row = f64::from(top_point.to_display_point(&snapshot).row().0);
                let position = editor.scroll_position(cx);
                editor.set_scroll_position(gpui::point(position.x, row), window, cx);
                let new_row = editor.scroll_position(cx).y;
                (new_row != position.y).then_some(new_row)
            });
            if let Some(new_row) = new_row {
                cx.default_global::<EditorRowsSetByPreview>()
                    .0
                    .insert(editor.entity_id(), new_row);
            }
        },
    )
    .detach();
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        if let Some(window) = window {
            let workspace_entity = cx.entity();
            cx.subscribe_in(
                &workspace_entity,
                window,
                |workspace, _, event: &workspace::Event, window, cx| {
                    let workspace::Event::ItemAdded { item } = event else {
                        return;
                    };
                    if let Some(preview) = item.downcast::<MarkdownPreviewView>() {
                        if mark_known_markdown_tab(&preview, cx) {
                            scroll_editor_with_preview(&preview, window, cx);
                        }
                        return;
                    }
                    let Some(editor) = item.downcast::<Editor>() else {
                        return;
                    };
                    let Some(buffer) = singleton_buffer(&editor, cx) else {
                        return;
                    };
                    let languages = workspace.project().read(cx).languages().clone();
                    if !is_markdown_buffer(&buffer, &languages, cx)
                        || !mark_known_markdown_tab(&editor, cx)
                    {
                        return;
                    }
                    scroll_previews_with_editor(&editor, window, cx);
                    if workspace.is_restoring() {
                        return;
                    }
                    let layout = match MarkdownLayoutSettings::get_global(cx).default_layout {
                        MarkdownDefaultLayout::Editor => return,
                        MarkdownDefaultLayout::Split => MarkdownLayout::EditorAndPreview,
                        MarkdownDefaultLayout::Preview => MarkdownLayout::PreviewOnly,
                    };
                    // Deferred so it runs after every other `ItemAdded` handler,
                    // whichever order they were registered in.
                    cx.defer_in(window, move |workspace, window, cx| {
                        apply_default_layout(workspace, &buffer, layout, window, cx);
                    });
                },
            )
            .detach();
        }

        workspace.register_action(|workspace, _: &ShowEditorOnly, window, cx| {
            let pane = workspace.active_pane().clone();
            set_layout(workspace, &pane, MarkdownLayout::EditorOnly, window, cx);
        });
        workspace.register_action(|workspace, _: &ShowEditorAndPreview, window, cx| {
            let pane = workspace.active_pane().clone();
            set_layout(
                workspace,
                &pane,
                MarkdownLayout::EditorAndPreview,
                window,
                cx,
            );
        });
        workspace.register_action(|workspace, _: &ShowPreviewOnly, window, cx| {
            let pane = workspace.active_pane().clone();
            set_layout(workspace, &pane, MarkdownLayout::PreviewOnly, window, cx);
        });
    })
    .detach();
}

struct MarkdownSource {
    editor: Entity<Editor>,
    buffer: Entity<Buffer>,
}

#[derive(Default)]
struct OpenTabs {
    editors: Vec<(Entity<Pane>, Entity<Editor>)>,
    previews: Vec<(Entity<Pane>, Entity<MarkdownPreviewView>)>,
}

fn singleton_buffer(editor: &Entity<Editor>, cx: &App) -> Option<Entity<Buffer>> {
    editor.read(cx).buffer().read(cx).as_singleton()
}

fn is_markdown_buffer(
    buffer: &Entity<Buffer>,
    languages: &Arc<LanguageRegistry>,
    cx: &App,
) -> bool {
    let buffer = buffer.read(cx);
    if let Some(language) = buffer.language() {
        return language.name() == "Markdown";
    }
    // The language loads after the tab opens; fall back to the file name so the
    // switcher does not appear late.
    buffer.file().is_some_and(|file| {
        MarkdownPreviewView::is_markdown_path(file.path().as_std_path(), languages)
    })
}

fn markdown_source(
    item: &dyn ItemHandle,
    languages: &Arc<LanguageRegistry>,
    cx: &App,
) -> Option<MarkdownSource> {
    if item.downcast::<MarkdownPreviewView>().is_some() {
        let editor = item.act_as::<Editor>(cx)?;
        let buffer = singleton_buffer(&editor, cx)?;
        return Some(MarkdownSource { editor, buffer });
    }
    let editor = item.downcast::<Editor>()?;
    let buffer = singleton_buffer(&editor, cx)?;
    is_markdown_buffer(&buffer, languages, cx).then_some(MarkdownSource { editor, buffer })
}

fn open_tabs(workspace: &Workspace, buffer: &Entity<Buffer>, cx: &App) -> OpenTabs {
    let mut tabs = OpenTabs::default();
    for pane in workspace.panes() {
        for item in pane.read(cx).items() {
            if let Some(preview) = item.downcast::<MarkdownPreviewView>() {
                let previews_buffer = item
                    .act_as::<Editor>(cx)
                    .and_then(|editor| singleton_buffer(&editor, cx))
                    .is_some_and(|previewed| &previewed == buffer);
                if previews_buffer {
                    tabs.previews.push((pane.clone(), preview));
                }
            } else if let Some(editor) = item.downcast::<Editor>()
                && singleton_buffer(&editor, cx).as_ref() == Some(buffer)
            {
                tabs.editors.push((pane.clone(), editor));
            }
        }
    }
    tabs
}

fn is_active_in_pane(pane: &Entity<Pane>, item_id: gpui::EntityId, cx: &App) -> bool {
    pane.read(cx)
        .active_item()
        .is_some_and(|active| active.item_id() == item_id)
}

fn layout_for_source(
    workspace: &Workspace,
    pane: &Entity<Pane>,
    source: &MarkdownSource,
    cx: &App,
) -> MarkdownLayout {
    let tabs = open_tabs(workspace, &source.buffer, cx);
    let editor_visible = tabs
        .editors
        .iter()
        .any(|(pane, editor)| is_active_in_pane(pane, editor.entity_id(), cx));
    let preview_visible = tabs
        .previews
        .iter()
        .any(|(pane, preview)| is_active_in_pane(pane, preview.entity_id(), cx));
    if editor_visible && preview_visible {
        return MarkdownLayout::EditorAndPreview;
    }
    let shows_preview = pane
        .read(cx)
        .active_item()
        .is_some_and(|item| item.downcast::<MarkdownPreviewView>().is_some());
    if shows_preview {
        MarkdownLayout::PreviewOnly
    } else {
        MarkdownLayout::EditorOnly
    }
}

/// The layout the Markdown file shown in `pane` is in, or `None` when the
/// pane's active tab is not a Markdown editor or preview.
pub fn current_layout(
    workspace: &Workspace,
    pane: &Entity<Pane>,
    cx: &App,
) -> Option<MarkdownLayout> {
    let languages = workspace.project().read(cx).languages().clone();
    let item = pane.read(cx).active_item()?;
    let source = markdown_source(item.as_ref(), &languages, cx)?;
    Some(layout_for_source(workspace, pane, &source, cx))
}

/// Rearranges the editor and preview tabs of the Markdown file shown in
/// `origin_pane`. Does nothing when that pane's active tab is not Markdown.
pub fn set_layout(
    workspace: &mut Workspace,
    origin_pane: &Entity<Pane>,
    layout: MarkdownLayout,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let languages = workspace.project().read(cx).languages().clone();
    let Some(item) = origin_pane.read(cx).active_item() else {
        return;
    };
    let Some(source) = markdown_source(item.as_ref(), &languages, cx) else {
        return;
    };
    let tabs = open_tabs(workspace, &source.buffer, cx);
    match layout {
        MarkdownLayout::EditorOnly => {
            show_editor_tab(workspace, origin_pane, &source, &tabs, window, cx);
            for (pane, preview) in &tabs.previews {
                close_preview(pane, preview, window, cx);
            }
        }
        MarkdownLayout::EditorAndPreview => {
            let (editor_pane, editor) =
                show_editor_tab(workspace, origin_pane, &source, &tabs, window, cx);
            let mut side_previews = tabs
                .previews
                .iter()
                .filter(|(pane, _)| pane != &editor_pane);
            let side_preview = side_previews
                .clone()
                .find(|(pane, _)| pane == origin_pane)
                .or_else(|| side_previews.next());
            for (pane, preview) in &tabs.previews {
                let kept = side_preview.is_some_and(|(_, kept)| kept == preview);
                if !kept {
                    close_preview(pane, preview, window, cx);
                }
            }
            match side_preview {
                Some((pane, preview)) => pane.update(cx, |pane, cx| {
                    if let Some(index) = pane.index_for_item(preview) {
                        pane.activate_item(index, false, false, window, cx);
                    }
                }),
                None => MarkdownPreviewView::open_preview_to_the_side_of_pane(
                    workspace,
                    editor,
                    editor_pane,
                    window,
                    cx,
                ),
            }
        }
        MarkdownLayout::PreviewOnly => {
            if item.downcast::<MarkdownPreviewView>().is_none() {
                MarkdownPreviewView::open_preview_in_pane(
                    workspace,
                    source.editor.clone(),
                    origin_pane.clone(),
                    window,
                    cx,
                );
            }
            for (pane, preview) in &tabs.previews {
                if pane != origin_pane {
                    close_preview(pane, preview, window, cx);
                }
            }
            for (pane, editor) in &tabs.editors {
                // A preview tab shows no unsaved-changes marker and closes without
                // asking, so an editor with unsaved changes keeps its tab.
                if editor.read(cx).is_dirty(cx) {
                    continue;
                }
                pane.update(cx, |pane, cx| {
                    pane.remove_item(editor.entity_id(), false, true, window, cx);
                });
            }
        }
    }
    cx.notify();
}

fn show_editor_tab(
    workspace: &mut Workspace,
    origin_pane: &Entity<Pane>,
    source: &MarkdownSource,
    tabs: &OpenTabs,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> (Entity<Pane>, Entity<Editor>) {
    let existing = tabs
        .editors
        .iter()
        .find(|(pane, _)| pane == origin_pane)
        .or_else(|| tabs.editors.first())
        .cloned();
    if let Some((pane, editor)) = existing {
        workspace.activate_item(&editor, true, true, window, cx);
        return (pane, editor);
    }
    MarkdownPreviewView::suppress_auto_preview_for(&source.editor, cx);
    mark_known_markdown_tab(&source.editor, cx);
    workspace.add_item(
        origin_pane.clone(),
        Box::new(source.editor.clone()),
        None,
        true,
        true,
        window,
        cx,
    );
    (origin_pane.clone(), source.editor.clone())
}

fn close_preview(
    pane: &Entity<Pane>,
    preview: &Entity<MarkdownPreviewView>,
    window: &mut Window,
    cx: &mut App,
) {
    pane.update(cx, |pane, cx| {
        pane.close_item_by_id(preview.entity_id(), SaveIntent::Skip, window, cx)
    })
    .detach_and_log_err(cx);
}

pub struct MarkdownLayoutToolbar {
    workspace: WeakEntity<Workspace>,
    languages: Arc<LanguageRegistry>,
    active_item: Option<Box<dyn ItemHandle>>,
    _language_subscription: Option<Subscription>,
}

impl MarkdownLayoutToolbar {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        Self {
            workspace: workspace.weak_handle(),
            languages: workspace.project().read(cx).languages().clone(),
            active_item: None,
            _language_subscription: None,
        }
    }

    fn source(&self, cx: &App) -> Option<MarkdownSource> {
        markdown_source(self.active_item.as_deref()?, &self.languages, cx)
    }

    /// Whether the buttons show: the active item is a Markdown editor or preview.
    #[cfg(test)]
    fn is_shown(&self, cx: &App) -> bool {
        self.source(cx).is_some()
    }
}

/// Must run outside the toolbar item's own update: switching layouts changes the
/// pane's active tab, which calls back into every toolbar item of that pane.
fn set_layout_for_item(
    workspace: &WeakEntity<Workspace>,
    item: &dyn ItemHandle,
    layout: MarkdownLayout,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        return;
    };
    workspace.update(cx, |workspace, cx| {
        if let Some(pane) = workspace.pane_for(item) {
            set_layout(workspace, &pane, layout, window, cx);
        }
    });
}

impl EventEmitter<ToolbarItemEvent> for MarkdownLayoutToolbar {}

impl ToolbarItemView for MarkdownLayoutToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_item = active_pane_item.map(|item| item.boxed_clone());
        self._language_subscription = active_pane_item
            .and_then(|item| item.downcast::<Editor>())
            .and_then(|editor| singleton_buffer(&editor, cx))
            .map(|buffer| {
                cx.subscribe(&buffer, |_, _, event: &BufferEvent, cx| {
                    if matches!(event, BufferEvent::LanguageChanged(_)) {
                        cx.notify();
                    }
                })
            });
        cx.notify();
        // Drawn in the tab bar (`Pane::add_tab_bar_end_item`), like
        // PhpStorm; the toolbar only tells it which item is active.
        ToolbarItemLocation::Hidden
    }
}

impl Render for MarkdownLayoutToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = self.workspace.upgrade().and_then(|workspace| {
            let workspace = workspace.read(cx);
            let pane = workspace.pane_for(self.active_item.as_deref()?)?;
            let source = self.source(cx)?;
            Some(layout_for_source(workspace, &pane, &source, cx))
        });
        let (Some(current), Some(active_item)) = (current, self.active_item.as_ref()) else {
            return Empty.into_any_element();
        };

        h_flex()
            .gap_0p5()
            .children(MarkdownLayout::ALL.map(|layout| {
                IconButton::new(layout.button_id(), layout.icon())
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(layout == current)
                    .tooltip(move |_, cx| {
                        Tooltip::for_action(layout.label(), layout.action().as_ref(), cx)
                    })
                    .on_click({
                        let workspace = self.workspace.clone();
                        let item = active_item.boxed_clone();
                        move |_, window, cx| {
                            set_layout_for_item(&workspace, item.as_ref(), layout, window, cx);
                        }
                    })
            }))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{Focusable as _, TestAppContext, UpdateGlobal as _, WindowHandle};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace};

    struct TestWorkspace {
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: WindowHandle<MultiWorkspace>,
    }

    async fn markdown_workspace(cx: &mut TestAppContext) -> TestWorkspace {
        cx.update(|cx| {
            cx.set_global(db::AppDatabase::test_new());
            AppState::test(cx);
            editor::init(cx);
            markdown_preview::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/project"), json!({ "guide.md": "# Guide\n" }))
            .await;
        let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
        project
            .read_with(cx, |project, _| project.languages().clone())
            .add(Arc::new(language::Language::new(
                language::LanguageConfig {
                    name: "Markdown".into(),
                    matcher: Arc::new(language::LanguageMatcher {
                        path_suffixes: vec!["md".into()],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                None,
            )));
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .update(cx, |multi_workspace, _, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        TestWorkspace {
            project,
            workspace,
            window,
        }
    }

    impl TestWorkspace {
        async fn open_guide(&self, cx: &mut TestAppContext) {
            let project_path = self.project.read_with(cx, |project, cx| ProjectPath {
                worktree_id: project.worktrees(cx).next().unwrap().read(cx).id(),
                path: rel_path("guide.md").into(),
            });
            let open = self
                .window
                .update(cx, |_, window, cx| {
                    self.workspace.update(cx, |workspace, cx| {
                        workspace.open_path(project_path, None, true, window, cx)
                    })
                })
                .unwrap();
            open.await.unwrap();
            cx.run_until_parked();
        }

        fn set_layout(&self, layout: MarkdownLayout, cx: &mut TestAppContext) {
            self.window
                .update(cx, |_, window, cx| {
                    self.workspace.update(cx, |workspace, cx| {
                        let pane = workspace.active_pane().clone();
                        set_layout(workspace, &pane, layout, window, cx);
                    })
                })
                .unwrap();
            cx.run_until_parked();
        }

        /// The active tab of every pane, as `E` for an editor and `P` for a preview.
        fn visible_tabs(&self, cx: &mut TestAppContext) -> String {
            self.workspace.read_with(cx, |workspace, cx| {
                workspace
                    .panes()
                    .iter()
                    .map(|pane| match pane.read(cx).active_item() {
                        Some(item) if item.downcast::<MarkdownPreviewView>().is_some() => "P",
                        Some(item) if item.downcast::<Editor>().is_some() => "E",
                        Some(_) => "?",
                        None => "-",
                    })
                    .collect::<Vec<_>>()
                    .join("|")
            })
        }

        fn tab_count(&self, cx: &mut TestAppContext) -> usize {
            self.workspace.read_with(cx, |workspace, cx| {
                workspace
                    .panes()
                    .iter()
                    .map(|pane| pane.read(cx).items().count())
                    .sum()
            })
        }

        fn layout(&self, cx: &mut TestAppContext) -> Option<MarkdownLayout> {
            self.workspace.read_with(cx, |workspace, cx| {
                current_layout(workspace, workspace.active_pane(), cx)
            })
        }

        fn active_editor(&self, cx: &mut TestAppContext) -> Entity<Editor> {
            self.workspace.read_with(cx, |workspace, cx| {
                workspace.active_item_as::<Editor>(cx).unwrap()
            })
        }
    }

    #[gpui::test]
    async fn switches_between_the_three_layouts(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.layout(cx), Some(MarkdownLayout::EditorOnly));

        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.layout(cx), Some(MarkdownLayout::EditorAndPreview));

        test.set_layout(MarkdownLayout::PreviewOnly, cx);
        assert_eq!(test.visible_tabs(cx), "P");
        assert_eq!(test.tab_count(cx), 1);
        assert_eq!(test.layout(cx), Some(MarkdownLayout::PreviewOnly));

        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.tab_count(cx), 2);

        test.set_layout(MarkdownLayout::EditorOnly, cx);
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.tab_count(cx), 1);
        assert_eq!(test.layout(cx), Some(MarkdownLayout::EditorOnly));
    }

    #[gpui::test]
    async fn choosing_the_current_layout_changes_nothing(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        test.open_guide(cx).await;

        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.tab_count(cx), 2);

        test.set_layout(MarkdownLayout::PreviewOnly, cx);
        test.set_layout(MarkdownLayout::PreviewOnly, cx);
        assert_eq!(test.visible_tabs(cx), "P");
        assert_eq!(test.tab_count(cx), 1);
    }

    #[gpui::test]
    async fn preview_only_keeps_an_editor_with_unsaved_changes(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        test.open_guide(cx).await;
        let editor = test.active_editor(cx);
        test.window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| editor.insert("unsaved ", window, cx));
            })
            .unwrap();

        test.set_layout(MarkdownLayout::PreviewOnly, cx);

        assert_eq!(test.visible_tabs(cx), "P");
        assert_eq!(test.tab_count(cx), 2);
    }

    #[gpui::test]
    async fn editor_only_survives_open_markdown_files_in_preview(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .markdown_preview
                        .get_or_insert_default()
                        .open_markdown_files_in_preview = Some(true);
                });
            });
        });
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "P");
        assert_eq!(test.layout(cx), Some(MarkdownLayout::PreviewOnly));

        test.set_layout(MarkdownLayout::EditorOnly, cx);
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.tab_count(cx), 1);

        test.set_layout(MarkdownLayout::PreviewOnly, cx);
        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.tab_count(cx), 2);
    }

    fn set_default_layout(default_layout: MarkdownDefaultLayout, cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .markdown_preview
                        .get_or_insert_default()
                        .default_layout = Some(default_layout);
                });
            });
        });
    }

    fn set_open_in_preview(cx: &mut TestAppContext) {
        cx.update(|cx| {
            settings::SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .markdown_preview
                        .get_or_insert_default()
                        .open_markdown_files_in_preview = Some(true);
                });
            });
        });
    }

    #[gpui::test]
    async fn default_layout_split_opens_editor_and_preview(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        set_default_layout(MarkdownDefaultLayout::Split, cx);
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.tab_count(cx), 2);

        // The default applies once; the user's later choice stays.
        test.set_layout(MarkdownLayout::PreviewOnly, cx);
        test.set_layout(MarkdownLayout::EditorOnly, cx);
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.tab_count(cx), 1);
    }

    #[gpui::test]
    async fn default_layout_preview_opens_the_preview_alone(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        set_default_layout(MarkdownDefaultLayout::Preview, cx);
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "P");
        assert_eq!(test.tab_count(cx), 1);

        test.set_layout(MarkdownLayout::EditorOnly, cx);
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.tab_count(cx), 1);
    }

    #[gpui::test]
    async fn default_layout_editor_leaves_the_editor_alone(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        set_default_layout(MarkdownDefaultLayout::Editor, cx);
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "E");
        assert_eq!(test.tab_count(cx), 1);
    }

    #[gpui::test]
    async fn default_layout_split_wins_over_open_in_preview(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        set_open_in_preview(cx);
        set_default_layout(MarkdownDefaultLayout::Split, cx);
        test.open_guide(cx).await;
        assert_eq!(test.visible_tabs(cx), "E|P");
        assert_eq!(test.tab_count(cx), 2);
    }

    #[gpui::test]
    async fn scrolling_syncs_editor_and_preview_both_ways(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        test.open_guide(cx).await;
        let editor = test.active_editor(cx);
        test.window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    let lines = (0..400)
                        .map(|line| format!("paragraph {line}\n\n"))
                        .collect::<String>();
                    editor.insert(&lines, window, cx);
                    // A new preview scrolls to the cursor, so start it at the top.
                    editor.move_to_beginning(&editor::actions::MoveToBeginning, window, cx);
                });
            })
            .unwrap();
        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        cx.executor()
            .advance_clock(std::time::Duration::from_millis(500));
        cx.run_until_parked();
        let preview = test.workspace.read_with(cx, |workspace, cx| {
            workspace
                .items_of_type::<MarkdownPreviewView>(cx)
                .next()
                .unwrap()
        });
        let editor_row = |cx: &mut TestAppContext| {
            test.window
                .update(cx, |_, _, cx| {
                    editor.update(cx, |editor, cx| editor.scroll_position(cx).y)
                })
                .unwrap()
        };
        let page_down_in_preview = |cx: &mut TestAppContext| {
            test.window
                .update(cx, |_, window, cx| {
                    preview.focus_handle(cx).dispatch_action(
                        &markdown_preview::ScrollPageDown,
                        window,
                        cx,
                    );
                })
                .unwrap();
            cx.run_until_parked();
        };

        // Preview to editor: a page down in the preview moves the editor.
        assert_eq!(editor_row(cx), 0.);
        page_down_in_preview(cx);
        let row_after_first_page = editor_row(cx);
        assert!(
            row_after_first_page > 0.,
            "editor did not follow the preview"
        );

        // Editor to preview: after the editor jumps far down, a page down in the
        // preview continues from there, which it only can if the preview followed.
        test.window
            .update(cx, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_scroll_position(gpui::point(0., 400.), window, cx);
                });
            })
            .unwrap();
        cx.run_until_parked();
        assert_eq!(editor_row(cx), 400.);
        page_down_in_preview(cx);
        assert!(
            editor_row(cx) > 400.,
            "preview did not follow the editor: row {}",
            editor_row(cx)
        );
    }

    #[gpui::test]
    async fn toolbar_buttons_switch_layouts(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        let toolbar_item = test
            .window
            .update(cx, |_, window, cx| {
                self::add_toolbar_item(&test.workspace, window, cx)
            })
            .unwrap();
        test.open_guide(cx).await;

        for (layout, visible_tabs) in [
            (MarkdownLayout::EditorAndPreview, "E|P"),
            (MarkdownLayout::PreviewOnly, "P"),
            (MarkdownLayout::EditorOnly, "E"),
        ] {
            test.window
                .update(cx, |_, window, cx| {
                    let (workspace, item) = toolbar_item.read_with(cx, |toolbar_item, _| {
                        (
                            toolbar_item.workspace.clone(),
                            toolbar_item.active_item.as_ref().unwrap().boxed_clone(),
                        )
                    });
                    set_layout_for_item(&workspace, item.as_ref(), layout, window, cx);
                })
                .unwrap();
            cx.run_until_parked();
            assert_eq!(test.visible_tabs(cx), visible_tabs);
            assert_eq!(
                toolbar_item.read_with(cx, |toolbar_item, cx| toolbar_item.is_shown(cx)),
                true
            );
        }
    }

    fn add_toolbar_item(
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<MarkdownLayoutToolbar> {
        workspace.update(cx, |workspace, cx| {
            let pane = workspace.active_pane().clone();
            let toolbar_item = cx.new(|cx| MarkdownLayoutToolbar::new(workspace, cx));
            pane.update(cx, |pane, cx| {
                pane.toolbar().update(cx, |toolbar, cx| {
                    toolbar.add_item(toolbar_item.clone(), window, cx);
                });
            });
            toolbar_item
        })
    }

    #[gpui::test]
    async fn other_files_have_no_layout(cx: &mut TestAppContext) {
        let test = markdown_workspace(cx).await;
        assert_eq!(test.layout(cx), None);
        test.set_layout(MarkdownLayout::EditorAndPreview, cx);
        assert_eq!(test.tab_count(cx), 0);
    }
}
