//! Asks for a server password or key passphrase.

use deployment::prompt::{SecretAnswer, SecretKind, SecretRequest};
use futures::channel::oneshot;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable};
use ui::{Checkbox, ToggleState, prelude::*};
use ui_input::InputField;
use workspace::ModalView;

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
        let placeholder = match request.kind {
            SecretKind::Password => "Password",
            SecretKind::Passphrase => "Key passphrase",
        };
        let input = cx.new(|cx| InputField::new(window, cx, placeholder).masked(true));
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
        let title = match self.request.kind {
            SecretKind::Password => format!("Password for {}", self.request.server_name),
            SecretKind::Passphrase => "Key passphrase".to_string(),
        };
        let save = self.save;

        v_flex()
            .key_context("DeploymentSecretModal")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .w(rems(26.))
            .p_4()
            .gap_3()
            .elevation_3(cx)
            .child(Headline::new(title).size(HeadlineSize::Small))
            .child(Label::new(self.request.target.clone()).color(Color::Muted))
            .when_some(self.request.error.clone(), |this, error| {
                this.child(Label::new(error).color(Color::Error))
            })
            .child(self.input.clone())
            .child(
                Checkbox::new("save-secret", ToggleState::from(save))
                    .label("Save in Keychain")
                    .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                        this.save = state.selected();
                        cx.notify();
                    })),
            )
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("cancel", "Cancel")
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    )
                    .child(
                        Button::new("connect", "Connect")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx)
                            })),
                    ),
            )
    }
}
