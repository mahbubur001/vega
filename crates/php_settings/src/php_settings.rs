//! A project's PHP settings, like PhpStorm's Settings › PHP: language level,
//! CLI interpreter, include paths, the stubs the language server loads, and
//! Composer options. Saved in `.vega/php.json`; the language-server parts go
//! to `.vega/settings.json` for phpactor and Intelephense.

pub mod config;
pub mod interpreters;
pub mod store;
pub mod stubs;
mod window;

use gpui::App;
use workspace::Workspace;
use zed_actions::php::OpenPhpSettings;

pub use config::PhpConfig;
pub use store::{PhpSettingsStore, cached_config, config_for_project, project_root};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenPhpSettings, _, cx| {
            open_for_workspace(workspace, cx)
        });
    })
    .detach();
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
