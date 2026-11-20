# Zed Markdown preview: current state and feasibility of an IntelliJ-style three-mode view

Scope notes for the report writer:

- Local citations are `path:line` relative to the repository root. Line numbers are from the working tree on branch `vega` at commit `72811c6b22`.
- The fork's merge base with upstream `main` is `dd091a1b4f` (2026-11-18). `git diff upstream/main...vega` over `crates/markdown`, `crates/markdown_preview`, `crates/svg_preview` and the quick action bar shows only a 1-line change in `quick_action_bar/repl_menu.rs`. So everything described under "local code" is upstream Zed as of 2026-11-18, not fork-specific work.
- Nothing was built or run. All behavior claims come from reading source and test names, not from observing the app.
- Web vote counts were read on 2026-11-20 through the GitHub search API and issue pages, via a summarizing fetch tool. Treat counts as approximate snapshots.

## 1. What views, actions and keybindings exist in `crates/markdown_preview`, and how is the preview tab created and placed?

### Takeaway
There is exactly one view, `MarkdownPreviewView`, a separate workspace `Item` (its own tab) bound to a source `Editor`. Three open actions exist (same pane, side pane, following), plus an opt-in "open .md in preview" hook. "Split" is implemented by putting the preview tab in the pane to the right, not by a composite item.

### Cited Findings
- The crate has three source files: `markdown_preview.rs` (49 lines, actions + init), `markdown_preview_settings.rs` (27 lines), `markdown_preview_view.rs` (5031 lines, of which roughly 2360 onward are tests) — `crates/markdown_preview/src/markdown_preview.rs:1-49`, `crates/markdown_preview/src/markdown_preview_view.rs:2359`.
- `MarkdownPreviewView` struct fields: weak workspace, `active_editor: Option<EditorState>`, `markdown: Entity<Markdown>`, `active_source_index`, `scroll_handle: ScrollHandle`, image cache, `base_directory`, `pending_update_task`, `mode: MarkdownPreviewMode`, nav history — `crates/markdown_preview/src/markdown_preview_view.rs:71-89`.
- `MarkdownPreviewMode` has two variants: `Default` (always shows the editor it was created for) and `Follow` (follows the active editor). This enum is about *which file* is previewed, not about layout — `crates/markdown_preview/src/markdown_preview_view.rs:91-97`.
- Actions defined in the crate (namespace `markdown`): `ScrollPageUp`, `ScrollPageDown`, `ScrollUp`, `ScrollDown`, `ScrollUpByItem`, `ScrollDownByItem`, `ScrollToTop`, `ScrollToBottom`, `OpenFollowingPreview`, `CloseAndReturnToEditor` — `crates/markdown_preview/src/markdown_preview.rs:11-37`.
- `markdown::OpenPreview` and `markdown::OpenPreviewToTheSide` are defined in the shared `zed_actions` crate and re-exported — `crates/zed_actions/src/lib.rs:897-910`, `crates/markdown_preview/src/markdown_preview.rs:7`.
- Registration: `init` registers the view as a serializable item and calls `MarkdownPreviewView::register` for every new `Workspace` — `crates/markdown_preview/src/markdown_preview.rs:39-49`; called from app startup at `crates/zed/src/main.rs:804`.
- `OpenPreview` handler: takes `workspace.item_for_action`, requires it to act as a Markdown `Editor`, opens the preview as a new tab in the same pane as that item — `crates/markdown_preview/src/markdown_preview_view.rs:200-208`, `:243-251`.
- `OpenPreviewToTheSide` handler: resolves the active Markdown editor, then `open_preview_to_the_side_of_pane`, which uses `workspace.adjacent_pane_of(&origin_pane, …)`, adds/activates the preview there without focusing it, and re-focuses the editor — `crates/markdown_preview/src/markdown_preview_view.rs:210-215`, `:253-271`.
- `Workspace::adjacent_pane_of` returns the existing pane to the right if one exists, otherwise splits the origin pane to the right — `crates/workspace/src/workspace.rs:6705-6715`.
- `OpenFollowingPreview` handler: reuses an existing `Follow` preview in the active pane or adds a new one to the *active* pane (not to the side) — `crates/markdown_preview/src/markdown_preview_view.rs:217-240`.
- De-duplication: `activate_or_add_preview` reuses an existing `Default`-mode preview for the same buffer in the target pane (matched by buffer entity, not editor entity) — `crates/markdown_preview/src/markdown_preview_view.rs:273-317`.
- Auto-preview hook: `register` subscribes to `workspace::Event::ItemAdded`; when `open_markdown_files_in_preview` is on, the workspace is not restoring, and the added item is a Markdown editor, it adds a preview to the same pane and, if the editor is not dirty, removes the editor tab (or replaces the pane's temporary "preview tab" slot) — `crates/markdown_preview/src/markdown_preview_view.rs:137-198`.
- A global `SuppressedAutoPreviews` set stops the hook from re-triggering after the user returns to source — `crates/markdown_preview/src/markdown_preview_view.rs:120-123`, `:147-153`, `:1041-1053`.
- `CloseAndReturnToEditor` activates the source editor (re-adding it to the pane if it was closed) and closes the preview tab; it is also exposed as a tab context-menu entry "Show Source" — `crates/markdown_preview/src/markdown_preview_view.rs:1018-1072`, `:1757-1763`.
- Default keybindings, macOS: `cmd-k v` = `markdown::OpenPreviewToTheSide`, `cmd-shift-v` = `markdown::OpenPreview`, in context `Editor && extension == md` — `assets/keymaps/default-macos.json:643-647`. In context `MarkdownPreview`: `pageup`/`pagedown`, `up`/`down`, `alt-up`/`alt-down`, `cmd-up`/`cmd-down`, and `cmd-shift-v` = `markdown::CloseAndReturnToEditor` — `assets/keymaps/default-macos.json:1557-1567`.
- Linux/Windows equivalents: `ctrl-k v`, `ctrl-shift-v` — `assets/keymaps/default-linux.json:607-608`, `:1463-1473`; `assets/keymaps/default-windows.json:605-606`, `:1489-1500`.
- Other keymaps: Atom `ctrl-shift-m` and Sublime `shift-alt-m` map to `OpenPreviewToTheSide` — `assets/keymaps/macos/atom.json:36`, `assets/keymaps/macos/sublime_text.json:54`; Vim scroll bindings in the preview — `assets/keymaps/vim.json:1232-1241`.
- `OpenFollowingPreview` has no default keybinding in any keymap file (grep over `assets/keymaps` found none; only the lines listed above reference `markdown::`).
- The preview tab title is `Preview <file name>`, icon `IconName::FileDoc` — `crates/markdown_preview/src/markdown_preview_view.rs:1663-1665`, `:1678-1685`.
- The preview acts as an `Editor` for `act_as::<Editor>` lookups, forwards save/save-as to the source editor, is searchable, can be split and cloned into another pane, and is restored across restarts from a `markdown_previews` SQLite table storing `abs_path` and `mode` — `crates/markdown_preview/src/markdown_preview_view.rs:1646-1661`, `:1784-1828`, `:1854-1893`, `:2214-2302`, `:2317-2330`.

### Inferences
- "Editor + Preview" in today's Zed is two tabs in two panes. Nothing ties their lifetimes together: closing the editor pane leaves the preview, and vice versa.
- "Preview only" already exists in two forms: `OpenPreview` (adds a preview tab beside the editor tab in the same pane) and the auto-preview setting (replaces the editor tab). Neither is presented to the user as a mode.
- The existing `MarkdownPreviewMode` name collides conceptually with an IntelliJ "view mode". A new layout enum should use a different name (for example `MarkdownLayout`) to avoid confusion and merge conflicts.

### Gaps
- Not verified at runtime whether `cmd-shift-v` in an editor pane with an existing preview tab toggles cleanly both ways; the code path suggests it does (editor: `OpenPreview`; preview: `CloseAndReturnToEditor`), and a user on issue #10966 reports "ctrl-shift-V toggles the modes".

## 2. How does the preview follow the editor, sync scroll/selection, map clicks back to source, and update on edits?

### Takeaway
Sync is cursor-driven and block-granular, not scroll-driven. Moving the editor cursor scrolls the preview to the matching block and highlights it; clicking in the preview moves the editor cursor and centers it. Scrolling either side with the wheel or scrollbar does not move the other side. Edits re-parse after a 200 ms debounce.

### Cited Findings
- The preview subscribes to these `EditorEvent`s only: `Edited`, `BufferEdited`, `DirtyChanged`, `BuffersEdited` (re-parse), `FileHandleChanged` (re-base directory), and `SelectionsChanged` (sync). There is no handler for `EditorEvent::ScrollPositionChanged` — `crates/markdown_preview/src/markdown_preview_view.rs:561-594`.
- `EditorEvent::ScrollPositionChanged { local, autoscroll }` exists and is emitted by the editor's scroll manager — `crates/editor/src/scroll.rs:410`, `crates/editor/src/editor.rs:12774`.
- Editor to preview: on `SelectionsChanged`, the last selection's start offset becomes `active_source_index`; the markdown entity marks the containing root block active and, if the editor or the preview is focused, requests an autoscroll to that source index — `crates/markdown_preview/src/markdown_preview_view.rs:578-590`, `:723-762`.
- The autoscroll is "reveal with margin", not "align": it only moves the preview if the target line is within 3 line-heights of the viewport edge or outside it — `crates/markdown/src/markdown.rs:2588-2622`.
- The active block is drawn with a marker because the preview enables `show_root_block_markers()` — `crates/markdown_preview/src/markdown_preview_view.rs:1127`; `crates/markdown/src/markdown.rs:1951-1954`, `:2765-2778`.
- Preview to editor: a single click in the preview calls `on_source_click`, which sets the editor selection to the clicked source offset with `Autoscroll::center()` and does not move focus — `crates/markdown_preview/src/markdown_preview_view.rs:1175-1186`, `:805-824`.
- Same-document `#heading` links scroll the preview and move the editor cursor (with focus); links to other Markdown files open in the preview (alt-click opens source instead) and can target a heading or `:row:col` / `#L` position — `crates/markdown_preview/src/markdown_preview_view.rs:1246-1297`, `:1299-1349`.
- Update on edit: `REPARSE_DEBOUNCE` is 200 ms; a debounced update is skipped if one is already pending; the whole buffer is copied to a string and passed to `Markdown::reset`, which re-parses in the background — `crates/markdown_preview/src/markdown_preview_view.rs:63`, `:648-721`; `crates/markdown/src/markdown.rs:1064-1085`.
- `Follow` mode observes the workspace and rebinds to whichever Markdown editor becomes the active item — `crates/markdown_preview/src/markdown_preview_view.rs:437-447`, `:469-482`.
- `Default` mode rebinds to the canonical editor for its buffer on `ItemAdded`/`ItemRemoved`, so cursor sync survives workspace restore — `crates/markdown_preview/src/markdown_preview_view.rs:448-462`, `:608-646`.
- The preview's own scroll position lives in a GPUI `ScrollHandle` tracked on a `div` with `overflow_y_scroll()`; scrolling actions adjust `scroll_handle.set_offset` directly — `crates/markdown_preview/src/markdown_preview_view.rs:904-908`, `:1949-1955`.
- The markdown crate has internal position mapping in both directions (`position_for_source_index`, `source_index_for_visible_position`, `source_index_for_position`), but these are private methods of the rendered-text type — `crates/markdown/src/markdown.rs:5084`, `:5094`, `:4811`.
- Upstream's original preview PR (#6958, 2024) had a reviewer request that scrolling the rendered view also scroll the editor — [PR #6958 (via search summary)](https://github.com/zed-industries/zed/pull/6958).
- An upstream PR in June 2026 restored preview tabs on reload and kept "cursor-driven scroll sync (`EditorEvent::SelectionsChanged`)" working — [PR #56972 (via search summary)](https://github.com/zed-industries/zed/pull/56972).

### Inferences
- True scroll sync (IntelliJ's "Auto-Scroll Preview" / scroll-lock behavior) is missing in both directions. Adding it needs (a) a subscription to `EditorEvent::ScrollPositionChanged` in the preview and (b) a public source-offset-to-pixel mapping exported from `crates/markdown`, plus an echo guard so each side does not re-trigger the other.
- (b) requires touching the upstream file `crates/markdown/src/markdown.rs` (9595 lines, actively changed upstream), so it is the least merge-friendly part of the feature. A small additive `pub fn` is the thinnest possible seam.
- Because the existing autoscroll is "reveal" rather than "align top", cursor sync alone can leave the two panes visibly offset by most of a viewport.

### Gaps
- Not measured: re-parse cost on large documents (upstream issue #58933 reports slow rendering with many tables).
- The exact semantics of `ScrollPositionChanged { local, autoscroll }` flags were not read in detail; they matter for the echo guard.

## 3. Which Markdown elements are rendered, with what parser, and how are `crates/markdown` and `crates/markdown_preview` split?

### Takeaway
Rendering is done entirely by the shared `crates/markdown` crate (pulldown-cmark parser, GPUI element, html5ever for an HTML subset, `merman` for Mermaid). `crates/markdown_preview` is now only the workspace item wrapper. The migration from a separate preview-only parser/renderer is finished (March 2026). Mermaid is supported; math is explicitly not.

### Cited Findings
- `crates/markdown` depends on `pulldown-cmark`, `html5ever`, `markup5ever_rcdom`, `mermaid_render`, `linkify` — `crates/markdown/Cargo.toml:21-40`.
- Parser options enabled: tables, footnotes (new and old style), strikethrough, task lists, smart punctuation, heading attributes, `+++` metadata blocks, GFM, superscript, subscript; YAML metadata blocks are added conditionally — `crates/markdown/src/parser.rs:13-23`, `:298`.
- Explicitly not enabled: `ENABLE_MATH`, `ENABLE_DEFINITION_LIST`, `ENABLE_WIKILINKS` — `crates/markdown/src/parser.rs:1044-1047`. Math events are dropped if they ever appear — `crates/markdown/src/parser.rs:731`.
- `ENABLE_MATH` was removed in January 2025 because it broke links containing a dollar sign — local git log, commit `a653e8adda` "Remove ENABLE_MATH option from pulldown_cmark to fix links which contain dollar sign (#22647)".
- The preview constructs `Markdown` with `parse_html: true`, `render_mermaid_diagrams: true`, `parse_heading_slugs: true`, `render_metadata_blocks: true` — `crates/markdown_preview/src/markdown_preview_view.rs:391-405`; option definitions at `crates/markdown/src/markdown.rs:528-538`.
- Task-list checkboxes are clickable in the preview and edit the source: `on_checkbox_toggle` replaces the `[ ]`/`[x]` marker range in the editor buffer — `crates/markdown_preview/src/markdown_preview_view.rs:1187-1195`, `:1201-1230`; covered by test `toggles_task_checkbox_and_saves_when_preview_is_active` at `:3230`.
- Code blocks: rendered with a copy button on hover, no wrap button, no border; a language registry is passed in for syntax highlighting — `crates/markdown_preview/src/markdown_preview_view.rs:1121-1125`, `:391-395`.
- Images: resolved through `resolve_preview_image` against the file's directory, the first worktree root, and (for remote projects) the project image store; an image cache entity is retained per view — `crates/markdown_preview/src/markdown_preview_view.rs:1128-1141`, `:1526-1597`, `:80`.
- HTML: a whitelisted subset is parsed and rendered. Element names referenced in the HTML parser: `a`, `b`, `blockquote`, `caption`, `del`, `em`, `h1`–`h6`, `i`, `img`, `ins`, `li`, `ol`, `p`, `strong`, `table`, `tbody`, `td`, `th`, `thead`, `tr`, `ul`; attributes `align`, `alt`, `colspan`, `rowspan`, `height`, `width`, `href`, `src`, `style` — `crates/markdown/src/html/html_parser.rs:262-427` (name list from `grep local_name!`).
- Footnotes: references and definitions are parsed and clickable — `crates/markdown/src/parser.rs:42`, `:474-475`, `:721-723`; `crates/markdown/src/markdown.rs:1042-1047`, `:2378`, `:2420-2423`.
- Front matter: parsed as metadata blocks and rendered when `render_metadata_blocks` is on — `crates/markdown/src/markdown.rs:1542`, `:2224`, `:2758`.
- GFM alert block quotes (`[!NOTE]` etc.) are preserved by the parser as `BlockQuote(Option<BlockQuoteKind>)` — `crates/markdown/src/parser.rs:878`, `:1987-1990`.
- Mermaid: `crates/mermaid_render` renders Mermaid text to SVG with the `merman` crate, with these diagram features enabled: class, er, flowchart, gantt, git-graph, journey, mindmap, pie, quadrant-chart, sequence, state, timeline, xychart — `crates/mermaid_render/Cargo.toml:21-37`; crate docs at `crates/mermaid_render/src/mermaid_render.rs:4-30`. Diagram zoom exists — `crates/markdown/src/mermaid.rs:22-25`, `crates/markdown/src/markdown.rs:1935-1941`.
- Preview search (find in preview) is implemented via `SearchableItem`, without replace — `crates/markdown_preview/src/markdown_preview_view.rs:2044-2057`.
- Preview context menu offers "Copy", "Copy as Markdown", "Copy Link" — `crates/markdown_preview/src/markdown_preview_view.rs:1961-2013`.
- The migration is complete: commit `9efe3c5a21` (2026-03-26) "markdown_preview: Refactor to use shared markdown crate (#52008)" deleted `markdown_elements.rs`, `markdown_minifier.rs`, `markdown_parser.rs`, `markdown_renderer.rs` from `crates/markdown_preview/src` — local `git log --diff-filter=D`.
- Upstream closed "Mermaid diagram rendering in Markdown preview" (#10696, 364 thumbs-up) as completed on 2026-02-13 via merged PR #49064 — [GitHub search API, mermaid](https://api.github.com/search/issues?q=repo:zed-industries/zed+mermaid+in:title&sort=reactions-%2B1&order=desc&per_page=12).
- Upstream closed "Select text in markdown preview" (#16727, 97 thumbs-up) as completed on 2026-03-27 — [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+scroll+sync+OR+synchronized+OR+side-by-side+OR+split&sort=reactions-%2B1&order=desc&per_page=12).

### Inferences
- Since rendering is shared with the agent panel and notebooks (the settings doc says heading weight "also applies to rendered Markdown cells in notebooks", `crates/settings_content/src/settings_content.rs:1368-1370`), any fork change inside `crates/markdown` affects more than the preview and will conflict with frequent upstream edits. Layout/mode work should stay out of that crate.
- Gaps versus IntelliJ's renderer (which is an embedded browser): no math, no arbitrary HTML/CSS (`<details>`, `<kbd>`, `<sub>`/`<sup>` as HTML tags are not in the whitelist), no definition lists, no wikilinks, no custom CSS, no PlantUML.

### Gaps
- Not verified how GFM alert kinds are styled at render time (only the parser side was read).
- Not verified whether `http(s)` image URLs load in the preview; `resolve_preview_image` lines 1526-1559 were not read in full. Test names cover workspace-absolute, workspace-relative and remote-project images only (`:2565`, `:2615`, `:2654`, `:2698`).
- Upstream documentation: `zed.dev/docs/languages/markdown` does not describe the preview at all (see section 7), so there is no official feature list to compare against.

## 4. How do the editor toolbar / quick action bar and per-file-type preview buttons work, and is there a precedent for one item hosting editor + preview?

### Takeaway
There is already an eye-icon "Preview Markdown" button in the quick action bar (click = same pane, alt-click = split), shared with SVG and tabular-data previews. It disappears when the preview tab is active, because the quick action bar is only shown for `Editor` items. A mode switcher that is visible in all three modes therefore needs its own `ToolbarItemView`; several small toolbar items of that shape already exist. A composite "editor + something" item also has a precedent (`SplittableEditor`), but only for editor + editor.

### Cited Findings
- Pane toolbars are composed of `ToolbarItemView` entities; each returns a `ToolbarItemLocation` (`Hidden`, `PrimaryLeft`, `PrimaryRight`, `Secondary`) from `set_active_pane_item` — `crates/workspace/src/toolbar.rs:24-41`, `:57-62`. The toolbar renders nothing when no item is visible — `crates/workspace/src/toolbar.rs:72-76`, `:110-111`.
- Toolbar items are added per pane in `initialize_pane` in the app crate (breadcrumbs, buffer search bar, quick action bar, diff toolbars, etc.) — `crates/zed/src/zed.rs:1469-1529` (quick action bar at `:1496`).
- `QuickActionBar` is `PrimaryRight` only when the active pane item downcasts to `Editor`; otherwise `Hidden` — `crates/zed/src/zed/quick_action_bar.rs:83-87`, `:100-106`, `:773-822`.
- The preview button: `render_preview_button` picks a `PreviewTarget` (`Markdown`, `Svg`, `TabularData`) from the toolbar's own pane item, renders `IconButton::new("toggle-markdown-preview", IconName::Eye)` with tooltip "Preview Markdown" and meta text "alt-click to open in a split", and calls `MarkdownPreviewView::open_preview_in_pane` or `open_preview_to_the_side_of_pane` — `crates/zed/src/zed/quick_action_bar/preview.rs:10-134`; inserted into the bar at `crates/zed/src/zed/quick_action_bar.rs:710`.
- SVG precedent: `crates/svg_preview` mirrors the Markdown crate's shape (same three actions, `open_preview_in_pane`, `open_preview_to_the_side_of_pane` via `adjacent_pane_of`, `is_svg_file`) — `crates/svg_preview/src/svg_preview.rs:6-24`, `crates/svg_preview/src/svg_preview_view.rs:230-310`. Keybindings mirror Markdown — `assets/keymaps/default-macos.json:651-655`.
- Tabular-data precedent: `crates/tabular_data_preview` has `TabularDataPreviewPane` with the same two open functions — `crates/tabular_data_preview/src/tabular_data_preview.rs:65`, `:110-119`, `:211`.
- Small per-item toolbar precedent: `SoloDiffStyleToolbar` stores a weak handle to the active item if it `act_as::<SoloDiffView>`, returns `PrimaryLeft` or `Hidden`, and renders an `h_flex` of `IconButton`s plus `DiffStyleControls` (a split/unified switch) — `crates/git_ui/src/solo_diff_view.rs:558-650`. `FileDiffStyleToolbar` is the same pattern — `crates/git_ui_core/src/file_diff_view.rs:414-430`.
- Composite item precedent: `SplittableEditor` is a single workspace `Item` that owns a right-hand `Editor` and an optional left-hand `Editor`, with split state, a "too narrow for split" fallback and its own `Render` — `crates/editor/src/split.rs:534-554`, `impl Item` at `:1966`. It is used by diff views (`git_ui`, `agent_ui`) per the knowledge graph.
- `MarkdownPreviewView` does not override `show_toolbar`, so the default (`true`) applies — `crates/workspace/src/item.rs:372-374`; but with the quick action bar hidden and no breadcrumbs implementation in the view, no toolbar item is visible when a preview tab is active (inferred from `:1643-1909`, which contains no `breadcrumbs`/`breadcrumb_location` methods).
- Upstream closed "preview button in md/html/latex and svg files" (#10619, 22 thumbs-up) as completed on 2025-03-04 — [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+table+editing+OR+%22side+by+side%22+OR+%22split+preview%22+OR+%22toggle+preview%22+in:title,body+is:issue&sort=reactions-%2B1&order=desc&per_page=10).

### Inferences
- Two implementation shapes are possible:
  - **A. Pane-based modes (thin seam).** Keep editor and preview as separate items. A new `ToolbarItemView` (in a new fork crate) shows three buttons whenever the pane's active item is a Markdown `Editor` or a `MarkdownPreviewView`, and drives existing public functions: Editor only = close this buffer's `Default` previews (in this pane and the adjacent pane) and activate the editor; Split = `open_preview_to_the_side_of_pane`; Preview only = `open_preview_in_pane` then close/hide the editor tab (the auto-preview hook already does exactly this at `markdown_preview_view.rs:186-195`). Upstream files touched: one `toolbar.add_item` line in `crates/zed/src/zed.rs` and one `init` line in `crates/zed/src/main.rs`, plus workspace `Cargo.toml` membership.
  - **B. Composite item (IntelliJ-faithful).** A new item type that owns an `Editor` and a `MarkdownPreviewView` and renders one, the other, or both with a resizable divider inside a single tab, like `SplittableEditor`. This gives one tab per file and a toolbar inside the tab, but it has to be opened *instead of* `Editor` for `.md` files, must forward the large `Item` surface (save, search, navigation, breadcrumbs, serialization, `act_as::<Editor>`), and every feature that does `item.downcast::<Editor>()` (including `QuickActionBar::active_editor`, `quick_action_bar.rs:83-87`) would stop recognizing Markdown files unless adapted.
- Shape A cannot make the toolbar look identical in both panes for free, but each pane has its own toolbar instance (`initialize_pane` runs per pane), so the same switcher can appear above the editor pane and above the preview pane, each resolving "its" buffer through `act_as::<Editor>` (`markdown_preview_view.rs:1654-1657`).
- In shape A the current mode is derived state, not stored state: "is there a `Default` preview for this buffer in the adjacent pane / in this pane, and is an editor tab for it present". `find_existing_independent_preview_item_idx` and `is_previewing` are private (`markdown_preview_view.rs:300`, `:319`); the fork crate can re-implement the lookup with public API (`pane.items_of_type::<MarkdownPreviewView>()` plus `act_as::<Editor>`), or add a one-line `pub` to the upstream file.
- The existing eye button would become redundant next to a three-button switcher. Removing it means editing `quick_action_bar/preview.rs` (upstream file); leaving it costs nothing in merges.

### Gaps
- Not verified whether `PrimaryRight` items from two different `ToolbarItemView`s lay out acceptably together, or whether `PrimaryLeft` (used by the diff toolbars) collides with breadcrumbs for editor items.
- No precedent was found for a composite item that mixes an `Editor` with a non-editor view; `SplittableEditor` is editor + editor only.
- A public fork (not upstream) merged a combined "Editor and Preview" split view with a toolbar icon and two-way scroll sync on 2026-10-28; its file-level design was not readable from the PR page — [superzent PR #71](https://github.com/currybab/superzent/pull/71).

## 5. What Markdown-preview settings exist, and how would a default-layout setting be added?

### Takeaway
A `markdown_preview` settings group already exists with eight keys, including the boolean `open_markdown_files_in_preview`. A three-valued default layout is a natural superset of that boolean. Adding a key touches four upstream files in a well-worn pattern; a fork-only settings struct in a new crate can reduce that but not to zero, because all settings content types live in `crates/settings_content`.

### Cited Findings
- Content type: `MarkdownPreviewSettingsContent` with `font_family`, `code_font_family`, `font_size`, `theme`, `heading_font_weight`, `open_markdown_files_in_preview`, `limit_content_width`, `max_width` — `crates/settings_content/src/settings_content.rs:1353-1389`; mounted on the root as `pub markdown_preview: Option<MarkdownPreviewSettingsContent>` — `crates/settings_content/src/settings_content.rs:249`.
- Runtime type: `MarkdownPreviewSettings { open_markdown_files_in_preview: bool, max_width: Option<Pixels> }` with `#[derive(RegisterSetting)]` and `impl Settings::from_settings` — `crates/markdown_preview/src/markdown_preview_settings.rs:5-27`. Font and theme keys are read through `ThemeSettings` instead — `crates/theme_settings/src/settings.rs:68-82`, `:436-467`.
- Defaults: `assets/settings/default.json:116-135` (`open_markdown_files_in_preview: false`, `limit_content_width: true`, `max_width: 800`, `heading_font_weight: 600`).
- Settings UI: entries are hand-written `SettingsPageItem::SettingItem` records with `json_path`, `pick` and `write` closures; "Open Markdown Files in Preview" is at `crates/settings_ui/src/page_data.rs:11222-11243`, width items at `:11244-11300`, font section at `:1344-1440`.
- Docs for the group: `docs/src/reference/all-settings.md:4221-4241`.
- A separate top-level key `mermaid_font_family` exists — `docs/src/reference/all-settings.md:4251-4255`.
- Consumers of the boolean: the `ItemAdded` hook — `crates/markdown_preview/src/markdown_preview_view.rs:154`.
- Upstream PR #63462 (merged 2026-09-04) grouped the settings and added `open_markdown_files_in_preview`; release note: "Added a setting to open Markdown files directly in preview mode and grouped Markdown preview settings together." The reviewer called the lack of an obvious way back to source "a major drawback" but approved it as opt-in — [PR #63462](https://github.com/zed-industries/zed/pull/63462).

### Inferences
- Minimal-diff recipe for a `default_layout` key (`"editor" | "split" | "preview"`):
  1. Add an enum and an `Option<…>` field to `MarkdownPreviewSettingsContent` (`crates/settings_content/src/settings_content.rs`, additive lines at the end of the struct).
  2. Add the default to `assets/settings/default.json` inside the `markdown_preview` block.
  3. Read it in a fork-owned settings struct (new crate, own `RegisterSetting` + `Settings` impl reading `content.markdown_preview`), so `markdown_preview_settings.rs` stays untouched.
  4. Optionally add one `SettingItem` to `crates/settings_ui/src/page_data.rs` next to line 11243 and a line to `docs/src/reference/all-settings.md`.
- Behavior seam: the fork crate can register its own `workspace::Event::ItemAdded` subscription (same pattern as `markdown_preview_view.rs:140-198`) and call the public `open_preview_to_the_side_of_pane` for `"split"`. For `"preview"` it can either delegate to the upstream boolean or call `open_preview_in_pane`. The upstream guards worth copying are `workspace.is_restoring()` (`crates/workspace/src/workspace.rs:3004`), the dirty-editor check, and the temporary-tab handling.
- If both the upstream boolean and the fork's enum are set, precedence must be defined (for example: the enum wins when present; `open_markdown_files_in_preview: true` is treated as `"preview"` when the enum is absent). Upstream may itself later generalize the boolean, which would conflict; keeping the fork key name distinct lowers that risk.
- `SuppressedAutoPreviews` is private to the upstream file (`markdown_preview_view.rs:120-123`), so a fork hook that auto-opens a split needs its own suppression set to avoid re-opening the preview after the user picks "Editor only".

### Gaps
- Not checked whether `crates/settings_content` enforces that every content field appears in the settings UI or docs (some repos have a test for this). A grep for `markdown_preview` in `crates/migrator/src` returned nothing, although PR #63462 says legacy keys are migrated; the migration location was not found.
- Per-file persistence of the chosen mode (IntelliJ remembers layout per file) has no existing store; the `markdown_previews` table (`markdown_preview_view.rs:2317-2330`) only records preview tabs.

## 6. What tests exist for Markdown preview, and how could new behavior be tested?

### Takeaway
The preview has a large in-file GPUI test suite (about 45 tests) built on fake filesystems and `MultiWorkspace` test windows, with reusable helpers for opening files, opening previews, toggling the auto-preview setting and asserting pane contents. New mode/toolbar behavior can be tested the same way without rendering.

### Cited Findings
- Test module starts at `crates/markdown_preview/src/markdown_preview_view.rs:2359`.
- Setup helpers: `init_test` (test DB, `AppState::test`, `editor::init`, `crate::init`) at `:4830-4838`; `markdown_workspace` (FakeFs tree, `Project::test`, `MultiWorkspace::test_new`) at `:4840-4866`; `open_markdown_file` (uses `open_paths`) at `:4184-4220`; `open_preview_for_active_editor` at `:4222-4240`; `set_auto_preview_enabled` (writes user settings through `SettingsStore::update_global`) at `:4908-4917`; `register_markdown_language` / `register_lazy_markdown_language` at `:4938-4950`.
- Assertion helpers: `assert_editor_is_active_and_focused` `:4258`, `assert_no_markdown_preview_items` `:4279`, `assert_editor_replaced_by_preview` `:4894`.
- Pane/placement tests: `preview_opens_for_the_given_pane_not_the_focused_editor` `:4683`, `default_preview_stays_bound_to_invoking_editor_across_splits` `:4099`, `splitting_preview_clones_into_new_pane` `:3568`.
- Auto-preview tests: `opens_markdown_files_in_preview_when_enabled` `:4298`, `workspace_restore_does_not_auto_preview_markdown` `:4382`, `dirty_markdown_editor_is_retained_when_auto_preview_runs` `:4424`, `automatic_previews_preserve_temporary_tab_semantics` `:4482`, `automatic_preview_preserves_pane_activation_and_focus` `:4527`.
- Return-to-source tests: `close_and_return_to_editor_closes_preview_and_focuses_source_editor` `:3635` and three siblings through `:3789`.
- Sync/navigation tests: `scrolls_position_link_in_newly_created_preview` `:3121`, `targeted_markdown_open_reveals_the_source_position` `:4622`, `opens_markdown_links_in_preview` `:2885`.
- Editing-from-preview test: `toggles_task_checkbox_and_saves_when_preview_is_active` `:3230`.
- Serialization tests: `preview_serialized_path_updates_when_source_file_is_renamed` `:3838`, `follow_preview_serialized_path_updates_when_followed_editor_changes` `:3968`.
- Autoscroll unit tests exist in the markdown crate (assertions on `autoscroll_request`) — `crates/markdown/src/markdown.rs:5378`, `:5410`.
- Dev-dependencies needed for this style of test: `db`, `editor`, `gpui`, `workspace` with `test-support`, plus `fs`, `serde_json`, `buffer_diff` — `crates/markdown_preview/Cargo.toml:36-43`.

### Inferences
- A fork crate can copy the helper pattern and assert, per mode: number of panes, which pane holds an `Editor` vs a `MarkdownPreviewView` for the buffer, which item is active and focused, and that the dirty buffer is never closed. The helpers themselves are private to the upstream test module, so they must be duplicated rather than imported.
- The toolbar item can be unit-tested by calling `set_active_pane_item` with an editor item and a preview item and asserting the returned `ToolbarItemLocation`, as the trait is plain Rust (`crates/workspace/src/toolbar.rs:24-30`).
- Scroll-sync tests would need layout (the mapping exists only after `MarkdownElement` paints), so they are more expensive; the superzent fork lists tests "covering scrolling each side, reflow, Mermaid anchors, code block rows, multibyte text" as feasible under `cargo test -p markdown -p markdown_preview` — [superzent PR #71](https://github.com/currybab/superzent/pull/71).

### Gaps
- No existing test exercises `QuickActionBar::render_preview_button`; none was found by name. No visual/screenshot test for the preview was looked for.

## 7. What does upstream Zed document and what are users asking for (issues, votes, maintainer statements, 2025–2026 PRs)?

### Takeaway
Upstream has been shipping steady incremental preview work through 2026 (shared renderer, Mermaid, text selection, auto-open in preview, "Show Source", clone/split, find, link navigation). It has not shipped, and staff have not committed to, a combined split view with a mode switcher, scroll sync, WYSIWYG/live preview, or math. Live preview was explicitly declined for the near term; math has two open community PRs with no staff decision.

### Cited Findings
- The official Markdown language page covers only syntax highlighting of code blocks, Prettier formatting, list continuation/indent and trailing whitespace. It does not mention the preview, Mermaid, math or tables — [zed.dev/docs/languages/markdown](https://zed.dev/docs/languages/markdown). The local copy matches: headings are "Syntax Highlighting Code Blocks", "Configuration", "Format", "List Continuation", "List Indentation", "Trailing Whitespace" — `docs/src/languages/markdown.md:6-77`.
- Open by default: issue #10966 "Configure markdown files to open in preview mode by default" is open with about 118 thumbs-up and 38 comments, labeled good first issue. After PR #63462 shipped, a user wrote (2026-09-18) "I'd like it to open in split preview so I can both edit the Markdown", and another (2026-09-21) said it is "implemented in 1.20.1, and it's great!" but that there is no visible way to switch modes — [issue #10966](https://github.com/zed-industries/zed/issues/10966); counts from [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+preview+in:title+state:open&sort=reactions-%2B1&order=desc&per_page=40).
- Follow-up to that feedback: PR #65081 added a "Show Source" entry to the preview tab menu (present locally as commit `80ee0a3563`, 2026-10-03) — [PR #63462 thread](https://github.com/zed-industries/zed/pull/63462).
- WYSIWYG: issue #21717 "WYSIWYG Markdown Preview" is open with about 91 thumbs-up, 22 rockets, 42 comments; no staff comment on plans, only label changes — [issue #21717](https://github.com/zed-industries/zed/issues/21717).
- Live preview in a single view: issue #15066 was closed 2025-06-04 with a staff comment from notpeter: "While I understand the desire for such a feature, this is not something that is on our nearterm roadmap." — [issue #15066](https://github.com/zed-industries/zed/issues/15066).
- Community PR #62593 "Add Obsidian-style live preview for markdown buffers" (new `markdown_live_preview` crate, in-place table editing, image resize) was closed without merging on 2026-08-14. Member dinocosta wrote they "couldn't find confirmation from a Zed staff member that we want to pursue this particular product direction" and that "we would need a relevant Zed owner to confirm the direction and help establish the intended scope" — [PR #62593](https://github.com/zed-industries/zed/pull/62593).
- A follow-up proposal for an opt-in extension API for live preview was posted 2026-11-16 and had no replies when read — [discussion #67063](https://github.com/zed-industries/zed/discussions/67063).
- Math: issue #40813 "Support LaTex rendering in markdown preview" is open with about 76 thumbs-up. Member MrSubidubi said (2025-10-22) "This will be quite an undertaking, because that would require support for something within GPUI." and (2026-01-19) that PR #44525 "is on it's way and might land at some point" with "no final decision made yet"; #44525 was later closed — [issue #40813](https://github.com/zed-industries/zed/issues/40813).
- Two math PRs are open: #54668 "Add LaTeX rendering to Markdown preview" (RaTeX library, KaTeX fonts, about 56 thumbs-up, last pushed 2026-11-17, no visible staff decision) and #57339 "Render LaTeX math in Markdown previews" (about 49 thumbs-up) — [PR #54668](https://github.com/zed-industries/zed/pull/54668); [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+preview+in:title+state:open&sort=reactions-%2B1&order=desc&per_page=40).
- Mermaid: delivered (see section 3). Remaining open Mermaid bugs include complex-diagram rendering (#51480, 10 thumbs-up), zoom shortcuts (#61710), horizontal scrolling (#62171), `%%{init}%%` directive (#61361) — [GitHub search API, mermaid](https://api.github.com/search/issues?q=repo:zed-industries/zed+mermaid+in:title&sort=reactions-%2B1&order=desc&per_page=12).
- HTML: open issues for `<details>`/`<summary>` (#52387), `<sub>`/`<sup>` (#22056); select-all and rich-text copy (#61966) — [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+preview+in:title+state:open&sort=reactions-%2B1&order=desc&per_page=40).
- Scroll: preview-scroll issues found are about scroll sensitivity (#66883, open), horizontal scroll (#56127, closed), opening the preview at the editor's scroll position (#61552, closed 2026-07-25), and scroll shortcuts (#21324, closed via PR #44585 on 2025-12-11). No open upstream issue or merged PR for wheel-scroll synchronization was found — [GitHub search API, scroll](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+preview+scroll+in:title&sort=updated&order=desc&per_page=15).
- The long-running tracking discussion (#23951, started by maintainer nathansobo, about 68 thumbs-up) lists LSP, highlighting, render/embed, HTML and misc items; it contains no staff statement on scroll sync, split view, WYSIWYG, math or tables. Scroll sync / split-view sync appear only as a commenter suggestion — [discussion #23951](https://github.com/zed-industries/zed/discussions/23951).
- Table-related: "Format Shorthand Markdown Tables" (#5382) and "Tables misaligned in preview" (#11024) are closed as completed; "Slow markdown preview rendering with many tables" (#58933) is open — [GitHub search API](https://api.github.com/search/issues?q=repo:zed-industries/zed+markdown+table+editing+OR+%22side+by+side%22+OR+%22split+preview%22+OR+%22toggle+preview%22+in:title,body+is:issue&sort=reactions-%2B1&order=desc&per_page=10).
- Local upstream history for 2026 (all in the fork already): shared-crate refactor #52008 (2026-03-26); links open in preview #61784 (2026-07-28); Mermaid zoom #62115 (2026-08-05); find in preview #62280 (2026-08-19); settings group + auto preview #63462 (2026-09-04); "Show Source" #65081 (2026-10-03); clone/split #65136 (2026-10-05); nav-back fix #65299 (2026-10-09) — local `git log -- crates/markdown_preview crates/mermaid_render`.

### Inferences
- Likely to arrive from upstream on its own (so the fork should wait or avoid diverging): incremental preview polish, Mermaid fixes, possibly math (two active PRs, but a year without a staff decision), more HTML tags.
- Unlikely to arrive from upstream soon (so the fork would need to build it): a three-mode switcher on the editor, split as the default layout, bidirectional scroll sync, WYSIWYG/live preview, table editing. Staff closed the one large live-preview PR on process and scope grounds and asked for a discussion and an owner first.
- The user demand that maps directly to the three-mode idea is visible in #10966 comments (split by default; a visible way to switch modes), but there is no dedicated high-vote upstream issue for "split view + mode toolbar" or for scroll sync.
- Upstream's direction for "preview only" is tab replacement plus a keyboard toggle and a tab context-menu item. A fork toolbar built on the same public functions stays compatible with that direction; a composite item would not.

### Gaps
- GitHub Discussions could not be searched by vote count (no unauthenticated API); only #23951 and #67063 were read. A scroll-sync or split-default discussion with meaningful votes may exist and was not found.
- One search result page was truncated (19 of roughly 40 open "markdown preview" titles read), so lower-voted open items are missing.
- A commenter's claim that LaTeX was called "out of scope and not on our roadmap" by staff could not be traced to a visible staff comment; treat it as unconfirmed — [PR #54668](https://github.com/zed-industries/zed/pull/54668).
- IntelliJ's own behavior (exact toolbar, scroll-sync options, per-file layout memory) was outside this assignment and is not sourced here.

## 8. Feasibility summary: gaps versus IntelliJ and the concrete seams

### Takeaway
The three modes, the switcher and the default-layout setting can be built almost entirely in a new crate using public functions that already exist, with two or three one-line registrations in upstream files and a few additive lines for the setting. Scroll sync is the only part that needs a real (though additive) change inside an upstream crate.

### Cited Findings
- Existing public entry points a new crate can call: `MarkdownPreviewView::open_preview_in_pane` — `crates/markdown_preview/src/markdown_preview_view.rs:243`; `open_preview_to_the_side_of_pane` — `:253`; `create_markdown_view` — `:346`; `is_markdown_file` — `:525`; `resolve_active_item_as_markdown_editor` — `:332`; `Workspace::adjacent_pane_of` — `crates/workspace/src/workspace.rs:6705`; `Workspace::pane_for` — `:6717`.
- Toolbar seam: one `toolbar.add_item(...)` line alongside the others — `crates/zed/src/zed.rs:1477-1529`. Init seam: one line next to `markdown_preview::init(cx)` — `crates/zed/src/main.rs:804`.
- Action seam: new actions can live in the fork crate (as `markdown_preview.rs:11-37` does) and be registered per workspace with `workspace.register_action` (as at `markdown_preview_view.rs:200-240`). Keybindings can be added under the existing contexts `Editor && extension == md` and `MarkdownPreview` — `assets/keymaps/default-macos.json:643-647`, `:1557-1567`.
- Setting seam: `crates/settings_content/src/settings_content.rs:1353-1389`, `assets/settings/default.json:116-135`, `crates/settings_ui/src/page_data.rs:11222-11243`.
- Scroll-sync seams: editor side `EditorEvent::ScrollPositionChanged` — `crates/editor/src/scroll.rs:410`; editor scroll setters `set_scroll_position` — `crates/editor/src/scroll.rs:660`; preview side `ScrollHandle::offset`/`set_offset`/`top_item`/`bounds_for_item` — `crates/gpui/src/elements/div.rs:4358-4494`; mapping functions that are currently private — `crates/markdown/src/markdown.rs:5084-5094`; existing source-index autoscroll entry point that is already public — `crates/markdown/src/markdown.rs:1029-1040`.
- The preview's `scroll_handle` and `markdown` fields are private — `crates/markdown_preview/src/markdown_preview_view.rs:76`, `:79`.

### Inferences
- Gap list versus an IntelliJ-style Markdown view, from the code read above:
  1. No single control showing/switching Editor / Split / Preview; the eye button only opens a preview and vanishes on the preview tab.
  2. Split is two independent tabs in two panes, not one tab; closing one leaves the other.
  3. No "split" default; only "preview" default via a boolean.
  4. No wheel/scrollbar scroll sync in either direction; cursor sync only, with reveal-style scrolling.
  5. No vertical/horizontal split orientation choice (always to the right).
  6. No per-file memory of the chosen layout.
  7. Renderer gaps: math, arbitrary HTML/CSS, definition lists; no table editing aids in the preview.
- Rough effort ordering (smallest upstream footprint first):
  1. Mode switcher toolbar + three actions, pane-based (new crate; about 2 upstream one-liners).
  2. `default_layout` setting + `ItemAdded` hook (new crate plus additive lines in 2–4 upstream files).
  3. Editor-to-preview scroll follow without pixel mapping: on `ScrollPositionChanged`, compute the buffer offset of the editor's top visible row and call the already-public `request_autoscroll_to_source_index`. This needs access to the preview's `markdown` entity (a small `pub` accessor on the upstream view) and yields "reveal" rather than "align" behavior because of the margin logic at `markdown.rs:2604-2617`.
  4. Precise two-way sync: additive public mapping API in `crates/markdown` plus an echo guard; this is what the superzent fork describes (recording per-visual-row layout positions and interpolating) — [superzent PR #71](https://github.com/currybab/superzent/pull/71).
  5. Composite single-tab item: largest, and the most exposed to upstream churn in `Item`, `Pane` and editor-detection call sites.
- A pane-based "Preview only" should reuse upstream's replace-the-tab semantics so that it stays consistent with `open_markdown_files_in_preview` and `CloseAndReturnToEditor`.

### Gaps
- No effort estimate in lines or days is given; nothing was prototyped.
- Whether closing a pane emptied by "Editor only" (removing the right-hand preview) collapses the split automatically was not checked in `Pane`/`Workspace` code.
- Whether two toolbars (editor pane and preview pane) showing the same switcher feels right is a product question that code reading cannot answer.
