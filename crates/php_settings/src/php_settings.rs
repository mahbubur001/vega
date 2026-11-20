//! A project's PHP settings, like PhpStorm's Settings › PHP: language level,
//! CLI interpreter, include paths, the stubs the language server loads, and
//! Composer options. Saved in `.vega/php.json`; the language-server parts go
//! to `.vega/settings.json` for phpactor and Intelephense.

pub mod config;
pub mod interpreters;
mod phpactor;
pub mod store;
pub mod stubs;
mod window;

use gpui::App;
use workspace::Toast;
use workspace::Workspace;
use workspace::notifications::NotificationId;
use zed_actions::php::{OpenPhpSettings, Reindex};

pub use config::PhpConfig;
pub use store::{PhpSettingsStore, cached_config, config_for_project, project_root};

struct PhpToast;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        workspace
            .register_action(|workspace, _: &OpenPhpSettings, _, cx| {
                open_for_workspace(workspace, cx)
            })
            .register_action(|workspace, _: &Reindex, _, cx| {
                let count = phpactor::reindex(&workspace.project().clone(), cx);
                let message = if count == 0 {
                    "phpactor isn't running for this project; open a PHP file first.".to_string()
                } else {
                    "phpactor is rebuilding its index.".to_string()
                };
                workspace.show_toast(
                    Toast::new(NotificationId::unique::<PhpToast>(), message).autohide(),
                    cx,
                );
            });
        if workspace.project().read(cx).is_local() {
            phpactor::watch(workspace.project().clone(), cx);
        }
    })
    .detach();
}

/// Points the project's phpactor at the downloaded PHP stubs (call after
/// downloading them).
pub fn use_downloaded_stubs(project: &gpui::Entity<project::Project>, cx: &mut App) {
    if project.read(cx).is_local() {
        phpactor::sync_stub_settings(project, true, cx);
    }
}

/// Opens the PHP settings of the workspace's project.
pub fn open_for_workspace(workspace: &Workspace, cx: &mut App) {
    let project = workspace.project().clone();
    let Some(root) = project_root(&project, cx) else {
        return;
    };
    let fs = project.read(cx).fs().clone();
    window::open(root, fs, cx);
}
