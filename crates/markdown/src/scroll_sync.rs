//! VEGA: lets another crate keep a scrolled Markdown element and its source
//! editor aligned. Kept in its own file so that `markdown.rs` only gains a
//! module declaration, one field and one call.

use gpui::{App, Context, EventEmitter, Pixels, Point, ScrollHandle, Window, point};

use super::{AutoscrollBehavior, Markdown, MarkdownElement, RenderedText};

/// Emitted when the element's scroll container was scrolled by anything other
/// than [`Markdown::scroll_source_index_to_top`] or the element's own
/// autoscroll: the mouse wheel, the scrollbar, a scroll action.
pub struct MarkdownScrolled {
    /// The source offset of the text now at the top of the viewport.
    pub top_source_index: usize,
}

impl EventEmitter<MarkdownScrolled> for Markdown {}

#[derive(Default)]
pub(super) struct ScrollSyncState {
    scroll_handle: Option<ScrollHandle>,
    rendered_text: Option<RenderedText>,
    /// The scroll offset `rendered_text` was laid out with.
    layout_offset: Point<Pixels>,
    /// The offset the container was last left at by the element or by
    /// `scroll_source_index_to_top`. A different offset on the next frame means
    /// something else scrolled it.
    settled_offset: Option<Point<Pixels>>,
}

impl Markdown {
    /// Scrolls the element's scroll container so the text rendered from
    /// `source_index` sits at the top of the viewport, moved up by
    /// `line_fraction` of a line. Does nothing until the element has been
    /// painted inside a scroll container it controls.
    pub fn scroll_source_index_to_top(
        &mut self,
        source_index: usize,
        line_fraction: f32,
        cx: &mut Context<Self>,
    ) {
        let state = &mut self.scroll_sync;
        let (Some(scroll_handle), Some(rendered_text)) =
            (&state.scroll_handle, &state.rendered_text)
        else {
            return;
        };
        let Some((position, line_height)) = rendered_text.position_for_source_index(source_index)
        else {
            return;
        };
        let distance_below_viewport_top =
            position.y + line_height * line_fraction - scroll_handle.bounds().top();
        let current_offset = scroll_handle.offset();
        let offset = point(
            current_offset.x,
            (state.layout_offset.y - distance_below_viewport_top)
                .clamp(-scroll_handle.max_offset().y, Pixels::ZERO),
        );
        if offset == current_offset {
            return;
        }
        scroll_handle.set_offset(offset);
        state.settled_offset = Some(offset);
        cx.refresh_windows();
    }
}

impl MarkdownElement {
    pub(super) fn autoscroll_and_track_scroll(
        &self,
        rendered_text: &RenderedText,
        window: &mut Window,
        cx: &mut App,
    ) {
        let AutoscrollBehavior::Controlled(scroll_handle) = &self.autoscroll else {
            self.autoscroll(rendered_text, window, cx);
            return;
        };
        // Read before the autoscroll below moves it: the lines were laid out
        // with this offset.
        let layout_offset = scroll_handle.offset();
        let scrolled_by_something_else = self
            .markdown
            .read(cx)
            .scroll_sync
            .settled_offset
            .is_some_and(|settled_offset| settled_offset != layout_offset);
        if scrolled_by_something_else
            && let Some(top_source_index) =
                rendered_text.top_source_index(scroll_handle.bounds().top())
        {
            self.markdown.update(cx, |_, cx| {
                cx.emit(MarkdownScrolled { top_source_index });
            });
        }
        self.autoscroll(rendered_text, window, cx);
        let settled_offset = scroll_handle.offset();
        self.markdown.update(cx, |markdown, _| {
            markdown.scroll_sync = ScrollSyncState {
                scroll_handle: Some(scroll_handle.clone()),
                rendered_text: Some(rendered_text.clone()),
                layout_offset,
                settled_offset: Some(settled_offset),
            };
        });
    }
}

impl RenderedText {
    fn top_source_index(&self, viewport_top: Pixels) -> Option<usize> {
        self.lines.iter().find_map(|line| {
            // Only a prepainted line has bounds to ask for.
            line.visible_bounds.get()?;
            let bounds = line.layout.bounds();
            if bounds.bottom() <= viewport_top {
                return None;
            }
            let position = point(bounds.left(), viewport_top.max(bounds.top()));
            match line.source_index_for_position(position) {
                Ok(source_index) | Err(source_index) => Some(source_index),
            }
        })
    }
}
