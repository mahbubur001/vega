//! The deployment status bar item: running transfers (click to cancel), or
//! when idle the default server and whether automatic upload is on (click to
//! pick another default server).

use deployment::config::AutoUpload;
use gpui::{Entity, Subscription, WeakEntity};
use ui::{CommonAnimationExt as _, Tooltip, prelude::*};
use workspace::{HideStatusItem, ItemHandle, StatusItemView, Workspace};
use zed_actions::deployment::CancelTransfers;

use crate::server_picker::{self, ServerPicker};
use crate::servers_modal::server_type_icon;
use crate::store::DeploymentStore;

pub struct TransfersIndicator {
    store: Entity<DeploymentStore>,
    workspace: WeakEntity<Workspace>,
    _subscription: Subscription,
}

impl TransfersIndicator {
    pub fn new(
        store: Entity<DeploymentStore>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&store, |_, _, cx| cx.notify());
        Self {
            store,
            workspace,
            _subscription: subscription,
        }
    }

    fn pick_default_server(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(engine) = self.store.read(cx).engines().into_iter().next() else {
            return;
        };
        let entries = server_picker::entries(&engine, true);
        let store = self.store.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    ServerPicker::new(
                        "Default server",
                        entries,
                        move |server_id, _, cx| {
                            let result = engine
                                .config_mut()
                                .set_default_server(Some(server_id.as_str()));
                            if result.is_ok() {
                                engine.reload_config();
                                store.update(cx, |_, cx| cx.notify());
                            }
                        },
                        window,
                        cx,
                    )
                });
            })
            .ok();
    }
}

impl Render for TransfersIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let transfers = self.store.read(cx).transfers().clone();
        if transfers.active == 0 {
            let Some(engine) = self.store.read(cx).engines().into_iter().next() else {
                return div().into_any_element();
            };
            let Some(default) = engine.default_server() else {
                return div().into_any_element();
            };
            let auto_upload = engine.config().options().auto_upload;
            let (icon, color) = server_type_icon(default.server.kind);
            let tooltip = match auto_upload {
                AutoUpload::Never => "Default deployment server · automatic upload off",
                AutoUpload::OnExplicitSave => {
                    "Default deployment server · uploads files when you save them"
                }
                AutoUpload::Always => "Default deployment server · uploads every change",
            };
            return h_flex()
                .id("deployment-default-server")
                .gap_1()
                .px_1()
                .rounded_sm()
                .cursor_pointer()
                .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                .child(Icon::new(icon).size(IconSize::XSmall).color(color))
                .child(
                    Label::new(default.server.name)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .when(auto_upload != AutoUpload::Never, |row| {
                    row.child(
                        Icon::new(IconName::ArrowUp)
                            .size(IconSize::XSmall)
                            .color(Color::Accent),
                    )
                })
                .tooltip(Tooltip::text(tooltip))
                .on_click(cx.listener(|this, _, window, cx| this.pick_default_server(window, cx)))
                .into_any_element();
        }

        let label = match &transfers.current {
            Some(current) if transfers.active > 1 => {
                format!("{current} (+{} more)", transfers.active - 1)
            }
            Some(current) => current.to_string(),
            None => format!("{} transfers", transfers.active),
        };

        h_flex()
            .id("deployment-transfers")
            .gap_1()
            .child(
                Icon::new(IconName::ArrowCircle)
                    .size(IconSize::Small)
                    .color(Color::Muted)
                    .with_rotate_animation(2),
            )
            .child(Label::new(label).size(LabelSize::Small).color(Color::Muted))
            .tooltip(Tooltip::text("Deployment transfers. Click to cancel."))
            .on_click(|_, window, cx| {
                window.dispatch_action(Box::new(CancelTransfers), cx);
            })
            .into_any_element()
    }
}

impl StatusItemView for TransfersIndicator {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    /// Only shown in projects with a deployment configuration.
    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}
