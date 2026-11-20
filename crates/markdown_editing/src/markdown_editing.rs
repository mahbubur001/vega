//! VEGA: editing commands for Markdown source: format toggles, links, pasting
//! a URL over a selection, and moving through table cells.

mod text_edits;

use std::ops::Range;

use editor::{Editor, MultiBufferOffset, SelectionEffects};
use gpui::{Action, App, Context, WeakEntity, Window, actions};
use text_edits::{CellDirection, InlineStyle, Replacement};

actions!(
    markdown_editing,
    [
        /// Makes the selection bold, or removes the bold.
        ToggleBold,
        /// Makes the selection italic, or removes the italic.
        ToggleItalic,
        /// Strikes through the selection, or removes the strikethrough.
        ToggleStrikethrough,
        /// Marks the selection as inline code, or removes the marks.
        ToggleCode,
        /// Turns the selection into a link.
        InsertLink,
        /// Pastes; a URL pasted over selected text becomes a link on that text.
        Paste,
        /// Moves to the next cell of the table under the cursor.
        NextTableCell,
        /// Moves to the previous cell of the table under the cursor.
        PreviousTableCell,
        /// Adds an empty row below the table row under the cursor.
        InsertTableRow,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|editor: &mut Editor, _, cx| {
        if !editor.mode().is_full() {
            return;
        }
        let handle = cx.entity().downgrade();
        register::<ToggleBold>(editor, &handle, |editor, window, cx| {
            toggle_inline_style(editor, InlineStyle::Bold, window, cx)
        });
        register::<ToggleItalic>(editor, &handle, |editor, window, cx| {
            toggle_inline_style(editor, InlineStyle::Italic, window, cx)
        });
        register::<ToggleStrikethrough>(editor, &handle, |editor, window, cx| {
            toggle_inline_style(editor, InlineStyle::Strikethrough, window, cx)
        });
        register::<ToggleCode>(editor, &handle, |editor, window, cx| {
            toggle_inline_style(editor, InlineStyle::Code, window, cx)
        });
        register::<InsertLink>(editor, &handle, insert_link);
        register::<Paste>(editor, &handle, paste);
        register::<NextTableCell>(editor, &handle, |editor, window, cx| {
            move_table_cell(editor, CellDirection::Next, window, cx)
        });
        register::<PreviousTableCell>(editor, &handle, |editor, window, cx| {
            move_table_cell(editor, CellDirection::Previous, window, cx)
        });
        register::<InsertTableRow>(editor, &handle, insert_table_row);
    })
    .detach();
}

/// Registers `run` for `A`. When `run` returns false the action is left for
/// the next handler or key binding, so that keys such as tab keep their usual
/// meaning outside the cases these commands cover.
fn register<A: Action>(
    editor: &mut Editor,
    handle: &WeakEntity<Editor>,
    run: fn(&mut Editor, &mut Window, &mut Context<Editor>) -> bool,
) {
    let handle = handle.clone();
    editor
        .register_action(move |_: &A, window, cx| {
            let handled = handle
                .update(cx, |editor, cx| run(editor, window, cx))
                .unwrap_or(false);
            if !handled {
                cx.propagate();
            }
        })
        .detach();
}

struct MarkdownText {
    text: String,
    selections: Vec<Range<usize>>,
}

fn markdown_text(editor: &mut Editor, cx: &mut Context<Editor>) -> Option<MarkdownText> {
    if editor.read_only(cx) {
        return None;
    }
    let buffer = editor.buffer().read(cx).as_singleton()?;
    let buffer = buffer.read(cx);
    if buffer.language()?.name() != "Markdown" {
        return None;
    }
    let text = buffer.text();
    let snapshot = editor.display_snapshot(cx);
    let selections = editor
        .selections
        .all::<MultiBufferOffset>(&snapshot)
        .into_iter()
        .map(|selection| selection.start.0..selection.end.0)
        .collect();
    Some(MarkdownText { text, selections })
}

fn apply(
    editor: &mut Editor,
    replacements: Vec<Replacement>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let applied = text_edits::combine(replacements);
    let multi_buffer_range =
        |range: Range<usize>| MultiBufferOffset(range.start)..MultiBufferOffset(range.end);
    editor.transact(window, cx, |editor, window, cx| {
        editor.edit(
            applied
                .edits
                .into_iter()
                .map(|(range, text)| (multi_buffer_range(range), text)),
            cx,
        );
        editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
            selections.select_ranges(applied.selections.into_iter().map(multi_buffer_range));
        });
    });
}

fn toggle_inline_style(
    editor: &mut Editor,
    style: InlineStyle,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let Some(markdown) = markdown_text(editor, cx) else {
        return false;
    };
    let replacements = markdown
        .selections
        .iter()
        .map(|selection| text_edits::toggle_inline_style(&markdown.text, selection.clone(), style))
        .collect();
    apply(editor, replacements, window, cx);
    true
}

fn insert_link(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) -> bool {
    let Some(markdown) = markdown_text(editor, cx) else {
        return false;
    };
    let replacements = markdown
        .selections
        .iter()
        .map(|selection| text_edits::insert_link(&markdown.text, selection.clone()))
        .collect();
    apply(editor, replacements, window, cx);
    true
}

fn paste(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) -> bool {
    let Some(markdown) = markdown_text(editor, cx) else {
        return false;
    };
    let Some(item) = cx.read_from_clipboard() else {
        return true;
    };
    let links = item
        .text()
        .map(|text| text.trim().to_string())
        .filter(|url| text_edits::is_url(url))
        .and_then(|url| {
            markdown
                .selections
                .iter()
                .map(|selection| {
                    text_edits::link_from_pasted_url(&markdown.text, selection.clone(), &url)
                })
                .collect::<Option<Vec<_>>>()
        });
    match links {
        Some(links) => apply(editor, links, window, cx),
        None => editor.paste_item(&item, window, cx),
    }
    true
}

/// The cursor, when these table commands apply: one cursor, and no completion
/// menu that tab or enter should answer instead.
fn table_cursor(editor: &Editor, markdown: &MarkdownText) -> Option<usize> {
    if editor.context_menu_visible() {
        return None;
    }
    match markdown.selections.as_slice() {
        [selection] => Some(selection.start),
        _ => None,
    }
}

fn move_table_cell(
    editor: &mut Editor,
    direction: CellDirection,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    let Some(markdown) = markdown_text(editor, cx) else {
        return false;
    };
    let Some(target) = table_cursor(editor, &markdown)
        .and_then(|cursor| text_edits::table_cell_target(&markdown.text, cursor, direction))
    else {
        return false;
    };
    editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
        selections.select_ranges([MultiBufferOffset(target.start)..MultiBufferOffset(target.end)]);
    });
    true
}

fn insert_table_row(editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>) -> bool {
    let Some(markdown) = markdown_text(editor, cx) else {
        return false;
    };
    let Some(row) = table_cursor(editor, &markdown)
        .and_then(|cursor| text_edits::table_row_insertion(&markdown.text, cursor))
    else {
        return false;
    };
    apply(editor, vec![row], window, cx);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, ClipboardItem, Entity, TestAppContext, WindowHandle};
    use language::{Buffer, Language, LanguageConfig};
    use std::sync::Arc;

    struct TestEditor {
        editor: Entity<Editor>,
        window: WindowHandle<Editor>,
    }

    fn markdown_editor(
        text: &str,
        language_name: &'static str,
        cx: &mut TestAppContext,
    ) -> TestEditor {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let language = Arc::new(Language::new(
            LanguageConfig {
                name: language_name.into(),
                ..Default::default()
            },
            None,
        ));
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        let window = cx.add_window(|window, cx| Editor::for_buffer(buffer, None, window, cx));
        let editor = window.root(cx).unwrap();
        TestEditor { editor, window }
    }

    impl TestEditor {
        fn select(&self, range: Range<usize>, cx: &mut TestAppContext) {
            self.window
                .update(cx, |editor, window, cx| {
                    editor.change_selections(
                        SelectionEffects::default(),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([
                                MultiBufferOffset(range.start)..MultiBufferOffset(range.end)
                            ]);
                        },
                    );
                })
                .unwrap();
        }

        fn run(
            &self,
            command: fn(&mut Editor, &mut Window, &mut Context<Editor>) -> bool,
            cx: &mut TestAppContext,
        ) -> bool {
            self.window
                .update(cx, |editor, window, cx| command(editor, window, cx))
                .unwrap()
        }

        fn text(&self, cx: &mut TestAppContext) -> String {
            self.editor.read_with(cx, |editor, cx| editor.text(cx))
        }

        fn selection(&self, cx: &mut TestAppContext) -> Range<usize> {
            self.window
                .update(cx, |editor, _, cx| {
                    let snapshot = editor.display_snapshot(cx);
                    let selection = editor.selections.newest::<MultiBufferOffset>(&snapshot);
                    selection.start.0..selection.end.0
                })
                .unwrap()
        }
    }

    #[gpui::test]
    fn toggles_bold_and_undoes_in_one_step(cx: &mut TestAppContext) {
        let test = markdown_editor("some words here", "Markdown", cx);
        test.select(5..10, cx);

        assert!(test.run(
            |editor, window, cx| toggle_inline_style(editor, InlineStyle::Bold, window, cx),
            cx
        ));
        assert_eq!(test.text(cx), "some **words** here");
        assert_eq!(test.selection(cx), 7..12);

        test.window
            .update(cx, |editor, window, cx| {
                editor.undo(&editor::actions::Undo, window, cx)
            })
            .unwrap();
        assert_eq!(test.text(cx), "some words here");
    }

    #[gpui::test]
    fn pasting_a_url_over_a_selection_makes_a_link(cx: &mut TestAppContext) {
        let test = markdown_editor("see the docs", "Markdown", cx);
        cx.write_to_clipboard(ClipboardItem::new_string(" https://example.com\n".into()));

        test.select(8..12, cx);
        assert!(test.run(paste, cx));
        assert_eq!(test.text(cx), "see the [docs](https://example.com)");

        // Without a selection it is a plain paste.
        test.select(0..0, cx);
        assert!(test.run(paste, cx));
        assert!(test.text(cx).starts_with(" https://example.com\nsee the"));
    }

    #[gpui::test]
    fn table_commands_move_and_add_rows(cx: &mut TestAppContext) {
        let test = markdown_editor("| a | b |\n|---|---|\n| c | d |", "Markdown", cx);
        test.select(2..2, cx);

        assert!(test.run(
            |editor, window, cx| move_table_cell(editor, CellDirection::Next, window, cx),
            cx
        ));
        assert_eq!(test.selection(cx), 6..7);

        assert!(test.run(insert_table_row, cx));
        assert_eq!(test.text(cx), "| a | b |\n|---|---|\n|  |  |\n| c | d |");
        assert_eq!(test.selection(cx), 22..22);
    }

    #[gpui::test]
    fn commands_leave_other_text_alone(cx: &mut TestAppContext) {
        let prose = markdown_editor("no table here", "Markdown", cx);
        prose.select(3..3, cx);
        assert!(!prose.run(
            |editor, window, cx| move_table_cell(editor, CellDirection::Next, window, cx),
            cx
        ));
        assert!(!prose.run(insert_table_row, cx));

        let rust = markdown_editor("let x = 1;", "Rust", cx);
        rust.select(4..5, cx);
        assert!(!rust.run(
            |editor, window, cx| toggle_inline_style(editor, InlineStyle::Bold, window, cx),
            cx
        ));
        assert!(!rust.run(paste, cx));
        assert_eq!(rust.text(cx), "let x = 1;");
    }
}
