//! Asks for a server password or key passphrase.

use deployment::prompt::{SecretAnswer, SecretKind, SecretRequest};
use futures::channel::oneshot;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable};
use ui::{Callout, Checkbox, Modal, ModalHeader, Section, Severity, ToggleState, prelude::*};
use ui_input::InputField;
use workspace::ModalView;

use crate::dialog;

pub(crate) struct SecretModal {
    request: SecretRequest,
    input: Entity<InputField>,
    save: bool,
    reply: Option<oneshot::Sender<Option<SecretAnswer>>>,
    focus_handle: FocusHandle,
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

impl ModalView for SecretModal {
    fn on_before_dismiss(
        &mut self,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        // Closing without confirming answers "no secret".
        if let Some(reply) = self.reply.take() {
            reply.send(None).ok();
        }
        workspace::DismissDecision::Dismiss(true)
    }
}

impl Render for SecretModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .w(rems(28.))
            .elevation_3(cx)
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
