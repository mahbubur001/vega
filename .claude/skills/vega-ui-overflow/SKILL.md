---
name: vega-ui-overflow
description: Fix VEGA/GPUI UI where content spills out of its box — popup or picker rows, hover backgrounds, separators or right-aligned labels drawn past a popover's edge, text running out of a panel, a modal wider than its border. Use when a screenshot shows anything "out of the box", and before building a new picker, popover or modal.
---

# Content spilling out of a popup, picker or panel

## 1. Run the check first

```sh
script/vega-check-ui-widths
```

It lists every file that builds a `Picker` and narrows it with a wrapper
`.w(...)` instead of `.initial_width(...)`. That has been the cause every time
so far: the branch popup, Scratches list, Deployment server picker and Settings
Sync history were all fixed on 2026-11-21.

## 2. The known cause: a Picker narrowed from the outside

`Picker` sizes itself. It opens at `picker::DEFAULT_MODAL_WIDTH` (34 rem)
unless it's built with `.initial_width(...)`. A wrapper narrows only its own
box, so:

```rust
// WRONG: the box is 30 rem, but the rows stay 34 rem and draw past its right edge.
v_flex().w(rems(30.)).child(self.picker.clone())

// RIGHT: the picker owns its width; the wrapper has none.
let picker = cx.new(|cx| Picker::list(delegate, window, cx).initial_width(rems(30.)));
v_flex().key_context("…").child(self.picker.clone())
```

The symptom is unmistakable: the popup's border and background stop, but the
selected row's highlight, the separators and the end slots (`origin/dev ›`)
carry on to the right.

## 3. Other causes, if the check passes

| Symptom | Cause | Fix |
|---|---|---|
| A long label pushes the row wider than the popup | Flex children don't shrink below their content | `.min_w_0()` on the flex child and `.truncate()` on the `Label` |
| An end slot sits outside the row | The fixed-width end slot plus the start content is wider than the row | Put the variable part in `.flex_1().min_w_0()` and keep the end slot `.flex_none()` |
| A modal is wider than its frame | The width is set on an inner child while the frame wraps something else | Set the width on the outermost element the frame wraps (`Modal`, `vega_dialog`) |
| A scroll area's content draws over the next element | No clipping | `.overflow_hidden()` (or `.overflow_y_scroll()` with an `id`) on the container |
| A popover goes off screen | The anchor is near the window edge | `PopoverMenu::anchor(Corner::…)` and `.attach(...)` |

## 4. Verify

1. `script/vega-check-ui-widths` passes.
2. `cargo check -p <crate>`, then build and relaunch VEGA Dev (`script/vega-dev-app <project>`; quit with `osascript -e 'tell application "VEGA Dev" to quit'`).
3. Open the popup, hover the longest row, and confirm the highlight ends at the border.
4. If the cause was new, add a row to the table above. If a script can detect it, extend `script/vega-check-ui-widths`.
