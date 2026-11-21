//! Check TODO before commit (IntelliJ's "Check TODO" commit option): the TODO
//! items in the lines a commit adds or changes. When there are some, VEGA asks
//! whether to commit anyway or review them in the TODO tool window first.

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use git_ui::vega_commit_check::{CommitCheckRequest, register_commit_check};
use gpui::{App, PromptLevel, Task};
use settings::Settings as _;
use util::command::new_command;

use crate::TodoSettings;
use crate::panel::{FileTodos, TodoPanel, scan_file};

/// Commits touching more files than this aren't checked file by file.
const MAX_FILES: usize = 300;
/// Items listed in the question; the rest are counted.
const LISTED_ITEMS: usize = 5;

pub(crate) fn init(cx: &mut App) {
    register_commit_check(
        Rc::new(|request: &CommitCheckRequest, window, cx| {
            if !TodoSettings::get_global(cx).check_before_commit
                || !request.project.read(cx).is_local()
            {
                return Task::ready(true);
            }
            let project = request.project.clone();
            let workspace = request.workspace.clone();
            let work_directory = request.work_directory.clone();
            let staged_only = request.staged_only;
            window.spawn(cx, async move |cx| {
                let changed = changed_lines(&work_directory, staged_only).await;
                if changed.is_empty() || changed.len() > MAX_FILES {
                    return true;
                }
                let mut files: Vec<FileTodos> = Vec::new();
                for (path, rows) in changed {
                    let open = project.update(cx, |project, cx| {
                        project.open_local_buffer(work_directory.join(&path), cx)
                    });
                    let Ok(buffer) = open.await else {
                        continue;
                    };
                    if let Some(mut file) = scan_file(buffer, cx).await {
                        file.items.retain(|item| {
                            let item_rows = item.row..item.row + item.ranges.len() as u32;
                            rows.iter().any(|changed| overlaps(changed, &item_rows))
                        });
                        if !file.items.is_empty() {
                            files.push(file);
                        }
                    }
                }
                if files.is_empty() {
                    return true;
                }
                let (message, detail) = question(&files);
                let answer = cx.update(|window, cx| {
                    window.prompt(
                        PromptLevel::Warning,
                        &message,
                        Some(&detail),
                        &["Commit", "Review", "Cancel"],
                        cx,
                    )
                });
                let Ok(answer) = answer else {
                    return false;
                };
                match answer.await {
                    Ok(0) => true,
                    Ok(1) => {
                        cx.update(|window, cx| {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(panel) = workspace.focus_panel::<TodoPanel>(window, cx)
                                {
                                    panel.update(cx, |panel, cx| {
                                        panel.show_changed_lines(files, cx)
                                    });
                                }
                            })
                        })
                        .ok();
                        false
                    }
                    _ => false,
                }
            })
        }),
        cx,
    );
}

fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start < b.end && b.start < a.end
}

fn question(files: &[FileTodos]) -> (String, String) {
    let count: usize = files.iter().map(|file| file.items.len()).sum();
    let message = if count == 1 {
        "1 TODO item found in the changed lines".to_string()
    } else {
        format!("{count} TODO items found in the changed lines")
    };
    let mut lines: Vec<String> = files
        .iter()
        .flat_map(|file| {
            file.items
                .iter()
                .map(move |item| format!("{}:{}  {}", file.name, item.row + 1, item.text))
        })
        .take(LISTED_ITEMS)
        .collect();
    if count > LISTED_ITEMS {
        lines.push(format!("…and {} more", count - LISTED_ITEMS));
    }
    lines.push(String::new());
    lines.push("Commit them anyway, or review them in the TODO tool window?".to_string());
    (message, lines.join("\n"))
}

/// The rows (zero-based, in the new version) each file's commit adds or
/// changes, by path relative to the repository root.
async fn changed_lines(
    work_directory: &Path,
    staged_only: bool,
) -> Vec<(PathBuf, Vec<Range<u32>>)> {
    let mut args = vec![
        "-c",
        "core.quotePath=false",
        "diff",
        "-U0",
        "--no-color",
        "--no-ext-diff",
    ];
    args.push(if staged_only { "--cached" } else { "HEAD" });
    let output = new_command("git")
        .args(&args)
        .current_dir(work_directory)
        .output()
        .await;
    match output {
        Ok(output) if output.status.success() => {
            parse_diff(&String::from_utf8_lossy(&output.stdout))
        }
        _ => Vec::new(),
    }
}

/// Reads `git diff -U0` output: each new-side file and the row ranges of its
/// added lines.
pub(crate) fn parse_diff(diff: &str) -> Vec<(PathBuf, Vec<Range<u32>>)> {
    let mut files: Vec<(PathBuf, Vec<Range<u32>>)> = Vec::new();
    let mut current: Option<usize> = None;
    for line in diff.lines() {
        if let Some(path) = line.strip_prefix("+++ ") {
            current = path.strip_prefix("b/").map(|path| {
                files.push((PathBuf::from(path), Vec::new()));
                files.len() - 1
            });
        } else if line.starts_with("diff --git ") {
            current = None;
        } else if let (Some(ix), Some(hunk)) = (current, line.strip_prefix("@@ ")) {
            if let Some(rows) = hunk_new_rows(hunk) {
                files[ix].1.push(rows);
            }
        }
    }
    files.retain(|(_, rows)| !rows.is_empty());
    files
}

/// `-a,b +c,d @@ …` → the zero-based rows `c-1 .. c-1+d` (none when `d` is 0).
fn hunk_new_rows(hunk: &str) -> Option<Range<u32>> {
    let new = hunk.split(' ').find_map(|part| part.strip_prefix('+'))?;
    let (start, count) = match new.split_once(',') {
        Some((start, count)) => (start.parse::<u32>().ok()?, count.parse::<u32>().ok()?),
        None => (new.parse::<u32>().ok()?, 1),
    };
    (count > 0).then(|| start.saturating_sub(1)..start.saturating_sub(1) + count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_added_rows_from_a_zero_context_diff() {
        let diff = "\
diff --git a/src/a.rs b/src/a.rs
index 1..2 100644
--- a/src/a.rs
+++ b/src/a.rs
@@ -3,0 +4,2 @@ fn main() {
+    // TODO: one
+    x();
@@ -10 +12 @@
-old
+new
@@ -20,3 +21,0 @@
-gone
diff --git a/deleted.rs b/deleted.rs
--- a/deleted.rs
+++ /dev/null
@@ -1,2 +0,0 @@
-a
-b
diff --git a/new file.txt b/new file.txt
--- /dev/null
+++ b/new file.txt
@@ -0,0 +1 @@
+hello
";
        assert_eq!(
            parse_diff(diff),
            vec![
                (PathBuf::from("src/a.rs"), vec![3..5, 11..12]),
                (PathBuf::from("new file.txt"), vec![0..1]),
            ]
        );
    }

    #[test]
    fn ranges_overlap_only_when_they_share_a_row() {
        assert!(overlaps(&(3..5), &(4..5)));
        assert!(!overlaps(&(3..5), &(5..6)));
        assert!(overlaps(&(3..5), &(0..4)));
    }
}
