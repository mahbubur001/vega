//! "Upload to…", "Download from…", "Compare with Deployed To…" and "Sync with
//! Deployed To…": a searchable list of the project's deployment servers.

use std::sync::Arc;

use deployment::config::ServerType;
use deployment::engine::Engine;
use gpui::{App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use picker::{Picker, PickerDelegate};
use ui::{ListItem, ListItemSpacing, prelude::*};
use workspace::ModalView;

use crate::servers_modal::server_type_icon;

type OnPick = Box<dyn FnOnce(String, &mut Window, &mut App)>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ServerEntry {
    pub id: String,
    pub name: String,
    pub detail: String,
    pub kind: ServerType,
    pub is_default: bool,
}

/// The servers `engine` can transfer to, groups included when `groups` is set.
pub(crate) fn entries(engine: &Engine, groups: bool) -> Vec<ServerEntry> {
    let default = engine.default_server().map(|resolved| resolved.server.id);
    let mut entries: Vec<ServerEntry> = engine
        .servers()
        .into_iter()
        .map(|resolved| resolved.server)
        .filter(|server| server.kind.transfers() || (groups && server.kind == ServerType::Group))
        .filter(|server| groups || server.kind != ServerType::Group)
        .map(|server| ServerEntry {
            is_default: default.as_deref() == Some(server.id.as_str()),
            detail: match server.kind {
                ServerType::Group => format!("{} servers", server.members.len()),
                ServerType::Local | ServerType::InPlace => server.root_path.clone(),
                _ => server.host.clone(),
            },
            id: server.id,
            name: server.name,
            kind: server.kind,
        })
        .collect();
    // The default server first, then by name.
    entries.sort_by(|a, b| {
        b.is_default
            .cmp(&a.is_default)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    entries
}

/// Entries whose name or host contains every word of `query` (any case).
pub(crate) fn filter(entries: &[ServerEntry], query: &str) -> Vec<usize> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            let haystack = format!("{} {}", entry.name, entry.detail).to_lowercase();
            words.iter().all(|word| haystack.contains(word))
        })
        .map(|(index, _)| index)
        .collect()
}

pub(crate) struct ServerPicker {
    picker: Entity<Picker<ServerPickerDelegate>>,
}

impl ServerPicker {
    pub(crate) fn new(
        title: &str,
        entries: Vec<ServerEntry>,
        on_pick: impl FnOnce(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let matches = (0..entries.len()).collect();
        let delegate = ServerPickerDelegate {
            picker: cx.entity().downgrade(),
            placeholder: format!("{title} — search servers…").into(),
            entries,
            matches,
            selected_index: 0,
            on_pick: Some(Box::new(on_pick)),
        };
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).initial_width(rems(30.)));
        Self { picker }
    }
}

impl Render for ServerPicker {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl Focusable for ServerPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ServerPicker {}
impl ModalView for ServerPicker {}

pub(crate) struct ServerPickerDelegate {
    picker: WeakEntity<ServerPicker>,
    placeholder: Arc<str>,
    entries: Vec<ServerEntry>,
    matches: Vec<usize>,
    selected_index: usize,
    on_pick: Option<OnPick>,
}

impl PickerDelegate for ServerPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "deployment server picker"
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
        self.matches = filter(&self.entries, &query);
        self.selected_index = 0;
        Task::ready(())
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let entry = self
            .matches
            .get(self.selected_index)
            .and_then(|index| self.entries.get(*index));
        if let (Some(entry), Some(on_pick)) = (entry, self.on_pick.take()) {
            let id = entry.id.clone();
            cx.defer_in(window, move |_, window, cx| on_pick(id, window, cx));
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.picker.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.entries.get(*self.matches.get(ix)?)?;
        let (icon, color) = server_type_icon(entry.kind);
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(Icon::new(icon).size(IconSize::Small).color(color))
                .child(h_flex().gap_2().child(Label::new(entry.name.clone())).when(
                    entry.is_default,
                    |row| {
                        row.child(
                            Label::new("default")
                                .size(LabelSize::XSmall)
                                .color(Color::Success),
                        )
                    },
                ))
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

    fn entry(name: &str, detail: &str) -> ServerEntry {
        ServerEntry {
            id: name.to_lowercase(),
            name: name.into(),
            detail: detail.into(),
            kind: ServerType::Sftp,
            is_default: false,
        }
    }

    #[test]
    fn search_matches_every_word_in_name_or_host() {
        let entries = vec![
            entry("RT - radiustheme.net", "ftp.radiustheme.net"),
            entry("Docker SFTP", "localhost"),
            entry("Maps Central", "app.mapplugin.com"),
        ];
        assert_eq!(filter(&entries, ""), [0, 1, 2]);
        assert_eq!(filter(&entries, "LOCAL"), [1]);
        assert_eq!(filter(&entries, "rt ftp"), [0]);
        assert_eq!(filter(&entries, "maps plugin"), [2]);
        assert!(filter(&entries, "nothing").is_empty());
    }
}
