//! Deployment in VEGA: upload on save, Upload/Download/Compare actions,
//! transfer progress, the Remote Host panel and Sync with Deployed, on top of the `deployment`
//! engine.

mod deployment_window;
mod permissions_modal;
mod prompter;
mod remote_edit_bar;
mod remote_host_panel;
mod secret_modal;
mod secrets;
mod server_picker;
mod servers_modal;
mod store;
mod sync_view;
mod transfers_indicator;
mod transfers_panel;

pub(crate) use vega_dialog::{dialog, name_modal};

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use deployment::config::AutoUpload;
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
use workspace::{MultiWorkspace, OpenOptions, Toast, Workspace};
use zed_actions::deployment::{
    CancelTransfers, CompareWithDeployedTo, CompareWithDeployedVersion, DownloadFrom,
    DownloadFromDefaultServer, EditRemoteFile, OpenDeploymentConfig, OpenDeploymentOptions,
    OpenDeploymentServers, SyncWithDeployed, SyncWithDeployedTo, ToggleAutomaticUpload,
    ToggleFileTransfers, ToggleRemoteHost, UploadTo, UploadToDefaultServer,
};

use crate::server_picker::ServerPicker;

use crate::prompter::{Request, WindowPrompter};
use crate::remote_host_panel::RemoteHostPanel;
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

        let weak_workspace = cx.weak_entity();
        let indicator =
            cx.new(|cx| TransfersIndicator::new(store.clone(), weak_workspace.clone(), cx));
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_right_item(indicator, window, cx);
        });

        let panel = cx.new(|cx| RemoteHostPanel::new(store.clone(), weak_workspace.clone(), cx));
        workspace.add_panel(panel, window, cx);
        let transfers =
            cx.new(|cx| transfers_panel::TransfersPanel::new(store.clone(), weak_workspace, cx));
        workspace.add_panel(transfers, window, cx);

        // The bar above editors showing a server copy, in every pane.
        let add_bar = {
            let store = store.clone();
            move |pane: &Entity<workspace::Pane>, window: &mut Window, cx: &mut App| {
                let bar = cx.new(|_| remote_edit_bar::RemoteEditBar::new(store.clone()));
                pane.update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(bar, window, cx));
                });
            }
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

        register_actions(workspace, store);
    })
    .detach();
}

fn register_actions(workspace: &mut Workspace, store: Entity<DeploymentStore>) {
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &UploadToDefaultServer, window, cx| {
            run_on_selection(workspace, &store, Operation::Upload, None, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &DownloadFromDefaultServer, window, cx| {
            run_on_selection(workspace, &store, Operation::Download, None, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &CompareWithDeployedVersion, window, cx| {
            run_on_selection(workspace, &store, Operation::Compare, None, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &SyncWithDeployed, window, cx| {
            sync_with_deployed(workspace, &store, None, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &UploadTo, window, cx| {
            pick_server_for(workspace, &store, Operation::Upload, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &DownloadFrom, window, cx| {
            pick_server_for(workspace, &store, Operation::Download, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &CompareWithDeployedTo, window, cx| {
            pick_server_for(workspace, &store, Operation::Compare, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &SyncWithDeployedTo, window, cx| {
            pick_server_for(workspace, &store, Operation::Sync, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &EditRemoteFile, window, cx| {
            edit_remote_file(workspace, &store, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |workspace, _: &ToggleAutomaticUpload, window, cx| {
            toggle_automatic_upload(workspace, &store, window, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |_, _: &OpenDeploymentOptions, window, cx| {
            let handle = window.window_handle().downcast::<MultiWorkspace>();
            deployment_window::open(store.clone(), handle, true, cx);
        }
    });
    workspace.register_action({
        let store = store.clone();
        move |_, _: &CancelTransfers, _, cx| store.read(cx).cancel_transfers()
    });
    workspace.register_action({
        move |_, _: &OpenDeploymentServers, window, cx| {
            let handle = window.window_handle().downcast::<MultiWorkspace>();
            deployment_window::open(store.clone(), handle, false, cx);
        }
    });
    workspace.register_action(|workspace, _: &ToggleRemoteHost, window, cx| {
        workspace.toggle_panel_focus::<RemoteHostPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &ToggleFileTransfers, window, cx| {
        workspace.toggle_panel_focus::<transfers_panel::TransfersPanel>(window, cx);
    });
    workspace.register_action(|workspace, _: &OpenDeploymentConfig, window, cx| {
        open_config(workspace, window, cx);
    });
}

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Upload,
    Download,
    Compare,
    Sync,
}

impl Operation {
    fn title(self) -> &'static str {
        match self {
            Operation::Upload => "Upload to",
            Operation::Download => "Download from",
            Operation::Compare => "Compare with",
            Operation::Sync => "Sync with",
        }
    }
}

/// The engine of the selected file's project folder (or the first one), or a
/// message when the project has no deployment servers.
fn engine_for_selection(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    window: &Window,
    cx: &mut Context<Workspace>,
) -> Option<(Option<PathBuf>, Arc<Engine>)> {
    let selected = selected_path(workspace, window, cx);
    let engine = selected
        .as_ref()
        .and_then(|path| store.read(cx).engine_for_path(path))
        .or_else(|| store.read(cx).engines().into_iter().next());
    match engine {
        Some(engine) => Some((selected, engine)),
        None => {
            show_message(
                workspace,
                "This project has no deployment servers. Use Tools › Deployment › Configuration… to add one.",
                cx,
            );
            None
        }
    }
}

/// "Upload to…" and friends: asks which server, then runs `operation` there.
fn pick_server_for(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    operation: Operation,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some((_, engine)) = engine_for_selection(workspace, store, window, cx) else {
        return;
    };
    // Groups can receive uploads and downloads, not comparisons.
    let groups = matches!(operation, Operation::Upload | Operation::Download);
    let entries = server_picker::entries(&engine, groups);
    let store = store.clone();
    let this = cx.weak_entity();
    workspace.toggle_modal(window, cx, move |window, cx| {
        ServerPicker::new(
            operation.title(),
            entries,
            move |server_id, window, cx| {
                this.update(cx, |workspace, cx| match operation {
                    Operation::Sync => {
                        sync_with_deployed(workspace, &store, Some(server_id), window, cx)
                    }
                    _ => {
                        run_on_selection(workspace, &store, operation, Some(server_id), window, cx)
                    }
                })
                .ok();
            },
            window,
            cx,
        )
    });
}

/// Opens the default server's copy of the selected file; saving it uploads it back.
fn edit_remote_file(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some((Some(path), engine)) = engine_for_selection(workspace, store, window, cx) else {
        return;
    };
    let target = engine
        .targets(None)
        .ok()
        .and_then(|targets| targets.into_iter().next());
    let Some((server_id, remote)) =
        target.and_then(|target| Some((target.server.id.clone(), target.mapper.to_remote(&path)?)))
    else {
        show_message(
            workspace,
            "This file isn't mapped to the default server.",
            cx,
        );
        return;
    };
    let open = Tokio::spawn(cx, async move {
        engine.remote_open(&server_id, &remote, true).await
    });
    cx.spawn_in(window, async move |workspace, cx| {
        let local = match open.await {
            Ok(Ok(local)) => local,
            Ok(Err(error)) => {
                let message = error.to_string();
                workspace
                    .update(cx, |workspace, cx| show_message(workspace, &message, cx))
                    .ok();
                return;
            }
            Err(_) => return,
        };
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(local, OpenOptions::default(), window, cx)
            })
            .ok();
    })
    .detach();
}

/// Tools › Deployment › Automatic Upload: on save ↔ never.
fn toggle_automatic_upload(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some((_, engine)) = engine_for_selection(workspace, store, window, cx) else {
        return;
    };
    let result = {
        let mut config = engine.config_mut();
        let mut options = config.options().clone();
        options.auto_upload = match options.auto_upload {
            AutoUpload::Never => AutoUpload::OnExplicitSave,
            AutoUpload::Always | AutoUpload::OnExplicitSave => AutoUpload::Never,
        };
        let state = options.auto_upload;
        config.set_options(options).map(|_| state)
    };
    let message = match result {
        Ok(AutoUpload::Never) => "Automatic upload is off.".to_string(),
        Ok(_) => "Automatic upload is on: saved files go to the default server.".to_string(),
        Err(error) => format!("Couldn't change automatic upload: {error}"),
    };
    engine.reload_config();
    store.update(cx, |_, cx| cx.notify());
    show_message(workspace, &message, cx);
}

/// Runs `operation` on the project panel selection when it has focus,
/// otherwise on the active file.
fn run_on_selection(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    operation: Operation,
    server_id: Option<String>,
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

    let task = Tokio::spawn(cx, run(engine, operation, server_id, path));
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
    server_id: Option<String>,
    path: PathBuf,
) -> deployment::Result<Option<String>> {
    let server = match &server_id {
        Some(id) => engine
            .servers()
            .into_iter()
            .find(|resolved| resolved.server.id == *id)
            .map(|resolved| resolved.server.name),
        None => engine.default_server().map(|server| server.server.name),
    }
    .unwrap_or_else(|| "the server".to_string());
    let server_id = server_id.as_deref();
    match operation {
        Operation::Upload => {
            let summary = engine.upload(server_id, &[path], true).await?;
            Ok(Some(summary.describe("Uploaded", &server)))
        }
        Operation::Download => {
            let summary = engine.download(server_id, &[path]).await?;
            Ok(Some(summary.describe("Downloaded", &server)))
        }
        // The engine opens the diff through the prompter.
        Operation::Compare => engine.compare(server_id, &path).await.map(|_| None),
        // Sync opens its own view (see `sync_with_deployed`).
        Operation::Sync => Ok(None),
    }
}

/// Opens a Sync view for the selected file or folder, or for the whole
/// project when the selection is the project root or there is none.
fn sync_with_deployed(
    workspace: &mut Workspace,
    store: &Entity<DeploymentStore>,
    server_id: Option<String>,
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
    sync_view::open(workspace, engine, roots, server_id, window, cx);
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

pub(crate) fn show_request(
    request: Request,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    match request {
        Request::Secret { request, reply } => {
            crate::secret_modal::open_window(request, reply, window.bounds(), cx);
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
