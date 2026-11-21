//! Finding TODO items in text, like IntelliJ: a pattern word (TODO, FIXME, …)
//! inside a comment, the text after it to the end of the line, and following
//! comment lines indented past the keyword (a multi-line TODO).

use std::ops::Range;

use regex::Regex;

/// The words used when the `todo.patterns` setting is empty.
pub const DEFAULT_PATTERNS: [&str; 2] = ["TODO", "FIXME"];

/// One TODO item found in a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    /// Zero-based row of the keyword.
    pub row: u32,
    /// Byte column of the keyword in its row.
    pub column: u32,
    /// The pattern word as written (`TODO`, `todo`, `FIXME`).
    pub keyword: String,
    /// The text from the keyword to the end of the item, continuation lines
    /// joined with spaces.
    pub text: String,
    /// Byte ranges of the item in the whole text (one per line), for
    /// highlighting.
    pub ranges: Vec<Range<usize>>,
}

/// Builds the matcher for the pattern words: whole words, case-insensitive
/// unless `case_sensitive`. Words are taken literally. Returns `None` when
/// there are no usable words.
pub fn matcher(patterns: &[String], case_sensitive: bool) -> Option<Regex> {
    let words = words_pattern(patterns)?;
    let flags = if case_sensitive { "" } else { "(?i)" };
    Regex::new(&format!("{flags}{words}")).ok()
}

/// `\b(?:TODO|FIXME)\b` for the pattern words (taken literally), without case
/// flags, for project search.
pub fn words_pattern(patterns: &[String]) -> Option<String> {
    let words: Vec<String> = patterns
        .iter()
        .map(|word| word.trim())
        .filter(|word| !word.is_empty())
        .map(regex::escape)
        .collect();
    (!words.is_empty()).then(|| format!(r"\b(?:{})\b", words.join("|")))
}

/// The TODO items in `text`. `in_comment(offset)` says whether the byte at
/// `offset` is inside a comment; matches outside comments are skipped, so
/// `todo!()` or a variable named `todo` don't count.
pub fn scan(text: &str, matcher: &Regex, in_comment: impl Fn(usize) -> bool) -> Vec<Found> {
    let lines = line_starts(text);
    let mut found: Vec<Found> = Vec::new();
    let mut last_row: Option<usize> = None;
    for m in matcher.find_iter(text) {
        let row = lines.partition_point(|&start| start <= m.start()) - 1;
        // One item per line: the first keyword wins.
        if last_row == Some(row) || !in_comment(m.start()) {
            continue;
        }
        // A keyword on a line already taken by the previous item's
        // continuation starts a new item; drop the overlap.
        if let Some(previous) = found.last_mut()
            && previous
                .ranges
                .iter()
                .any(|range| range.contains(&m.start()))
        {
            previous.ranges.retain(|range| range.end <= lines[row]);
            previous.text = item_text_from_ranges(text, &previous.ranges);
        }
        last_row = Some(row);
        let line_start = lines[row];
        let first_line_end = line_end(text, &lines, row);
        let column = m.start() - line_start;
        let first_end = m.start() + trimmed_len(&text[m.start()..first_line_end]);
        let mut ranges = vec![m.start()..first_end];

        // Continuation lines: comment lines whose text starts right of the keyword.
        let mut next = row + 1;
        while next < lines.len() {
            let start = lines[next];
            let end = line_end(text, &lines, next);
            let line = &text[start..end];
            let Some(content) = continuation_content(line, column) else {
                break;
            };
            let content_start = start + content;
            if !in_comment(content_start) || matcher.is_match(&text[content_start..end]) {
                break;
            }
            let content_end = content_start + trimmed_len(&text[content_start..end]);
            if content_end <= content_start {
                break;
            }
            ranges.push(content_start..content_end);
            next += 1;
        }

        found.push(Found {
            row: row as u32,
            column: column as u32,
            keyword: m.as_str().to_string(),
            text: item_text_from_ranges(text, &ranges),
            ranges,
        });
    }
    found
}

/// Whether the part of a line before a match looks like it is in a comment,
/// for files whose language doesn't mark comments: a line comment prefix or a
/// block comment start before the match, or a line starting with `*` (inside
/// a doc block). With no language at all, everything counts.
pub fn looks_like_comment(before: &str, line_prefixes: &[&str], block_start: Option<&str>) -> bool {
    if line_prefixes.is_empty() && block_start.is_none() {
        return true;
    }
    let trimmed = before.trim_start();
    trimmed.starts_with('*')
        || line_prefixes
            .iter()
            .map(|prefix| prefix.trim())
            .filter(|prefix| !prefix.is_empty())
            .any(|prefix| before.contains(prefix))
        || block_start
            .is_some_and(|start| !start.trim().is_empty() && before.contains(start.trim()))
}

fn line_starts(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(text.match_indices('\n').map(|(ix, _)| ix + 1))
        .collect()
}

fn line_end(text: &str, lines: &[usize], row: usize) -> usize {
    let end = lines.get(row + 1).map_or(text.len(), |next| next - 1);
    if end > lines[row] && text.as_bytes()[end - 1] == b'\r' {
        end - 1
    } else {
        end
    }
}

/// Length of `text` without trailing whitespace and comment closers
/// (`*/`, `-->`, `#}`, `--}}`).
fn trimmed_len(text: &str) -> usize {
    let mut text = text.trim_end();
    loop {
        let before = text;
        for closer in ["*/", "-->", "#}", "--}}"] {
            if let Some(stripped) = text.strip_suffix(closer) {
                text = stripped.trim_end();
            }
        }
        if text == before {
            return text.len();
        }
    }
}

/// For a continuation line: the byte offset of its text after the comment
/// marker, when that text starts right of the keyword column.
fn continuation_content(line: &str, keyword_column: usize) -> Option<usize> {
    let indent = line.len() - line.trim_start().len();
    let rest = &line[indent..];
    let marker = ["///", "//!", "//", "#", "--", ";;", ";", "*"]
        .into_iter()
        .find(|marker| rest.starts_with(marker))?;
    let after_marker = indent + marker.len();
    let content =
        after_marker + (line[after_marker..].len() - line[after_marker..].trim_start().len());
    (content > keyword_column && content < line.len()).then_some(content)
}

fn item_text_from_ranges(text: &str, ranges: &[Range<usize>]) -> String {
    ranges
        .iter()
        .map(|range| text[range.clone()].trim())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn patterns() -> Regex {
        matcher(&["TODO".to_string(), "FIXME".to_string()], false).unwrap()
    }

    /// Treats everything after `//`, `#` or inside `/* */` on a line as comment.
    fn naive_comment(text: &str) -> impl Fn(usize) -> bool + '_ {
        move |offset| {
            let line_start = text[..offset].rfind('\n').map_or(0, |ix| ix + 1);
            let before = &text[line_start..offset];
            looks_like_comment(before, &["//", "#"], Some("/*"))
        }
    }

    fn texts(found: &[Found]) -> Vec<(u32, u32, &str)> {
        found
            .iter()
            .map(|item| (item.row, item.column, item.text.as_str()))
            .collect()
    }

    #[test]
    fn finds_keywords_in_comments_only() {
        let text = "let todo = 1; // TODO: rename\nfn x() { todo!() }\n# fixme later\n";
        let found = scan(text, &patterns(), naive_comment(text));
        assert_eq!(
            texts(&found),
            vec![(0, 17, "TODO: rename"), (2, 2, "fixme later")]
        );
        assert_eq!(found[1].keyword, "fixme");
    }

    #[test]
    fn whole_words_only_and_case_sensitivity_is_optional() {
        let text = "// TODOS and todo_list are not items\n// todo: but this is\n";
        let found = scan(text, &patterns(), naive_comment(text));
        assert_eq!(texts(&found), vec![(1, 3, "todo: but this is")]);

        let strict = matcher(&["TODO".to_string()], true).unwrap();
        assert!(scan(text, &strict, naive_comment(text)).is_empty());
    }

    #[test]
    fn strips_comment_closers() {
        let text = "/* TODO: close me */\n<!-- FIXME: html -->\n";
        let found = scan(text, &patterns(), |_| true);
        assert_eq!(
            texts(&found),
            vec![(0, 3, "TODO: close me"), (1, 5, "FIXME: html")]
        );
        assert_eq!(&text[found[0].ranges[0].clone()], "TODO: close me");
    }

    #[test]
    fn indented_comment_lines_continue_an_item() {
        let text = "// TODO: first line\n//   second line\n// not indented\n    /**\n     * FIXME: a\n     *        b\n     */\n";
        let found = scan(text, &patterns(), naive_comment(text));
        assert_eq!(
            texts(&found),
            vec![(0, 3, "TODO: first line second line"), (4, 7, "FIXME: a b")]
        );
        assert_eq!(found[0].ranges.len(), 2);
        assert_eq!(&text[found[0].ranges[1].clone()], "second line");
    }

    #[test]
    fn a_new_keyword_ends_the_previous_item() {
        let text = "// TODO: one\n//    TODO: two\n";
        let found = scan(text, &patterns(), naive_comment(text));
        assert_eq!(
            texts(&found),
            vec![(0, 3, "TODO: one"), (1, 6, "TODO: two")]
        );
    }

    #[test]
    fn no_patterns_means_no_matcher() {
        assert!(matcher(&[], false).is_none());
        assert!(matcher(&["  ".to_string()], false).is_none());
        // Words are literal, not regexes.
        let dot = matcher(&["A.B".to_string()], true).unwrap();
        assert!(dot.is_match("A.B") && !dot.is_match("AxB"));
    }

    #[test]
    fn languages_without_comment_markers_count_everything() {
        assert!(looks_like_comment("anything", &[], None));
        assert!(!looks_like_comment("let x = ", &["//"], Some("/*")));
        assert!(looks_like_comment("   * ", &["//"], Some("/*")));
        assert!(looks_like_comment("x = 1  # ", &["# "], None));
    }
}
