//! File names for new scratches and buffers, and how to run a scratch.

use std::collections::HashSet;
use std::time::SystemTime;

/// Number of plain-text scratch buffers before the oldest is reused.
pub(crate) const BUFFER_COUNT: usize = 5;

/// `scratch.<ext>`, or `scratch_1.<ext>`, `scratch_2.<ext>`… when taken.
pub(crate) fn next_scratch_name(existing: &HashSet<String>, extension: &str) -> String {
    free_name("scratch", extension, existing)
}

/// `<stem>.<ext>` if free, else `<stem>_1.<ext>`, `<stem>_2.<ext>`…
pub(crate) fn free_name(stem: &str, extension: &str, existing: &HashSet<String>) -> String {
    let with_extension = |stem: &str| {
        if extension.is_empty() {
            stem.to_string()
        } else {
            format!("{stem}.{extension}")
        }
    };
    let first = with_extension(stem);
    if !existing.contains(&first) {
        return first;
    }
    (1..)
        .map(|n| with_extension(&format!("{stem}_{n}")))
        .find(|name| !existing.contains(name))
        .unwrap_or(first)
}

/// The scratch buffer to open: the first of `buffer1.txt`…`buffer5.txt` that
/// doesn't exist, else the one changed longest ago (reused).
pub(crate) fn next_buffer_name(existing: &[(String, SystemTime)]) -> String {
    let names: Vec<String> = (1..=BUFFER_COUNT)
        .map(|n| format!("buffer{n}.txt"))
        .collect();
    if let Some(free) = names
        .iter()
        .find(|name| !existing.iter().any(|(existing, _)| existing == *name))
    {
        return free.clone();
    }
    existing
        .iter()
        .filter(|(name, _)| names.contains(name))
        .min_by_key(|(_, modified)| *modified)
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| names[0].clone())
}

/// `name` with its extension replaced by `extension` (Change Language).
pub(crate) fn with_extension(name: &str, extension: &str) -> String {
    let stem = stem_of(name);
    if extension.is_empty() {
        stem.to_string()
    } else {
        format!("{stem}.{extension}")
    }
}

/// The name without its last extension (`.env` keeps its name).
pub(crate) fn stem_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(dot) if dot > 0 => &name[..dot],
        _ => name,
    }
}

pub(crate) fn extension_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(dot) if dot > 0 => &name[dot + 1..],
        _ => "",
    }
}

/// The program (and leading arguments) that runs a scratch with this
/// extension; the scratch's path is appended. `php` is the PHP interpreter.
pub(crate) fn run_command(extension: &str, php: &str) -> Option<(String, Vec<String>)> {
    let (program, args): (&str, &[&str]) = match extension.to_ascii_lowercase().as_str() {
        "php" => (php, &[]),
        "js" | "mjs" | "cjs" => ("node", &[]),
        "ts" | "mts" | "cts" => ("npx", &["--yes", "tsx"]),
        "py" => ("python3", &[]),
        "sh" | "bash" => ("bash", &[]),
        "zsh" => ("zsh", &[]),
        "rb" => ("ruby", &[]),
        "pl" => ("perl", &[]),
        "lua" => ("lua", &[]),
        "go" => ("go", &["run"]),
        "swift" => ("swift", &[]),
        _ => return None,
    };
    Some((
        program.to_string(),
        args.iter().map(|arg| arg.to_string()).collect(),
    ))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn names(list: &[&str]) -> HashSet<String> {
        list.iter().map(|name| name.to_string()).collect()
    }

    #[test]
    fn scratch_names_count_up_like_phpstorm() {
        assert_eq!(next_scratch_name(&names(&[]), "php"), "scratch.php");
        assert_eq!(
            next_scratch_name(&names(&["scratch.php"]), "php"),
            "scratch_1.php"
        );
        assert_eq!(
            next_scratch_name(&names(&["scratch.php", "scratch_1.php"]), "php"),
            "scratch_2.php"
        );
        assert_eq!(
            next_scratch_name(&names(&["scratch.php"]), "js"),
            "scratch.js"
        );
        assert_eq!(next_scratch_name(&names(&[]), ""), "scratch");
    }

    #[test]
    fn buffers_fill_up_then_reuse_the_oldest() {
        let at = |secs| SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        assert_eq!(next_buffer_name(&[]), "buffer1.txt");
        assert_eq!(
            next_buffer_name(&[("buffer1.txt".into(), at(5))]),
            "buffer2.txt"
        );
        let all: Vec<(String, SystemTime)> = (1..=5)
            .map(|n| (format!("buffer{n}.txt"), at(if n == 3 { 1 } else { 10 })))
            .collect();
        assert_eq!(next_buffer_name(&all), "buffer3.txt");
    }

    #[test]
    fn changing_language_swaps_the_extension() {
        assert_eq!(with_extension("scratch.php", "js"), "scratch.js");
        assert_eq!(with_extension("notes", "md"), "notes.md");
        assert_eq!(with_extension("a.b.txt", "json"), "a.b.json");
        assert_eq!(with_extension(".env", "txt"), ".env.txt");
        assert_eq!(extension_of("scratch.PHP"), "PHP");
        assert_eq!(extension_of(".env"), "");
    }

    #[test]
    fn runs_scripts_by_extension() {
        assert_eq!(
            run_command("php", "/opt/homebrew/bin/php"),
            Some(("/opt/homebrew/bin/php".into(), vec![]))
        );
        assert_eq!(
            run_command("ts", "php"),
            Some(("npx".into(), vec!["--yes".into(), "tsx".into()]))
        );
        assert_eq!(
            run_command("JS", "php").map(|(p, _)| p),
            Some("node".into())
        );
        assert_eq!(run_command("json", "php"), None);
        assert_eq!(run_command("txt", "php"), None);
    }
}
