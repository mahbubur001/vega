//! "External Libraries" at the bottom of the project panel, like PhpStorm:
//! the PHP interpreter's extensions (with JetBrains' phpstorm-stubs when
//! downloaded), Composer packages from `composer.lock` and Node.js packages
//! from `package.json`. Library files open read-only.

mod model;
mod protection;
mod rows;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, anyhow};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::{StreamExt as _, io::BufReader};
use gpui::{App, AppContext as _, Entity, Window};
use http_client::{AsyncBody, HttpClient};
use project_panel::ProjectPanel;
use workspace::Workspace;

use crate::model::{
    Package, composer_packages, installed_version, node_dependencies, parse_php_info,
};

/// Where downloaded PHP stubs live (one folder per PHP extension).
pub fn stubs_dir() -> PathBuf {
    paths::data_dir().join("php-stubs")
}

const STUBS_URL: &str =
    "https://github.com/JetBrains/phpstorm-stubs/archive/refs/heads/master.tar.gz";

const PHP_INFO: &str = r#"echo PHP_VERSION, "\n", implode("\n", get_loaded_extensions());"#;

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
    pub groups: Vec<PackageGroup>,
}

/// Reads everything the External Libraries node lists. `roots` are the
/// project folders.
pub(crate) async fn load_libraries(fs: Arc<dyn Fs>, roots: Vec<PathBuf>, php: String) -> Libraries {
    let mut libraries = Libraries {
        php: load_php(fs.as_ref(), &php).await,
        groups: Vec::new(),
    };
    for root in roots {
        let folder = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        if let Ok(lock) = fs.load(&root.join("composer.lock")).await {
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

async fn load_php(fs: &dyn Fs, php: &str) -> Option<PhpRuntime> {
    let output = util::command::new_command(php)
        .args(["-r", PHP_INFO])
        .output()
        .await
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let (version, extensions) = parse_php_info(&String::from_utf8_lossy(&output.stdout))?;
    let stubs = stub_folders(fs).await;
    let has_stubs = !stubs.is_empty();
    let extensions = extensions
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
        version,
        extensions,
        has_stubs,
    })
}

/// The extension folders of the downloaded stubs.
async fn stub_folders(fs: &dyn Fs) -> Vec<(String, PathBuf)> {
    let mut folders = Vec::new();
    let Ok(mut entries) = fs.read_dir(&stubs_dir()).await else {
        return folders;
    };
    while let Some(Ok(path)) = entries.next().await {
        if fs.is_dir(&path).await
            && let Some(name) = path.file_name()
        {
            folders.push((name.to_string_lossy().into_owned(), path));
        }
    }
    folders
}

/// Downloads JetBrains' phpstorm-stubs (Apache-2.0) into `stubs_dir()`.
pub(crate) async fn download_stubs(
    fs: Arc<dyn Fs>,
    http: Arc<dyn HttpClient>,
) -> anyhow::Result<()> {
    let target = stubs_dir();
    let staging = target.with_extension("download");
    fs.remove_dir(
        &staging,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    fs.create_dir(&staging).await?;

    let mut response = http
        .get(STUBS_URL, AsyncBody::default(), true)
        .await
        .context("couldn't reach GitHub")?;
    if !response.status().is_success() {
        return Err(anyhow!("GitHub answered {}", response.status()));
    }
    let archive = Archive::new(GzipDecoder::new(BufReader::new(response.body_mut())));
    archive
        .unpack(&staging)
        .await
        .context("couldn't unpack the stubs")?;

    // The archive holds one folder (phpstorm-stubs-master); it becomes `stubs_dir()`.
    let mut entries = fs.read_dir(&staging).await?;
    let unpacked = entries
        .next()
        .await
        .transpose()?
        .ok_or_else(|| anyhow!("the stubs archive was empty"))?;
    fs.remove_dir(
        &target,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    fs.rename(&unpacked, &target, RenameOptions::default())
        .await?;
    fs.remove_dir(
        &staging,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    Ok(())
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
        let libraries = load_libraries(
            fs.clone(),
            vec![PathBuf::from("/site")],
            "vega-no-such-php".into(),
        )
        .await;
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
