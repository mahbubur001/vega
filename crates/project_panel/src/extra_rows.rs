//! VEGA: rows that add-ons show below the project tree, in the same scrolling
//! list (External Libraries, Scratches), styled like the
//! panel's own rows. Kept in its own file so the upstream panel changes stay
//! a few lines.

use gpui::{AnyElement, App, Context, ElementId, Entity, Hsla, Stateful, Subscription, Window};
use settings::{ProjectPanelEntrySpacing, Settings as _};
use ui::{ListItem, ListItemSpacing, prelude::*};

use crate::{ProjectPanel, ProjectPanelSettings, get_item_color};

/// An add-on's rows. Indexes are local to the add-on (0 is its first row).
pub trait ProjectPanelExtraRows: 'static + Sized {
    fn row_count(&self, cx: &App) -> usize;
    /// Indentation level, used for the panel's indent guides.
    fn row_depth(&self, index: usize, cx: &App) -> usize;
    fn render_row(
        &mut self,
        index: usize,
        state: ExtraRowState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement;
}

/// What the panel tells an add-on when it renders its rows.
#[derive(Clone, Copy, Debug)]
pub struct ExtraRowState {
    /// A project entry is selected, so add-on rows shouldn't look selected.
    pub panel_has_selection: bool,
    /// The panel has keyboard focus (selected rows get the focus border).
    pub panel_focused: bool,
}

pub(crate) trait AnyExtraRows {
    fn row_count(&self, cx: &App) -> usize;
    fn row_depth(&self, index: usize, cx: &App) -> usize;
    fn render_row(
        &self,
        index: usize,
        state: ExtraRowState,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}

impl<T: ProjectPanelExtraRows> AnyExtraRows for Entity<T> {
    fn row_count(&self, cx: &App) -> usize {
        self.read(cx).row_count(cx)
    }

    fn row_depth(&self, index: usize, cx: &App) -> usize {
        self.read(cx).row_depth(index, cx)
    }

    fn render_row(
        &self,
        index: usize,
        state: ExtraRowState,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        self.update(cx, |rows, cx| rows.render_row(index, state, window, cx))
    }
}

pub(crate) struct ExtraRows {
    rows: Box<dyn AnyExtraRows>,
    _observe: Subscription,
}

impl ProjectPanel {
    /// Shows `rows` below the project tree. The panel re-renders when they notify.
    pub fn add_extra_rows<T: ProjectPanelExtraRows>(
        &mut self,
        rows: Entity<T>,
        cx: &mut Context<Self>,
    ) {
        let observe = cx.observe(&rows, |_, _, cx| cx.notify());
        self.extra_rows.push(ExtraRows {
            rows: Box::new(rows),
            _observe: observe,
        });
        cx.notify();
    }

    /// Clears the project selection (an add-on row was selected instead).
    pub fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selection = None;
        self.marked_entries.clear();
        cx.notify();
    }

    pub fn workspace(&self) -> gpui::WeakEntity<workspace::Workspace> {
        self.workspace.clone()
    }

    pub fn project(&self) -> &Entity<project::Project> {
        &self.project
    }

    pub(crate) fn extra_row_count(&self, cx: &App) -> usize {
        self.extra_rows
            .iter()
            .map(|extra| extra.rows.row_count(cx))
            .sum()
    }

    /// The add-on row at `index` (counted from the first add-on row).
    fn extra_row_at(&self, mut index: usize, cx: &App) -> Option<(&dyn AnyExtraRows, usize)> {
        for extra in &self.extra_rows {
            let count = extra.rows.row_count(cx);
            if index < count {
                return Some((extra.rows.as_ref(), index));
            }
            index -= count;
        }
        None
    }

    /// Renders the add-on rows of the list range `range`; `tree_rows` is the
    /// number of project rows before them.
    pub(crate) fn render_extra_rows(
        &self,
        range: std::ops::Range<usize>,
        tree_rows: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Stateful<Div>> {
        let state = ExtraRowState {
            panel_has_selection: self.selection.is_some(),
            panel_focused: !self.mouse_down && self.contains_focus(window, cx),
        };
        (range.start.max(tree_rows)..range.end)
            .filter_map(|index| {
                let (rows, local) = self.extra_row_at(index - tree_rows, cx)?;
                let row = rows.render_row(local, state, window, cx);
                Some(div().id(("vega-extra-row", index)).child(row))
            })
            .collect()
    }

    /// Indent depths of the add-on rows in `range`, for the indent guides.
    pub(crate) fn extra_row_depths(
        &self,
        range: std::ops::Range<usize>,
        tree_rows: usize,
        cx: &App,
    ) -> Vec<usize> {
        (range.start.max(tree_rows)..range.end)
            .filter_map(|index| {
                let (rows, local) = self.extra_row_at(index - tree_rows, cx)?;
                Some(rows.row_depth(local, cx))
            })
            .collect()
    }
}

/// The outer element of an add-on row: same background, hover and focus
/// border as a project entry.
pub fn extra_row_container(
    id: impl Into<ElementId>,
    selected: bool,
    focused: bool,
    cx: &App,
) -> Stateful<Div> {
    let colors = get_item_color(false, cx);
    let background = if selected {
        colors.marked
    } else {
        colors.default
    };
    let border: Hsla = if selected && focused {
        colors.focused
    } else {
        background
    };
    let hover = if selected {
        colors.marked
    } else {
        colors.hover
    };
    div()
        .id(id)
        .relative()
        .cursor_pointer()
        .rounded_none()
        .bg(background)
        .border_1()
        .border_r_2()
        .border_color(border)
        .hover(move |style| {
            style
                .bg(hover)
                .border_color(if selected && focused { border } else { hover })
        })
}

/// The list item inside an add-on row, indented like a project entry.
pub fn extra_row_item(id: impl Into<ElementId>, depth: usize, cx: &App) -> ListItem {
    let settings = ProjectPanelSettings::get_global(cx);
    ListItem::new(id)
        .indent_level(depth)
        .indent_step_size(px(settings.indent_size))
        .spacing(match settings.entry_spacing {
            ProjectPanelEntrySpacing::Comfortable => ListItemSpacing::Dense,
            ProjectPanelEntrySpacing::Standard => ListItemSpacing::ExtraDense,
        })
        .selectable(false)
}
