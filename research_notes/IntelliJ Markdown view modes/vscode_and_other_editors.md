# Markdown viewing/editing/preview in VS Code and other non-JetBrains editors (baseline vs. IntelliJ's Editor / Editor + Preview / Preview)

Research date: 2026-11-20. Source-quality notes:
- VS Code facts come from the official docs page, official release notes, and the built-in extension's `package.json` on `main` (primary sources).
- Zed facts come from reading the Zed source in a local checkout of a Zed fork whose `crates/markdown` and `crates/markdown_preview` differ from `upstream/main` (ref dated 2026-11-19) by only 11 changed lines. Links point at the equivalent upstream files. Zed's public Markdown docs page says nothing about the preview.
- Typora, Sublime Text, markdown-preview.nvim and Nova facts mostly come from search-result summaries of the cited pages, not from full reads. Treat them as lower confidence.
- Anything I know only from background knowledge and did not verify this session is listed under Gaps, not under Cited Findings.

## 1. VS Code: how the Markdown preview is designed (in depth)

### Takeaway
VS Code's classic model is "source editor tab + a separate preview tab" (same group or beside), not a mode of one editor. The preview follows the active Markdown editor unless locked, with two-way scroll sync on by default. Since mid-2026 VS Code also ships an experimental in-place rich "Markdown Editor" and a "reopen as preview" custom editor, so a single `.md` tab can now be Text Editor, Markdown Preview, or Markdown Editor — a three-way choice that is close to IntelliJ's, except that "editor + preview" is still two tabs in two editor groups.

### Cited Findings

**Commands, keybindings, buttons**
- Toggle preview in the same group: Shift+Cmd+V (Ctrl+Shift+V). Side-by-side preview: Cmd+K V (Ctrl+K V). — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- The keybindings map to `markdown.togglePreview` (Shift+Ctrl/Cmd+V) and `markdown.showPreviewToSide` (Ctrl/Cmd+K V). Other commands: `markdown.showPreview`, `markdown.showLockedPreviewToSide`, `markdown.showSource`, `markdown.preview.refresh`, `markdown.preview.toggleLock`, `markdown.showPreviewSecuritySelector`, `markdown.reopenAsPreview`, `markdown.reopenAsSource`, `markdown.findAllFileReferences`. — [markdown-language-features package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- Editor-title button: `markdown.showPreviewToSide` is the primary button for Markdown files; its `alt` command (hold Alt) is `markdown.showPreview`, which opens in the current group. A second title button, `markdown.reopenAsPreview`, turns the current tab into a preview. When a preview is active, the title shows `markdown.showSource` / `markdown.reopenAsSource`. — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- A reviewer of VS Code 1.135 (Aug 27, 2026) describes the title-bar area as having "Open Preview to the Side", "Open as Preview", "Split Editor Right/Down", "More Actions", plus a dropdown just below with "Text Editor" (default), "Markdown Preview", "Markdown Editor", and a "Set default for .md" option. — [Thurrott](https://www.thurrott.com/dev/340776/visual-studio-code-now-includes-a-full-markdown-editor)

**Locked vs. following previews**
- "By default, previews update to the active Markdown file." A locked preview stays on one file and shows **[Preview]** in its title; **Markdown: Toggle Preview Locking** is available only when the preview tab is active. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)

**Scroll sync, selection marker, click-to-source (defaults from source)**
- `markdown.preview.scrollPreviewWithEditor`: default `true`. `markdown.preview.scrollEditorWithPreview`: default `true`. `markdown.preview.markEditorSelection`: default `false`. `markdown.preview.doubleClickToSwitchToEditor`: default `false`. — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- The docs describe the selected editor line appearing "as a light gray bar in the preview margin", and that double-clicking a preview element opens the editor and scrolls to the nearest line. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- Conflict to note: the docs describe the selection marker and double-click jump as standard behaviour, while `package.json` on `main` (as summarised by the fetch tool) reports both defaults as `false`. I did not resolve which is current; older VS Code versions had both `true`. — [docs](https://code.visualstudio.com/docs/languages/markdown); [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- `markdown.preview.openMarkdownLinks`: `inPreview` (default) or `inEditor`. `markdown.links.openLocation`: `currentGroup` (default) or `beside`. — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)

**Preview as a custom editor; opening `.md` directly in preview**
- Two custom editors are contributed: `vscode.markdown.preview.editor` ("Markdown Preview", priority `option`) and `vscode.markdown.editor` ("Markdown Editor", `option` for text editors, `explicit` for diff editors; also selectable for `*.txt`). — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- Default all `.md` files to preview with `"workbench.editorAssociations": {"*.md": "vscode.markdown.preview.editor"}`. `workbench.diffEditorAssociations` does the same for diffs only and takes precedence in diff views; rendered diffs support side-by-side and inline layouts. One-off: **View: Reopen Editor With...** then **Markdown Preview**. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)

**Experimental rich Markdown Editor (in-place editing)**
- Introduced in VS Code 1.131 (July 29, 2026) as "a new hybrid Markdown editor in the agents window", experimental, to view Markdown, edit it in place, and add comments for an agent to act on. Setting: `workbench.editor.markdownDefaultEditorInAgentsWindow`. Switch with "Reopen Editor With"; works in regular editor windows too. — [VS Code 1.131 release notes](https://code.visualstudio.com/updates/v1_131)
- Docs: "Markdown Editor (experimental)" edits rendered Markdown directly with a toggle between **Locked** and **Editing** modes; reopen via the breadcrumbs editor-type dropdown or **View: Reopen Editor With...**. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- Settings: `markdown.editor.initialMode` (`read` default, or `edit`; experimental), `markdown.editor.highlightActiveBlock` (default `true`), `markdown.editor.renderWhitespace` (default `false`). It has its own command set (cursor movement, line move/copy, `toggleLocked`, `insertTable`, table row/column insert/delete, column alignment), gated on `activeCustomEditorId == 'vscode.markdown.editor'`. — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- 1.142 (Oct 14, 2026): in editing mode "each source line appears on a separate visual line" (read-only mode still renders as before); paste an image directly; Rename Symbol (F2) for file links; Outline and breadcrumbs integration; copy button on code blocks. — [VS Code 1.142 release notes](https://code.visualstudio.com/updates/v1_142)
- 1.144 Insiders (dated Oct 20, 2026 per search summary): resize Mermaid diagrams, expand to editor width, or open full view in the rich Markdown editor. Insiders only; not confirmed in Stable. — [VS Code 1.144 Insiders notes](https://code.visualstudio.com/updates/v1_144)
- "Markdown code block editors are experimental and apply to the Markdown editor, not the standard Markdown preview"; they load only in a trusted workspace. — [Markdown extension guide](https://code.visualstudio.com/api/extension-guides/markdown-extension)

**Rendering engine, math, Mermaid, front matter**
- The preview is built on markdown-it and "supports the CommonMark specification"; extensions add syntax by contributing markdown-it plugins. — [Markdown extension guide](https://code.visualstudio.com/api/extension-guides/markdown-extension)
- Math: KaTeX, inline `$...$` and block `$$...$$`; disable with `"markdown.math.enabled": false`. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- Mermaid became built-in in VS Code 1.121 (May 20, 2026) via a bundled "Mermaid Markdown Features" extension that merges the former "Markdown Preview Mermaid Support" extension. It renders in the Markdown preview, notebook Markdown cells, and chat. Diagrams pan and zoom; right-click copies the source. — [VS Code 1.121 release notes](https://code.visualstudio.com/updates/v1_121)
- Mermaid interaction: Alt-drag pans, scroll zooms, click zooms in, Alt+Shift+click zooms out, pinch zooms; context menu **Copy Diagram Source**. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- `markdown.preview.frontMatter` (1.121): `table` (default, rendered at top of preview), `codeBlock`, `hide`. — [VS Code 1.121 release notes](https://code.visualstudio.com/updates/v1_121)
- Other render toggles: `markdown.preview.breaks` (false), `markdown.preview.linkify` (true), `markdown.preview.typographer` (false). — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)

**Styling**
- `markdown.styles`: array of `https` URLs or workspace-relative stylesheet paths, default `[]`. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- `markdown.preview.fontFamily` (system UI stack), `markdown.preview.fontSize` (14), `markdown.preview.lineHeight` (1.6). — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)

**Security levels**
- Strict (default): "Only loads trusted content and disables script execution", blocks `http` images. Allow insecure content: scripts still disabled, `http` content allowed. Disable: scripts and `http` allowed, not recommended. Changed via **Markdown: Change preview security settings**; applies to all workspace files. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)

**Extension API**
- Contribution points: `markdown.previewStyles` (loaded "after the built-in Markdown preview styles but before a user's `markdown.styles`"), `markdown.previewScripts` ("loaded asynchronously and reloaded on every content change"), `markdown.markdownItPlugins: true` plus an `extendMarkdownIt(md)` function returned from `activate`. Plugin extensions activate lazily when a preview is first shown. — [Markdown extension guide](https://code.visualstudio.com/api/extension-guides/markdown-extension)

**Images, drop and paste**
- Shift-drag a file into the editor to insert it; paste files, links or URLs and choose Markdown link vs. plain text. Commands **Markdown: Insert Image from Workspace** and **Insert Link to File in Workspace**. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- `markdown.editor.drop.enabled` and `markdown.editor.filePaste.enabled`: `smart` (default), `always`, `never`. `markdown.editor.drop.copyIntoWorkspace` and `markdown.editor.filePaste.copyIntoWorkspace`: `mediaFiles` (default) or `never`. `markdown.copyFiles.destination` maps globs to destinations with variables such as `${documentBaseName}`; `markdown.copyFiles.overwriteBehavior`: `nameIncrementally` (default) or `overwrite`. `markdown.editor.pasteUrlAsFormattedLink.enabled`: `smartWithSelection` default. Video and audio pastes insert `<video>` / `<audio>` HTML snippets. — [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)

**Language features in the source editor**
- Link validation is off by default (`markdown.validate.enabled: false`); sub-settings cover file links, fragment links, reference links, unused and duplicate link definitions, and `ignoredLinks`. External http(s) links are not checked. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown); [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)
- F2 on a header renames it and updates links; F2 on an internal link renames the file and updates links. Find All References on headers and links. `markdown.updateLinksOnFileMove.enabled`: `never` (default), `prompt`, `always`. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown)
- Outline of headers, go-to-header in file and workspace, smart selection, path and header completions (`##` for workspace headers), snippets; Copilot-backed alt-text generation code action. `[markdown]` defaults: `editor.wordWrap: on`, quick suggestions off. — [VS Code Markdown docs](https://code.visualstudio.com/docs/languages/markdown); [package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json)

### Inferences
- VS Code's "Editor + Preview" is a workbench layout (two tabs in two editor groups), not a property of the file's tab. Consequences: the pairing is not remembered per file, the split orientation is whatever the editor-group layout is, and closing the source leaves a following preview that retargets. IntelliJ's three-mode switch keeps source and preview inside one tab.
- The preview-follows-active-editor default plus "lock" is VS Code's answer to having one shared preview pane; IntelliJ needs no such concept because each tab owns its preview.
- By late 2026 VS Code effectively offers three per-file editor types (Text Editor / Markdown Preview / Markdown Editor) selectable from a dropdown with a per-extension default. This converges on IntelliJ's selector, with "in-place rich editing" taking the place of IntelliJ's side-by-side middle mode.
- The table commands and image paste in the Markdown Editor show that table editing was absent from the classic text-editor workflow and is being added only in the rich editor.

### Gaps
- I did not find documentation of built-in export (HTML/PDF) from the VS Code preview. I believe none exists and that export comes from extensions, but did not verify.
- I did not confirm from an official source that the classic preview is a webview. The `package.json` menu conditions refer to an active webview panel `markdown.preview`, which supports it. The extension guide does not state it.
- Popular extensions (Markdown All in One, Markdown Preview Enhanced, Mark Sharp, markdownlint) were not researched from primary sources this session. Only this was found: Markdown Preview Enhanced has an "Edit Markdown" context item with an in-preview editor option — [MPE repo](https://github.com/shd101wyy/vscode-markdown-preview-enhanced). Unverified background knowledge: Markdown All in One adds shortcuts, TOC generation, list continuation and a table formatter; MPE adds its own preview with PlantUML, code-chunk execution, presentation mode and PDF/HTML export. Mark Sharp was not found at all.
- Exact default values of `markEditorSelection` and `doubleClickToSwitchToEditor` are in conflict (see above).
- Current default of `workbench.editor.markdownDefaultEditorInAgentsWindow` is not stated in the release notes.

## 2. What VS Code (and Cursor) users complain about; what IntelliJ switchers miss

### Takeaway
Recorded complaints cluster around scroll-sync jumpiness, lost preview scroll position, and the tedium of a non-editable side-by-side preview. When Cursor replaced the dual-pane preview with a single combined editor, users pushed back, which shows that both the split and the in-place camps are real. I found no direct IntelliJ-to-VS Code migration testimony.

### Cited Findings
- microsoft/vscode issue #63690: Markdown preview does not preserve scroll position and jumps when focus returns; closed, but the reporter said it persisted. — [vscode#63690](https://www.github.com/microsoft/vscode/issues/63690)
- Foam issue #1329 (a VS Code extension, not core): editor and preview "both get confused", scroll positions fight, "sometimes, it is impossible to scroll down the preview". — [foam#1329](https://www.github.com/foambubble/foam/issues/1329)
- microsoft/vscode issue #50975: dragging a Markdown Preview tab to an editor edge to split did not work like normal tabs. — [vscode#50975](https://www.github.com/microsoft/vscode/issues/50975)
- microsoft/vscode issue #84520 tracked making the preview a custom editor so previews "work as existing VS Code users would expect". — [vscode#84520](https://www.github.com/microsoft/vscode/issues/84520)
- A long-form reviewer found VS Code's side-by-side style "tedious" for writing and lacking a true WYSIWYG mode; the new Markdown Editor was "a bit quirky", "oddly jittery", with the cursor "hopping back up into previously written text". — [Thurrott, Aug 27, 2026](https://www.thurrott.com/dev/340776/visual-studio-code-now-includes-a-full-markdown-editor)
- Cursor 2.4 replaced the VS Code dual-pane preview with a combined preview/source editor. A user (Jan 11, 2026): "Please make this an option because the UX is a total regression in editing and previewing markdown." Another (Feb 6, 2026): "i rely on yaml headers for indexing; this makes it gross to work with", and LaTeX rendering looked poor. Staff asked for use cases and passed feedback on; no revert setting was confirmed. Related threads report mode-switching problems, formatting loss and layout shifts. — [Cursor forum](https://forum.cursor.com/t/please-do-not-convert-to-your-new-markdown-editor-keep-the-open-in-preview/148571)
- Third parties ship editable previews for VS Code because users were "tired of a split preview pane they couldn't edit in" (search-summary paraphrase of a Product Hunt post). — [Product Hunt activity page](https://www.producthunt.com/@vadimmelnicuk/activity)

### Inferences
- Scroll sync between a line-based source view and a variable-height rendered view is the most fragile part of any split design; images, wrapped lines and collapsed HTML make the mapping non-linear.
- Replacing a split preview with in-place rendering by default is risky: users who edit raw Markdown (front matter, exact formatting) want the source untouched and a faithful read-only render beside it. Offer in-place rendering as an option, not as a replacement.

### Gaps
- No Reddit or Hacker News threads were retrieved; my searches returned docs and blogs only. I found no reliable source on what users who moved from IntelliJ to VS Code miss or prefer.
- No issue-count or vote data for VS Code Markdown complaints.
- Windsurf: nothing found on Markdown behaviour differing from VS Code.

## 3. Obsidian and Typora: in-place rendering and how the modes map to IntelliJ's three

### Takeaway
Obsidian has three presentations (Source mode, Live Preview, Reading view) and Typora has two (seamless live preview, Source Code Mode). Their middle ground is in-place rendering in a single pane, whereas IntelliJ's middle mode is a side-by-side split.

### Cited Findings
- Obsidian Reading view: "shows your note without Markdown syntax, offering a clean, readable format for focused review." Editing view has two editing modes. Live Preview: "shows formatted text inline while hiding most Markdown syntax"; underlying syntax appears when the cursor enters formatted content. Source mode: "displays all Markdown syntax exactly as written." — [Obsidian Help: Views and editing mode](https://obsidian.md/help/edit-and-read)
- Defaults: new tabs open in editing view; default editing mode is Live Preview. Ctrl/Cmd+E toggles Reading vs. Editing view. "Toggle Live Preview/Source mode" has no default hotkey. — [Obsidian Help](https://obsidian.md/help/edit-and-read)
- Side by side: Ctrl/Cmd-click the view switcher opens the same note in Editing and Reading view at once. — [Obsidian Help](https://obsidian.md/help/edit-and-read)
- Typora's positioning (marketing text via a mirror): it removes "the preview window, mode switcher, syntax symbols of markdown source code, and all other unnecessary distractions", replaced with "a real live preview feature". — [StackShare mirror of Typora text](https://stackshare.io/companies/typora/blog)
- Typora: View > Source Code Mode disables the live editor and reveals all hidden Markdown formatting. — [Markdown Guide: Typora](https://markdownguide.org/tools/typora/)
- Typora toggle shortcut reported as Ctrl+/ (Windows/Linux), Cmd+/ (macOS). Secondary source. — [Typora guide blog](https://laurence-wu.github.io/2025/03/24/typora-guide)
- User comparison: Typora's default view is "almost exactly like Obsidian's 'preview' mode, except that it's editable", and its source code mode is much like Obsidian's editing mode. — [Obsidian forum](https://forum.obsidian.md/t/a-wysiwym-typora-like-editing-mode/1953/37)

### Inferences
- Mapping to IntelliJ: IntelliJ "Editor" = Obsidian Source mode = Typora Source Code Mode = VS Code Text Editor. IntelliJ "Preview" = Obsidian Reading view = VS Code Markdown Preview (Typora has no read-only equivalent). IntelliJ "Editor + Preview" has no single-pane equivalent in Obsidian or Typora; the nearest are Obsidian's Ctrl/Cmd-click side-by-side and VS Code's Open Preview to the Side. Live Preview is a fourth category IntelliJ's three-mode switch does not have.
- Why in-place rendering appeals: one pane instead of two (half the screen is not duplicated), no scroll sync to break, and no mode switch to check output. The cursor-reveals-syntax rule keeps the file plain Markdown and editable as text.
- Cost of in-place rendering: layout shifts as syntax appears and disappears under the cursor (the same jitter reported for VS Code's new editor and Cursor's editor), and less control over exact source formatting.

### Gaps
- Typora's official docs were not read in full; Typora claims rest on secondary sources. Typora's table editing, export formats, image handling, and diagram/math support were not verified. (Unverified background knowledge: Typora has GUI table editing, export to PDF/HTML/Word via Pandoc, image copy/upload on paste, and Mermaid and MathJax rendering.)
- Obsidian specifics for table editing, Mermaid, MathJax, CSS snippets, PDF export were not verified this session. (Unverified background knowledge: all are supported; Live Preview has a table editor.)
- No sourced user-survey data on why users prefer in-place rendering; the reasons above are inference.

## 4. JetBrains Fleet: Markdown preview and product status

### Takeaway
Fleet offered a preview toggle with three layouts (right, below, full) and a default-mode setting, close to IntelliJ's three-mode design. Fleet was discontinued in December 2025 and its help pages now redirect to the shutdown announcement.

### Cited Findings
- Fleet rendered HTML in "a live preview pane"; support "is based on the CommonMark specification with an additional support for the most part of the GitHub Flavored Markdown dialect." — [Fleet 1.38 Help: Markdown](https://jetbrains.com/help/fleet/1.38/markdown.html)
- Open: "Click the preview icon located in the upper-right corner of the editor. Alternatively, press ⌥ ⌘ P." Layouts in 1.38 docs: split right, full screen, split down; newer docs name "Show Preview on the Right" and "Show Preview Below"; clicking the selected mode again turns the preview off. — [Fleet 1.38 Help](https://jetbrains.com/help/fleet/1.38/markdown.html); [Fleet 1.48 Help](https://www.jetbrains.com.cn/en-us/help/fleet/markdown.html)
- A default mode could be set in Settings via a "Markdown preview" drop-down; Markdown files then opened in that mode. — [Fleet 1.38 Help](https://jetbrains.com/help/fleet/1.38/markdown.html)
- Markdown preview, highlighting and completion arrived in Fleet 1.31. — [Fleet 1.31 blog post](https://blog.jetbrains.com/?p=449793/)
- Status: JetBrains announced on Dec 8, 2025 that Fleet would be discontinued; downloads ended Dec 22, 2025. Installed copies keep working without updates; server-backed features may stop. The platform is reused for JetBrains Air, an agentic development tool. — [devclass](https://www.devclass.com/ai-ml/2025/12/09/jetbrains-abandons-fleet-ide-pins-hopes-on-forthcoming-air-agentic-development-tool/1728894); [AlternativeTo](https://alternativeto.net/news/2025/12/jetbrains-will-discontinue-its-fleet-ide-on-december-22-2025-due-to-overlapping-purposes)
- The current Fleet Markdown help URL 301-redirects to the announcement post "The Future of Fleet". — [JetBrains blog](https://blog.jetbrains.com/fleet/2025/12/the-future-of-fleet/) (redirect observed; post body not read)

### Inferences
- Fleet's design put orientation in the mode choice itself (right / below / full) with a persisted default, which is a compact way to expose both split direction and preview-only in one control.

### Gaps
- Fleet's rendering internals (native Skia-drawn vs. embedded browser) were not found in any source. Fleet's UI was not Chromium-based, but I found no statement on how the Markdown preview was drawn.
- Fleet scroll sync, Mermaid, math, and custom CSS support: not found.
- The JetBrains announcement was not read directly; status rests on secondary coverage.

## 5. Native (non-webview) and terminal Markdown rendering: Zed, Neovim, Sublime Text, Nova, others

### Takeaway
Zed proves a GPU-native preview can cover CommonMark/GFM, a whitelisted HTML subset, images, native Mermaid, click-to-source and checkbox toggling without a web view, but math is parsed and dropped and there is no custom CSS. Editors without a rendering surface (Neovim, Sublime Text) either open a browser or decorate the buffer in place.

### Cited Findings

**Zed (native GPUI preview)**
- Actions: `markdown::OpenPreview` (cmd-shift-v) and `markdown::OpenPreviewToTheSide` (cmd-k v), the same chords as VS Code; `markdown::OpenFollowingPreview` ("Opens a following markdown preview that syncs with the editor"); `markdown::CloseAndReturnToEditor` (cmd-shift-v inside the preview); scroll actions by line, page, and by Markdown element (alt-up / alt-down). — [zed markdown_preview.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown_preview/src/markdown_preview.rs); [default-macos.json](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json)
- Two preview modes: `Default` ("will always show the contents of the provided editor") and `Follow` ("will 'follow' the currently active editor"). This is the inverse default of VS Code, where following is default and locking is opt-in. — [markdown_preview_view.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown_preview/src/markdown_preview_view.rs)
- Editor to preview: the preview tracks the editor's selection (`selected_source_index`), highlights the active root block and autoscrolls to it. Preview to editor: a single click on rendered content moves the editor selection to the matching source offset with centred autoscroll. Checkboxes in the preview write the toggle back to the buffer. — [markdown_preview_view.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown_preview/src/markdown_preview_view.rs)
- The preview is created with `parse_html: true` and `render_mermaid_diagrams: true`. Mermaid is drawn natively through an in-tree `mermaid_render` crate as SVG images with zoom (10 percentage points per scroll tick). — [markdown_preview_view.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown_preview/src/markdown_preview_view.rs); [mermaid.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown/src/mermaid.rs)
- HTML is a whitelist handled by a custom parser. Elements matched in the parser: `a`, `b`, `strong`, `i`, `em`, `del`, `ins`, `p`, `h1`–`h6`, `blockquote`, `ul`, `ol`, `li`, `img`, `table`/`thead`/`tbody`/`tr`/`th`/`td`/`caption`; attributes `href`, `src`, `alt`, `width`, `height`, `align`, `colspan`, `rowspan`, `style` (the style values matched include bold, lighter, italic, oblique, underline, left, center, right). — [html_parser.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown/src/html/html_parser.rs)
- Math: the parser matches `InlineMath` and `DisplayMath` events and does nothing with them. — [parser.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown/src/parser.rs)
- A Dec 24, 2025 feature request for LaTeX rendering (33 votes, closed, no maintainer reply) states "Current Zed does not support math rendering across the app" and concedes "implementing such things in GPUI from scratch is not easy"; it suggests the ReX library. — [zed discussion #45616](https://github.com/zed-industries/zed/discussions/45616)
- Settings under `markdown_preview`: `font_size`, `font_family`, `code_font_family`, `heading_font_weight` (600), `open_markdown_files_in_preview` (false), `limit_content_width` (true) with `max_width`. No stylesheet setting exists. — [default.json](https://github.com/zed-industries/zed/blob/main/assets/settings/default.json)
- Zed's public Markdown docs cover tree-sitter highlighting, Prettier formatting, list continuation and trailing whitespace, and do not document the preview. There is no Markdown language server. — [Zed docs: Markdown](https://zed.dev/docs/languages/markdown)
- Zed cannot preview HTML files; a discussion asks for a general HTML preview extension API for LaTeX, Typst, AsciiDoc. — [zed discussion #27163](https://github.com/zed-industries/zed/discussions/27163)

**Neovim**
- render-markdown.nvim runs "entirely inside Neovim with no external windows", using treesitter (`markdown`, `markdown_inline`), virtual text, extmarks and `conceallevel = 3`. Default `render_modes = { 'n', 'c', 't' }`, so the buffer shows raw source in insert mode. Anti-conceal is on by default: "This enables hiding added text on the line the cursor is on." Updates are debounced 100 ms and limited to the visible window; files over 10 MB are skipped. — [render-markdown.nvim README](https://github.com/MeanderingProgrammer/render-markdown.nvim)
- It renders headings, code blocks, tables (borders, auto-aligned cells), checkboxes with custom states, callouts (GitHub and Obsidian), block quotes, links with icons, and LaTeX converted to Unicode text through `utftex` or `latex2text`. Images get an icon only. — [render-markdown.nvim README](https://github.com/MeanderingProgrammer/render-markdown.nvim)
- markdown-preview.nvim previews in an external browser with synchronised scrolling and supports KaTeX, PlantUML, Mermaid and Chart.js. — [markdown-preview.nvim](https://www.github.com/iamcco/markdown-preview.nvim)

**Sublime Text**
- The MarkdownPreview package (about 960K installs) builds HTML and offers "Preview in Browser"; live updates need the LiveReload package. A newer MarkdownPreviewEnhanced package (ST4 build 4107+) adds SSE updates, KaTeX, Mermaid, ECharts and presentation mode. Both are browser-based. — [MarkdownPreview](https://github.com/facelessuser/MarkdownPreview/); [MarkdownPreviewEnhanced](https://packagecontrol.io/packages/MarkdownPreviewEnhanced)

**Nova (Panic)**
- Nova opens a Markdown document in a Preview tab from the Preview icon in the editor path bar; "Preview tabs can refresh automatically as you save, or live as you type"; the preview can sit in a Split View with the editor. — [Nova help: Preview](https://help.panic.com/nova/preview)

### Inferences
- Limits that native rendering imposes, as seen in Zed: (1) raw HTML works only for a hand-maintained whitelist, so `<details>`, `<kbd>`, `<sub>`/`<sup>`, `<video>`, `<iframe>`, inline SVG and scripts need individual work or stay unsupported; (2) no user CSS, only font and width settings, so "match GitHub's look" must be designed in; (3) math needs a TeX layout engine, which is the largest missing piece; (4) Mermaid needs a native renderer (Zed has one), and every other diagram language (PlantUML and so on) needs its own; (5) export to HTML/PDF does not come for free.
- What native rendering gains: every rendered block maps to a source range, so click-to-source, block-level active highlight, element-wise scrolling and write-back (checkbox toggles) are simple and exact. There is no preview security model to design because there are no scripts.
- render-markdown.nvim's "render in normal mode, raw in insert mode, reveal the cursor line" is the cheapest form of Obsidian-style Live Preview and needs no layout engine beyond the editor's own inlays and conceals. A GPU editor with inlays, block decorations and folds could do the same in its main editor.
- For a native editor with no web view, the practical order is: a native split preview with exact source mapping first (IntelliJ-style three-mode switch per tab), then optional in-buffer decoration. Full WYSIWYG is the most expensive and the most complained-about.

### Gaps
- Lapce and Helix: not researched this session. (Unverified background knowledge: Helix has no Markdown preview; Lapce has no mature built-in one.)
- Whether Zed's preview-to-editor sync includes scroll following (not just click) was not established from the code read; I saw editor-selection-driven sync and click-driven jump only.
- Zed footnotes, task lists, syntax-highlighted code blocks, and image formats supported were not individually verified.
- Nova's rendering is presumably WebKit-based since its Preview feature also renders web projects, but I found no statement. Nova scroll sync, math and Mermaid: not found.
- Sublime Text has no built-in preview as far as I know, but no source stating so was retrieved.

## 6. Comparison matrix

### Takeaway
Three interaction models exist: separate preview tab or split (VS Code classic, Zed, Nova, Fleet), in-place rendering (Obsidian Live Preview, Typora, VS Code's experimental Markdown Editor, Cursor's editor, render-markdown.nvim), and external browser (Neovim and Sublime plugins). Only JetBrains-style products (IntelliJ, Fleet) make split orientation and preview-only a per-tab mode.

### Cited Findings
Cells are filled only where a source in sections 1–5 supports them; "?" means not verified this session.

| Capability | VS Code (built-in, late 2026) | Zed (native) | Obsidian | Typora | Fleet (discontinued) | Neovim render-markdown.nvim | Neovim markdown-preview.nvim | Sublime MarkdownPreview | Nova |
|---|---|---|---|---|---|---|---|---|---|
| View modes | Text Editor; Preview (tab or custom editor); experimental rich Markdown Editor (Locked/Editing) | Editor; Preview tab (pinned or following) | Source; Live Preview; Reading | Live preview; Source Code Mode | Editor; preview right / below / full | Raw in insert mode, rendered in normal mode | Editor + browser | Editor + browser | Editor; Preview tab |
| Editor + preview together | Second tab in another editor group ("to the Side") | Second tab in another pane | Ctrl/Cmd-click view switcher | No (single pane) | Yes, in-tab mode | n/a (in-buffer) | Separate browser window | Separate browser window | Split View |
| Split orientation control | Editor-group layout, not Markdown-specific | Pane layout | Pane layout | n/a | Right or below, in the mode menu | n/a | n/a | n/a | ? |
| Open `.md` straight into preview | `workbench.editorAssociations` | `open_markdown_files_in_preview` | Default view for new tabs | n/a | Default "Markdown preview" mode setting | n/a | ? | No | ? |
| Scroll sync | Both directions, each with a setting, default on | Editor selection to preview; preview click to editor | n/a in single pane | n/a | ? | n/a | Editor to browser | Not built in | ? |
| Click-to-source | Double-click (setting) | Single click | Edit in place | Edit in place | ? | n/a | ? | No | ? |
| Edit in rendered view | Experimental Markdown Editor | Checkbox toggle only | Live Preview | Yes | ? | Cursor-line reveal | No | No | ? |
| Mermaid | Built in since 1.121 | Built in, native renderer | ? | ? | ? | No | Yes | Via Enhanced package | ? |
| Math | KaTeX built in | No (parsed, dropped) | ? | ? | ? | Unicode approximation | KaTeX | Via Enhanced package | ? |
| Raw HTML | Yes (browser engine; scripts gated by security level) | Whitelisted subset | ? | ? | ? | Comments concealed only | Yes (browser) | Yes (browser) | ? |
| Custom CSS | `markdown.styles` + extension styles | No (fonts, width only) | ? | ? | ? | Highlight groups / config | ? | ? | ? |
| Image paste/drop | Yes, copy into workspace with destination rules | ? | ? | ? | ? | No | No | No | ? |
| Table editing | Commands in experimental Markdown Editor only | ? | ? | ? | ? | Display alignment only | No | No | ? |
| Front matter | `table` / `codeBlock` / `hide` | ? | ? | ? | ? | Via yaml parser | ? | ? | ? |
| Export | None found built in | None found | ? | ? | ? | No | No | HTML output | ? |
| Extensibility of renderer | markdown-it plugins, preview scripts and styles | None found | ? | ? | ? | Lua config | ? | ? | ? |

Sources for the matrix: [VS Code docs](https://code.visualstudio.com/docs/languages/markdown); [VS Code package.json](https://raw.githubusercontent.com/microsoft/vscode/main/extensions/markdown-language-features/package.json); [VS Code 1.121](https://code.visualstudio.com/updates/v1_121); [Zed markdown_preview_view.rs](https://github.com/zed-industries/zed/blob/main/crates/markdown_preview/src/markdown_preview_view.rs); [Zed default.json](https://github.com/zed-industries/zed/blob/main/assets/settings/default.json); [Obsidian Help](https://obsidian.md/help/edit-and-read); [Markdown Guide: Typora](https://markdownguide.org/tools/typora/); [Fleet 1.38 Help](https://jetbrains.com/help/fleet/1.38/markdown.html); [render-markdown.nvim](https://github.com/MeanderingProgrammer/render-markdown.nvim); [markdown-preview.nvim](https://www.github.com/iamcco/markdown-preview.nvim); [MarkdownPreview](https://github.com/facelessuser/MarkdownPreview/); [Nova help](https://help.panic.com/nova/preview)

### Inferences
- Design choices that look best for a native GPU editor with no web view, judged against the evidence above:
  1. Make the view mode a property of the Markdown tab (Editor / Split / Preview) with a remembered default and a choice of split direction, as IntelliJ and Fleet do. VS Code's two-tab approach is the source of its lock/follow complexity and of the "tedious" complaint.
  2. Keep VS Code's chords (cmd-shift-v, cmd-k v) since Zed already uses them and users arrive with that muscle memory.
  3. Use exact source-range mapping for sync instead of proportional scroll sync: cursor-driven highlight and reveal in the preview, click (or double-click) in the preview to move the cursor. This avoids the scroll-fighting bugs reported for web-view previews.
  4. Treat math as the main gap to close natively; Mermaid is already solved in Zed.
  5. Keep raw HTML as a documented whitelist and extend it by demand (`details`/`summary`, `kbd`, `sub`/`sup`, `br`, `hr` are the usual README needs).
  6. Offer in-buffer decoration (render-markdown.nvim style) as an optional later step; do not replace the split preview with it, given the Cursor backlash.
  7. Table commands and image paste with copy-into-workspace rules are source-editor features and do not depend on any web view.

### Gaps
- Many matrix cells for Obsidian, Typora, Fleet and Nova are "?" because I did not verify them from sources this session.
- IntelliJ's own column is deliberately omitted (covered by other researchers).
