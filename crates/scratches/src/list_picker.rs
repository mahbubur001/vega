//! A searchable list in a modal (Show Scratch Files).

use std::sync::Arc;

use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, ListItemSpacing, prelude::*};
use workspace::ModalView;

type OnPick = Box<dyn FnOnce(usize, &mut Window, &mut App)>;

#[derive(Clone, Debug)]
pub(crate) struct PickerEntry {
    pub label: SharedString,
    /// Shown muted at the end of the row (an extension, a folder, a date).
    pub detail: SharedString,
    /// A file icon path (from `FileIcons`), if any.
    pub icon: Option<SharedString>,
}

/// Indexes of the entries whose label contains every word of `query`, in order.
pub(crate) fn filter(entries: &[PickerEntry], query: &str) -> Vec<usize> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            let label = entry.label.to_lowercase();
            words.iter().all(|word| label.contains(word))
        })
        .map(|(index, _)| index)
        .collect()
}

pub(crate) struct ListPicker {
    picker: Entity<Picker<ListPickerDelegate>>,
}

impl ListPicker {
    pub(crate) fn new(
        placeholder: &str,
        entries: Vec<PickerEntry>,
        selected: usize,
        on_pick: impl FnOnce(usize, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let matches = (0..entries.len()).collect();
        let delegate = ListPickerDelegate {
            modal: cx.entity().downgrade(),
            placeholder: placeholder.into(),
            entries,
            matches,
            selected_index: selected,
            on_pick: Some(Box::new(on_pick)),
        };
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).initial_width(rems(28.)));
        Self { picker }
    }
}

impl Render for ListPicker {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl Focusable for ListPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ListPicker {}
impl ModalView for ListPicker {}

pub(crate) struct ListPickerDelegate {
    modal: WeakEntity<ListPicker>,
    placeholder: Arc<str>,
    entries: Vec<PickerEntry>,
    matches: Vec<usize>,
    selected_index: usize,
    on_pick: Option<OnPick>,
}

impl PickerDelegate for ListPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "scratch list picker"
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        self.placeholder.clone()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query_is_empty = query.trim().is_empty();
        self.matches = filter(&self.entries, &query);
        if !query_is_empty {
            self.selected_index = 0;
        }
        self.selected_index = self
            .selected_index
            .min(self.matches.len().saturating_sub(1));
        Task::ready(())
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let picked = self.matches.get(self.selected_index).copied();
        if let (Some(index), Some(on_pick)) = (picked, self.on_pick.take()) {
            cx.defer_in(window, move |_, window, cx| on_pick(index, window, cx));
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.modal.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.entries.get(*self.matches.get(ix)?)?;
        let icon = match &entry.icon {
            Some(path) => Icon::from_path(path.clone()),
            None => Icon::new(IconName::File),
        };
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(icon.size(IconSize::Small).color(Color::Muted))
                .child(Label::new(entry.label.clone()))
                .end_slot(
                    Label::new(entry.detail.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(label: &str) -> PickerEntry {
        PickerEntry {
            label: label.to_string().into(),
            detail: SharedString::default(),
            icon: None,
        }
    }

    #[test]
    fn search_keeps_order_and_matches_every_word() {
        let entries = vec![entry("PHP"), entry("JavaScript"), entry("Plain Text")];
        assert_eq!(filter(&entries, ""), [0, 1, 2]);
        assert_eq!(filter(&entries, "p"), [0, 1, 2]);
        assert_eq!(filter(&entries, "ph"), [0]);
        assert_eq!(filter(&entries, "plain te"), [2]);
        assert!(filter(&entries, "rust").is_empty());
    }
}
