//! Shows running deployment transfers in the status bar.

use gpui::{Entity, Subscription};
use ui::{CommonAnimationExt as _, Tooltip, prelude::*};
use workspace::{HideStatusItem, ItemHandle, StatusItemView};
use zed_actions::deployment::CancelTransfers;

use crate::store::DeploymentStore;

pub struct TransfersIndicator {
    store: Entity<DeploymentStore>,
    _subscription: Subscription,
}

impl TransfersIndicator {
    pub fn new(store: Entity<DeploymentStore>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe(&store, |_, _, cx| cx.notify());
        Self {
            store,
            _subscription: subscription,
        }
    }
}

impl Render for TransfersIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let transfers = self.store.read(cx).transfers().clone();
        if transfers.active == 0 {
            return div().into_any_element();
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

    /// Only shown while transfers run, which needs a deployment configuration.
    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}
