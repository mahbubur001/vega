//! The Deployment window: the servers dialog in its own resizable OS window,
//! like PhpStorm's Deployment configuration and VEGA's Settings window. Its
//! size is remembered; it is reused when already open. Password prompts raised
//! while testing a connection open centred over this window; diffs and
//! notifications go to the workspace it was opened from.

use std::sync::Arc;
use std::time::Duration;

use db::kvp::KeyValueStore;
use futures::StreamExt as _;
use futures::channel::mpsc;
use gpui::{
    App, AppContext as _, Context, DismissEvent, Entity, FocusHandle, Focusable, PromptLevel,
    Render, Size, Subscription, Task, TitlebarOptions, WindowBounds, WindowHandle, WindowKind,
    WindowOptions, point, px,
};
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use settings::Settings as _;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{MultiWorkspace, WorkspaceSettings};

use deployment::prompt::Level;

use crate::prompter::{Request, WindowPrompter};
use crate::servers_modal::ServersModal;
use crate::store::DeploymentStore;

/// Key-value store entry that remembers the window's size ("width,height").
const WINDOW_SIZE_KEY: &str = "vega_deployment_window_size";
const DEFAULT_SIZE: Size<Pixels> = Size {
    width: px(1000.),
    height: px(680.),
};
const MIN_SIZE: Size<Pixels> = Size {
    width: px(760.),
    height: px(440.),
};

pub(crate) struct DeploymentWindow {
    workspace: Option<WindowHandle<MultiWorkspace>>,
    servers: Entity<ServersModal>,
    title_bar: Option<Entity<PlatformTitleBar>>,
    focus_handle: FocusHandle,
    save_size: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
    _requests: Task<()>,
}

/// Opens the Deployment window for the project of `store`, or brings the open one forward.
/// `options` opens the Options page instead of the selected server.
pub(crate) fn open(
    store: Entity<DeploymentStore>,
    workspace: Option<WindowHandle<MultiWorkspace>>,
    options: bool,
    cx: &mut App,
) {
    let existing = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<DeploymentWindow>());
    if let Some(existing) = existing {
        existing
            .update(cx, |this, window, cx| {
                this.workspace = workspace;
                window.activate_window();
                this.focus_handle.focus(window, cx);
                if options {
                    this.servers
                        .update(cx, |servers, cx| servers.show_options(window, cx));
                }
            })
            .ok();
        return;
    }

    // Defer to get the workspace off the stack (like the Settings window).
    cx.defer(move |cx| {
        let Some((root, engine, _)) = store.read(cx).servers_dialog_context(cx) else {
            return;
        };
        let credentials = crate::secrets::credentials(cx);
        let size = saved_size(cx).unwrap_or(DEFAULT_SIZE);
        let display_id = cx.active_display().map(|display| display.id());
        let window_decorations = match WorkspaceSettings::get_global(cx).window_decorations {
            settings::WindowDecorations::Server => gpui::WindowDecorations::Server,
            settings::WindowDecorations::Client => gpui::WindowDecorations::Client,
        };
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("VEGA — Deployment".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.), px(12.))),
                }),
                focus: true,
                show: true,
                is_movable: true,
                is_resizable: true,
                is_minimizable: true,
                kind: WindowKind::Normal,
                window_background: cx.theme().window_background_appearance(),
                app_id: Some(ReleaseChannel::global(cx).app_id().to_owned()),
                window_decorations: Some(window_decorations),
                window_min_size: Some(MIN_SIZE),
                window_bounds: Some(WindowBounds::centered(display_id, size, cx)),
                display_id,
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let (prompter, requests) = WindowPrompter::new();
                    let servers = cx.new(|cx| {
                        ServersModal::new(
                            root,
                            engine,
                            Arc::new(prompter),
                            credentials,
                            true,
                            window,
                            cx,
                        )
                    });
                    let dismiss = cx.subscribe_in(
                        &servers,
                        window,
                        |_: &mut DeploymentWindow, _, _: &DismissEvent, window, _| {
                            window.remove_window();
                        },
                    );
                    let bounds = cx.observe_window_bounds(
                        window,
                        |this: &mut DeploymentWindow, window, cx| {
                            this.remember_size(window, cx);
                        },
                    );
                    let requests = DeploymentWindow::handle_requests(requests, window, cx);
                    servers.focus_handle(cx).focus(window, cx);
                    if options {
                        servers.update(cx, |servers, cx| servers.show_options(window, cx));
                    }
                    DeploymentWindow {
                        workspace,
                        servers,
                        title_bar: (!cfg!(target_os = "macos")).then(|| {
                            cx.new(|cx| PlatformTitleBar::new("deployment-title-bar", cx))
                        }),
                        focus_handle: cx.focus_handle(),
                        save_size: None,
                        _subscriptions: vec![dismiss, bounds],
                        _requests: requests,
                    }
                })
            },
        )
        .log_err();
    });
}

fn saved_size(cx: &App) -> Option<Size<Pixels>> {
    let value = KeyValueStore::global(cx)
        .read_kvp(WINDOW_SIZE_KEY)
        .log_err()
        .flatten()?;
    parse_size(&value)
}

/// Parses "width,height" in pixels, refusing sizes below the minimum.
pub(crate) fn parse_size(value: &str) -> Option<Size<Pixels>> {
    let (width, height) = value.split_once(',')?;
    let width: f32 = width.trim().parse().ok()?;
    let height: f32 = height.trim().parse().ok()?;
    (width >= f32::from(MIN_SIZE.width) && height >= f32::from(MIN_SIZE.height)).then(|| Size {
        width: px(width),
        height: px(height),
    })
}

impl DeploymentWindow {
    fn handle_requests(
        mut requests: mpsc::UnboundedReceiver<Request>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |this, cx| {
            while let Some(request) = requests.next().await {
                let handled = this.update_in(cx, |this, window, cx| this.show(request, window, cx));
                if handled.is_err() {
                    break;
                }
            }
        })
    }

    fn show(&mut self, request: Request, window: &mut Window, cx: &mut Context<Self>) {
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
            // Diffs open as editor tabs and notifications show as toasts in the workspace.
            request @ (Request::Diff { .. } | Request::Notify { .. }) => {
                if let Some(workspace) = self.workspace {
                    workspace
                        .update(cx, |multi_workspace, window, cx| {
                            multi_workspace.workspace().update(cx, |workspace, cx| {
                                crate::show_request(request, workspace, window, cx)
                            });
                        })
                        .ok();
                }
            }
        }
    }

    /// Saves the window size half a second after resizing stops.
    fn remember_size(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let WindowBounds::Windowed(bounds) = window.window_bounds() else {
            return;
        };
        let value = format!(
            "{},{}",
            f32::from(bounds.size.width).round(),
            f32::from(bounds.size.height).round()
        );
        let kvp = KeyValueStore::global(cx);
        self.save_size = Some(cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(500))
                .await;
            kvp.write_kvp(WINDOW_SIZE_KEY.into(), value).await.log_err();
        }));
    }
}

impl Focusable for DeploymentWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DeploymentWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme_settings::setup_ui_font(window, cx);
        workspace::client_side_decorations(
            v_flex()
                .size_full()
                .text_color(cx.theme().colors().text)
                .children(self.title_bar.clone())
                .child(
                    div()
                        .id("deployment-window")
                        .key_context("DeploymentWindow")
                        .track_focus(&self.focus_handle)
                        .relative()
                        .flex_1()
                        .min_h_0()
                        .font(ui_font)
                        .bg(cx.theme().colors().background)
                        .when(!cfg!(target_os = "macos"), |content| {
                            content
                                .border_t_1()
                                .border_color(cx.theme().colors().border)
                        })
                        .child(self.servers.clone()),
                ),
            window,
            cx,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_sizes_must_fit_the_minimum() {
        assert_eq!(
            parse_size("1200,800"),
            Some(Size {
                width: px(1200.),
                height: px(800.)
            })
        );
        assert_eq!(
            parse_size(" 900 , 500 "),
            Some(Size {
                width: px(900.),
                height: px(500.)
            })
        );
        assert_eq!(parse_size("300,200"), None);
        assert_eq!(parse_size("wide,tall"), None);
        assert_eq!(parse_size(""), None);
    }
}
