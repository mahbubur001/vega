//! "External Libraries" at the bottom of the project panel, like PhpStorm:
//! "PHP" with the include paths outside the project, and "Project @types/*"
//! with `node_modules/@types`. Shown only for projects that have either,
//! unless the user shows or hides it for the project. Library files open
//! read-only.

mod model;
mod protection;
mod rows;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Window};
use php_settings::PhpConfig;
use php_settings::stubs::stub_folders;
use project_panel::ProjectPanel;
use workspace::Workspace;

pub use php_settings::stubs::stubs_dir;

use crate::model::{Package, installed_version};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        if !workspace.project().read(cx).is_local() {
            return;
        }
        protection::protect_library_files(workspace, cx);

        // The "Non-project file" bar, in every pane.
        let weak_workspace = cx.entity().downgrade();
        let project = workspace.project().clone();
        let add_bar = move |pane: &Entity<workspace::Pane>, window: &mut Window, cx: &mut App| {
            let bar =
                cx.new(|_| protection::NonProjectBar::new(weak_workspace.clone(), project.clone()));
            pane.update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(bar, window, cx));
            });
        };
        for pane in workspace.panes().to_vec() {
            add_bar(&pane, window, cx);
        }
        cx.subscribe_in(
            &cx.entity(),
            window,
            move |_, _, event: &workspace::Event, window, cx| {
                if let workspace::Event::PaneAdded(pane) = event {
                    add_bar(pane, window, cx);
                }
            },
        )
        .detach();
    })
    .detach();

    cx.observe_new(|panel: &mut ProjectPanel, window, cx| {
        if window.is_none() || !panel.project().read(cx).is_local() {
            return;
        }
        let workspace = panel.workspace();
        let project = panel.project().clone();
        let panel_handle = cx.entity().downgrade();
        let rows =
            cx.new(|cx| rows::LibraryRows::new(workspace.clone(), project, panel_handle, cx));
        let weak_rows = rows.downgrade();
        panel.add_extra_rows(rows, cx);
        // Tools › Show/Hide External Libraries, for this project.
        if let Some(workspace) = workspace.upgrade() {
            cx.defer(move |cx| {
                workspace.update(cx, |workspace, _| {
                    workspace.register_action(
                        move |_, _: &zed_actions::php::ToggleExternalLibraries, _, cx| {
                            weak_rows
                                .update(cx, |rows, cx| rows.toggle_visibility(cx))
                                .ok();
                        },
                    );
                });
            });
        }
    })
    .detach();
}

/// `node_modules/@types` packages of one project folder (PhpStorm's
/// "Project @types/*" library).
#[derive(Clone, Debug)]
pub(crate) struct TypesGroup {
    /// The project folder's name, shown when the project has several.
    pub folder: String,
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Libraries {
    /// Whether JetBrains' phpstorm-stubs are downloaded (searched by Find in
    /// Files with the include paths).
    pub has_stubs: bool,
    /// Whether the project is a PHP project (`composer.json`, VEGA PHP
    /// settings, or include paths): the "PHP" node is shown.
    pub is_php_project: bool,
    /// Include paths outside the project folders, with whether each exists.
    /// Paths inside the project are already in the project tree.
    pub include_paths: Vec<(PathBuf, bool)>,
    pub types: Vec<TypesGroup>,
}

impl Libraries {
    /// Whether External Libraries has anything to show for this project
    /// (when the user hasn't chosen to show or hide it).
    pub fn has_content(&self) -> bool {
        self.is_php_project || !self.types.is_empty()
    }
}

/// Reads everything the External Libraries node lists. `roots` are the
/// project folders; `config` is the project's PHP settings.
pub(crate) async fn load_libraries(
    fs: Arc<dyn Fs>,
    roots: Vec<PathBuf>,
    config: PhpConfig,
) -> Libraries {
    let all_include_paths = config.include_paths();
    let mut is_php_project = !all_include_paths.is_empty();
    for root in &roots {
        if fs.is_file(&root.join("composer.json")).await
            || fs.is_file(&php_settings::config::config_path(root)).await
        {
            is_php_project = true;
        }
    }
    let mut libraries = Libraries {
        has_stubs: is_php_project && !stub_folders(fs.as_ref()).await.is_empty(),
        is_php_project,
        include_paths: Vec::new(),
        types: Vec::new(),
    };
    for path in outside_roots(all_include_paths, &roots) {
        let exists = fs.is_dir(&path).await;
        libraries.include_paths.push((path, exists));
    }
    for root in roots {
        let folder = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let types_dir = root.join("node_modules").join("@types");
        let mut packages = Vec::new();
        for (name, is_dir) in list_dir(fs.as_ref(), &types_dir).await {
            if !is_dir {
                continue;
            }
            let path = types_dir.join(&name);
            let version = match fs.load(&path.join("package.json")).await {
                Ok(json) => installed_version(&json),
                Err(_) => None,
            };
            packages.push(Package {
                name,
                version,
                path,
                dev: false,
            });
        }
        if !packages.is_empty() {
            libraries.types.push(TypesGroup { folder, packages });
        }
    }
    libraries
}

/// The include paths that aren't inside one of the project folders.
fn outside_roots(paths: Vec<PathBuf>, roots: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|path| !roots.iter().any(|root| path.starts_with(root)))
        .collect()
}

/// Names and kinds of the entries in `dir`, folders first, then by name.
pub(crate) async fn list_dir(fs: &dyn Fs, dir: &Path) -> Vec<(String, bool)> {
    let mut entries = Vec::new();
    if let Ok(mut paths) = fs.read_dir(dir).await {
        while let Some(Ok(path)) = paths.next().await {
            let Some(name) = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
            else {
                continue;
            };
            if name == ".DS_Store" {
                continue;
            }
            entries.push((name, fs.is_dir(&path).await));
        }
    }
    entries.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
    });
    entries
}

#[cfg(test)]
mod tests {
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;

    use super::*;

    #[gpui::test]
    async fn lists_outside_include_paths_and_types(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/site",
            json!({
                "composer.json": "{}",
                "vendor": { "psr": { "log": {} } },
                "node_modules": {
                    "lodash": { "package.json": r#"{"version": "4.17.21"}"#, "index.js": "" },
                    "@types": {
                        "react": { "package.json": r#"{"version": "19.0.1"}"#, "index.d.ts": "" },
                        "lodash": { "index.d.ts": "" },
                    },
                },
            }),
        )
        .await;
        fs.insert_tree("/plain", json!({ "README.md": "" })).await;
        let config = PhpConfig {
            interpreter: Some("vega-no-such-php".into()),
            include_paths: vec!["/libs/wordpress".into(), "/site/vendor/psr/log".into()],
            ..Default::default()
        };
        let libraries = load_libraries(fs.clone(), vec![PathBuf::from("/site")], config).await;
        assert!(libraries.is_php_project && libraries.has_content());
        // vendor/ is inside the project: already in the project tree.
        assert_eq!(
            libraries.include_paths,
            [(PathBuf::from("/libs/wordpress"), false)]
        );
        assert!(!libraries.has_stubs);
        assert_eq!(libraries.types.len(), 1);
        let types: Vec<(&str, Option<&str>)> = libraries.types[0]
            .packages
            .iter()
            .map(|package| (package.name.as_str(), package.version.as_deref()))
            .collect();
        assert_eq!(types, [("lodash", None), ("react", Some("19.0.1"))]);

        let plain = load_libraries(
            fs.clone(),
            vec![PathBuf::from("/plain")],
            PhpConfig::default(),
        )
        .await;
        assert!(!plain.has_content());

        assert_eq!(
            list_dir(fs.as_ref(), Path::new("/site/node_modules/lodash")).await,
            [
                ("index.js".to_string(), false),
                ("package.json".to_string(), false)
            ]
        );
    }
}
