//! Deployment in VEGA: upload on save, Upload/Download/Compare actions,
//! transfer progress, the Remote Host panel and Sync with Deployed, on top of the `deployment`
//! engine.

mod name_modal;
mod permissions_modal;
mod prompter;
mod remote_host_panel;
mod secret_modal;
mod secrets;
mod servers_modal;
mod store;
mod sync_view;
mod transfers_indicator;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use deployment::engine::Engine;
use deployment::project::CONFIG_PATH;
use deployment::prompt::Level;
use futures::StreamExt as _;
use futures::channel::mpsc;
use git_ui_core::file_diff_view::FileDiffView;
use gpui::{App, AppContext as _, Entity, Focusable as _, PromptLevel, Window};
use gpui_tokio::Tokio;
use project_panel::ProjectPanel;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::notifications::NotificationId;
use workspace::{OpenOptions, Toast, Workspace};
use zed_actions::deployment::{
    CancelTransfers, CompareWithDeployedVersion, DownloadFromDefaultServer, OpenDeploymentConfig,
    OpenDeploymentServers, SyncWithDeployed, ToggleRemoteHost, UploadToDefaultServer,
};

use crate::prompter::{Request, WindowPrompter};
use crate::remote_host_panel::RemoteHostPanel;
use crate::secret_modal::SecretModal;
use crate::store::DeploymentStore;
use crate::transfers_indicator::TransfersIndicator;

pub use store::TransfersStatus;

/// A starter config for `OpenDeploymentConfig`. See `vega/DEPLOYMENT.md`.
const CONFIG_TEMPLATE: &str = r#"{
  "version": 1,
  "defaultServer": "staging",
  "options": {
    "autoUpload": "onExplicitSave"
  },
  "servers": [
    {
      "id": "staging",
      "name": "Staging",
      "type": "sftp",
      "host": "example.com",
      "port": 22,
      "username": "deploy",
      "rootPath": "/var/www/site",
      "mappings": [{ "local": "", "remote": "" }]
    }
  ]
}
"#;

struct DeploymentToast;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let project = workspace.project().clone();
        if !project.read(cx).is_local() {
            return;
        }

        let (prompter, requests) = WindowPrompter::new();
        let store = cx.new(|cx| DeploymentStore::new(project, Arc::new(prompter), cx));
        show_requests(requests, window, cx);

        let indicator = cx.new(|cx| TransfersIndicator::new(store.clone(), cx));
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_right_item(indicator, window, cx);
        });

        let weak_workspace = cx.weak_entity();
        let panel = cx.new(|cx| RemoteHostPanel::new(store.clone(), weak_workspace, cx));
        workspace.add_panel(panel, window, cx);

        register_actions(workspace, store);
    })
    .detach();
}

fn register_actions(workspace: &mut Workspace, store: Entity<DeploymentStore>) {
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &UploadToDefaultServer, window, cx| {
            run_on_selection(workspace, &store, Operation::Upload, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &DownloadFromDefaultServer, window, cx| {
            run_on_selection(workspace, &store, Operation::Download, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &CompareWithDeployedVersion, window, cx| {
            run_on_selection(workspace, &store, Operation::Compare, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &SyncWithDeployed, window, cx| {
            sync_with_deployed(workspace, &store, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |_, _: &CancelTransfers, _, cx| store.read(cx).cancel_transfers()
    });
    workspace.register_action({
        move |workspace, _: &OpenDeploymentServers, window, cx| {
            let Some((root, engine, prompter)) = store.read(cx).servers_dialog_context(cx) else {
                return;
            };
            let credentials = secrets::credentials(cx);
            workspace.toggle_modal(window, cx, |window, cx| {
                servers_modal::ServersModal::new(root, engine, prompter, credentials, window, cx)
            });
        }
    });
    workspace.register_action(|workspace, _: &ToggleRemoteHost, window, cx| {
        workspace.toggle_panel_focus::<RemoteHostPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &OpenDeploymentConfig, window, cx| {
        open_config(workspace, window, cx);
    });
}

#[derive(Clone, Copy)]
enum Operation {
    Upload,
    Download,
    Compare,
}

/// Runs `operation` on the project panel selection when it has focus,
/// otherwise on the active file.
fn run_on_selection(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    operation: Operation,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(path) = selected_path(workspace, window, cx) else {
        show_message(workspace, "Select a file to deploy.", cx);
        return;
    };
    let Some(engine) = store.read(cx).engine_for_path(&path) else {
        show_message(
            workspace,
            "This project has no deployment servers. Run \"deployment: open deployment config\" to add one.",
            cx,
        );
        return;
    };

    let task = Tokio::spawn(cx, run(engine, operation, path));
    cx.spawn_in(window, async move |workspace, cx| {
        let message = match task.await {
            Ok(Ok(Some(message))) => message,
            Ok(Ok(None)) => return,
            Ok(Err(error)) => error.to_string(),
            Err(error) => format!("Deployment task failed: {error}"),
        };
        workspace
            .update(cx, |workspace, cx| show_message(workspace, &message, cx))
            .ok();
    })
    .detach();
}

/// Returns a message to show when done, if any.
async fn run(
    engine: Arc<Engine>,
    operation: Operation,
    path: PathBuf,
) -> deployment::Result<Option<String>> {
    let server = engine
        .default_server()
        .map(|server| server.server.name)
        .unwrap_or_else(|| "the server".to_string());
    match operation {
        Operation::Upload => {
            let summary = engine.upload(None, &[path], true).await?;
            Ok(Some(summary.describe("Uploaded", &server)))
        }
        Operation::Download => {
            let summary = engine.download(None, &[path]).await?;
            Ok(Some(summary.describe("Downloaded", &server)))
        }
        // The engine opens the diff through the prompter.
        Operation::Compare => engine.compare(None, &path).await.map(|_| None),
    }
}

/// Opens a Sync view for the selected file or folder, or for the whole
/// project when the selection is the project root or there is none.
fn sync_with_deployed(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let selected = selected_path(workspace, window, cx);
    let engine = selected
        .as_ref()
        .and_then(|path| store.read(cx).engine_for_path(path))
        .or_else(|| store.read(cx).engines().into_iter().next());
    let Some(engine) = engine else {
        show_message(
            workspace,
            "This project has no deployment servers. Run \"deployment: open deployment servers\" to add one.",
            cx,
        );
        return;
    };
    let roots = selected
        .filter(|path| path.starts_with(engine.root()) && path != engine.root())
        .into_iter()
        .collect();
    sync_view::open(workspace, engine, roots, window, cx);
}

fn selected_path(workspace: &Workspace, window: &Window, cx: &App) -> Option<PathBuf> {
    let project = workspace.project().read(cx);
    if let Some(panel) = workspace.panel::<ProjectPanel>(cx)
        && panel.focus_handle(cx).contains_focused(window, cx)
        && let Some(project_path) = panel.read(cx).selected_entry_project_path(cx)
    {
        return project.absolute_path(&project_path, cx);
    }
    let project_path = workspace.active_item(cx)?.project_path(cx)?;
    project.absolute_path(&project_path, cx)
}

fn open_config(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let roots: Vec<_> = workspace
        .project()
        .read(cx)
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).abs_path())
        .collect();
    let root = selected_path(workspace, window, cx)
        .and_then(|path| roots.iter().find(|root| path.starts_with(root)).cloned())
        .or_else(|| roots.first().cloned());
    let Some(root) = root else {
        show_message(workspace, "Open a folder to set up deployment.", cx);
        return;
    };

    let path = root.join(CONFIG_PATH);
    let fs = workspace.app_state().fs.clone();
    cx.spawn_in(window, async move |workspace, cx| {
        if !fs.is_file(&path).await {
            if let Some(parent) = path.parent() {
                fs.create_dir(parent).await?;
            }
            fs.atomic_write(path.clone(), CONFIG_TEMPLATE.to_string())
                .await
                .context("creating the deployment config")?;
        }
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(path, OpenOptions::default(), window, cx)
            })?
            .await?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

/// Shows the engine's questions and messages in the workspace window.
fn show_requests(
    mut requests: mpsc::UnboundedReceiver<Request>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    cx.spawn_in(window, async move |workspace, cx| {
        while let Some(request) = requests.next().await {
            let handled = workspace.update_in(cx, |workspace, window, cx| {
                show_request(request, workspace, window, cx);
            });
            if handled.is_err() {
                break;
            }
        }
    })
    .detach();
}

fn show_request(
    request: Request,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    match request {
        Request::Secret { request, reply } => {
            workspace.toggle_modal(window, cx, |window, cx| {
                SecretModal::new(request, reply, window, cx)
            });
        }
        Request::Choice {
            level,
            message,
            detail,
            answers,
            reply,
        } => {
            let level = match level {
                Level::Info => PromptLevel::Info,
                Level::Warning => PromptLevel::Warning,
                Level::Error => PromptLevel::Critical,
            };
            let answers: Vec<&str> = answers.iter().map(String::as_str).collect();
            let answer = window.prompt(level, &message, Some(&detail), &answers, cx);
            cx.background_spawn(async move {
                if let Ok(index) = answer.await {
                    reply.send(index).ok();
                }
            })
            .detach();
        }
        Request::Diff { old, new, reply } => {
            let open = FileDiffView::open(old, new, None, cx.weak_entity(), window, cx);
            cx.background_spawn(async move {
                open.await.log_err();
                reply.send(()).ok();
            })
            .detach();
        }
        Request::Notify { level, message } => {
            let toast = Toast::new(NotificationId::unique::<DeploymentToast>(), message);
            let toast = if level == Level::Info {
                toast.autohide()
            } else {
                toast
            };
            workspace.show_toast(toast, cx);
        }
    }
}

fn show_message(workspace: &mut Workspace, message: &str, cx: &mut Context<Workspace>) {
    workspace.show_toast(
        Toast::new(
            NotificationId::unique::<DeploymentToast>(),
            message.to_string(),
        )
        .autohide(),
        cx,
    );
}
