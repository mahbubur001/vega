//! The window that walks the user through GitHub's device flow.

use anyhow::Context as _;
use gpui::{
    App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    MouseDownEvent, Render, Subscription, Window, WindowBounds, WindowKind, WindowOptions, point,
};
use ui::{ButtonLike, CommonAnimationExt, prelude::*};
use util::ResultExt as _;

use crate::account::{AccountStatus, GitHubAccount};
use crate::github::DeviceFlow;

pub(crate) fn open(account: &Entity<GitHubAccount>, window: &Window, cx: &mut App) {
    let center = window.bounds().center();
    let size = gpui::size(px(420.), px(380.));
    let bounds = WindowBounds::Windowed(gpui::bounds(
        center - point(size.width / 2.0, size.height / 2.0),
        size,
    ));
    cx.open_window(
        WindowOptions {
            kind: WindowKind::Floating,
            window_bounds: Some(bounds),
            is_resizable: false,
            is_movable: true,
            titlebar: Some(gpui::TitlebarOptions {
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| cx.new(|cx| SignInWindow::new(account, window, cx)),
    )
    .context("opening the GitHub sign-in window")
    .log_err();
}

struct SignInWindow {
    status: AccountStatus,
    connect_clicked: bool,
    focus_handle: FocusHandle,
    account: Entity<GitHubAccount>,
    _subscription: Subscription,
}

impl Focusable for SignInWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for SignInWindow {}

impl SignInWindow {
    fn new(account: &Entity<GitHubAccount>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        window.on_window_should_close(cx, |window, cx| {
            if let Some(this) = window.root::<SignInWindow>().flatten() {
                this.update(cx, |this, cx| this.before_dismiss(cx));
            }
            true
        });
        cx.subscribe_in(
            &cx.entity(),
            window,
            |this, _, _: &DismissEvent, window, cx| {
                window.remove_window();
                this.before_dismiss(cx);
            },
        )
        .detach();

        Self {
            status: account.read(cx).status().clone(),
            connect_clicked: false,
            focus_handle: cx.focus_handle(),
            account: account.clone(),
            _subscription: cx.observe(account, |this, account, cx| {
                this.status = account.read(cx).status().clone();
                cx.notify();
            }),
        }
    }

    fn before_dismiss(&mut self, cx: &mut Context<Self>) {
        self.account
            .update(cx, |account, cx| account.cancel_sign_in(cx));
    }

    fn render_device_code(user_code: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let copied = cx
            .read_from_clipboard()
            .is_some_and(|item| item.text().as_deref() == Some(user_code));
        let user_code = user_code.to_owned();

        ButtonLike::new("copy-code")
            .full_width()
            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
            .size(ButtonSize::Medium)
            .child(
                h_flex()
                    .w_full()
                    .p_1()
                    .justify_between()
                    .child(Label::new(user_code.clone()))
                    .child(Label::new(if copied { "Copied!" } else { "Copy" })),
            )
            .on_click(move |_, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(user_code.clone()));
                window.refresh();
            })
    }

    fn render_prompt(
        connect_clicked: bool,
        device_flow: &DeviceFlow,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let connect_label = if connect_clicked {
            "Waiting for GitHub…"
        } else {
            "Open GitHub"
        };
        let verification_uri = device_flow.verification_uri.clone();
        let user_code = device_flow.user_code.clone();

        v_flex()
            .w_full()
            .gap_2p5()
            .items_center()
            .text_center()
            .child(Headline::new("Sign in with GitHub").size(HeadlineSize::Large))
            .child(
                Label::new("Copy this code, then paste it on GitHub to sync your settings.")
                    .color(Color::Muted),
            )
            .child(Self::render_device_code(&device_flow.user_code, cx))
            .child(
                v_flex()
                    .w_full()
                    .gap_1()
                    .child(
                        Button::new("open-github", connect_label)
                            .full_width()
                            .style(ButtonStyle::Outlined)
                            .size(ButtonSize::Medium)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(user_code.clone()));
                                cx.open_url(&verification_uri);
                                this.connect_clicked = true;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("cancel", "Cancel")
                            .full_width()
                            .size(ButtonSize::Medium)
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    ),
            )
            .child(
                Label::new("VEGA only asks for access to your gists.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    fn render_message(
        headline: &'static str,
        message: SharedString,
        button: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .items_center()
            .text_center()
            .child(Headline::new(headline).size(HeadlineSize::Large))
            .child(Label::new(message).color(Color::Muted))
            .child(
                Button::new("done", button)
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .size(ButtonSize::Medium)
                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
            )
    }
}

impl Render for SignInWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.status.clone() {
            AccountStatus::Starting
            | AccountStatus::SignedOut
            | AccountStatus::SigningIn { device_flow: None } => Icon::new(IconName::ArrowCircle)
                .color(Color::Muted)
                .with_rotate_animation(2)
                .into_any_element(),
            AccountStatus::SigningIn {
                device_flow: Some(device_flow),
            } => Self::render_prompt(self.connect_clicked, &device_flow, cx).into_any_element(),
            AccountStatus::SignedIn(user) => Self::render_message(
                "You're signed in",
                format!("Signed in to GitHub as @{}.", user.login).into(),
                "Done",
                cx,
            )
            .into_any_element(),
            AccountStatus::Error(message) => {
                Self::render_message("Sign-in failed", message, "Close", cx).into_any_element()
            }
        };

        v_flex()
            .id("github-sign-in")
            .track_focus(&self.focus_handle)
            .size_full()
            .px_4()
            .py_8()
            .gap_4()
            .items_center()
            .justify_center()
            .elevation_3(cx)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_any_mouse_down(cx.listener(|this, _: &MouseDownEvent, window, cx| {
                window.focus(&this.focus_handle, cx);
            }))
            .child(
                Icon::new(IconName::Github)
                    .size(IconSize::Custom(rems(3.)))
                    .color(Color::Default),
            )
            .child(content)
    }
}
