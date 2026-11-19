//! Library files (Composer's `vendor`, `node_modules`, PHP stubs) open
//! read-only, like PhpStorm's Non-Project Files Protection, with a bar that
//! lets you edit this file, the files of this package, or any library file
//! for the rest of the session.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use editor::Editor;
use gpui::{App, Context, Entity, EventEmitter, Global, Render, WeakEntity, Window};
use project::Project;
use ui::{Tooltip, prelude::*};
use workspace::{ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

use crate::stubs_dir;

/// Where a library file comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LibraryKind {
    Composer,
    Node,
    PhpStubs,
    /// A folder from the PHP settings' Include Path list.
    IncludePath,
}

/// The package folder holding `path` (`vendor/<vendor>/<package>`,
/// `node_modules/<package>` or `node_modules/@scope/<package>`, a stubs
/// extension folder), if `path` is a library file.
pub(crate) fn library_root(
    path: &Path,
    project_roots: &[PathBuf],
    stubs: &Path,
    include_paths: &[PathBuf],
) -> Option<(PathBuf, LibraryKind)> {
    let package = |base: PathBuf, rest: &Path, depth: usize, kind| {
        let parts: Vec<_> = rest.components().collect();
        // A file directly in the library folder (vendor/autoload.php) belongs to it.
        if parts.len() <= depth {
            return Some((base, kind));
        }
        let mut root = base;
        for part in parts.iter().take(depth) {
            root.push(part);
        }
        Some((root, kind))
    };
    if let Ok(rest) = path.strip_prefix(stubs) {
        return package(stubs.to_path_buf(), rest, 1, LibraryKind::PhpStubs);
    }
    if let Some(include) = include_paths
        .iter()
        .find(|include| path.starts_with(include))
    {
        return Some((include.clone(), LibraryKind::IncludePath));
    }
    for root in project_roots {
        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        if let Ok(rest) = relative.strip_prefix("vendor") {
            return package(root.join("vendor"), rest, 2, LibraryKind::Composer);
        }
        if let Ok(rest) = relative.strip_prefix("node_modules") {
            let scoped = rest
                .components()
                .next()
                .is_some_and(|first| first.as_os_str().to_string_lossy().starts_with('@'));
            let depth = if scoped { 2 } else { 1 };
            return package(root.join("node_modules"), rest, depth, LibraryKind::Node);
        }
    }
    None
}

/// What the owner allowed editing this session.
#[derive(Default)]
pub(crate) struct EditAllowances {
    any: bool,
    files: HashSet<PathBuf>,
    folders: HashSet<PathBuf>,
}

impl Global for EditAllowances {}

impl EditAllowances {
    fn allows(&self, file: &Path, folder: &Path) -> bool {
        self.any || self.files.contains(file) || self.folders.contains(folder)
    }
}

fn include_paths(project: &Entity<Project>, cx: &App) -> Vec<PathBuf> {
    php_settings::cached_config(project, cx)
        .map(|config| config.include_paths())
        .unwrap_or_default()
}

fn project_roots(project: &Entity<Project>, cx: &App) -> Vec<PathBuf> {
    project
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        .collect()
}

fn editor_path(editor: &Entity<Editor>, project: &Entity<Project>, cx: &App) -> Option<PathBuf> {
    let project_path = editor.project_path(cx)?;
    project.read(cx).absolute_path(&project_path, cx)
}

/// The library folder of `editor`'s file when it must stay read-only.
fn protected(
    editor: &Entity<Editor>,
    project: &Entity<Project>,
    cx: &App,
) -> Option<(PathBuf, PathBuf, LibraryKind)> {
    let path = editor_path(editor, project, cx)?;
    let (root, kind) = library_root(
        &path,
        &project_roots(project, cx),
        &stubs_dir(),
        &include_paths(project, cx),
    )?;
    let allowed = cx
        .try_global::<EditAllowances>()
        .is_some_and(|allowances| allowances.allows(&path, &root));
    (!allowed).then_some((path, root, kind))
}

/// Makes library files read-only as they open in `workspace`.
pub(crate) fn protect_library_files(workspace: &mut Workspace, cx: &mut Context<Workspace>) {
    cx.subscribe(
        &cx.entity(),
        |workspace, _, event: &workspace::Event, cx| {
            if let workspace::Event::ItemAdded { item } = event {
                lock_if_library(item.as_ref(), workspace.project(), cx);
            }
        },
    )
    .detach();
    let project = workspace.project().clone();
    let items: Vec<Box<dyn ItemHandle>> = workspace.items(cx).cloned().collect();
    for item in items {
        lock_if_library(item.as_ref(), &project, cx);
    }
}

fn lock_if_library(item: &dyn ItemHandle, project: &Entity<Project>, cx: &mut App) {
    let Some(editor) = item.act_as::<Editor>(cx) else {
        return;
    };
    if protected(&editor, project, cx).is_some() {
        editor.update(cx, |editor, cx| {
            editor.set_read_only(true);
            cx.notify();
        });
    }
}

/// The "Non-project file" bar above a read-only library file.
pub(crate) struct NonProjectBar {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active: Option<(Entity<Editor>, PathBuf, PathBuf, LibraryKind)>,
}

impl NonProjectBar {
    pub(crate) fn new(workspace: WeakEntity<Workspace>, project: Entity<Project>) -> Self {
        Self {
            workspace,
            project,
            active: None,
        }
    }

    fn allow(&mut self, change: impl FnOnce(&mut EditAllowances), cx: &mut Context<Self>) {
        change(cx.default_global::<EditAllowances>());
        // Unlock every open file the new allowance covers.
        if let Some(workspace) = self.workspace.upgrade() {
            let editors: Vec<Entity<Editor>> =
                workspace.read(cx).items_of_type::<Editor>(cx).collect();
            for editor in editors {
                let library = editor_path(&editor, &self.project, cx).and_then(|path| {
                    library_root(
                        &path,
                        &project_roots(&self.project, cx),
                        &stubs_dir(),
                        &include_paths(&self.project, cx),
                    )
                });
                if library.is_some() && protected(&editor, &self.project, cx).is_none() {
                    editor.update(cx, |editor, cx| {
                        editor.set_read_only(false);
                        cx.notify();
                    });
                }
            }
        }
        self.active = None;
        cx.emit(ToolbarItemEvent::ChangeLocation(
            ToolbarItemLocation::Hidden,
        ));
        cx.notify();
    }
}

impl EventEmitter<ToolbarItemEvent> for NonProjectBar {}

impl ToolbarItemView for NonProjectBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active = active_pane_item
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| {
                let (path, root, kind) = protected(&editor, &self.project, cx)?;
                Some((editor, path, root, kind))
            });
        if self.active.is_some() {
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for NonProjectBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some((_, path, root, kind)) = self.active.clone() else {
            return div().into_any_element();
        };
        let status = cx.theme().status();
        let package = match kind {
            LibraryKind::PhpStubs => root
                .file_name()
                .map(|name| format!("PHP stubs: {}", name.to_string_lossy()))
                .unwrap_or_else(|| "PHP stubs".into()),
            LibraryKind::IncludePath => format!("include path {}", root.display()),
            LibraryKind::Composer | LibraryKind::Node => {
                let base = if kind == LibraryKind::Composer {
                    "vendor"
                } else {
                    "node_modules"
                };
                root.components()
                    .rev()
                    .take_while(|part| part.as_os_str() != base)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|part| part.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/")
            }
        };
        let why = match kind {
            LibraryKind::Composer => "Changes are overwritten by composer install.",
            LibraryKind::Node => "Changes are overwritten by npm install.",
            LibraryKind::PhpStubs => "Stubs describe PHP's built-in functions.",
            LibraryKind::IncludePath => "It is outside the project.",
        };
        let folder_label = match kind {
            LibraryKind::PhpStubs => "Edit All Stubs Here",
            LibraryKind::IncludePath => "Edit Files in This Folder",
            _ => "Edit Files in This Package",
        };
        h_flex()
            .w_full()
            .px_3()
            .py_1()
            .gap_2()
            .bg(status.warning_background)
            .border_b_1()
            .border_color(status.warning_border)
            .child(
                Icon::new(IconName::FileLock)
                    .size(IconSize::Small)
                    .color(Color::Warning),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(Label::new("Non-project file").size(LabelSize::Small))
                    .when(!package.is_empty(), |row| {
                        row.child(
                            Label::new(format!("· {package}"))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .child(
                        Label::new(format!("· read-only. {why}"))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
            )
            .child(
                Button::new("non-project-edit-file", "Edit This File")
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        let path = path.clone();
                        this.allow(|allowances| _ = allowances.files.insert(path), cx)
                    })),
            )
            .child(
                Button::new("non-project-edit-folder", folder_label)
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        let root = root.clone();
                        this.allow(|allowances| _ = allowances.folders.insert(root), cx)
                    })),
            )
            .child(
                Button::new("non-project-edit-any", "Edit Any Library File")
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text("Until VEGA quits"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.allow(|allowances| allowances.any = true, cx)
                    })),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_package_of_library_files() {
        let roots = [PathBuf::from("/site")];
        let stubs = Path::new("/data/php-stubs");
        let includes = [PathBuf::from("/libs/wordpress")];
        let root = |path: &str| library_root(Path::new(path), &roots, stubs, &includes);

        assert_eq!(
            root("/site/vendor/guzzlehttp/guzzle/src/Client.php"),
            Some((
                PathBuf::from("/site/vendor/guzzlehttp/guzzle"),
                LibraryKind::Composer
            ))
        );
        assert_eq!(
            root("/site/vendor/autoload.php"),
            Some((PathBuf::from("/site/vendor"), LibraryKind::Composer))
        );
        assert_eq!(
            root("/site/node_modules/lodash/index.js"),
            Some((
                PathBuf::from("/site/node_modules/lodash"),
                LibraryKind::Node
            ))
        );
        assert_eq!(
            root("/site/node_modules/@types/node/fs.d.ts"),
            Some((
                PathBuf::from("/site/node_modules/@types/node"),
                LibraryKind::Node
            ))
        );
        assert_eq!(
            root("/data/php-stubs/standard/standard_0.php"),
            Some((
                PathBuf::from("/data/php-stubs/standard"),
                LibraryKind::PhpStubs
            ))
        );
        assert_eq!(
            root("/libs/wordpress/wp-includes/plugin.php"),
            Some((PathBuf::from("/libs/wordpress"), LibraryKind::IncludePath))
        );
        assert_eq!(root("/site/src/vendor.php"), None);
        assert_eq!(root("/elsewhere/vendor/a/b/c.php"), None);
    }

    #[test]
    fn allowances_cover_files_folders_or_everything() {
        let file = Path::new("/site/vendor/a/b/src/X.php");
        let folder = Path::new("/site/vendor/a/b");
        let mut allowances = EditAllowances::default();
        assert!(!allowances.allows(file, folder));
        allowances.files.insert(file.to_path_buf());
        assert!(allowances.allows(file, folder));
        let mut allowances = EditAllowances::default();
        allowances.folders.insert(folder.to_path_buf());
        assert!(allowances.allows(file, folder));
        let allowances = EditAllowances {
            any: true,
            ..Default::default()
        };
        assert!(allowances.allows(Path::new("/x"), Path::new("/y")));
    }
}
