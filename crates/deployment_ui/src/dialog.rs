//! Shared look for the deployment dialogs, close to PhpStorm's: a solid
//! primary button, outlined secondary buttons with key binding hints, and
//! bordered combo boxes with the choice's icon inside.

use gpui::{App, ClickEvent, Entity, FocusHandle, Rems, Stateful};
use ui::{ButtonLike, ContextMenu, KeyBinding, ModalFooter, PopoverMenu, prelude::*};

/// The dialog's main button (OK, Connect, Apply…): solid accent, bound to Enter.
pub(crate) fn primary_button(
    id: &'static str,
    label: impl Into<SharedString>,
    focus: &FocusHandle,
    cx: &App,
) -> PrimaryButton {
    let background = cx.theme().status().info;
    PrimaryButton {
        element: h_flex()
            .id(id)
            .h_7()
            .px_3()
            .gap_1p5()
            .rounded_md()
            .cursor_pointer()
            .bg(background)
            .hover(move |style| style.bg(background.opacity(0.85)))
            .child(
                Label::new(label.into())
                    .color(Color::Custom(gpui::white()))
                    .weight(gpui::FontWeight::MEDIUM),
            )
            .child(
                KeyBinding::for_action_in(&menu::Confirm, focus, cx)
                    .size(rems_from_px(12_f32))
                    .color(Color::Custom(gpui::white().opacity(0.8))),
            ),
    }
}

/// A solid button; see [`primary_button`].
#[derive(IntoElement)]
pub(crate) struct PrimaryButton {
    element: Stateful<Div>,
}

impl PrimaryButton {
    pub(crate) fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.element = self.element.on_click(handler);
        self
    }
}

impl RenderOnce for PrimaryButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.element
    }
}

/// The dialog's Cancel button, bound to Escape.
pub(crate) fn cancel_button(id: &'static str, focus: &FocusHandle, cx: &App) -> Button {
    secondary_button(id, "Cancel").key_binding(
        KeyBinding::for_action_in(&menu::Cancel, focus, cx)
            .map(|binding| binding.size(rems_from_px(12_f32))),
    )
}

/// An outlined button (Cancel, Apply, Test Connection…).
pub(crate) fn secondary_button(id: &'static str, label: impl Into<SharedString>) -> Button {
    Button::new(id, label).style(ButtonStyle::Outlined)
}

/// A footer with Cancel and the primary button on the right.
pub(crate) fn footer(cancel: Button, primary: PrimaryButton) -> ModalFooter {
    ModalFooter::new().end_slot(h_flex().gap_2().child(cancel).child(primary))
}

/// A PhpStorm-style combo box: a bordered box with the choice's icon and label
/// on the left and an arrow on the right; `menu` builds the list of choices.
pub(crate) fn combo_box(
    id: impl Into<SharedString>,
    icon: Option<(IconName, Color)>,
    label: impl Into<SharedString>,
    width: Option<Rems>,
    menu: impl Fn(&mut Window, &mut App) -> Entity<ContextMenu> + 'static,
) -> impl IntoElement {
    let id: SharedString = id.into();
    let label: SharedString = label.into();
    let trigger = ButtonLike::new(SharedString::from(format!("{id}-trigger")))
        .style(ButtonStyle::Outlined)
        .full_width()
        .child(
            h_flex()
                .w_full()
                .min_w_0()
                .gap_1p5()
                .px_1()
                .children(
                    icon.map(|(icon, color)| Icon::new(icon).size(IconSize::Small).color(color)),
                )
                .child(Label::new(label).truncate())
                .child(div().flex_1())
                .child(
                    Icon::new(IconName::ChevronDown)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                ),
        );
    div()
        .map(|container| match width {
            Some(width) => container.w(width),
            None => container.w_full(),
        })
        .child(
            PopoverMenu::new(id)
                .full_width(true)
                .anchor(gpui::Anchor::TopLeft)
                .trigger(trigger)
                .menu(move |window, cx| Some(menu(window, cx))),
        )
}
