//! The "External Libraries" node in the project panel.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use db::kvp::KeyValueStore;
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, ClickEvent, ClipboardItem, Context, DismissEvent, Entity, Focusable as _,
    MouseButton, MouseDownEvent, Pixels, Point, Subscription, Task, WeakEntity, Window, anchored,
    deferred,
};
use project::Project;
use project_panel::{ExtraRowState, ProjectPanel, ProjectPanelExtraRows};
use ui::{ContextMenu, prelude::*};
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::{OpenOptions, OpenVisible, Toast, Workspace};

use crate::model::short_version;
use crate::{GroupKind, Libraries, download_stubs, list_dir, load_libraries, stubs_dir};

const EXPANDED_KEY: &str = "vega_external_libraries_expanded";
const ROOT: &str = "root";
const PHP: &str = "php";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RowKind {
    Root,
    Php,
    PhpExtension,
    Group(GroupKind),
    Package,
    Dir,
    File,
    Note,
}

#[derive(Clone, Debug)]
struct Row {
    key: SharedString,
    kind: RowKind,
    label: SharedString,
    detail: Option<SharedString>,
    /// Shown dimmed (a package that isn't installed, an extension without stubs).
    dimmed: bool,
    path: Option<PathBuf>,
    depth: usize,
}

impl Row {
    fn expandable(&self) -> bool {
        match self.kind {
            RowKind::Root | RowKind::Php | RowKind::Group(_) | RowKind::Dir => true,
            RowKind::PhpExtension | RowKind::Package => self.path.is_some() && !self.dimmed,
            RowKind::File | RowKind::Note => false,
        }
    }
}

pub(crate) struct LibraryRows {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    panel: WeakEntity<ProjectPanel>,
    libraries: Option<Libraries>,
    expanded: HashSet<SharedString>,
    /// Folder listings, loaded when a folder is first expanded.
    listings: HashMap<PathBuf, Vec<(String, bool)>>,
    rows: Vec<Row>,
    selected: Option<SharedString>,
    context_menu: Option<(
        Entity<ContextMenu>,
        Point<Pixels>,
        SharedString,
        Subscription,
    )>,
    downloading: bool,
    _subscriptions: Vec<Subscription>,
    _load: Task<()>,
    _download: Task<()>,
}

impl LibraryRows {
    pub(crate) fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        panel: WeakEntity<ProjectPanel>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut expanded = HashSet::default();
        if KeyValueStore::global(cx)
            .read_kvp(EXPANDED_KEY)
            .log_err()
            .flatten()
            .is_some_and(|value| value == "1")
        {
            expanded.insert(SharedString::from(ROOT));
        }
        let mut subscriptions = vec![cx.subscribe(&project, |this, _, event, cx| match event {
            project::Event::WorktreeAdded(_) | project::Event::WorktreeRemoved(_) => {
                this.reload(cx)
            }
            project::Event::WorktreeUpdatedEntries(_, changes) => {
                let manifest_changed = changes.iter().any(|(path, _, _)| {
                    path.parent().is_some_and(|parent| parent.is_empty())
                        && matches!(
                            path.file_name(),
                            Some("composer.lock" | "package.json" | "package-lock.json")
                        )
                });
                if manifest_changed {
                    this.reload(cx);
                }
            }
            _ => {}
        })];
        if let Some(workspace) = workspace.upgrade() {
            subscriptions.push(cx.subscribe(&workspace, |this, workspace, event, cx| {
                if let workspace::Event::ActiveItemChanged = event {
                    this.reveal_active(workspace, cx);
                }
            }));
        }
        let mut this = Self {
            workspace,
            project,
            panel,
            libraries: None,
            expanded,
            listings: HashMap::default(),
            rows: Vec::new(),
            selected: None,
            context_menu: None,
            downloading: false,
            _subscriptions: subscriptions,
            _load: Task::ready(()),
            _download: Task::ready(()),
        };
        this.reload(cx);
        this
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        let fs = self.project.read(cx).fs().clone();
        let roots: Vec<PathBuf> = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .collect();
        let load = cx.background_spawn(load_libraries(fs, roots, "php".into()));
        self._load = cx.spawn(async move |this, cx| {
            let libraries = load.await;
            this.update(cx, |this, cx| {
                this.libraries = Some(libraries);
                this.listings.clear();
                let open: Vec<PathBuf> = this
                    .rows
                    .iter()
                    .filter(|row| this.expanded.contains(&row.key))
                    .filter_map(|row| row.path.clone())
                    .collect();
                for path in open {
                    this.load_listing(path, cx);
                }
                this.rebuild(cx);
            })
            .ok();
        });
    }

    fn load_listing(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let fs = self.project.read(cx).fs().clone();
        cx.spawn(async move |this, cx| {
            let entries = list_dir(fs.as_ref(), &path).await;
            this.update(cx, |this, cx| {
                this.listings.insert(path, entries);
                this.rebuild(cx);
            })
            .ok();
        })
        .detach();
    }

    fn rebuild(&mut self, cx: &mut Context<Self>) {
        let mut rows = vec![Row {
            key: ROOT.into(),
            kind: RowKind::Root,
            label: "External Libraries".into(),
            detail: None,
            dimmed: false,
            path: None,
            depth: 0,
        }];
        if self.expanded.contains(ROOT) {
            match &self.libraries {
                None => rows.push(note("Loading…")),
                Some(libraries) => self.push_libraries(libraries, &mut rows),
            }
        }
        self.rows = rows;
        cx.notify();
    }

    fn push_libraries(&self, libraries: &Libraries, rows: &mut Vec<Row>) {
        if let Some(php) = &libraries.php {
            rows.push(Row {
                key: PHP.into(),
                kind: RowKind::Php,
                label: "PHP Runtime".into(),
                detail: Some(format!("PHP {}", short_version(&php.version)).into()),
                dimmed: false,
                path: None,
                depth: 1,
            });
            if self.expanded.contains(PHP) {
                if !php.has_stubs {
                    rows.push(Row {
                        depth: 2,
                        ..note(if self.downloading {
                            "Downloading PHP stubs…"
                        } else {
                            "Right-click PHP Runtime › Download PHP Stubs to browse them"
                        })
                    });
                }
                for (extension, folder) in &php.extensions {
                    let key: SharedString = format!("php/{extension}").into();
                    rows.push(Row {
                        key: key.clone(),
                        kind: RowKind::PhpExtension,
                        label: extension.clone().into(),
                        detail: None,
                        dimmed: folder.is_none(),
                        path: folder.clone(),
                        depth: 2,
                    });
                    if let Some(folder) = folder
                        && self.expanded.contains(&key)
                    {
                        self.push_folder(folder, 3, rows);
                    }
                }
            }
        }
        let several_folders = libraries
            .groups
            .iter()
            .map(|group| &group.folder)
            .collect::<HashSet<_>>()
            .len()
            > 1;
        for (index, group) in libraries.groups.iter().enumerate() {
            let key: SharedString = format!("group/{index}").into();
            let name = match group.kind {
                GroupKind::Composer => "Composer",
                GroupKind::Node => "Node.js",
            };
            rows.push(Row {
                key: key.clone(),
                kind: RowKind::Group(group.kind),
                label: if several_folders {
                    format!("{name} — {}", group.folder).into()
                } else {
                    name.into()
                },
                detail: Some(match group.packages.len() {
                    1 => "1 package".into(),
                    count => format!("{count} packages").into(),
                }),
                dimmed: false,
                path: None,
                depth: 1,
            });
            if !self.expanded.contains(&key) {
                continue;
            }
            for (package, installed) in &group.packages {
                let key: SharedString = package.path.to_string_lossy().into_owned().into();
                let mut detail = package.version.clone().unwrap_or_default();
                if package.dev {
                    detail = format!("{detail} dev").trim().to_string();
                }
                if !installed {
                    detail = format!("{detail} · not installed")
                        .trim_start_matches(" · ")
                        .to_string();
                }
                rows.push(Row {
                    key: key.clone(),
                    kind: RowKind::Package,
                    label: package.name.clone().into(),
                    detail: (!detail.is_empty()).then(|| detail.into()),
                    dimmed: !installed,
                    path: Some(package.path.clone()),
                    depth: 2,
                });
                if *installed && self.expanded.contains(&key) {
                    self.push_folder(&package.path, 3, rows);
                }
            }
        }
        if libraries.php.is_none() && libraries.groups.is_empty() {
            rows.push(note(
                "No PHP interpreter, composer.lock or package.json found",
            ));
        }
    }

    fn push_folder(&self, folder: &Path, depth: usize, rows: &mut Vec<Row>) {
        let Some(entries) = self.listings.get(folder) else {
            rows.push(Row {
                depth,
                ..note("Loading…")
            });
            return;
        };
        for (name, is_dir) in entries {
            let path = folder.join(name);
            let key: SharedString = path.to_string_lossy().into_owned().into();
            let expanded = *is_dir && self.expanded.contains(&key);
            rows.push(Row {
                key,
                kind: if *is_dir { RowKind::Dir } else { RowKind::File },
                label: name.clone().into(),
                detail: None,
                dimmed: false,
                path: Some(path.clone()),
                depth,
            });
            if expanded {
                self.push_folder(&path, depth + 1, rows);
            }
        }
    }

    fn select(&mut self, key: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = Some(key);
        if let Some(panel) = self.panel.upgrade() {
            panel.update(cx, |panel, cx| panel.clear_selection(cx));
            window.focus(&panel.focus_handle(cx), cx);
        }
        cx.notify();
    }

    fn toggle(&mut self, row: &Row, cx: &mut Context<Self>) {
        if !self.expanded.remove(&row.key) {
            self.expanded.insert(row.key.clone());
            if let Some(path) = &row.path
                && !self.listings.contains_key(path)
            {
                self.load_listing(path.clone(), cx);
            }
        }
        if row.kind == RowKind::Root {
            let kvp = KeyValueStore::global(cx);
            let value = if self.expanded.contains(ROOT) {
                "1"
            } else {
                "0"
            };
            cx.background_spawn(async move {
                kvp.write_kvp(EXPANDED_KEY.into(), value.into())
                    .await
                    .log_err();
            })
            .detach();
        }
        self.rebuild(cx);
    }

    fn click(
        &mut self,
        row: &Row,
        event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if row.kind == RowKind::Note {
            return;
        }
        self.select(row.key.clone(), window, cx);
        if row.expandable() {
            self.toggle(row, cx);
        } else if row.kind == RowKind::File
            && let Some(path) = row.path.clone()
        {
            self.open(path, event.click_count() > 1, window, cx);
        }
    }

    fn open(&mut self, path: PathBuf, focus: bool, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        window
            .spawn(cx, async move |cx| {
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
                if let Ok(open) = open {
                    open.await.log_err();
                }
            })
            .detach();
    }

    fn deploy_menu(
        &mut self,
        row: Row,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if row.kind == RowKind::Note {
            return;
        }
        self.select(row.key.clone(), window, cx);
        let this = cx.entity().downgrade();
        let has_stubs = self
            .libraries
            .as_ref()
            .and_then(|libraries| libraries.php.as_ref())
            .is_some_and(|php| php.has_stubs);
        let downloading = self.downloading;
        let menu_row = row.clone();
        let menu = ContextMenu::build(window, cx, move |menu, _, _| {
            let row = menu_row;
            let act = |operation: fn(&mut Self, &Row, &mut Window, &mut Context<Self>)| {
                let this = this.clone();
                let row = row.clone();
                move |window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| operation(this, &row, window, cx))
                        .ok();
                }
            };
            menu.when(row.kind == RowKind::File, |menu| {
                menu.entry("Open", None, act(Self::open_row)).separator()
            })
            .when(row.kind == RowKind::Php && !downloading, |menu| {
                menu.entry(
                    if has_stubs {
                        "Update PHP Stubs"
                    } else {
                        "Download PHP Stubs"
                    },
                    None,
                    act(Self::download_stubs),
                )
                .separator()
            })
            .when(row.path.is_some(), |menu| {
                menu.entry("Reveal in Finder", None, act(Self::reveal))
                    .entry("Copy Path", None, act(Self::copy_path))
                    .separator()
            })
            .entry("Refresh", None, act(Self::refresh))
        });
        window.focus(&menu.focus_handle(cx), cx);
        let subscription = cx.subscribe(&menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu = None;
            cx.notify();
        });
        self.context_menu = Some((menu, position, row.key, subscription));
        cx.notify();
    }

    fn open_row(&mut self, row: &Row, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = row.path.clone() {
            self.open(path, true, window, cx);
        }
    }

    fn reveal(&mut self, row: &Row, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = &row.path {
            cx.reveal_path(path);
        }
    }

    fn copy_path(&mut self, row: &Row, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = &row.path {
            cx.write_to_clipboard(ClipboardItem::new_string(
                path.to_string_lossy().into_owned(),
            ));
        }
    }

    fn refresh(&mut self, _: &Row, _: &mut Window, cx: &mut Context<Self>) {
        self.reload(cx);
    }

    fn download_stubs(&mut self, _: &Row, _: &mut Window, cx: &mut Context<Self>) {
        let fs = self.project.read(cx).fs().clone();
        let http = cx.http_client();
        self.downloading = true;
        self.expanded.insert(PHP.into());
        self.rebuild(cx);
        let download = cx.background_spawn(download_stubs(fs, http));
        let workspace = self.workspace.clone();
        self._download = cx.spawn(async move |this, cx| {
            let result = download.await;
            this.update(cx, |this, cx| {
                this.downloading = false;
                this.reload(cx);
            })
            .ok();
            let message = match result {
                Ok(()) => format!("PHP stubs downloaded to {}", stubs_dir().display()),
                Err(error) => format!("Couldn't download the PHP stubs: {error:#}"),
            };
            workspace
                .update(cx, |workspace, cx| {
                    workspace.show_toast(
                        Toast::new(NotificationId::unique::<LibraryRows>(), message).autohide(),
                        cx,
                    )
                })
                .ok();
        });
    }

    /// Selects the active editor's file in the tree when it is a library file.
    fn reveal_active(&mut self, workspace: Entity<Workspace>, cx: &mut Context<Self>) {
        let Some(path) = workspace
            .read(cx)
            .active_item(cx)
            .and_then(|item| item.project_path(cx))
            .and_then(|project_path| self.project.read(cx).absolute_path(&project_path, cx))
        else {
            return;
        };
        let Some(libraries) = &self.libraries else {
            return;
        };
        let mut owner: Option<(Vec<SharedString>, PathBuf)> = None;
        if let Some(php) = &libraries.php {
            for (extension, folder) in &php.extensions {
                if let Some(folder) = folder
                    && path.starts_with(folder)
                {
                    owner = Some((
                        vec![ROOT.into(), PHP.into(), format!("php/{extension}").into()],
                        folder.clone(),
                    ));
                }
            }
        }
        for (index, group) in libraries.groups.iter().enumerate() {
            for (package, installed) in &group.packages {
                if *installed && path.starts_with(&package.path) {
                    owner = Some((
                        vec![
                            ROOT.into(),
                            format!("group/{index}").into(),
                            package.path.to_string_lossy().into_owned().into(),
                        ],
                        package.path.clone(),
                    ));
                }
            }
        }
        let Some((keys, base)) = owner else {
            return;
        };
        self.expanded.extend(keys);
        let mut folders = vec![base.clone()];
        let mut folder = path.parent();
        while let Some(current) = folder
            && current.starts_with(&base)
            && current != base
        {
            self.expanded
                .insert(current.to_string_lossy().into_owned().into());
            folders.push(current.to_path_buf());
            folder = current.parent();
        }
        for folder in folders {
            if !self.listings.contains_key(&folder) {
                self.load_listing(folder, cx);
            }
        }
        self.selected = Some(path.to_string_lossy().into_owned().into());
        self.rebuild(cx);
    }
}

fn note(text: &str) -> Row {
    Row {
        key: format!("note/{text}").into(),
        kind: RowKind::Note,
        label: text.to_string().into(),
        detail: None,
        dimmed: true,
        path: None,
        depth: 1,
    }
}

impl ProjectPanelExtraRows for LibraryRows {
    fn row_count(&self, _: &App) -> usize {
        self.rows.len()
    }

    fn row_depth(&self, index: usize, _: &App) -> usize {
        self.rows.get(index).map_or(0, |row| row.depth)
    }

    fn render_row(
        &mut self,
        index: usize,
        state: ExtraRowState,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(row) = self.rows.get(index).cloned() else {
            return div().into_any_element();
        };
        let selected = !state.panel_has_selection && self.selected.as_ref() == Some(&row.key);
        let expanded = self.expanded.contains(&row.key);
        let chevron = row.expandable().then(|| {
            FileIcons::get_chevron_icon(expanded, cx)
                .map(|chevron| Icon::from_path(chevron).color(Color::Muted))
                .unwrap_or_else(|| {
                    Icon::new(if expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    })
                    .color(Color::Muted)
                })
        });
        let file_icon = |name: &str| {
            FileIcons::get_icon(Path::new(name), cx)
                .map(Icon::from_path)
                .unwrap_or_else(|| Icon::new(IconName::File))
        };
        let icon = match row.kind {
            RowKind::Root => Icon::new(IconName::BookCopy),
            RowKind::Php => file_icon("runtime.php"),
            RowKind::PhpExtension => Icon::new(IconName::FileCode),
            RowKind::Group(GroupKind::Composer) => file_icon("composer.json"),
            RowKind::Group(GroupKind::Node) => file_icon("package.json"),
            RowKind::Package => Icon::new(if expanded {
                IconName::BoxOpen
            } else {
                IconName::Box
            }),
            RowKind::Dir => FileIcons::get_folder_icon(expanded, Path::new(row.label.as_ref()), cx)
                .map(Icon::from_path)
                .unwrap_or_else(|| Icon::new(IconName::Folder)),
            RowKind::File => file_icon(row.label.as_ref()),
            RowKind::Note => Icon::new(IconName::Info),
        }
        .color(Color::Muted);
        let label_color = if row.dimmed {
            Color::Muted
        } else {
            Color::Default
        };
        let menu = self
            .context_menu
            .as_ref()
            .filter(|(_, _, key, _)| *key == row.key)
            .map(|(menu, position, _, _)| (menu.clone(), *position));
        let click_row = row.clone();
        let menu_row = row.clone();
        project_panel::extra_row_container(
            SharedString::from(format!("library-row-{index}")),
            selected,
            state.panel_focused,
            cx,
        )
        .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
            this.click(&click_row, event, window, cx)
        }))
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                this.deploy_menu(menu_row.clone(), event.position, window, cx)
            }),
        )
        .child(
            project_panel::extra_row_item(
                SharedString::from(format!("library-item-{index}")),
                row.depth,
                cx,
            )
            .child(
                h_flex()
                    .gap_0p5()
                    .child(match chevron {
                        Some(chevron) => chevron.into_any_element(),
                        None => h_flex()
                            .size(IconSize::default().rems())
                            .flex_none()
                            .into_any_element(),
                    })
                    .child(icon),
            )
            .child(
                h_flex()
                    .h_6()
                    .min_w_0()
                    .gap_1p5()
                    .child(
                        Label::new(row.label)
                            .color(label_color)
                            .single_line()
                            .truncate(),
                    )
                    .when_some(row.detail, |label, detail| {
                        label.child(
                            Label::new(detail)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .single_line(),
                        )
                    }),
            ),
        )
        .children(menu.map(|(menu, position)| {
            deferred(
                anchored()
                    .position(position)
                    .anchor(gpui::Anchor::TopLeft)
                    .child(menu),
            )
            .with_priority(1)
        }))
        .into_any_element()
    }
}
