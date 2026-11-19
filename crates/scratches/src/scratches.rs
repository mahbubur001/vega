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
use db::kvp::KeyValueStore;
use editor::Editor;
use file_icons::FileIcons;
use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, AsyncWindowContext, Task, WeakEntity, Window};
use language::LanguageRegistry;
use project::{Project, ProjectPath};
use project_panel::ProjectPanel;
use settings::SeedQuerySetting;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::searchable::SearchableItem as _;
use workspace::{OpenOptions, OpenVisible, Toast, Workspace};
use zed_actions::scratches::{
    NewScratchBuffer, NewScratchFile, RevealScratchesFolder, RunScratch, ShowScratchFiles,
};

use crate::list_picker::{ListPicker, PickerEntry};
use crate::naming::{extension_of, next_buffer_name, next_scratch_name, run_command};

const RECENT_LANGUAGES_KEY: &str = "vega_scratch_recent_languages";
/// How many recently used languages New Scratch File lists first.
const RECENT_LANGUAGES: usize = 4;

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

/// A language New Scratch File can create.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LanguageChoice {
    pub name: String,
    pub extension: String,
}

/// Every language with a file extension, the recently used ones (and
/// `preferred`, the active editor's language) first.
pub(crate) fn language_choices(
    registry: &Arc<LanguageRegistry>,
    recent: &[String],
    preferred: Option<String>,
) -> (Vec<LanguageChoice>, usize) {
    let mut all: Vec<LanguageChoice> = registry
        .language_names()
        .into_iter()
        .filter_map(|name| {
            let language = registry.available_language_for_name(name.as_ref())?;
            if language.hidden() {
                return None;
            }
            let extension = if name.as_ref() == "Plain Text" {
                "txt".to_string()
            } else {
                language
                    .matcher()
                    .path_suffixes
                    .iter()
                    .find(|suffix| !suffix.contains('/') && !suffix.is_empty())?
                    .to_string()
            };
            Some(LanguageChoice {
                name: name.to_string(),
                extension,
            })
        })
        .collect();
    if !all.iter().any(|choice| choice.name == "Plain Text") {
        all.push(LanguageChoice {
            name: "Plain Text".into(),
            extension: "txt".into(),
        });
    }
    order_languages(all, recent, preferred)
}

/// `preferred`, then `recent` (most recent first, at most four), then the rest
/// by name. Returns the list and how many entries come before the rest.
fn order_languages(
    mut all: Vec<LanguageChoice>,
    recent: &[String],
    preferred: Option<String>,
) -> (Vec<LanguageChoice>, usize) {
    all.sort_by_key(|choice| choice.name.to_lowercase());
    let mut first: Vec<LanguageChoice> = Vec::new();
    for name in preferred
        .into_iter()
        .chain(recent.iter().take(RECENT_LANGUAGES).cloned())
    {
        if let Some(index) = all.iter().position(|choice| choice.name == name) {
            first.push(all.remove(index));
        }
    }
    let count = first.len();
    first.extend(all);
    (first, count)
}

fn recent_languages(cx: &App) -> Vec<String> {
    KeyValueStore::global(cx)
        .read_kvp(RECENT_LANGUAGES_KEY)
        .log_err()
        .flatten()
        .map(|value| value.lines().map(str::to_string).collect())
        .unwrap_or_default()
}

fn remember_language(name: &str, cx: &mut App) {
    let mut recent = recent_languages(cx);
    recent.retain(|recent| recent != name);
    recent.insert(0, name.to_string());
    recent.truncate(RECENT_LANGUAGES * 2);
    let kvp = KeyValueStore::global(cx);
    cx.background_spawn(async move {
        kvp.write_kvp(RECENT_LANGUAGES_KEY.into(), recent.join("\n"))
            .await
            .log_err();
    })
    .detach();
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

/// The file icon for files with this extension.
pub(crate) fn icon_for_extension(extension: &str, cx: &App) -> Option<SharedString> {
    FileIcons::get_icon(Path::new(&format!("scratch.{extension}")), cx)
}

/// File › New › Scratch File: choose a language, then create and open
/// `scratch.<ext>` (with the editor's selection, if any).
pub(crate) fn new_scratch_file(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let registry = workspace.project().read(cx).languages().clone();
    let (selection, language) = selection_and_language(workspace, window, cx);
    let preferred = language.filter(|_| selection.is_some());
    let (languages, first_count) = language_choices(&registry, &recent_languages(cx), preferred);
    let entries = language_entries(&languages, first_count, cx);
    let fs = workspace.project().read(cx).fs().clone();
    let handle = cx.entity().downgrade();
    let title = if selection.is_some() {
        "New Scratch File from Selection — choose a language…"
    } else {
        "New Scratch File — choose a language…"
    };
    workspace.toggle_modal(window, cx, move |window, cx| {
        ListPicker::new(
            title,
            entries,
            0,
            move |index, window, cx| {
                let Some(choice) = languages.get(index).cloned() else {
                    return;
                };
                remember_language(&choice.name, cx);
                create_scratch(
                    handle,
                    fs,
                    NewName::Scratch(choice.extension),
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

pub(crate) fn language_entries(
    languages: &[LanguageChoice],
    first_count: usize,
    cx: &App,
) -> Vec<PickerEntry> {
    languages
        .iter()
        .enumerate()
        .map(|(index, choice)| PickerEntry {
            label: choice.name.clone().into(),
            detail: format!(".{}", choice.extension).into(),
            icon: icon_for_extension(&choice.extension, cx),
            separator_above: first_count > 0 && index == first_count,
        })
        .collect()
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
    /// `scratch.<ext>`, numbered when taken.
    Scratch(String),
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
                    NewName::Scratch(extension) => {
                        let names: HashSet<String> =
                            existing.into_iter().map(|(name, _, _)| name).collect();
                        let path = dir.join(next_scratch_name(&names, &extension));
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
                            separator_above: false,
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

    fn choice(name: &str, extension: &str) -> LanguageChoice {
        LanguageChoice {
            name: name.into(),
            extension: extension.into(),
        }
    }

    #[test]
    fn recent_and_preferred_languages_come_first() {
        let all = vec![
            choice("PHP", "php"),
            choice("JavaScript", "js"),
            choice("Plain Text", "txt"),
            choice("JSON", "json"),
            choice("Python", "py"),
        ];
        let (ordered, first) = order_languages(
            all.clone(),
            &["JSON".into(), "Missing".into(), "PHP".into()],
            None,
        );
        let names: Vec<&str> = ordered.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["JSON", "PHP", "JavaScript", "Plain Text", "Python"]);
        assert_eq!(first, 2);

        let (ordered, first) = order_languages(all, &["PHP".into()], Some("Python".into()));
        let names: Vec<&str> = ordered.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["Python", "PHP", "JavaScript", "JSON", "Plain Text"]);
        assert_eq!(first, 2);
    }
}
