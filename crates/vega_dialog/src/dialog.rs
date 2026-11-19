//! Shared look for the deployment dialogs, close to PhpStorm's: a solid
//! primary button, outlined secondary buttons with key binding hints, and
//! bordered combo boxes with the choice's icon inside.

use gpui::{App, ClickEvent, Entity, FocusHandle, Rems, Stateful};
use ui::{ButtonLike, ContextMenu, ModalFooter, PopoverMenu, prelude::*};

/// The dialog's main button (OK, Connect, Apply…): solid accent. Enter runs it.
pub fn primary_button(
    id: &'static str,
    label: impl Into<SharedString>,
    _focus: &FocusHandle,
    cx: &App,
) -> DialogButton {
    let background = cx.theme().status().info;
    let colors = cx.theme().colors();
    // The theme's text or background colour, whichever reads better on the accent.
    let text = readable_on(background, [colors.text, colors.editor_background]);
    DialogButton::new(id, label, cx)
        .bg(background)
        .hover(move |style| style.bg(background.opacity(0.85)))
        .text(Color::Custom(text), gpui::FontWeight::SEMIBOLD)
}

/// The candidate with the highest contrast against `background` (WCAG relative luminance).
pub fn readable_on(background: gpui::Hsla, candidates: [gpui::Hsla; 2]) -> gpui::Hsla {
    fn luminance(color: gpui::Hsla) -> f32 {
        let rgba = color.to_rgb();
        let channel = |c: f32| {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(rgba.r) + 0.7152 * channel(rgba.g) + 0.0722 * channel(rgba.b)
    }
    let contrast = |a: gpui::Hsla, b: gpui::Hsla| {
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    };
    let [first, second] = candidates;
    if contrast(first, background) >= contrast(second, background) {
        first
    } else {
        second
    }
}

/// The dialog's Cancel button. Escape runs it.
pub fn cancel_button(id: &'static str, _focus: &FocusHandle, cx: &App) -> DialogButton {
    secondary_button(id, "Cancel", cx)
}

/// An outlined button (Cancel, Apply…), the same size as [`primary_button`].
pub fn secondary_button(
    id: &'static str,
    label: impl Into<SharedString>,
    cx: &App,
) -> DialogButton {
    let colors = cx.theme().colors();
    let hover = colors.element_hover;
    DialogButton::new(id, label, cx)
        .border(colors.border)
        .hover(move |style| style.bg(hover))
}

/// A footer button: fixed height and padding so primary and secondary buttons line up.
#[derive(IntoElement)]
pub struct DialogButton {
    element: Stateful<Div>,
    label: SharedString,
    color: Color,
    weight: gpui::FontWeight,
}

impl DialogButton {
    fn new(id: &'static str, label: impl Into<SharedString>, _cx: &App) -> Self {
        Self {
            element: h_flex()
                .id(id)
                .h_7()
                .min_w(rems(5.))
                .px_3()
                .justify_center()
                .rounded_md()
                .cursor_pointer(),
            label: label.into(),
            color: Color::Default,
            weight: gpui::FontWeight::MEDIUM,
        }
    }

    fn bg(mut self, color: gpui::Hsla) -> Self {
        self.element = self.element.bg(color);
        self
    }

    fn border(mut self, color: gpui::Hsla) -> Self {
        self.element = self.element.border_1().border_color(color);
        self
    }

    fn hover(mut self, style: impl Fn(gpui::StyleRefinement) -> gpui::StyleRefinement) -> Self {
        self.element = self.element.hover(style);
        self
    }

    fn text(mut self, color: Color, weight: gpui::FontWeight) -> Self {
        self.color = color;
        self.weight = weight;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.element = self.element.on_click(handler);
        self
    }
}

impl RenderOnce for DialogButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.element
            .child(Label::new(self.label).color(self.color).weight(self.weight))
    }
}

/// A footer with Cancel and the primary button on the right.
pub fn footer(cancel: DialogButton, primary: DialogButton) -> ModalFooter {
    ModalFooter::new().end_slot(h_flex().gap_2().child(cancel).child(primary))
}

/// PhpStorm's round "?" help button.
pub fn help_button(id: &'static str, cx: &App) -> Stateful<Div> {
    let colors = cx.theme().colors();
    let hover = colors.element_hover;
    div()
        .id(id)
        .size_6()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_1()
        .border_color(colors.border)
        .cursor_pointer()
        .hover(move |style| style.bg(hover))
        .child(Label::new("?").size(LabelSize::Small).color(Color::Muted))
}

/// A PhpStorm-style combo box: a bordered box with the choice's icon and label
/// on the left and an arrow on the right; `menu` builds the list of choices.
pub fn combo_box(
    id: impl Into<SharedString>,
    icon: Option<(IconName, Color)>,
    label: impl Into<SharedString>,
    width: Option<Rems>,
    menu: impl Fn(&mut Window, &mut App) -> Entity<ContextMenu> + 'static,
    cx: &App,
) -> impl IntoElement {
    let field_background = cx.theme().colors().editor_background;
    let field_border = cx.theme().colors().border_variant;
    let id: SharedString = id.into();
    let label: SharedString = label.into();
    let trigger = ButtonLike::new(SharedString::from(format!("{id}-trigger")))
        .style(ButtonStyle::Transparent)
        .size(ButtonSize::Medium)
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
        // Looks like the text fields next to it: same background and border.
        .rounded_md()
        .bg(field_background)
        .border_1()
        .border_color(field_border)
        .child(
            PopoverMenu::new(id)
                .full_width(true)
                .anchor(gpui::Anchor::TopLeft)
                .trigger(trigger)
                .menu(move |window, cx| Some(menu(window, cx))),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_more_readable_label_colour() {
        let light_teal = gpui::rgb(0x83a598).into();
        let dark = gpui::rgb(0x1d2021).into();
        let cream = gpui::rgb(0xfbf1c7).into();
        assert_eq!(readable_on(light_teal, [cream, dark]), dark);
        let deep_blue = gpui::rgb(0x1e40af).into();
        assert_eq!(readable_on(deep_blue, [cream, dark]), cream);
    }
}
