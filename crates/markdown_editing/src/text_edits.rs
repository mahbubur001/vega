//! The text transformations behind the Markdown editing commands, kept free
//! of editor types so they can be tested as plain functions.

use std::ops::Range;

/// One replacement in the original text, with the selection to leave behind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Replacement {
    pub range: Range<usize>,
    pub text: String,
    /// Relative to the start of `text`.
    pub selection: Range<usize>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct AppliedEdits {
    pub edits: Vec<(Range<usize>, String)>,
    /// In the text as it is after `edits`.
    pub selections: Vec<Range<usize>>,
}

/// Turns per-selection replacements into one batch of edits. A replacement
/// that overlaps an earlier one (two cursors sharing the same markers) is
/// dropped.
pub(crate) fn combine(mut replacements: Vec<Replacement>) -> AppliedEdits {
    replacements.sort_by_key(|replacement| replacement.range.start);
    let mut applied = AppliedEdits::default();
    let mut previous_end = 0;
    let mut removed = 0;
    let mut added = 0;
    for replacement in replacements {
        if !applied.edits.is_empty() && replacement.range.start < previous_end {
            continue;
        }
        let start = replacement.range.start - removed + added;
        applied
            .selections
            .push(start + replacement.selection.start..start + replacement.selection.end);
        previous_end = replacement.range.end;
        removed += replacement.range.len();
        added += replacement.text.len();
        applied.edits.push((replacement.range, replacement.text));
    }
    applied
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InlineStyle {
    Bold,
    Italic,
    Strikethrough,
    Code,
}

impl InlineStyle {
    fn marker(self) -> &'static str {
        match self {
            Self::Bold => "**",
            Self::Italic => "*",
            Self::Strikethrough => "~~",
            Self::Code => "`",
        }
    }

    fn marker_byte(self) -> u8 {
        self.marker().as_bytes()[0]
    }

    /// Whether a run of `run` marker characters carries this style. Bold and
    /// italic share `*`: one is italic, two is bold, three is both.
    fn is_in_run(self, run: usize) -> bool {
        match self {
            Self::Bold => run >= 2,
            Self::Italic => run % 2 == 1,
            Self::Strikethrough => run >= 2,
            Self::Code => run >= 1,
        }
    }
}

fn run_before(text: &str, offset: usize, byte: u8) -> usize {
    text.as_bytes()[..offset]
        .iter()
        .rev()
        .take_while(|candidate| **candidate == byte)
        .count()
}

fn run_after(text: &str, offset: usize, byte: u8) -> usize {
    text.as_bytes()[offset..]
        .iter()
        .take_while(|candidate| **candidate == byte)
        .count()
}

/// Adds the style's markers around the selection, or removes them when the
/// selection already has them (inside or just outside it). With nothing
/// selected it inserts an empty pair, or removes the empty pair the cursor is
/// in.
pub(crate) fn toggle_inline_style(
    text: &str,
    selection: Range<usize>,
    style: InlineStyle,
) -> Replacement {
    let marker = style.marker();
    let byte = style.marker_byte();
    let width = marker.len();

    let selected = &text[selection.clone()];
    let content_start = selection.start + (selected.len() - selected.trim_start().len());
    let content_end = selection.end - (selected.len() - selected.trim_end().len());
    if content_start >= content_end {
        let cursor = selection.start;
        let run = run_before(text, cursor, byte).min(run_after(text, cursor, byte));
        if style.is_in_run(run) {
            return Replacement {
                range: cursor - width..cursor + width,
                text: String::new(),
                selection: 0..0,
            };
        }
        return Replacement {
            range: cursor..cursor,
            text: format!("{marker}{marker}"),
            selection: width..width,
        };
    }

    let content = &text[content_start..content_end];
    let inner_run = run_after(text, content_start, byte).min(run_before(text, content_end, byte));
    if content.len() > 2 * width && style.is_in_run(inner_run.min(content.len() / 2)) {
        let unwrapped = &content[width..content.len() - width];
        return Replacement {
            range: content_start..content_end,
            text: unwrapped.to_string(),
            selection: 0..unwrapped.len(),
        };
    }

    let outer_run = run_before(text, content_start, byte).min(run_after(text, content_end, byte));
    if style.is_in_run(outer_run) {
        return Replacement {
            range: content_start - width..content_end + width,
            text: content.to_string(),
            selection: 0..content.len(),
        };
    }

    Replacement {
        range: content_start..content_end,
        text: format!("{marker}{content}{marker}"),
        selection: width..width + content.len(),
    }
}

pub(crate) fn is_url(text: &str) -> bool {
    ["http://", "https://"].iter().any(|scheme| {
        text.strip_prefix(scheme)
            .is_some_and(|rest| !rest.is_empty() && !rest.contains(char::is_whitespace))
    })
}

/// Wraps the selection in a link and leaves the cursor in the part still to be
/// filled in: the address, or the label when the selection is itself a URL.
pub(crate) fn insert_link(text: &str, selection: Range<usize>) -> Replacement {
    let selected = &text[selection.clone()];
    if is_url(selected.trim()) {
        return Replacement {
            range: selection,
            text: format!("[]({})", selected.trim()),
            selection: 1..1,
        };
    }
    let link = format!("[{selected}]()");
    let cursor = if selected.is_empty() {
        1
    } else {
        link.len() - 1
    };
    Replacement {
        range: selection,
        text: link,
        selection: cursor..cursor,
    }
}

/// The link made by pasting `url` over the selection, or `None` when a plain
/// paste is the right thing: nothing selected, several lines selected, or the
/// selection is a URL being replaced by another.
pub(crate) fn link_from_pasted_url(
    text: &str,
    selection: Range<usize>,
    url: &str,
) -> Option<Replacement> {
    let selected = &text[selection.clone()];
    if selected.trim().is_empty() || selected.contains('\n') || is_url(selected.trim()) {
        return None;
    }
    let link = format!("[{selected}]({url})");
    let cursor = link.len();
    Some(Replacement {
        range: selection,
        text: link,
        selection: cursor..cursor,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CellDirection {
    Next,
    Previous,
}

struct TableLine {
    start: usize,
    end: usize,
}

impl TableLine {
    fn text<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.end]
    }
}

fn line_at(text: &str, offset: usize) -> TableLine {
    let start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
    let end = text[offset..]
        .find('\n')
        .map_or(text.len(), |index| offset + index);
    TableLine { start, end }
}

fn is_table_line(line: &str) -> bool {
    line.trim_start().starts_with('|')
}

/// The raw cell spans of a table line, between its unescaped pipes, relative
/// to the line's start.
fn cell_spans(line: &str) -> Vec<Range<usize>> {
    let bytes = line.as_bytes();
    let pipes = bytes
        .iter()
        .enumerate()
        .filter(|(index, byte)| **byte == b'|' && (*index == 0 || bytes[index - 1] != b'\\'))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let mut spans = pipes
        .windows(2)
        .map(|pair| pair[0] + 1..pair[1])
        .collect::<Vec<_>>();
    if let Some(last_pipe) = pipes.last()
        && !line[last_pipe + 1..].trim().is_empty()
    {
        spans.push(last_pipe + 1..line.len());
    }
    spans
}

/// Where the cursor goes inside a cell: around its text, or one space in when
/// the cell is empty.
fn cell_content(line: &str, span: &Range<usize>) -> Range<usize> {
    let cell = &line[span.clone()];
    let trimmed = cell.trim();
    if trimmed.is_empty() {
        let cursor = (span.start + 1).min(span.end);
        return cursor..cursor;
    }
    let start = span.start + (cell.len() - cell.trim_start().len());
    start..start + trimmed.len()
}

fn is_delimiter_row(line: &str) -> bool {
    let spans = cell_spans(line);
    !spans.is_empty()
        && spans.iter().all(|span| {
            let cell = line[span.clone()].trim();
            let dashes = cell.trim_start_matches(':').trim_end_matches(':');
            !dashes.is_empty() && dashes.bytes().all(|byte| byte == b'-')
        })
}

/// The lines of the table around `offset`, or `None` when `offset` is not in
/// one. A table is a run of lines starting with `|` that has a delimiter row.
fn table_lines(text: &str, offset: usize) -> Option<(Vec<TableLine>, usize)> {
    let current = line_at(text, offset);
    if !is_table_line(current.text(text)) {
        return None;
    }
    let mut lines = Vec::new();
    let mut start = current.start;
    while start > 0 {
        let previous = line_at(text, start - 1);
        if !is_table_line(previous.text(text)) {
            break;
        }
        start = previous.start;
        lines.push(previous);
    }
    lines.reverse();
    let current_index = lines.len();
    let mut end = current.end;
    lines.push(current);
    while end < text.len() {
        let next = line_at(text, end + 1);
        if !is_table_line(next.text(text)) {
            break;
        }
        end = next.end;
        lines.push(next);
    }
    lines
        .iter()
        .any(|line| is_delimiter_row(line.text(text)))
        .then_some((lines, current_index))
}

/// The cell to select when moving from the cell under `cursor`, or `None` when
/// `cursor` is not in a table. At the table's first or last cell the current
/// cell is returned.
pub(crate) fn table_cell_target(
    text: &str,
    cursor: usize,
    direction: CellDirection,
) -> Option<Range<usize>> {
    let (lines, current_index) = table_lines(text, cursor)?;
    let cells_of = |line: &TableLine| {
        let line_text = line.text(text);
        cell_spans(line_text)
            .iter()
            .map(|span| {
                let content = cell_content(line_text, span);
                (
                    line.start + span.end,
                    line.start + content.start..line.start + content.end,
                )
            })
            .collect::<Vec<_>>()
    };
    let is_data_row = |line: &&TableLine| {
        let line_text = line.text(text);
        !is_delimiter_row(line_text) && !cell_spans(line_text).is_empty()
    };

    let current_cells = cells_of(&lines[current_index]);
    let current_cell = current_cells
        .iter()
        .position(|(span_end, _)| cursor <= *span_end)
        .unwrap_or(current_cells.len().saturating_sub(1));
    let stay = || {
        current_cells
            .get(current_cell)
            .map(|(_, content)| content.clone())
    };

    match direction {
        CellDirection::Next => {
            if let Some((_, content)) = current_cells.get(current_cell + 1) {
                return Some(content.clone());
            }
            lines[current_index + 1..]
                .iter()
                .find(is_data_row)
                .and_then(|line| cells_of(line).first().map(|(_, content)| content.clone()))
                .or_else(stay)
        }
        CellDirection::Previous => {
            if current_cell > 0
                && let Some((_, content)) = current_cells.get(current_cell - 1)
            {
                return Some(content.clone());
            }
            lines[..current_index]
                .iter()
                .rev()
                .find(is_data_row)
                .and_then(|line| cells_of(line).last().map(|(_, content)| content.clone()))
                .or_else(stay)
        }
    }
}

/// An empty row under the row at `cursor` (under the delimiter row when
/// `cursor` is in the header), with the cursor in its first cell. `None` when
/// `cursor` is not in a table.
pub(crate) fn table_row_insertion(text: &str, cursor: usize) -> Option<Replacement> {
    let (lines, current_index) = table_lines(text, cursor)?;
    let current = &lines[current_index];
    let current_text = current.text(text);
    let column_count = lines
        .iter()
        .map(|line| cell_spans(line.text(text)).len())
        .max()
        .unwrap_or(0);
    if column_count == 0 {
        return None;
    }
    let anchor = match lines.get(current_index + 1) {
        Some(next) if is_delimiter_row(next.text(text)) => next,
        _ => current,
    };
    let indent = &current_text[..current_text.len() - current_text.trim_start().len()];
    let row = format!("\n{indent}|{}", "  |".repeat(column_count));
    let cursor_in_row = 1 + indent.len() + 2;
    Some(Replacement {
        range: anchor.end..anchor.end,
        text: row,
        selection: cursor_in_row..cursor_in_row,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Applies `edit` to a text written with `«` and `»` around the selection
    /// (or `ˇ` for a cursor) and returns the result in the same notation.
    fn check(marked: &str, edit: impl Fn(&str, Range<usize>) -> Option<Replacement>) -> String {
        let (text, selection) = unmark(marked);
        let Some(replacement) = edit(&text, selection) else {
            return "unchanged".to_string();
        };
        let applied = combine(vec![replacement]);
        let (range, new_text) = &applied.edits[0];
        let mut result = text;
        result.replace_range(range.clone(), new_text);
        mark(&result, applied.selections[0].clone())
    }

    fn unmark(marked: &str) -> (String, Range<usize>) {
        if let Some(cursor) = marked.find('ˇ') {
            return (marked.replace('ˇ', ""), cursor..cursor);
        }
        let start = marked.find('«').unwrap();
        let without_start = marked.replacen('«', "", 1);
        let end = without_start.find('»').unwrap();
        (without_start.replacen('»', "", 1), start..end)
    }

    fn mark(text: &str, selection: Range<usize>) -> String {
        let mut marked = text.to_string();
        if selection.is_empty() {
            marked.insert(selection.start, 'ˇ');
        } else {
            marked.insert(selection.end, '»');
            marked.insert(selection.start, '«');
        }
        marked
    }

    fn toggle(marked: &str, style: InlineStyle) -> String {
        check(marked, |text, selection| {
            Some(toggle_inline_style(text, selection, style))
        })
    }

    #[test]
    fn bold_wraps_and_unwraps() {
        assert_eq!(toggle("a «word» b", InlineStyle::Bold), "a **«word»** b");
        assert_eq!(toggle("a **«word»** b", InlineStyle::Bold), "a «word» b");
        assert_eq!(toggle("a «**word**» b", InlineStyle::Bold), "a «word» b");
        assert_eq!(toggle("a« word »b", InlineStyle::Bold), "a **«word»** b");
        assert_eq!(
            toggle("héllo «wörld»", InlineStyle::Bold),
            "héllo **«wörld»**"
        );
    }

    #[test]
    fn bold_and_italic_share_asterisks() {
        assert_eq!(toggle("**«word»**", InlineStyle::Italic), "***«word»***");
        assert_eq!(toggle("***«word»***", InlineStyle::Italic), "**«word»**");
        assert_eq!(toggle("***«word»***", InlineStyle::Bold), "*«word»*");
        assert_eq!(toggle("*«word»*", InlineStyle::Bold), "***«word»***");
        assert_eq!(toggle("*«word»*", InlineStyle::Italic), "«word»");
    }

    #[test]
    fn strikethrough_and_code() {
        assert_eq!(toggle("«gone»", InlineStyle::Strikethrough), "~~«gone»~~");
        assert_eq!(toggle("~~«gone»~~", InlineStyle::Strikethrough), "«gone»");
        assert_eq!(toggle("«let x»", InlineStyle::Code), "`«let x»`");
        assert_eq!(toggle("«`let x`»", InlineStyle::Code), "«let x»");
    }

    #[test]
    fn cursor_without_selection() {
        assert_eq!(toggle("a ˇ b", InlineStyle::Bold), "a **ˇ** b");
        assert_eq!(toggle("a **ˇ** b", InlineStyle::Bold), "a ˇ b");
        assert_eq!(toggle("ˇ", InlineStyle::Code), "`ˇ`");
        assert_eq!(toggle("a «  » b", InlineStyle::Bold), "a **ˇ**   b");
    }

    #[test]
    fn several_selections_keep_their_places() {
        let text = "one two three";
        let replacements = vec![
            toggle_inline_style(text, 8..13, InlineStyle::Bold),
            toggle_inline_style(text, 0..3, InlineStyle::Bold),
        ];
        let applied = combine(replacements);
        let mut result = text.to_string();
        for (range, new_text) in applied.edits.iter().rev() {
            result.replace_range(range.clone(), new_text);
        }
        assert_eq!(result, "**one** two **three**");
        assert_eq!(applied.selections, vec![2..5, 14..19]);
    }

    #[test]
    fn links() {
        let link =
            |marked: &str| check(marked, |text, selection| Some(insert_link(text, selection)));
        assert_eq!(link("see «the docs» now"), "see [the docs](ˇ) now");
        assert_eq!(link("see ˇ now"), "see [ˇ]() now");
        assert_eq!(
            link("«https://example.com/a»"),
            "[ˇ](https://example.com/a)"
        );

        let paste = |marked: &str| {
            check(marked, |text, selection| {
                link_from_pasted_url(text, selection, "https://example.com")
            })
        };
        assert_eq!(
            paste("see «the docs» now"),
            "see [the docs](https://example.com)ˇ now"
        );
        assert_eq!(paste("see ˇ now"), "unchanged");
        assert_eq!(paste("«two\nlines»"), "unchanged");
        assert_eq!(paste("«https://old.example»"), "unchanged");

        assert!(is_url("https://example.com/a?b=c"));
        assert!(!is_url("https://"));
        assert!(!is_url("https://a b"));
        assert!(!is_url("example.com"));
    }

    const TABLE: &str =
        "intro\n| Name | Role |\n|------|:----:|\n| Ada  |      |\n| Bo | Dev |\nafter";

    fn move_cell(marked: &str, direction: CellDirection) -> String {
        let (text, selection) = unmark(marked);
        match table_cell_target(&text, selection.start, direction) {
            Some(target) => mark(&text, target),
            None => "not a table".to_string(),
        }
    }

    #[test]
    fn tab_moves_through_cells() {
        let at = |needle: &str| TABLE.replacen(needle, &format!("ˇ{needle}"), 1);
        let selected = |needle: &str| TABLE.replacen(needle, &format!("«{needle}»"), 1);

        assert_eq!(
            move_cell(&at("Name"), CellDirection::Next),
            selected("Role")
        );
        // From the header's last cell, over the delimiter row, to the first data cell.
        assert_eq!(move_cell(&at("Role"), CellDirection::Next), selected("Ada"));
        // An empty cell gets a cursor one space in.
        assert_eq!(
            move_cell(&at("Ada"), CellDirection::Next),
            TABLE.replacen("| Ada  |      |", "| Ada  | ˇ     |", 1)
        );
        assert_eq!(move_cell(&at("Dev"), CellDirection::Next), selected("Dev"));

        assert_eq!(
            move_cell(&at("Dev"), CellDirection::Previous),
            selected("Bo")
        );
        assert_eq!(
            move_cell(&at("Ada"), CellDirection::Previous),
            selected("Role")
        );
        assert_eq!(
            move_cell(&at("Name"), CellDirection::Previous),
            selected("Name")
        );

        assert_eq!(move_cell(&at("intro"), CellDirection::Next), "not a table");
        assert_eq!(
            move_cell("| not | a table ˇ|", CellDirection::Next),
            "not a table"
        );
    }

    #[test]
    fn shift_enter_adds_a_row() {
        let insert = |marked: &str| {
            check(marked, |text, selection| {
                table_row_insertion(text, selection.start)
            })
        };
        assert_eq!(
            insert(&TABLE.replacen("Ada", "ˇAda", 1)),
            TABLE.replacen("| Ada  |      |\n", "| Ada  |      |\n| ˇ |  |\n", 1)
        );
        // From the header the row goes under the delimiter row.
        assert_eq!(
            insert(&TABLE.replacen("Name", "ˇName", 1)),
            TABLE.replacen("|------|:----:|\n", "|------|:----:|\n| ˇ |  |\n", 1)
        );
        assert_eq!(insert("plain ˇtext"), "unchanged");
    }
}
