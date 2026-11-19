//! Asks for a file or folder name (Remote Host New File, New Folder, Rename), or another short
//! value such as permissions.

use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Window};
use ui::prelude::*;
use ui_input::InputField;
use workspace::ModalView;

type OnConfirm = Box<dyn FnOnce(String, &mut Window, &mut App)>;
type Validate = fn(&str) -> Option<&'static str>;

pub(crate) struct NameModal {
    title: SharedString,
    input: Entity<InputField>,
    validate: Validate,
    on_confirm: Option<OnConfirm>,
}

impl NameModal {
    pub(crate) fn new(
        title: impl Into<SharedString>,
        initial: &str,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::with_validator(title, "Name", initial, name_problem, on_confirm, window, cx)
    }

    /// Like [`Self::new`], with another placeholder and check than for file names.
    pub(crate) fn with_validator(
        title: impl Into<SharedString>,
        placeholder: &str,
        initial: &str,
        validate: Validate,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input = cx.new(|cx| {
            let field = InputField::new(window, cx, placeholder);
            field.set_text(initial, window, cx);
            field
        });
        Self {
            title: title.into(),
            input,
            validate,
            on_confirm: Some(Box::new(on_confirm)),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.input.read(cx).text(cx).trim().to_string();
        if let Some(problem) = (self.validate)(&name) {
            self.input
                .update(cx, |input, cx| input.set_error(Some(problem), cx));
            return;
        }
        if let Some(on_confirm) = self.on_confirm.take() {
            on_confirm(name, window, cx);
        }
        cx.emit(DismissEvent);
    }
}

/// Why `name` can't be used as a single file or folder name, if it can't.
pub(crate) fn name_problem(name: &str) -> Option<&'static str> {
    if name.is_empty() {
        Some("Enter a name.")
    } else if name == "." || name == ".." {
        Some("Choose another name.")
    } else if name.contains('/') || name.contains('\\') {
        Some("A name can't contain slashes.")
    } else {
        None
    }
}

impl Focusable for NameModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for NameModal {}
impl ModalView for NameModal {}

impl Render for NameModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("RemoteHostName")
            .w(rems(24.))
            .p_3()
            .gap_2()
            .elevation_3(cx)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .child(Headline::new(self.title.clone()).size(HeadlineSize::Small))
            .child(self.input.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_must_be_single_path_components() {
        assert_eq!(name_problem("index.php"), None);
        assert_eq!(name_problem(".env"), None);
        assert!(name_problem("").is_some());
        assert!(name_problem("..").is_some());
        assert!(name_problem("a/b").is_some());
    }
}
