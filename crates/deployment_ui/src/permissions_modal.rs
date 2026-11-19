//! Remote Host "Change Permissions…" (PhpStorm's "Items Permissions"): an
//! R/W/X grid per owner, group and others plus the octal value, for files and,
//! when a folder is selected, for folders.

use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Subscription, Window,
};
use ui::{Checkbox, prelude::*};
use ui_input::{ErasedEditorEvent, InputField};
use workspace::ModalView;

/// What to apply: modes for files and folders (None leaves them alone), and
/// whether to go into a folder's contents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PermissionsChoice {
    pub file_mode: Option<u32>,
    pub dir_mode: Option<u32>,
    pub recursive: bool,
}

type OnConfirm = Box<dyn FnOnce(PermissionsChoice, &mut Window, &mut App)>;

const WHO: [&str; 3] = ["Owner", "Group", "Others"];
const WHAT: [&str; 3] = ["R", "W", "X"];

/// The permission bit for `who` (0 owner, 1 group, 2 others) and `what` (0 r, 1 w, 2 x).
pub(crate) fn bit(who: usize, what: usize) -> u32 {
    1 << (8 - (who * 3 + what))
}

/// Sets every bit of a row (or column) when any is missing, else clears them all.
pub(crate) fn toggle_bits(mode: u32, bits: impl IntoIterator<Item = u32>) -> u32 {
    let all = bits.into_iter().fold(0, |acc, bit| acc | bit);
    if mode & all == all {
        mode & !all
    } else {
        mode | all
    }
}

/// Parses "644" or "0755"; None for anything that isn't a permission mode.
pub(crate) fn parse_octal(text: &str) -> Option<u32> {
    let text = text.trim();
    if text.is_empty() || text.len() > 4 {
        return None;
    }
    u32::from_str_radix(text, 8).ok()
}

struct ModeEditor {
    mode: u32,
    octal: Entity<InputField>,
    _subscription: Subscription,
}

pub(crate) struct PermissionsModal {
    title: SharedString,
    files: Option<ModeEditor>,
    folders: Option<ModeEditor>,
    is_dir: bool,
    recursive: bool,
    on_confirm: Option<OnConfirm>,
    focus_handle: FocusHandle,
}

#[derive(Clone, Copy, PartialEq)]
enum Section {
    Files,
    Folders,
}

impl PermissionsModal {
    /// For a file, only `file_mode` is shown; for a folder both, with the
    /// file part applying to the files inside it.
    pub(crate) fn new(
        title: impl Into<SharedString>,
        is_dir: bool,
        file_mode: u32,
        dir_mode: u32,
        on_confirm: impl FnOnce(PermissionsChoice, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let files = Some(Self::editor(Section::Files, file_mode, window, cx));
        let folders = is_dir.then(|| Self::editor(Section::Folders, dir_mode, window, cx));
        Self {
            title: title.into(),
            files,
            folders,
            is_dir,
            recursive: true,
            on_confirm: Some(Box::new(on_confirm)),
            focus_handle: cx.focus_handle(),
        }
    }

    fn editor(
        section: Section,
        mode: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ModeEditor {
        let octal = cx.new(|cx| {
            let field = InputField::new(window, cx, "Octal, e.g. 644");
            field.set_text(&format!("{:o}", mode & 0o7777), window, cx);
            field
        });
        let this = cx.weak_entity();
        let editor = octal.read(cx).editor().clone();
        let subscription = editor.subscribe(
            Box::new(move |event, _, cx| {
                if !matches!(event, ErasedEditorEvent::BufferEdited) {
                    return;
                }
                this.update(cx, |this, cx| this.octal_edited(section, cx))
                    .ok();
            }),
            window,
            cx,
        );
        ModeEditor {
            mode,
            octal,
            _subscription: subscription,
        }
    }

    fn section(&mut self, section: Section) -> Option<&mut ModeEditor> {
        match section {
            Section::Files => self.files.as_mut(),
            Section::Folders => self.folders.as_mut(),
        }
    }

    fn octal_edited(&mut self, section: Section, cx: &mut Context<Self>) {
        let Some(editor) = self.section(section) else {
            return;
        };
        let text = editor.octal.read(cx).text(cx);
        let parsed = parse_octal(&text);
        if let Some(mode) = parsed {
            editor.mode = mode;
        }
        let error = parsed.is_none().then_some("Use octal, such as 644 or 755.");
        editor
            .octal
            .update(cx, |input, cx| input.set_error(error, cx));
        cx.notify();
    }

    fn set_mode(
        &mut self,
        section: Section,
        mode: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = self.section(section) else {
            return;
        };
        editor.mode = mode;
        let octal = editor.octal.clone();
        octal.update(cx, |input, cx| {
            input.set_text(&format!("{:o}", mode & 0o7777), window, cx);
            input.set_error(None::<SharedString>, cx);
        });
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let valid = |editor: &Option<ModeEditor>| {
            editor
                .as_ref()
                .is_none_or(|editor| parse_octal(&editor.octal.read(cx).text(cx)).is_some())
        };
        if !valid(&self.files) || !valid(&self.folders) {
            return;
        }
        let applies_to_files = !self.is_dir || self.recursive;
        let choice = PermissionsChoice {
            file_mode: self
                .files
                .as_ref()
                .filter(|_| applies_to_files)
                .map(|editor| editor.mode),
            dir_mode: self.folders.as_ref().map(|editor| editor.mode),
            recursive: self.is_dir && self.recursive,
        };
        if let Some(on_confirm) = self.on_confirm.take() {
            on_confirm(choice, window, cx);
        }
        cx.emit(DismissEvent);
    }

    fn render_section(
        &self,
        section: Section,
        label: &'static str,
        disabled: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = match section {
            Section::Files => self.files.as_ref()?,
            Section::Folders => self.folders.as_ref()?,
        };
        let mode = editor.mode;
        let id = match section {
            Section::Files => "files",
            Section::Folders => "folders",
        };
        let cell = || div().w(rems(4.)).flex().justify_center();
        let header = h_flex()
            .gap_1()
            .child(div().w(rems(5.)))
            .children((0..3).map(|what| {
                cell().child(
                    Button::new(SharedString::from(format!("{id}-col-{what}")), WHAT[what])
                        .style(ButtonStyle::Outlined)
                        .disabled(disabled)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            let bits = (0..3).map(move |who| bit(who, what));
                            this.set_mode(section, toggle_bits(mode, bits), window, cx);
                        })),
                )
            }));
        let rows = (0..3).map(|who| {
            h_flex()
                .gap_1()
                .child(
                    div().w(rems(5.)).child(
                        Button::new(SharedString::from(format!("{id}-row-{who}")), WHO[who])
                            .style(ButtonStyle::Outlined)
                            .disabled(disabled)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                let bits = (0..3).map(move |what| bit(who, what));
                                this.set_mode(section, toggle_bits(mode, bits), window, cx);
                            })),
                    ),
                )
                .children((0..3).map(|what| {
                    let bit = bit(who, what);
                    cell().child(
                        Checkbox::new(
                            SharedString::from(format!("{id}-{who}-{what}")),
                            (mode & bit != 0).into(),
                        )
                        .disabled(disabled)
                        .on_click(cx.listener(
                            move |this, _, window, cx| {
                                this.set_mode(section, mode ^ bit, window, cx);
                            },
                        )),
                    )
                }))
        });
        Some(
            v_flex()
                .gap_1()
                .child(Label::new(label).color(if disabled {
                    Color::Disabled
                } else {
                    Color::Default
                }))
                .child(header)
                .children(rows)
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            div()
                                .w(rems(5.))
                                .child(Label::new("Octal:").color(Color::Muted)),
                        )
                        .child(div().w(rems(12.5)).child(editor.octal.clone())),
                )
                .into_any_element(),
        )
    }
}

impl Focusable for PermissionsModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.files
            .as_ref()
            .map(|editor| editor.octal.focus_handle(cx))
            .unwrap_or_else(|| self.focus_handle.clone())
    }
}

impl EventEmitter<DismissEvent> for PermissionsModal {}
impl ModalView for PermissionsModal {}

impl Render for PermissionsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let files_disabled = self.is_dir && !self.recursive;
        let files_label = if self.is_dir { "Files inside" } else { "Files" };
        v_flex()
            .key_context("RemoteHostPermissions")
            .w(rems(24.))
            .p_3()
            .gap_3()
            .elevation_3(cx)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .child(Headline::new(self.title.clone()).size(HeadlineSize::Small))
            .children(self.render_section(Section::Folders, "Folders", false, cx))
            .when(self.is_dir, |modal| {
                modal.child(
                    Checkbox::new("permissions-recursive", self.recursive.into())
                        .label("Apply to everything inside the folder")
                        .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                            this.recursive = state.selected();
                            cx.notify();
                        })),
                )
            })
            .children(self.render_section(Section::Files, files_label, files_disabled, cx))
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("permissions-cancel", "Cancel")
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    )
                    .child(
                        Button::new("permissions-ok", "OK")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.confirm(&menu::Confirm, window, cx)
                            })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_follow_rwx_order() {
        assert_eq!(bit(0, 0), 0o400);
        assert_eq!(bit(1, 1), 0o020);
        assert_eq!(bit(2, 2), 0o001);
    }

    #[test]
    fn toggling_a_row_or_column_sets_or_clears_it() {
        let owner = || (0..3).map(|what| bit(0, what));
        assert_eq!(toggle_bits(0o644, owner()), 0o744);
        assert_eq!(toggle_bits(0o744, owner()), 0o044);
        let execute = || (0..3).map(|who| bit(who, 2));
        assert_eq!(toggle_bits(0o644, execute()), 0o755);
        assert_eq!(toggle_bits(0o755, execute()), 0o644);
    }

    #[test]
    fn octal_values() {
        assert_eq!(parse_octal("644"), Some(0o644));
        assert_eq!(parse_octal(" 0755"), Some(0o755));
        assert_eq!(parse_octal("9"), None);
        assert_eq!(parse_octal(""), None);
        assert_eq!(parse_octal("77777"), None);
    }
}
