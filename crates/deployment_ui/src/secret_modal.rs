//! Asks for a server password or key passphrase, in a small movable window of
//! its own (see [`open_window`]).

use deployment::prompt::{SecretAnswer, SecretKind, SecretRequest};
use futures::channel::oneshot;
use gpui::{
    Bounds, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Pixels, TitlebarOptions,
    WindowBounds, WindowKind, WindowOptions,
};
use ui::{Callout, Checkbox, Modal, ModalHeader, Section, Severity, ToggleState, prelude::*};
use ui_input::InputField;
use util::ResultExt as _;

use crate::dialog;

pub(crate) struct SecretModal {
    request: SecretRequest,
    input: Entity<InputField>,
    save: bool,
    reply: Option<oneshot::Sender<Option<SecretAnswer>>>,
    focus_handle: FocusHandle,
}

/// Opens the question as a small window centred over `parent` (the window
/// that asked). It can be moved like any window; closing it answers "no".
pub(crate) fn open_window(
    request: SecretRequest,
    reply: oneshot::Sender<Option<SecretAnswer>>,
    parent: Bounds<Pixels>,
    cx: &mut App,
) {
    // Taller when it has to show why the last attempt failed.
    let height = if request.error.is_some() { 330. } else { 250. };
    let size = gpui::size(px(440.), px(height));
    let origin = parent.center() - gpui::point(size.width / 2., size.height / 2.);
    let result = cx.open_window(
        WindowOptions {
            kind: WindowKind::Floating,
            window_bounds: Some(WindowBounds::Windowed(Bounds { origin, size })),
            is_resizable: false,
            is_movable: true,
            is_minimizable: false,
            titlebar: Some(TitlebarOptions {
                title: Some("VEGA — Sign in".into()),
                appears_transparent: true,
                traffic_light_position: Some(gpui::point(px(12.), px(12.))),
            }),
            window_background: cx.theme().window_background_appearance(),
            ..Default::default()
        },
        |window, cx| {
            cx.new(|cx| {
                cx.subscribe_in(&cx.entity(), window, |_, _, _: &DismissEvent, window, _| {
                    window.remove_window();
                })
                .detach();
                SecretModal::new(request, reply, window, cx)
            })
        },
    );
    result.log_err();
}

impl Drop for SecretModal {
    /// Closing the window without connecting answers "no secret".
    fn drop(&mut self) {
        if let Some(reply) = self.reply.take() {
            reply.send(None).ok();
        }
    }
}

impl SecretModal {
    pub fn new(
        request: SecretRequest,
        reply: oneshot::Sender<Option<SecretAnswer>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let label = match request.kind {
            SecretKind::Password => "Password",
            SecretKind::Passphrase => "Key passphrase",
        };
        let input = cx.new(|cx| InputField::new(window, cx, "").label(label).masked(true));
        input.read(cx).editor().focus_handle(cx).focus(window, cx);
        Self {
            request,
            input,
            save: true,
            reply: Some(reply),
            focus_handle: cx.focus_handle(),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        let value = self.input.read(cx).text(cx);
        if let Some(reply) = self.reply.take() {
            reply
                .send(Some(SecretAnswer {
                    value,
                    save: self.save,
                }))
                .ok();
        }
        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for SecretModal {}

impl Focusable for SecretModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SecretModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let (headline, icon) = match self.request.kind {
            SecretKind::Password => (
                format!("Sign in to {}", self.request.server_name),
                IconName::Lock,
            ),
            SecretKind::Passphrase => ("Unlock SSH key".to_string(), IconName::Lock),
        };
        let focus = self.focus_handle.clone();

        v_flex()
            .key_context("DeploymentSecretModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .size_full()
            .pt_6()
            .font(ui_font)
            .text_color(cx.theme().colors().text)
            .bg(cx.theme().colors().elevated_surface_background)
            .child(
                Modal::new("deployment-secret", None)
                    .header(
                        ModalHeader::new()
                            .icon(Icon::new(icon).color(Color::Muted))
                            .headline(headline)
                            .description(self.request.target.clone()),
                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_3()
                                .when_some(self.request.error.clone(), |column, error| {
                                    column.child(
                                        Callout::new()
                                            .severity(Severity::Error)
                                            .icon(IconName::Warning)
                                            .title("The server rejected the login")
                                            .description(error),
                                    )
                                })
                                .child(self.input.clone())
                                .child(
                                    Checkbox::new("save-secret", ToggleState::from(self.save))
                                        .label("Remember in Keychain")
                                        .on_click(cx.listener(
                                            |this, state: &ToggleState, _, cx| {
                                                this.save = state.selected();
                                                cx.notify();
                                            },
                                        )),
                                ),
                        ),
                    )
                    .footer(dialog::footer(
                        dialog::cancel_button("cancel", &focus, cx)
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                        dialog::primary_button("connect", "Connect", &focus, cx).on_click(
                            cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx)
                            }),
                        ),
                    )),
            )
    }
}
