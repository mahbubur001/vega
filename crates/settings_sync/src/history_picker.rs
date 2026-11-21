//! Settings Sync history: pick an earlier version of the synced settings and
//! restore it. Restoring writes it to GitHub as the newest version, then syncs,
//! so this machine's files are backed up before they change.

use std::sync::Arc;

use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, Focusable, Render, Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::engine::SettingsSync;
use crate::gist::GistVersion;

pub(crate) fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let Some(sync) = SettingsSync::global(cx) else {
        return;
    };
    workspace.toggle_modal(window, cx, |window, cx| HistoryModal::new(sync, window, cx));
}

pub(crate) struct HistoryModal {
    picker: Entity<Picker<HistoryDelegate>>,
    _load: Task<()>,
}

impl HistoryModal {
    fn new(sync: Entity<SettingsSync>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = HistoryDelegate {
            modal: cx.weak_entity(),
            sync: sync.clone(),
            versions: Vec::new(),
            matches: Vec::new(),
            selected_index: 0,
            message: "Loading the sync history…".into(),
        };
        let picker =
            cx.new(|cx| Picker::uniform_list(delegate, window, cx).initial_width(rems(28.)));
        let history = sync.read(cx).history(cx);
        let load = cx.spawn_in(window, {
            let picker = picker.clone();
            async move |_, cx| {
                let result = history.await;
                picker
                    .update_in(cx, |picker, window, cx| {
                        match result {
                            Ok(versions) => {
                                picker.delegate.message = "No saved versions yet.".into();
                                picker.delegate.versions = versions;
                            }
                            Err(error) => {
                                picker.delegate.message =
                                    format!("Couldn't load the history: {error}").into();
                            }
                        }
                        picker.refresh(window, cx);
                    })
                    .log_err();
            }
        });
        Self {
            picker,
            _load: load,
        }
    }
}

impl Focusable for HistoryModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for HistoryModal {}
impl ModalView for HistoryModal {}

impl Render for HistoryModal {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

pub(crate) struct HistoryDelegate {
    modal: WeakEntity<HistoryModal>,
    sync: Entity<SettingsSync>,
    versions: Vec<GistVersion>,
    /// Indices into `versions` that match the query.
    matches: Vec<usize>,
    selected_index: usize,
    message: SharedString,
}

impl HistoryDelegate {
    fn dismiss(&self, cx: &mut App) {
        self.modal.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }
}

impl PickerDelegate for HistoryDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "settings sync history"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Restore synced settings from…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(self.message.clone())
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
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.trim().to_lowercase();
        self.matches = self
            .versions
            .iter()
            .enumerate()
            .filter(|(_, version)| {
                query.is_empty() || version_label(version).to_lowercase().contains(&query)
            })
            .map(|(index, _)| index)
            .collect();
        self.selected_index = self
            .selected_index
            .min(self.matches.len().saturating_sub(1));
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(version) = self
            .matches
            .get(self.selected_index)
            .and_then(|index| self.versions.get(*index))
        else {
            return;
        };
        let version = version.version.clone();
        self.sync.update(cx, |sync, cx| sync.restore(version, cx));
        self.dismiss(cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.dismiss(cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let index = *self.matches.get(ix)?;
        let version = self.versions.get(index)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .child(Label::new(version_label(version)))
                        .when(index == 0, |row| {
                            row.child(Label::new("current").color(Color::Muted))
                        }),
                ),
        )
    }
}

/// "2026-11-19 08:03 UTC · 3f2a9c1"
fn version_label(version: &GistVersion) -> String {
    let when = version
        .committed_at
        .get(..16)
        .map(|stamp| stamp.replace('T', " "))
        .unwrap_or_else(|| version.committed_at.clone());
    let short: String = version.version.chars().take(7).collect();
    format!("{when} UTC · {short}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_labels_are_readable() {
        let version = GistVersion {
            version: "3f2a9c1d8e".into(),
            committed_at: "2026-11-19T08:03:12Z".into(),
        };
        assert_eq!(version_label(&version), "2026-11-19 08:03 UTC · 3f2a9c1");
    }
}
