//! TODO (IntelliJ's TODO tool window): every TODO and FIXME comment in the
//! project or the current file, grouped by file, and those comments
//! highlighted in the editor. The words are set with `todo.patterns`.

mod panel;
pub mod scan;

use editor::{Editor, EditorEvent, HighlightKey};
use gpui::{App, AppContext as _, Context, FontStyle, FontWeight, HighlightStyle, actions};
use language::BufferSnapshot;
use regex::Regex;
use settings::{RegisterSetting, Settings, SettingsStore};
use theme::ActiveTheme as _;
use workspace::Workspace;

pub use panel::TodoPanel;

actions!(
    todo,
    [
        /// Opens or focuses the TODO tool window.
        ToggleFocus,
        /// Searches the project for TODO items again.
        Refresh,
    ]
);

/// Files larger than this aren't highlighted (the scan reads the whole text).
const MAX_HIGHLIGHT_LEN: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, RegisterSetting)]
pub struct TodoSettings {
    pub patterns: Vec<String>,
    pub case_sensitive: bool,
    pub highlight: bool,
}

impl Settings for TodoSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let todo = content.todo.clone().unwrap_or_default();
        Self {
            patterns: todo.patterns.unwrap_or_else(|| {
                scan::DEFAULT_PATTERNS
                    .iter()
                    .map(|word| word.to_string())
                    .collect()
            }),
            case_sensitive: todo.case_sensitive.unwrap_or(false),
            highlight: todo.highlight.unwrap_or(true),
        }
    }
}

impl TodoSettings {
    pub fn matcher(&self) -> Option<Regex> {
        scan::matcher(&self.patterns, self.case_sensitive)
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let workspace_entity = cx.entity();
        let panel = cx.new(|cx| TodoPanel::new(workspace, workspace_entity, window, cx));
        workspace.add_panel(panel, window, cx);
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<TodoPanel>(window, cx);
        });
    })
    .detach();

    cx.observe_new(|editor: &mut Editor, window, cx| {
        if window.is_none() || !editor.mode().is_full() {
            return;
        }
        highlight(editor, cx);
        cx.subscribe_self(|editor, event: &EditorEvent, cx| {
            if matches!(event, EditorEvent::Reparsed(_)) {
                highlight(editor, cx);
            }
        })
        .detach();
        cx.observe_global::<SettingsStore>(highlight).detach();
    })
    .detach();
}

/// The TODO items of a buffer, with comments told apart by the language.
pub fn scan_buffer(snapshot: &BufferSnapshot, matcher: &Regex) -> Vec<scan::Found> {
    let text = snapshot.text();
    scan::scan(&text, matcher, |offset| in_comment(snapshot, &text, offset))
}

fn in_comment(snapshot: &BufferSnapshot, text: &str, offset: usize) -> bool {
    let Some(scope) = snapshot.language_scope_at(offset) else {
        return true;
    };
    if scope.override_name() == Some("comment") {
        return true;
    }
    let line_start = text[..offset].rfind('\n').map_or(0, |ix| ix + 1);
    let prefixes: Vec<&str> = scope
        .line_comment_prefixes()
        .iter()
        .map(|prefix| prefix.as_ref())
        .collect();
    let block_start = scope.block_comment().map(|block| block.start.as_ref());
    scan::looks_like_comment(&text[line_start..offset], &prefixes, block_start)
}

/// Colours the TODO items of a single-file editor.
fn highlight(editor: &mut Editor, cx: &mut Context<Editor>) {
    let settings = TodoSettings::get_global(cx).clone();
    let matcher = settings.highlight.then(|| settings.matcher()).flatten();
    let buffer = editor.buffer().read(cx).as_singleton();
    let (Some(matcher), Some(buffer)) = (matcher, buffer) else {
        editor.clear_highlights(HighlightKey::VegaTodo, cx);
        return;
    };
    let snapshot = buffer.read(cx).snapshot();
    if snapshot.len() > MAX_HIGHLIGHT_LEN {
        return;
    }
    let found = scan_buffer(&snapshot, &matcher);
    let multibuffer = editor.buffer().read(cx).snapshot(cx);
    let ranges = found
        .iter()
        .flat_map(|item| item.ranges.iter())
        .map(|range| {
            let start = snapshot.offset_to_point(range.start);
            let end = snapshot.offset_to_point(range.end);
            multibuffer.anchor_after(start)..multibuffer.anchor_before(end)
        })
        .collect::<Vec<_>>();
    if ranges.is_empty() {
        editor.clear_highlights(HighlightKey::VegaTodo, cx);
        return;
    }
    let style = HighlightStyle {
        color: Some(cx.theme().status().info),
        font_style: Some(FontStyle::Italic),
        font_weight: Some(FontWeight::MEDIUM),
        ..HighlightStyle::default()
    };
    editor.highlight_text(HighlightKey::VegaTodo, ranges, style, cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher};
    use std::sync::Arc;

    fn rust_like() -> Arc<Language> {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Test".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["test".into()],
                    ..LanguageMatcher::default()
                }
                .into(),
                line_comments: vec!["// ".into()],
                ..LanguageConfig::default()
            },
            None,
        ))
    }

    #[gpui::test]
    fn uses_the_language_comment_prefix(cx: &mut TestAppContext) {
        let text = "let todo = todo();\n// TODO: real one\n";
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(rust_like(), cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
        let matcher = scan::matcher(&["TODO".to_string()], false).unwrap();
        let found = scan_buffer(&snapshot, &matcher);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].text, "TODO: real one");
        assert_eq!(found[0].row, 1);
    }
}
