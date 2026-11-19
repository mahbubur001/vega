//! "External Libraries" at the bottom of the project panel, like PhpStorm:
//! the PHP interpreter's extensions (with JetBrains' phpstorm-stubs when
//! downloaded), Composer packages from `composer.lock` and Node.js packages
//! from `package.json`. Library files open read-only.

mod model;
mod protection;
mod rows;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, AppContext as _, Entity, Window};
use php_settings::PhpConfig;
use php_settings::interpreters::php_info;
use php_settings::stubs::stub_folders;
use project_panel::ProjectPanel;
use workspace::Workspace;

pub use php_settings::stubs::stubs_dir;

use crate::model::{Package, composer_packages, installed_version, node_dependencies};

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
        let rows = cx.new(|cx| rows::LibraryRows::new(workspace, project, panel_handle, cx));
        panel.add_extra_rows(rows, cx);
    })
    .detach();
}

/// The PHP interpreter VEGA found, and its extensions.
#[derive(Clone, Debug, Default)]
pub(crate) struct PhpRuntime {
    pub version: String,
    /// Extension name and, when stubs are downloaded, its stubs folder.
    pub extensions: Vec<(String, Option<PathBuf>)>,
    pub has_stubs: bool,
}

/// Packages of one project folder.
#[derive(Clone, Debug)]
pub(crate) struct PackageGroup {
    pub kind: GroupKind,
    /// The project folder's name, shown when the project has several.
    pub folder: String,
    pub packages: Vec<(Package, bool)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GroupKind {
    Composer,
    Node,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Libraries {
    pub php: Option<PhpRuntime>,
    /// Include paths from the PHP settings, with whether each exists.
    pub include_paths: Vec<(PathBuf, bool)>,
    pub groups: Vec<PackageGroup>,
}

/// Reads everything the External Libraries node lists. `roots` are the
/// project folders; `config` is the project's PHP settings.
pub(crate) async fn load_libraries(
    fs: Arc<dyn Fs>,
    roots: Vec<PathBuf>,
    config: PhpConfig,
) -> Libraries {
    let mut libraries = Libraries {
        php: load_php(fs.as_ref(), &config).await,
        include_paths: Vec::new(),
        groups: Vec::new(),
    };
    for path in config.include_paths() {
        let exists = fs.is_dir(&path).await;
        libraries.include_paths.push((path, exists));
    }
    for root in roots {
        let folder = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        if config.composer_libraries
            && let Ok(lock) = fs.load(&root.join("composer.lock")).await
        {
            let mut packages = Vec::new();
            for package in composer_packages(&lock, &root) {
                let installed = fs.is_dir(&package.path).await;
                packages.push((package, installed));
            }
            if !packages.is_empty() {
                libraries.groups.push(PackageGroup {
                    kind: GroupKind::Composer,
                    folder: folder.clone(),
                    packages,
                });
            }
        }
        if let Ok(package_json) = fs.load(&root.join("package.json")).await {
            let mut packages = Vec::new();
            for (name, dev) in node_dependencies(&package_json) {
                let path = root.join("node_modules").join(&name);
                let version = match fs.load(&path.join("package.json")).await {
                    Ok(json) => installed_version(&json),
                    Err(_) => None,
                };
                let installed = version.is_some();
                packages.push((
                    Package {
                        name,
                        version,
                        path,
                        dev,
                    },
                    installed,
                ));
            }
            if !packages.is_empty() {
                libraries.groups.push(PackageGroup {
                    kind: GroupKind::Node,
                    folder,
                    packages,
                });
            }
        }
    }
    libraries
}

/// The interpreter's extensions, or the extensions chosen in the PHP
/// settings, each with its stubs folder when stubs are downloaded.
async fn load_php(fs: &dyn Fs, config: &PhpConfig) -> Option<PhpRuntime> {
    let info = php_info(&config.php()).await?;
    let stubs = stub_folders(fs).await;
    let has_stubs = !stubs.is_empty();
    let names = config.stubs.clone().unwrap_or(info.extensions);
    let extensions = names
        .into_iter()
        .map(|extension| {
            let folder = stubs
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(&extension))
                .map(|(_, path)| path.clone());
            (extension, folder)
        })
        .collect();
    Some(PhpRuntime {
        version: info.version,
        extensions,
        has_stubs,
    })
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
    async fn lists_composer_and_node_packages(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/site",
            json!({
                "composer.lock": r#"{"packages": [
                    {"name": "psr/log", "version": "3.0.2"},
                    {"name": "missing/package", "version": "1.0.0"}
                ]}"#,
                "package.json": r#"{"dependencies": {"lodash": "^4", "left-pad": "^1"}}"#,
                "vendor": { "psr": { "log": { "src": { "LoggerInterface.php": "" } } } },
                "node_modules": {
                    "lodash": { "package.json": r#"{"version": "4.17.21"}"#, "index.js": "" }
                },
            }),
        )
        .await;
        let config = PhpConfig {
            interpreter: Some("vega-no-such-php".into()),
            include_paths: vec!["/libs/wordpress".into()],
            ..Default::default()
        };
        let libraries = load_libraries(fs.clone(), vec![PathBuf::from("/site")], config).await;
        assert_eq!(
            libraries.include_paths,
            [(PathBuf::from("/libs/wordpress"), false)]
        );
        assert!(libraries.php.is_none());
        assert_eq!(libraries.groups.len(), 2);

        let composer = &libraries.groups[0];
        assert_eq!(composer.kind, GroupKind::Composer);
        assert_eq!(composer.folder, "site");
        let composer: Vec<(&str, bool)> = composer
            .packages
            .iter()
            .map(|(package, installed)| (package.name.as_str(), *installed))
            .collect();
        assert_eq!(composer, [("missing/package", false), ("psr/log", true)]);

        let node: Vec<(&str, Option<&str>, bool)> = libraries.groups[1]
            .packages
            .iter()
            .map(|(package, installed)| {
                (
                    package.name.as_str(),
                    package.version.as_deref(),
                    *installed,
                )
            })
            .collect();
        assert_eq!(
            node,
            [("left-pad", None, false), ("lodash", Some("4.17.21"), true)]
        );

        assert_eq!(
            list_dir(fs.as_ref(), Path::new("/site/node_modules/lodash")).await,
            [
                ("index.js".to_string(), false),
                ("package.json".to_string(), false)
            ]
        );
    }
}
