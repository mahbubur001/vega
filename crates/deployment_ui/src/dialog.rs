//! Shared look for the deployment dialogs: Zed's modal header, sections and
//! footer, with an accent-tinted primary button and key binding hints.

use gpui::{App, FocusHandle};
use ui::{KeyBinding, ModalFooter, TintColor, prelude::*};

/// The dialog's main button (OK, Connect, …), bound to Enter.
pub(crate) fn primary_button(
    id: &'static str,
    label: impl Into<SharedString>,
    focus: &FocusHandle,
    cx: &App,
) -> Button {
    Button::new(id, label)
        .style(ButtonStyle::Tinted(TintColor::Accent))
        .key_binding(
            KeyBinding::for_action_in(&menu::Confirm, focus, cx)
                .map(|binding| binding.size(rems_from_px(12_f32))),
        )
}

/// The dialog's Cancel button, bound to Escape.
pub(crate) fn cancel_button(id: &'static str, focus: &FocusHandle, cx: &App) -> Button {
    Button::new(id, "Cancel")
        .style(ButtonStyle::Outlined)
        .key_binding(
            KeyBinding::for_action_in(&menu::Cancel, focus, cx)
                .map(|binding| binding.size(rems_from_px(12_f32))),
        )
}

/// A footer with Cancel and the primary button on the right.
pub(crate) fn footer(cancel: Button, primary: Button) -> ModalFooter {
    ModalFooter::new().end_slot(h_flex().gap_2().child(cancel).child(primary))
}
