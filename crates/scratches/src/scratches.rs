//! Scratch files, like PhpStorm's "Scratches and Consoles": files kept
//! outside projects in VEGA's config folder, shown under a "Scratches and
//! Consoles" node at the bottom of every project panel. New Scratch File
//! asks for a language; scratch buffers are plain text and reused after five.

mod list_picker;
mod naming;
mod rows;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::Context as _;
use editor::Editor;
use file_icons::FileIcons;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, AsyncWindowContext, Task, WeakEntity, Window};
use language::LanguageRegistry;
use language_selector::LanguageSelector;
use project::{Project, ProjectPath};
use project_panel::ProjectPanel;
use settings::SeedQuerySetting;
use ui::prelude::*;
use workspace::notifications::NotificationId;
use workspace::searchable::SearchableItem as _;
use workspace::{OpenOptions, OpenVisible, Toast, Workspace};
use zed_actions::scratches::{
    NewScratchBuffer, NewScratchFile, RevealScratchesFolder, RunScratch, ShowScratchFiles,
};

use crate::list_picker::{ListPicker, PickerEntry};
use crate::naming::{extension_of, free_name, next_buffer_name, run_command};

/// Where scratch files live (outside every project).
pub fn scratches_dir() -> PathBuf {
    paths::config_dir().join("scratches")
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace
            .register_action(|workspace, _: &NewScratchFile, window, cx| {
                new_scratch_file(workspace, window, cx)
            })
            .register_action(|workspace, _: &NewScratchBuffer, window, cx| {
                new_scratch_buffer(workspace, window, cx)
            })
            .register_action(|workspace, _: &ShowScratchFiles, window, cx| {
                show_scratch_files(workspace, window, cx)
            })
            .register_action(|workspace, _: &RunScratch, window, cx| {
                match active_scratch(workspace, cx) {
                    Some(path) => run_scratch(workspace, path, window, cx),
                    None => notify(workspace, "The active file isn't a scratch file.", cx),
                }
            })
            .register_action(|_, _: &RevealScratchesFolder, _, cx| {
                cx.reveal_path(&scratches_dir())
            });
    })
    .detach();

    cx.observe_new(|panel: &mut ProjectPanel, window, cx| {
        if window.is_none() || !panel.project().read(cx).is_local() {
            return;
        }
        let workspace = panel.workspace();
        let project = panel.project().clone();
        let panel_handle = cx.entity().downgrade();
        let rows = cx.new(|cx| rows::ScratchRows::new(workspace, project, panel_handle, cx));
        panel.add_extra_rows(rows, cx);
    })
    .detach();
}

/// How a scratch in `language` is named, `(stem, extension)`: `scratch.php`
/// for PHP, the file name itself for languages matched by name.
pub(crate) fn scratch_file_for(
    registry: &Arc<LanguageRegistry>,
    language: &str,
) -> (String, String) {
    if language == "Plain Text" {
        return ("scratch".into(), "txt".into());
    }
    let suffixes: Vec<String> = registry
        .available_language_for_name(language)
        .map(|language| {
            language
                .matcher()
                .path_suffixes
                .iter()
                .map(|suffix| suffix.to_string())
                .collect()
        })
        .unwrap_or_default();
    file_for_suffixes(&suffixes)
}

fn file_for_suffixes(suffixes: &[String]) -> (String, String) {
    let scratch = |extension: &str| ("scratch".to_string(), extension.to_string());
    if let Some(extension) = suffixes.iter().find(|suffix| is_plain_extension(suffix)) {
        return scratch(extension);
    }
    if let Some(extension) = suffixes
        .iter()
        .filter_map(|suffix| suffix.strip_prefix('.'))
        .find(|suffix| is_plain_extension(suffix))
    {
        return scratch(extension);
    }
    if let Some(name) = suffixes
        .iter()
        .find(|suffix| !suffix.is_empty() && !suffix.contains(['/', '*', '.']))
    {
        return (name.clone(), String::new());
    }
    scratch("txt")
}

/// The language whose files look like `name` (for "(current)" in Change Language).
pub(crate) fn language_for_file(registry: &Arc<LanguageRegistry>, name: &str) -> Option<String> {
    let extension = extension_of(name);
    registry.language_names().into_iter().find_map(|language| {
        let available = registry.available_language_for_name(language.as_ref())?;
        available
            .matcher()
            .path_suffixes
            .iter()
            .any(|suffix| {
                (!extension.is_empty() && suffix.trim_start_matches('.') == extension)
                    || suffix.as_ref() == name
            })
            .then(|| language.to_string())
    })
}

/// A suffix usable as `scratch.<suffix>` ("php", "c++"), not a file name
/// ("TAG_EDITMSG", ".env").
fn is_plain_extension(suffix: &str) -> bool {
    !suffix.is_empty()
        && suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '+' | '-' | '_'))
}

/// The text selected in the active editor (if any) and that editor's language.
fn selection_and_language(
    workspace: &Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> (Option<String>, Option<String>) {
    let Some(editor) = workspace.active_item_as::<Editor>(cx) else {
        return (None, None);
    };
    let text = editor.update(cx, |editor, cx| {
        editor.query_suggestion(Some(SeedQuerySetting::Selection), window, cx)
    });
    let language = editor
        .read(cx)
        .buffer()
        .read(cx)
        .as_singleton()
        .and_then(|buffer| buffer.read(cx).language().map(|language| language.name()))
        .map(|name| name.to_string());
    ((!text.is_empty()).then_some(text), language)
}

/// File › New › Scratch File: choose a language (VEGA's language picker),
/// then create and open `scratch.<ext>` (with the editor's selection, if any).
pub(crate) fn new_scratch_file(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let registry = workspace.project().read(cx).languages().clone();
    let (selection, language) = selection_and_language(workspace, window, cx);
    let fs = workspace.project().read(cx).fs().clone();
    let handle = cx.entity().downgrade();
    workspace.toggle_modal(window, cx, move |window, cx| {
        LanguageSelector::pick(
            registry.clone(),
            language,
            move |name, window, cx| {
                let (stem, extension) = scratch_file_for(&registry, name.as_ref());
                create_scratch(
                    handle,
                    fs,
                    NewName::Scratch { stem, extension },
                    selection.unwrap_or_default(),
                    window,
                    cx,
                );
            },
            window,
            cx,
        )
    });
}

/// File › New › Scratch Buffer: a plain-text `bufferN.txt`.
fn new_scratch_buffer(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let fs = workspace.project().read(cx).fs().clone();
    create_scratch(
        cx.entity().downgrade(),
        fs,
        NewName::Buffer,
        String::new(),
        window,
        cx,
    );
}

pub(crate) enum NewName {
    /// `<stem>.<extension>`, numbered when taken.
    Scratch { stem: String, extension: String },
    /// The next `bufferN.txt`, reused after five.
    Buffer,
}

/// Creates the scratch (or reuses a buffer) in the scratches folder and opens it.
pub(crate) fn create_scratch(
    workspace: WeakEntity<Workspace>,
    fs: Arc<dyn Fs>,
    name: NewName,
    content: String,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            let dir = scratches_dir();
            let result = async {
                fs.create_dir(&dir).await?;
                let existing = list_dir(fs.as_ref(), &dir).await;
                let path = match name {
                    NewName::Scratch { stem, extension } => {
                        let names: HashSet<String> =
                            existing.into_iter().map(|(name, _, _)| name).collect();
                        let path = dir.join(free_name(&stem, &extension, &names));
                        fs.write(&path, content.as_bytes()).await?;
                        path
                    }
                    NewName::Buffer => {
                        let files: Vec<(String, SystemTime)> = existing
                            .into_iter()
                            .filter(|(_, is_dir, _)| !is_dir)
                            .map(|(name, _, modified)| (name, modified))
                            .collect();
                        let path = dir.join(next_buffer_name(&files));
                        if !fs.is_file(&path).await {
                            fs.write(&path, b"").await?;
                        }
                        path
                    }
                };
                anyhow::Ok(path)
            }
            .await;
            match result {
                Ok(path) => open_scratch(&workspace, path, true, cx).await,
                Err(error) => report(
                    &workspace,
                    format!("Couldn't create the scratch: {error}"),
                    cx,
                ),
            }
        })
        .detach();
}

/// Opens `path` (a scratch) in the workspace without adding it to the project.
pub(crate) async fn open_scratch(
    workspace: &WeakEntity<Workspace>,
    path: PathBuf,
    focus: bool,
    cx: &mut AsyncWindowContext,
) {
    let open = workspace.update_in(cx, |workspace, window, cx| {
        workspace.open_abs_path(
            path,
            OpenOptions {
                visible: Some(OpenVisible::None),
                focus: Some(focus),
                ..Default::default()
            },
            window,
            cx,
        )
    });
    if let Ok(open) = open
        && let Err(error) = open.await
    {
        report(workspace, format!("Couldn't open the scratch: {error}"), cx);
    }
}

/// Names, kinds and modification times of the entries in `dir`.
pub(crate) async fn list_dir(fs: &dyn Fs, dir: &Path) -> Vec<(String, bool, SystemTime)> {
    let mut entries = Vec::new();
    let Ok(mut paths) = fs.read_dir(dir).await else {
        return entries;
    };
    while let Some(path) = paths.next().await {
        let Ok(path) = path else { continue };
        let Some(name) = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
        else {
            continue;
        };
        if name == ".DS_Store" {
            continue;
        }
        if let Ok(Some(metadata)) = fs.metadata(&path).await {
            entries.push((name, metadata.is_dir, metadata.mtime.timestamp_for_user()));
        }
    }
    entries
}

/// Every scratch file (in sub-folders too), most recently changed first.
async fn all_scratches(fs: &dyn Fs) -> Vec<(PathBuf, SystemTime)> {
    let mut files = Vec::new();
    let mut dirs = vec![scratches_dir()];
    while let Some(dir) = dirs.pop() {
        for (name, is_dir, modified) in list_dir(fs, &dir).await {
            if is_dir {
                dirs.push(dir.join(name));
            } else {
                files.push((dir.join(name), modified));
            }
        }
    }
    files.sort_by_key(|file| std::cmp::Reverse(file.1));
    files
}

/// Show Scratch Files: every scratch, most recently changed first.
fn show_scratch_files(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let fs = workspace.project().read(cx).fs().clone();
    let handle = cx.entity().downgrade();
    cx.spawn_in(window, async move |_, cx| {
        let files = all_scratches(fs.as_ref()).await;
        handle
            .update_in(cx, |workspace, window, cx| {
                if files.is_empty() {
                    notify(
                        workspace,
                        "No scratch files yet. Create one with New Scratch File.",
                        cx,
                    );
                    return;
                }
                let root = scratches_dir();
                let entries = files
                    .iter()
                    .map(|(path, _)| {
                        let name = path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        let folder = path
                            .parent()
                            .and_then(|parent| parent.strip_prefix(&root).ok())
                            .map(|folder| folder.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        PickerEntry {
                            icon: FileIcons::get_icon(path, cx),
                            label: name.into(),
                            detail: folder.into(),
                        }
                    })
                    .collect();
                let workspace_handle = cx.entity().downgrade();
                workspace.toggle_modal(window, cx, move |window, cx| {
                    ListPicker::new(
                        "Scratch files…",
                        entries,
                        0,
                        move |index, window, cx| {
                            let Some((path, _)) = files.get(index).cloned() else {
                                return;
                            };
                            window
                                .spawn(cx, async move |cx| {
                                    open_scratch(&workspace_handle, path, true, cx).await
                                })
                                .detach();
                        },
                        window,
                        cx,
                    )
                });
            })
            .ok();
    })
    .detach();
}

/// The active editor's file, when it is a scratch.
fn active_scratch(workspace: &Workspace, cx: &App) -> Option<PathBuf> {
    let project_path = workspace.active_item(cx)?.project_path(cx)?;
    let path = workspace
        .project()
        .read(cx)
        .absolute_path(&project_path, cx)?;
    path.starts_with(scratches_dir()).then_some(path)
}

/// Runs a scratch in a terminal (`php`, `node`, `python3`… by extension),
/// saving it first.
pub(crate) fn run_scratch(
    workspace: &mut Workspace,
    path: PathBuf,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let extension = extension_of(&name).to_string();
    let Some((program, mut args)) = run_command(&extension, "php") else {
        notify(
            workspace,
            format!("VEGA doesn't know how to run .{extension} scratches."),
            cx,
        );
        return;
    };
    args.push(path.to_string_lossy().into_owned());
    let cwd = workspace
        .project()
        .read(cx)
        .visible_worktrees(cx)
        .next()
        .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        .unwrap_or_else(scratches_dir);
    let label = format!("Run {name}");
    let spawn = task::SpawnInTerminal {
        id: task::TaskId(format!("vega-scratch-run-{}", path.display())),
        full_label: label.clone(),
        label,
        command_label: format!("{program} {}", args.join(" ")),
        command: Some(program),
        args,
        cwd: Some(cwd),
        use_new_terminal: false,
        allow_concurrent_runs: false,
        show_command: true,
        reveal: task::RevealStrategy::Always,
        ..Default::default()
    };
    let save = save_if_dirty(workspace.project(), &path, cx);
    cx.spawn_in(window, async move |workspace, cx| {
        save.await?;
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.spawn_in_terminal(spawn, window, cx).detach();
        })
    })
    .detach_and_log_err(cx);
}

/// Saves the open buffer of `path`, if it has unsaved changes.
pub(crate) fn save_if_dirty(
    project: &gpui::Entity<Project>,
    path: &Path,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let buffer = {
        let project = project.read(cx);
        project.find_worktree(path, cx).and_then(|(worktree, rel)| {
            project.get_open_buffer(
                &ProjectPath {
                    worktree_id: worktree.read(cx).id(),
                    path: rel,
                },
                cx,
            )
        })
    };
    match buffer {
        Some(buffer) if buffer.read(cx).is_dirty() => {
            let save = project.update(cx, |project, cx| project.save_buffer(buffer, cx));
            cx.background_spawn(async move { save.await.context("couldn't save the scratch") })
        }
        _ => Task::ready(Ok(())),
    }
}

struct ScratchesToast;

pub(crate) fn notify(
    workspace: &mut Workspace,
    message: impl Into<String>,
    cx: &mut Context<Workspace>,
) {
    workspace.show_toast(
        Toast::new(NotificationId::unique::<ScratchesToast>(), message.into()).autohide(),
        cx,
    );
}

pub(crate) fn report(
    workspace: &WeakEntity<Workspace>,
    message: String,
    cx: &mut AsyncWindowContext,
) {
    workspace
        .update(cx, |workspace, cx| {
            workspace.show_toast(
                Toast::new(NotificationId::unique::<ScratchesToast>(), message),
                cx,
            )
        })
        .ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_plain_extensions_name_scratches() {
        assert!(is_plain_extension("php"));
        assert!(is_plain_extension("c++"));
        assert!(is_plain_extension("mp4"));
        assert!(!is_plain_extension(".env"));
        assert!(!is_plain_extension("TAG_EDITMSG"));
        assert!(!is_plain_extension("Dockerfile"));
        assert!(!is_plain_extension(""));
    }

    #[test]
    fn scratches_are_named_after_the_language_suffixes() {
        let file = |suffixes: &[&str]| {
            file_for_suffixes(&suffixes.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        };
        assert_eq!(file(&["php", "phtml"]), ("scratch".into(), "php".into()));
        assert_eq!(file(&[".env", "env.*"]), ("scratch".into(), "env".into()));
        assert_eq!(
            file(&["TAG_EDITMSG", "COMMIT_EDITMSG"]),
            ("TAG_EDITMSG".into(), String::new())
        );
        assert_eq!(file(&[]), ("scratch".into(), "txt".into()));
    }
}
