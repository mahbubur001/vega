//! Asks for a file or folder name (New File, New Folder, Rename…).

use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Window};
use ui::{Modal, ModalHeader, Section, prelude::*};
use ui_input::InputField;
use workspace::ModalView;

use crate::dialog;

type OnConfirm = Box<dyn FnOnce(String, &mut Window, &mut App)>;

pub struct NameModal {
    title: SharedString,
    input: Entity<InputField>,
    validate: fn(&str) -> Option<&'static str>,
    on_confirm: Option<OnConfirm>,
}

impl NameModal {
    /// Asks for a single file or folder name.
    pub fn new(
        title: impl Into<SharedString>,
        initial: &str,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::build(title, "Name", initial, name_problem, on_confirm, window, cx)
    }

    /// Asks for a relative path (folders separated by `/`).
    pub fn new_path(
        title: impl Into<SharedString>,
        label: &'static str,
        initial: &str,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::build(title, label, initial, path_problem, on_confirm, window, cx)
    }

    fn build(
        title: impl Into<SharedString>,
        label: &'static str,
        initial: &str,
        validate: fn(&str) -> Option<&'static str>,
        on_confirm: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let input = cx.new(|cx| {
            let field = InputField::new(window, cx, label).label(label);
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
pub fn name_problem(name: &str) -> Option<&'static str> {
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

/// Why `path` can't be used as a relative path, if it can't.
pub fn path_problem(path: &str) -> Option<&'static str> {
    if path.is_empty() {
        Some("Enter a path.")
    } else if path.starts_with('/') || path.starts_with('\\') {
        Some("Enter a path relative to the project.")
    } else if path
        .split(['/', '\\'])
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        Some("Each part of the path needs a name.")
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
        let focus = self.input.focus_handle(cx);
        v_flex()
            .key_context("VegaNameModal")
            .w(rems(26.))
            .elevation_3(cx)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .child(
                Modal::new("vega-name-modal", None)
                    .header(ModalHeader::new().headline(self.title.clone()))
                    .section(Section::new().child(self.input.clone()))
                    .footer(dialog::footer(
                        dialog::cancel_button("name-cancel", &focus, cx)
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                        dialog::primary_button("name-ok", "OK", &focus, cx).on_click(cx.listener(
                            |this, _, window, cx| this.confirm(&menu::Confirm, window, cx),
                        )),
                    )),
            )
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

    #[test]
    fn paths_must_stay_inside_the_project() {
        assert_eq!(path_problem("src/scratch.php"), None);
        assert_eq!(path_problem("scratch.php"), None);
        assert!(path_problem("").is_some());
        assert!(path_problem("/etc/hosts").is_some());
        assert!(path_problem("../outside.php").is_some());
        assert!(path_problem("src//a.php").is_some());
        assert!(path_problem("src/").is_some());
    }
}
