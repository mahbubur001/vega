# JetBrains IDE Markdown editing features (source-editor side, bundled Markdown plugin)

Source key used below (all fetched 2026-11-20):

- **[DOC]** = https://www.jetbrains.com/help/idea/markdown.html (current help, labelled 2026.2/2026.3; raw text at `markdown.html.md`)
- **[DOC-SET]** = https://www.jetbrains.com/help/idea/markdown-reference.html
- **[DOC-KEYS]** = https://www.jetbrains.com/help/idea/settings-smart-keys-markdown.html
- **[DOC-STYLE]** = https://www.jetbrains.com/help/idea/code-style-markdown.html
- **[CORE]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/resources/intellij.markdown.xml (main descriptor: actions, shortcuts, handlers, registry keys)
- **[BUNDLE]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/resources/messages/MarkdownBundle.properties (every user-visible string)
- **[BACKEND]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/backend/resources/intellij.markdown.backend.xml (inspections, intentions, run markers)
- **[FRONTEND]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/frontend/resources/intellij.markdown.frontend.xml
- **[IMAGES]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/resources/intellij.markdown.images.xml
- **[MODEL]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/model/src/main/resources/intellij.markdown.model.xml
- **[PLUGIN]** = https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/plugin/resources/META-INF/plugin.xml

Caveat that applies everywhere: the GitHub sources are the `master` branch as of Nov 2026, which is ahead of the released 2026.2 docs. Where master and the docs disagree it is called out. macOS shortcuts come from [DOC]; the descriptor only declares `control X` on the `$default` keymap, which the macOS keymap shows as Cmd.

## Formatting: floating toolbar, shortcuts, Insert popup, header level

### Takeaway
Selecting text shows a floating toolbar with: header-style dropdown, Bold, Italic, Strikethrough, Code, Create Link, and a list-type dropdown; each inline style is a toggle with a default shortcut. `Cmd+N` / `Alt+Insert` opens an "Insert…" popup with exactly four items: Link, Table, Image, Table Of Contents. There are no "header level up/down" actions in the current plugin; header level is set through the dropdown only.

### Cited Findings
- Toolbar group id `Markdown.Toolbar.Floating` ("Markdown Editor Floating Toolbar"), order: `Floating.CodeToolbar.Extensions` (comment in source: "enables AI actions"), `Markdown.Styling.SetHeaderLevel`, separator, Bold, Italic, Strikethrough, Code span, `Markdown.Styling.CreateLink`, separator, `Markdown.Styling.CreateOrChangeList`. — [CORE]
- Shortcuts declared in the descriptor (default keymap): Bold `control B`; Italic `control I`; Strikethrough `control shift S`; Code span `control shift C`; Create Link `control shift U`; Insert Image `control U`. — [CORE], [IMAGES]
- Same shortcuts as documented per OS: Bold `Ctrl+B` / `⌘B`; Italic `Ctrl+I` / `⌘I`; Strikethrough `Ctrl+Shift+S` / `⌘⇧S`; Code `Ctrl+Shift+C` / `⌘⇧C`; Create Link `Ctrl+Shift+U` / `⌘⇧U`; Insert Image `Ctrl+U` / `⌘U`; Insert… popup `Alt+Insert` / `⌘N`. — [DOC]
- Action display names and descriptions: "Bold – Format text as bold (strong emphasis)", "Italic – Format text as italic (emphasis)", "Strikethrough – Format text with a strikethrough", "Code – Format text as an inline code span", "Create Link – Wrap the selected text as a link" which becomes "Remove Link – Unwrap the link to regular text" when the caret is in a link. — [BUNDLE]
- Action ids for keymap parity: `org.intellij.plugins.markdown.ui.actions.styling.ToggleBoldAction`, `…ToggleItalicAction`, `…ToggleStrikethroughAction`, `…ToggleCodeSpanAction`, `Markdown.Styling.CreateLink`, `org.intellij.plugins.markdown.ui.actions.styling.InsertImageAction`, `Markdown.Styling.SetHeaderLevel` ("Set Header Style"), `Markdown.Styling.CreateOrChangeList` ("Create Or Change List"). — [CORE], [BUNDLE]
- Header-style dropdown entries: "Normal" (level 0, strips the header), "Title (H1)", "Subtitle (H2)", then "Heading 1 (H3)" … "Heading 4 (H6)" (label is `Heading {level-2}` with secondary text `(H{level})`). On a plain line it "simply adds `#` at the line start" and adds blank lines around the new header if missing; on an existing header it rewrites the level. — [SetHeaderLevelImpl.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/ui/actions/styling/SetHeaderLevelImpl.kt), [BUNDLE]
- List dropdown entries: "Unordered List", "Ordered List", "Checkmark List". — [BUNDLE], [DOC]
- Inline toggles are stateful (pressed when the selection is already styled), work with multiple carets, and handle a selection spanning table cells by applying the style per cell; they are disabled when carets disagree (some styled, some not). — [BaseToggleStateAction.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/ui/actions/styling/BaseToggleStateAction.kt)
- Emphasis marker is configurable: Settings | Editor | General | Smart Keys | Markdown | "Emphasis style" — "Asterisks: *italic*, **bold**" (default) or "Underscores: _italic_, __bold__". — [BUNDLE], [DOC-KEYS], [MarkdownCodeInsightSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/settings/MarkdownCodeInsightSettings.kt)
- Create Link behaviour: selected text becomes the link text in `[ ]`; a URL on the clipboard is placed in `( )` as the destination. — [DOC]
- Pasting a URL over a non-blank selection produces `[selected text](pasted-url)` automatically; skipped inside code fences/blocks/spans, HTML blocks, existing links, images, autolinks, link definitions and front matter. — [MarkdownLinkPastePreProcessor.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/MarkdownLinkPastePreProcessor.kt)
- Insert popup: group `Markdown.InsertGroup` is appended to the standard `GenerateGroup`; action `Markdown.Insert` ("Insert…") uses the shortcut of `Generate` and is also in the editor context menu. Items: `Markdown.Styling.CreateLink` ("Link"), `Markdown.InsertEmptyTable` ("Table"), InsertImageAction ("Image"), `Markdown.GenerateTableOfContents` ("Table Of Contents", renamed "Update Table Of Contents" when a TOC exists). — [CORE], [FRONTEND], [IMAGES], [BUNDLE]
- Toolbar is customisable under Settings | Appearance & Behavior | Menus and Toolbars | "Markdown Editor Floating Toolbar", and can be turned off with Advanced Settings | Markdown | "Hide floating toolbar" (`markdown.hide.floating.toolbar`, default false). It is attached to Markdown editors and Markdown scratch files. — [DOC], [CORE], [AddFloatingToolbarTextEditorCustomizer.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/ui/floating/AddFloatingToolbarTextEditorCustomizer.kt)
- The floating toolbar was already documented in the 2022.1 help. — [2022.1 help](https://jetbrains.com/help/idea/2022.1/markdown.html)
- Comment toggle `Ctrl+/` / `⌘/`: docs say it writes `[//]: # (text)` with a blank line before it. — [DOC]; contradicted by master, where the line comment writes `<!-- text -->` by default and uses `[//]: # (text)` only when registry key `markdown.line.comment.link.definition.syntax.enabled` is true or the line contains `-->`; block comment wraps the selection in an HTML comment. — [MarkdownCommenter.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/lang/MarkdownCommenter.kt), [CORE]

### Inferences
- Minimum parity set for another editor: five toggles + link + header dropdown + list dropdown, a selection-triggered toolbar, URL-paste-over-selection, and one "Insert…" entry point bound to the editor's generic "generate/new" key.
- The header dropdown uses word-processor wording (Normal/Title/Subtitle/Heading n) rather than H1–H6, which suggests it targets non-developer writers.
- The comment-syntax change (HTML comment by default) is newer than the published docs, so it is probably a 2026.3 change.

### Gaps
- No "Header level up / Header level down" actions exist in the master descriptor. I could not confirm from a primary source whether older releases shipped them and when they were removed.
- No highlight (`==text==`), underline, or blockquote toggle action is registered; a "Highlight" colour key exists in [BUNDLE] but I found no toggle for it.
- Exact pixel/timing behaviour of the toolbar (delay, placement above selection) was not read from `MarkdownFloatingToolbar` source.

## Tables

### Takeaway
Tables get the richest tooling: a grid-picker insert, Tab/Shift+Tab cell navigation, Shift+Enter new row, Enter inserting `<br/>`, row/column bars (inlay hints) with move/insert/select/align/remove actions, a "Table" context submenu, intentions, three inspections, and a formatter with Aligned/Compact/Tight styles. Since 2026.x the default is *visual* alignment of pipes without touching the source; rewriting the table as you type is now off by default.

### Cited Findings
- Insert: `Markdown.InsertEmptyTable` ("Insert Table") opens a popup grid (`TableGridComponent`) starting at 4×4 that expands by 2 rows/columns; arrow keys move the selection and Enter confirms; rows are counted including the header; cells are created 3 characters wide; a newline is inserted first when the previous line is not empty; block-quote prefixes are preserved; the caret lands in the first cell. — [InsertEmptyTableAction.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/frontend/src/com/intellij/markdown/frontend/actions/InsertEmptyTableAction.kt)
- Tab / Shift+Tab: moves to the first non-whitespace offset of the next cell (or just after the last non-whitespace of the previous cell), crossing rows; Tab in the last cell moves the caret to the end of the row; ignored with multiple carets and for tables inside list items (falls through to list indent). Setting "Use Tab/Shift+Tab to navigate table cells", default on. — [MarkdownTableTabHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/tables/handlers/MarkdownTableTabHandler.kt), [DOC-KEYS]
- Shift+Enter (`⇧⏎`): inserts an empty row under the current line (same indentation) and puts the caret in its first cell; works inside a cell or at the end of a row. Setting "Use Shift+Enter to insert new table row", default on. — [MarkdownTableShiftEnterHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/tables/handlers/MarkdownTableShiftEnterHandler.kt), [DOC]
- Enter inside a cell inserts the literal `<br/>`; pressing Enter again at the same spot removes that `<br/>` and performs a normal line break. Setting "Insert HTML line break ('<br/>') instead of new line inside table cells", default on. — [MarkdownTableEnterHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/tables/handlers/MarkdownTableEnterHandler.kt)
- "Reformat table when typing" (rewrite source so columns fit content) defaults to **false** in master; typed/backspace/delete handlers plus hooks after Paste, Cut, Duplicate and Delete-to-word-start trigger the reformat when enabled. — [MarkdownCodeInsightSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/settings/MarkdownCodeInsightSettings.kt), [CORE], [FRONTEND]
- "Align table cells visually" (Settings | Languages & Frameworks | Markdown): "Visually align table cells and column separators in the editor without modifying the table's source text"; tooltip "Aligns column separators regardless of character width". — [DOC-SET], [BUNDLE]
- Row/column markers are inlay hints ("Table inlays": "Horizontal and vertical bars around the table that provide column and row actions… Click on the corresponding bar"), toggled at Settings | Editor | Inlay Hints | Other | Markdown | Table inlays. — [BUNDLE], [DOC]
- Column bar actions (ids): Move Column Left/Right (`Markdown.Table.SwapColumns.SwapWithLeftColumn` / `…RightColumn`), Insert Column Left/Right (`Markdown.Table.InsertTableColumn.InsertBefore` / `InsertAfter`), Select Column Cells (`Markdown.Table.SelectCurrentColumn.SelectContentCells`), Align Left/Center/Right (`Markdown.Table.SetColumnAlignment.Left|Center|Right`), Remove Column (`Markdown.Table.RemoveCurrentColumn`). Row bar: Move Row Up/Down (`Markdown.Table.SwapRows.SwapWithAbove|Below`), Insert Row Above/Below (`Markdown.Table.InsertRow.InsertAbove|Below`), Select Row (`Markdown.Table.SelectRow`), Remove Row (`Markdown.Table.RemoveCurrentRow`). None has a default shortcut. — [CORE], [BUNDLE], [DOC]
- The same actions appear in the editor right-click menu under a "Table" submenu (insert column/row, "Column Alignment" submenu, move column, remove column/row). "Align None" and "Select Whole Column" exist in source but are commented out. — [CORE]
- Intentions (Alt+Enter / `⌥⏎`): "Insert column to the left", "Insert column to the right", "Set column alignment", "Remove column", "Remove row", "Reformat table", "Fix cell alignment", "Fix table borders". — [CORE], [BACKEND], [BUNDLE]
- Inspections: "Incorrect table formatting" (weak warning, **disabled by default**), "Table doesn't have side borders" (warning, on: "all table rows should have borders (pipe symbols) at the start and at the end"), "Unescaped pipe in code span" (warning, on; fix "Escape '|'"). — [BACKEND], [BUNDLE]
- Formatter: Code Style | Markdown | When reformatting | "Format tables" (on) with "Table style": Aligned (default), Compact, Tight. — [BUNDLE], [MarkdownCustomCodeStyleSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/lang/formatter/settings/MarkdownCustomCodeStyleSettings.kt)
- Tables fold ("table" region); "Collapse tables" option under code folding settings. — [BUNDLE]
- History: 2022.1 help already describes column/row markers; the 2023.1 EAP added "a new intention action that allows you to correct the formatting of tables" and the dedicated Smart Keys | Markdown page. — [2022.1 help](https://jetbrains.com/help/idea/2022.1/markdown.html), [2023.1 EAP blog](https://blog.jetbrains.com/idea/2023/01/intellij-idea-2023-1-eap/)

### Inferences
- JetBrains moved from "rewrite the table on every keystroke" to "render pipes aligned, leave the text alone". That avoids noisy diffs and CJK/wide-character misalignment, and is the cheaper design to copy if the target editor supports per-character visual offsets or inlays.
- Priority order implied by defaults: Tab navigation, Shift+Enter row, `<br/>` on Enter, and visual alignment are on; source-rewriting and the strict formatting inspection are opt-in.

### Gaps
- Exact rule for when the grid picker grows (hover on last row/column vs. keyboard past the edge) was not read in full.
- The release that introduced "Align table cells visually" and flipped "Reformat table when typing" to off was not found in a What's New page.
- No CSV-to-table or sort-by-column action exists in the descriptors.

## Lists

### Takeaway
Enter continues the list, Enter on an empty item outdents or ends the list, Backspace removes the marker, Tab/Shift+Tab change nesting with children, and ordered lists can be renumbered automatically (off by default) in three numbering styles. Task lists are created from the toolbar and continued with an unchecked box.

### Cited Findings
- Enter handler does three things: "if it has no contents and is in a top-level list, it is removed; if it has no contents and is in a nested list, its nesting level is decreased; otherwise, a new list item is created next to the current one"; it also works inside block quotes and renumbers per the setting. A constant `CHECK_BOX_MARKER = "[ ] "` is used for new task items. — [MarkdownListEnterHandlerDelegate.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/lists/MarkdownListEnterHandlerDelegate.kt)
- "Use smart Enter and Backspace" (default on): "Press Enter to add new list items. Press Backspace to remove the item bullets." Separate backspace handlers exist for the marker and for the indent. — [DOC-KEYS], [CORE]
- Tab on a list item "increases nesting of the current/selected list item(s) by a single level"; with the caret in the indent before the marker only that line moves, otherwise children (paragraphs, sublists) move too; indent width follows the fixed-sublist-indent setting but is never smaller than the marker width; numbering of the affected lists is recomputed. Shift+Tab is the mirror handler. — [MarkdownListItemTabHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/lists/MarkdownListItemTabHandler.kt), [CORE]
- "Adjust indentation on type" (default on): "The IDE automatically adjusts indentation in lists as you type to match the hierarchy." — [DOC], [DOC-KEYS]
- "Renumber list when typing" (default **off** in master) with "List numerating": Sequentially (default), With '1.', With previous number (e.g. `5.`, `5.`, `5.`). — [DOC], [MarkdownCodeInsightSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/settings/MarkdownCodeInsightSettings.kt)
- Inspection "Incorrectly numbered list item" ("Expected item number {0}, but got {1}"), on by default, quick-fix "Fix list item numbering". — [BACKEND], [BUNDLE]
- Shift+Enter has a list-specific handler (`MarkdownListShiftEnterHandler` on `EditorStartNewLine`), and pasting into lists has a post-processor (`MarkdownListCopyPastePostProcessor`). — [CORE]
- Hitting Enter twice after a list item stops indentation ("there will be no indent if you will hit enter two times after list item on any indent level"); Enter in a block quote adds the `>` marker (with a space if "After blockquote marker" spacing is on). — [MarkdownEnterHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/MarkdownEnterHandler.kt)
- Formatter can normalise markers: Unordered list marker — Keep existing markers (default) / Force `*` / Force `-` / Force `+`; Ordered list marker — Keep / Force dot after number / Force parenthesis after number. — [BUNDLE], [MarkdownCustomCodeStyleSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/lang/formatter/settings/MarkdownCustomCodeStyleSettings.kt)
- Lists fold ("ordered list" / "unordered list" regions). Lists can be shown in the Structure view via registry key `markdown.structure.view.list.visibility` (default false). — [BUNDLE], [CORE]
- A string "Toggle Markdown Task" (`markdown.live.preview.toggle.task`) exists, tied to the experimental editable/live preview, not to a source-editor action. — [BUNDLE]

### Inferences
- There is no source-editor keyboard action for ticking a checkbox; toggling `[ ]`/`[x]` is by typing or (in master) by clicking in the experimental editable preview.

### Gaps
- Exact behaviour of Shift+Enter in a list (presumably a continuation line without a new marker) was not read from source.
- Whether new items copy `[x]` as `[ ]` in all cases was inferred from the constant, not from the full method body.

## Structure and navigation

### Takeaway
Headings drive everything: Structure tool window, File Structure popup, breadcrumbs, nav bar, nested folding, a generated TOC between `<!-- TOC -->` markers with an "outdated TOC" inspection, and full symbol semantics for header anchors (go to declaration, completion, Find Usages, Rename, Copy Reference).

### Cited Findings
- Structure tool window `Alt+7` / `⌘7` and File Structure popup `Ctrl+F12` / `⌘F12` "to view and jump to the relevant headings". Implemented by `MarkdownStructureViewFactory`; a nav-bar extension is registered too. — [DOC], [projectView descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/projectView/resources/intellij.markdown.projectView.xml)
- Breadcrumbs provider `MarkdownBreadcrumbsProvider` plus a file breadcrumbs collector. — [CORE]
- Folding regions: header sections h1–h6 (hierarchical, built from the header tree), ordered/unordered lists, block quotes, tables, code fences, front matter, table of contents, link destinations longer than 10 characters, base64 `data:` URIs (placeholder "base64 data"), and `<details>` blocks. Default-collapse options: Collapse front matter, links, tables, code fences, table of contents, `<details>` blocks. — [MarkdownFoldingBuilder.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/folding/MarkdownFoldingBuilder.kt), [BUNDLE], [xml descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/xml/src/main/resources/intellij.markdown.xml.xml)
- TOC generation: inserts at the caret a block delimited by `<!-- TOC -->` on both sides; one line per top-level header as `* [visible text](#anchor)`, indented `(level-1) × indent size` spaces; if TOC blocks exist, **all** of them are replaced and the action is renamed "Update Table Of Contents". Only top-level headers (not ones nested in lists/quotes) are collected. — [GenerateTableOfContentsAction.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/toc/GenerateTableOfContentsAction.kt), [DOC]
- Omit from TOC: intention "Omit heading from table of contents" writes `<!-- omit from toc -->`; the reader accepts the marker at the end of the header line or on the line above, and also the legacy `<!-- omit in toc -->` spelling "both used by the Markdown All in One extension for VS Code". — [TableOfContentsMarkers.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/toc/TableOfContentsMarkers.kt), [DOC]
- Inspection "Outdated table of contents section" ("This TOC section does not correspond to the actual structure of the document"), on by default, fix "Update table of contents section". — [BACKEND], [BUNDLE]
- Header anchors are symbols: declaration provider, reference provider on link destinations, rename usage searcher, usage searcher, stub indexes `HeaderTextIndex` / `HeaderAnchorIndex`; HTML `<a id>`/anchor attributes are also declarations and renameable. — [MODEL], [BACKEND]
- User-facing: `Ctrl+Click` on a link navigates to the header; Find Usages `Alt+F7` / `⌥F7` on a header or label lists links to it; Rename `Shift+F6` / `⇧F6` renames it "along with all relevant usages". — [DOC]
- Copy Reference is overridden for Markdown so it works on a header (`MarkdownHeaderCopyAction`, with a qualified-name provider for headers). — [BACKEND], [MarkdownHeaderCopyAction.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/backend/src/com/intellij/markdown/backend/reference/headers/MarkdownHeaderCopyAction.kt)
- Advanced setting "Squash multiple dashes in header anchors" (default off) controls anchor slug generation. — [CORE], [BUNDLE]
- Extend Selection has Markdown-specific steps for code spans and whole header sections; Focus Mode, Join Lines for block quotes, and Fill Paragraph are implemented for Markdown. — [CORE]
- Renaming a Markdown file offers "Rename bound documents" (same-named .html/.docx/.pdf) and the Project view can "Group documents with the same name, but different extensions (.md, .html, .docx, .pdf)". — [BUNDLE], [DOC-SET]

### Inferences
- The TOC is plain Markdown plus HTML comments, so it survives in any renderer; compatibility with the VS Code "Markdown All in One" omit marker shows JetBrains is tracking that extension as the de-facto convention.
- Because header anchors are indexed symbols, cross-file `other.md#section` links participate in rename and unresolved checks across the project, which is the hard part to replicate (requires a project-wide header index).

### Gaps
- No setting for TOC depth, ordered vs. bullet style, or auto-update on save was found; the bullet is hard-coded `*`.
- Sticky lines for headings were not verified.

## Links and images

### Takeaway
Link destinations are real references (files, header anchors, line numbers, GitHub-wiki style names) with completion, navigation and six inspections. Images can be inserted via a dialog, dragged in, or pasted from the clipboard (saved as a PNG next to the file with no dialog), and each image line has a gutter icon opening a "Configure Image" dialog that can convert to an HTML `<img>`.

### Cited Findings
- Reference/Completion providers: common link destinations, autolinks/web references, relative files without extension, GitHub wiki local references, line-number anchors, header anchors; a typed handler auto-opens completion for header anchors; link-label completion for reference links; completion for image tags. — [CORE], [MODEL], [IMAGES]
- Completion also works in an unclosed link: when the caret is after `](` with no closing `)`, the plugin virtually closes the link so file references can supply variants. — [MarkdownCompletionContributor.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/MarkdownCompletionContributor.kt)
- "IntelliJ IDEA provides completion for link labels that are available in the current context: files relative to the current file and valid header labels." — [DOC]
- Inspections (all warning level): "Unresolved file references" (on), "Unresolved header reference" ("Cannot resolve anchor {0}", on), "Unresolved link label" (on), "Unused link definition" (on), "Duplicate link definition" (on), "Links should not contain spaces" (on; fix "Replace with: {0}"), "Unresolved web link" ("Cannot find …", **off** by default, backed by an external annotator that checks internet resources). — [BACKEND], [MODEL], [BUNDLE]
- Intentions "Convert to reference link" and "Convert to inline link"; action "Convert to Reference – Convert an inline link to a reference-style link" which can offer to replace duplicate URLs ("has detected N duplicate links that can be replaced with a reference link"). — [BACKEND], [BUNDLE]
- Insert Image (`Ctrl+U` / `⌘U`) opens "Insert Image" dialog with fields: Path (browse "Select Image"), Width, Height, a "Screen Reader Text" panel with Title and Description, and a "Convert to HTML" checkbox. — [DOC], [MarkdownImagesBundle.properties](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/resources/messages/MarkdownImagesBundle.properties)
- Gutter icon "Configure image" on lines with a Markdown image or an HTML `<img>` (in Markdown and in embedded HTML); also intention "Configure image"; commands "Update HTML Image Attributes" and "Convert HTML Image To Markdown". — [IMAGES], [MarkdownImagesBundle.properties](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/resources/messages/MarkdownImagesBundle.properties)
- Drag-and-drop of files into the editor: inserts at every caret, one entry per line; path is relative to the Markdown file's directory, URL-encoded per segment (spaces become `%20`); image and SVG files become `![filename](path)`, anything else `[filename](path)`. The file is not copied or moved. Gated by "Insert links to images or files on drag-and-drop" (default on). — [MarkdownFileDropHandler.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/kotlin/org/intellij/plugins/markdown/images/editor/paste/MarkdownFileDropHandler.kt), [DOC-KEYS]
- Pasting files copied in a file manager produces the same links (`FileLinkPasteProvider`, same setting). — [FileLinkPasteProvider.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/kotlin/org/intellij/plugins/markdown/images/editor/paste/FileLinkPasteProvider.kt)
- Pasting an image bitmap (e.g. a screenshot): the platform writes a PNG named from the base "img" (`VfsUtil.getNextAvailableName(parent, "img", "png")`) into the **same directory as the Markdown file**, with no dialog, then inserts `![img.png](img.png)` at each caret. — [ImagePasteProvider.kt](https://github.com/JetBrains/intellij-community/blob/master/images/src/org/intellij/images/ide/ImagePasteProvider.kt), [MarkdownImagePasteProvider.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/images/src/main/kotlin/org/intellij/plugins/markdown/images/editor/paste/MarkdownImagePasteProvider.kt)
- Dropping a `.docx` onto the editor is handled by `MarkdownDocxFileDropHandler` (Word import). — [CORE]
- Backtick code spans can resolve to code symbols (reference contributor and searcher for backtick references; JVM variant in the Java module; registry `markdown.backtick.reference.symbol.length` default 10 as minimal symbol length). — [BACKEND], [java descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/java/resources/intellij.markdown.java.xml), [CORE]
- `@path` and `/path` completion contributors exist in master (`MarkdownAtPathCompletionContributor`, `MarkdownSlashPathCompletionContributor`), inserting paths prefixed with `@`. — [BACKEND], [MarkdownAtPathCompletionContributor.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/backend/src/com/intellij/markdown/backend/completion/MarkdownAtPathCompletionContributor.kt)
- External links: registry `markdown.open.link.in.external.browser` (default true) and a confirmation dialog "Are you sure you want to open {0}?" with "Don't ask again for '{scheme}://' links". — [CORE], [BUNDLE]

### Inferences
- Image paste is deliberately minimal: no naming prompt, no configurable assets folder, filename "img*.png". This is the most obvious place to do better (configurable destination and name), and is a common complaint pattern in other editors' trackers.
- The `@path` completion and the AGENTS/SKILL templates (see "Other") indicate 2026 work aimed at AI-agent instruction files written in Markdown.

### Gaps
- Exact naming sequence for subsequent pastes (img_1.png vs img1.png) not verified.
- No "paste as Markdown from HTML/rich text" or "copy as rich text/HTML" action was found in any descriptor.
- No automatic link update when a linked file is moved was verified beyond the generic file-reference machinery (references are bound, so platform move refactoring should update them, but I did not test or find a doc statement).

## Code fences

### Takeaway
A language tag on a fence injects that language (highlighting, completion, inspections, intentions), with completion for the language name itself, a toggle for showing errors, run gutter icons for shell blocks/lines and for other runnable languages, and an intention to copy the fence content.

### Cited Findings
- "If you specify the language for the code block, by default, the Markdown editor injects the corresponding language. This enables syntax highlighting and other coding assistance features… completion, inspections, and intention actions." — [DOC]
- Implementation: `CodeFenceInjector` and `CodeFenceTemporaryLangInjector` multi-host injectors; language providers for default languages, PowerShell, PlantUML, Mermaid, and a TextMate-bundle fallback module; an "additional fence language suggester" for embedded tokens. — [fenceInjection descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/fenceInjection/src/main/resources/intellij.markdown.fenceInjection.xml), [CORE], [PLUGIN]
- Language-name completion: basic completion on the fence info string (`FENCE_LANG` token) lists available languages. — [MarkdownCompletionContributor.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/editor/MarkdownCompletionContributor.kt)
- Settings (Languages & Frameworks | Markdown): "Inject languages in code fences"; "Show errors in code fences" ("Check code blocks for syntax errors according to the specified language"). Intention "Hide problems in code fences" flips the latter and shows a notification with a "Roll Back" action. — [DOC-SET], [BUNDLE], [CORE]
- Intention "Copy code fence content to the clipboard". — [BACKEND], [BUNDLE]
- Reformat does not format fence contents by default: registry `markdown.formatter.apply.to.code.fence` = false ("Should Markdown use injected languages' formatters to format code-fences"). — [CORE]
- Indent guides are filtered inside fences, and a background-highlighting pass paints fence backgrounds. — [CORE]
- Run: gutter icons with labels "Run Block", "Run Line", "Run in Terminal" (Shell Script / PowerShell), "Run '{0}'"; tooltip "Right-click to choose a terminal" with "Choose Terminal..."; action `Markdown.RunCodeFence` ("Run Code Fence") reuses the shortcut of `RunClass` = `Ctrl+Shift+F10` / `⌃⇧R`. Shell prompts are trimmed, line continuations are joined, and inline code spans on a line can be runnable too (matched against run configurations). — [BACKEND], [BUNDLE], [DOC], [MarkdownRunLineMarkersProvider.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/backend/src/com/intellij/markdown/backend/highlighting/markers/MarkdownRunLineMarkersProvider.kt)
- Run settings: "Detect commands that can be run right from Markdown files"; "Run commands from: Project directory / Markdown file parent directory". Untrusted projects get a safe-mode warning before executing code blocks. — [DOC-SET], [BUNDLE]
- Gutter adverts: "Install Mermaid plugin" line marker on mermaid fences and "Install PlantUML" on plantuml fences. — [BACKEND], [BUNDLE]
- Tracker: "False positive `class` or `interface` for Java in Markdown" (IDEA-220938, open since 2019, 10 votes) — an example of injected-language false errors on snippets. — [YouTrack IDEA-220938](https://youtrack.jetbrains.com/issue/IDEA-220938)

### Inferences
- Snippet fences are often not valid standalone programs, so error highlighting inside fences is a known irritant; JetBrains answered with a one-click "Hide problems in code fences" intention rather than smarter snippet parsing. An equivalent feature should default to highlighting-only (no diagnostics) or make the toggle very discoverable.

### Gaps
- Enter-inside-fence indentation and auto-closing of the fence on typing three backticks were seen as handlers (`MarkdownTypedHandler`, fence handling in `MarkdownEnterHandler`) but their exact rules were not read.
- No "edit fragment in separate editor" specifics were verified for Markdown (the platform's generic Edit Fragment applies to injections).

## Inspections, intentions, code style / reformat, spelling and grammar

### Takeaway
There are 16 Markdown-specific inspections and about 15 intentions; the formatter (Reformat Code, `Ctrl+Alt+L` / `⌘⌥L`) hard-wraps at the right margin by default, normalises blank lines and spaces, reformats tables, and can normalise list markers. Spell checking is wired in through a Markdown spellchecking strategy.

### Cited Findings
- Full inspection list (short name — default): MarkdownIncorrectTableFormatting — off, weak warning; MarkdownNoTableBorders — on; MarkdownTablePipeInCodeSpan — on; MarkdownUnresolvedFileReference — on; MarkdownIncorrectlyNumberedListItem — on; MarkdownOutdatedTableOfContents — on; MarkdownLinkDestinationWithSpaces — on; MarkdownSkillFrontMatter — on; MarkdownNonExistentInternetResource — off; MarkdownLazyBlockQuoteContinuation — on, weak warning ("cause inconsistent rendering across different Markdown parsers", fix "Add blockquote '>' markers"); MarkdownMissingHeaderSpace — on ("A header needs a space after the '#' marker", fix "Insert space after header marker"); MarkdownUnresolvedHeaderReference — on; MarkdownUnresolvedLinkLabel — on; MarkdownUnusedLinkDefinition — on; MarkdownDuplicateLinkDefinition — on. — [BACKEND], [MODEL], [BUNDLE]
- Intention category "Markdown": insert column left/right, set column alignment, remove column, remove row, reformat table, fix cell alignment, fix table borders, omit heading from TOC, convert to reference link, convert to inline link, copy code fence content, hide problems in code fences, configure image. — [CORE], [BACKEND], [IMAGES]
- Inspection suppression is supported for Markdown (`MarkdownSuppressionProvider`). — [xml descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/xml/src/main/resources/intellij.markdown.xml.xml)
- Code style, Wrapping: "Hard wrap at" (right margin), "Wrap on typing", "Wrap long text" (default on), "Wrap text inside block quotes" (on), "Visual guides"; When reformatting: "Keep line breaks inside text blocks" (on), "Insert block quote arrows" (on), "Format tables" (on). — [DOC-STYLE], [MarkdownCustomCodeStyleSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/lang/formatter/settings/MarkdownCustomCodeStyleSettings.kt)
- "By default, IntelliJ IDEA doesn't add hard wraps to long lines automatically. It will wrap lines when you reformat the file"; "Edit | Fill Paragraph" wraps only the current paragraph. — [DOC]
- Code style, Blank lines (min and max both default 1): Around header, Around block elements, Between paragraphs. Spaces (force exactly one, all default on): Between words, After header symbol, After list marker, After blockquote marker. Tabs and Indents: Use tab character, Smart tabs, Tab size, Indent, Continuation indent, Keep indents on empty lines, "Use fixed indents for sublists" (default on). — [DOC-STYLE], [MarkdownCustomCodeStyleSettings.kt](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/src/org/intellij/plugins/markdown/lang/formatter/settings/MarkdownCustomCodeStyleSettings.kt)
- Post-format processors for block quotes, list markers and tables; a Markdown-specific line-wrap position strategy; a long-line inspection policy. — [CORE], [BACKEND]
- "Strip trailing spaces in Markdown files on save" option plus a Markdown strip-trailing-spaces filter; inlay hint "Hard line breaks" ("Shows the soft wrap sign after the trailing spaces that make a hard line break"), enabled by default. — [BUNDLE], [CORE], [BACKEND]
- Spelling: `MarkdownSpellcheckingStrategy` registered for the Markdown language. — [spellchecker descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/spellchecker/src/main/resources/intellij.markdown.spellchecker.xml)
- TODO patterns are indexed in Markdown files (todo indexer + index pattern builder). — [CORE]
- Colour scheme page Editor | Color Scheme | Markdown with keys for text, bold/italic text and markers, strikethrough, highlight, six header levels, header marker, code span/marker/block/fence, blockquote and marker, horizontal rule, table separator, links (explicit, reference, auto, definition, text, label, destination, title, image, citation), lists, definition lists, footnotes, and five GitHub alert badges (Note, Tip, Important, Warning, Caution). — [BUNDLE]

### Inferences
- The default formatter behaviour (hard-wrap long text at the margin on Reformat Code) is opinionated and changes diffs heavily; an equivalent should make hard wrap opt-in or at least very visible.
- Several inspections exist for cross-renderer portability (side borders, lazy blockquote continuation, spaces in links, missing header space), i.e. JetBrains lints toward "renders the same on GitHub and elsewhere".

### Gaps
- Grammar checking (Grazie / Natural Languages plugin) in Markdown prose: I found no primary source in this session describing its Markdown-specific behaviour; it is a separate bundled plugin, not part of the Markdown plugin descriptors read here.
- No markdownlint-rule compatibility or `.markdownlint` config support was found.

## Import/export and conversion

### Takeaway
Tools | Markdown has three actions: Import Word Document…, Export Markdown File To… (HTML and PDF built in, DOCX through Pandoc), and Configure Pandoc…. Export depends on the rendered preview being open.

### Cited Findings
- Menu group `Markdown.Tools` in the Tools menu: `Markdown.ImportFromDocx` ("Import Word Document…"), `Markdown.Export` ("Export Markdown File To…"), `Markdown.ConfigurePandoc` ("Configure Pandoc…"). No default shortcuts. — [CORE], [BUNDLE]
- "The Markdown plugin can export your Markdown files to HTML and PDF out of the box. If you configure Pandoc, it will also be able to export to and import from Microsoft Word files (DOCX)." The plugin does not bundle Pandoc. — [DOC], [PLUGIN]
- "IntelliJ IDEA relies on rendered HTML to export Markdown to another format, so without the preview pane all export options will be disabled"; error string "Converting to {0} is only possible if the Markdown preview is open". — [DOC], [BUNDLE]
- Export dialog fields: Name, Location, Format; HTML export has "Save images to:" (when disabled, "images will be embedded in the HTML file"); registry `markdown.export.html.enforce.csp` (default true) adds a content-security-policy to exported HTML. — [BUNDLE], [CORE]
- Pandoc settings: "Path to Pandoc executable" (auto-detected, with Test button) and "Save images from Microsoft Word to". Word import can also be triggered by dropping a .docx. — [DOC-SET], [BUNDLE], [CORE]
- Export providers are an extension point (`org.intellij.markdown.markdownExportProvider`); DOCX is registered through it. — [CORE]

### Inferences
- PDF/HTML export is a by-product of the browser-based preview (print rendered HTML), which is why it needs the preview visible; an editor with a native (non-browser) preview would need a separate export pipeline.

### Gaps
- No generic "Convert to…" (e.g. Markdown to AsciiDoc/reStructuredText) and no "Copy as HTML / rich text" action exists in the descriptors. The only "Convert" items are image Markdown↔HTML and inline↔reference links.

## Other: front matter, HTML, doc comments, templates, MDX, README, AI

### Takeaway
Front matter (YAML and TOML) is injected and foldable with JSON-schema support; inline HTML is parsed as HTML; 2026 master adds AI-agent oriented pieces (AGENTS and SKILL file templates, a SKILL front-matter inspection, `.mdc` file type, `@path` completion) and an experimental "Editable Preview". MDX and grammar checking live outside this plugin.

### Cited Findings
- Front matter: modules `intellij.markdown.frontmatter`, `.frontmatter.yaml`, `.frontmatter.toml`; a language-injection performer for the header and a JSON Schema provider factory for it; folding region "front matter" with "Collapse front matter" option and a `frontMatterFoldingPolicy` extension point. — [PLUGIN], [frontmatter descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/frontmatter/src/main/resources/intellij.markdown.frontmatter.xml), [CORE], [BUNDLE]
- HTML in Markdown: the XML module supplies a multi-root file view provider (Markdown + HTML), `<details>` folding, HTML anchor symbols, and image gutter icons for HTML `<img>`. — [xml descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/xml/src/main/resources/intellij.markdown.xml.xml), [MODEL], [IMAGES]
- File type: extensions `markdown;md;mdc` in master; docs still list only `*.md` and `*.markdown`. — [CORE]; contradicted (older) by [DOC]
- New-file templates: "Markdown File" (New | Markdown File, search synonym "MD"), README (`# ${PROJECT_NAME}`), AGENTS (`# AGENT Instructions`), SKILL (YAML front matter with `name:` derived from the directory name, lower-case hyphenated, max 64 chars, and `description:`). — [CORE], [BUNDLE], [SKILL template](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/core/resources/fileTemplates/internal/SKILL.md.ft)
- Inspection "Invalid skill front matter": "Skill must start with YAML front matter"; `name` must be "1–64 lowercase letters, numbers, or hyphens and cannot start or end with a hyphen"; `description` required, non-blank, max 1024 characters; fixes insert/update the fields. — [BUNDLE], [BACKEND]
- Live templates: a `MARKDOWN` live-template context is registered ("Markdown"), so users can define templates scoped to Markdown. — [CORE], [BUNDLE]
- GitHub alerts (`> [!NOTE]` etc.) get gutter line markers ("Alerts", with "Disable Alert Icons") and colour keys. — [BACKEND], [BUNDLE]
- Math (`$…$`, `$$…$$`), Mermaid (separate plugin) and PlantUML (downloadable extension) are supported; fences for them have language providers. — [DOC], [CORE]
- AI: the floating toolbar's first slot is a reference to `Floating.CodeToolbar.Extensions`, commented "enables AI actions", i.e. AI Assistant actions are injected into the Markdown selection toolbar by the platform rather than by this plugin. — [CORE]
- Experimental editable preview in master: action "Editable Preview (Experimental)" (`Markdown.Layout.LivePreview`, also `Markdown.Diff.LivePreview` "Show the rendered Markdown in both sides of the diff"), registry `markdown.live.preview.enabled` default true ("Shows the Markdown live preview layout"), a setting "Render preview in: JCEF / Editor", image limits of 4 MB / 4 megapixels, and a "Toggle Markdown Task" command. — [CORE], [BUNDLE], [FRONTEND]
- Markdown in Java documentation comments: tracker issue "Support Markdown in Javadoc" (IDEA-109829) is resolved; follow-ups "Markdown Javadoc: Support rendering of Mermaid diagrams" (IDEA-388235, 24 votes, open) and "…Support PlantUML rendering" (IDEA-315387, 16 votes, open). — [YouTrack IDEA-109829](https://youtrack.jetbrains.com/issue/IDEA-109829), [YouTrack IDEA-388235](https://youtrack.jetbrains.com/issue/IDEA-388235), [YouTrack IDEA-315387](https://youtrack.jetbrains.com/issue/IDEA-315387)
- Misc: a VCS ignored-file provider for "Markdown personal settings"; a Figma advertiser module; project settings for custom preview CSS. — [vcs descriptor](https://github.com/JetBrains/intellij-community/blob/master/plugins/markdown/vcs/resources/intellij.markdown.vcs.xml), [PLUGIN], [BUNDLE]

### Inferences
- The editable preview being registry-enabled in master suggests JetBrains is moving toward an Obsidian/Typora-style rendered-while-editing mode; this overlaps the other researcher's scope and should be cross-checked against released 2026.x builds.
- JetBrains is treating Markdown as the configuration language of AI agents (AGENTS.md, SKILL.md, `.mdc` rule files) and adding validation for it.

### Gaps
- No bundled Markdown live templates were found (the guessed `liveTemplates/Markdown.xml` path returned 404); only the context type is confirmed.
- MDX: not part of this plugin's descriptors; I did not verify the separate MDX plugin in this session.
- No README-specific editing behaviour beyond the template and run-command detection was found.
- Which of the master-only items (`.mdc`, AGENTS/SKILL templates, skill inspection, `@path` completion, editable preview, HTML-comment commenting) shipped in 2026.2 versus a later release could not be pinned to a What's New page.

## What users value or complain about

### Takeaway
Evidence gathered here is thin: the public tracker query surfaced a long-open WYSIWYG request and fence false-positive complaints, and 2026 forum/news items are about the preview breaking rather than editing aids. Treat prioritisation below as partly inferred from JetBrains' own defaults.

### Cited Findings
- "Markdown: Add a WYSIWYG editor like Typora" — open since 2021, 15 votes. — [YouTrack IDEA-270624](https://youtrack.jetbrains.com/issue/IDEA-270624)
- "False positive `class` or `interface` for Java in Markdown" — open since 2019, 10 votes. — [YouTrack IDEA-220938](https://youtrack.jetbrains.com/issue/IDEA-220938)
- "Cannot edit language injected code snippet inside markdown" — resolved, 11 votes (2018). — [YouTrack IDEA-187780](https://youtrack.jetbrains.com/issue/IDEA-187780)
- Open low-vote items from 2024: "Some npm commands are not shown as executable inside .md" (IDEA-356645), "HTML highlighting broken in .md files" (IDEA-353032), "Injected files quick fixes are broken in md" (IDEA-352546). — [YouTrack IDEA-356645](https://youtrack.jetbrains.com/issue/IDEA-356645), [YouTrack IDEA-353032](https://youtrack.jetbrains.com/issue/IDEA-353032), [YouTrack IDEA-352546](https://youtrack.jetbrains.com/issue/IDEA-352546)
- A support-forum thread reports the Markdown preview broken after the 2026.1.1 update, and a 2026.1.x bug-fix release was covered in the press for "Markdown preview fixes". — [JetBrains support forum](https://intellij-support.jetbrains.com/hc/en-us/community/posts/35054846035218/comments/35087378231442), [Neowin](https://neowin.net/news/jetbrains-rolls-out-intellij-idea-update-with-markdown-preview-fixes-and-more/)
- JetBrains' own framing of what matters, from the plugin description: "Assistance for editing tables and lists: automatic alignment and numbering", "Drag and drop to insert images", "Language injection in code blocks", export to HTML/PDF/Word. — [PLUGIN]

### Inferences
- Suggested build priority, derived from what JetBrains enables by default and advertises: (1) list continuation/indent/outdent and smart Backspace; (2) inline format toggles with shortcuts and URL-paste-over-selection; (3) table Tab navigation, Shift+Enter row, visual alignment, row/column operations; (4) heading outline, folding, breadcrumbs; (5) link/anchor completion with unresolved-link diagnostics and header rename; (6) fence language injection with an easy way to silence errors; (7) image paste/drop; (8) TOC generate/update with outdated check; (9) export.
- Features JetBrains ships disabled (renumber on typing, reformat table on typing, strict table-format inspection, web-link checking) are likely the ones that annoyed users when on, so they are low priority or should be opt-in.
- Injected-language errors in fences and anything that rewrites source unprompted (hard wrap on reformat, table rewriting) are the recurring friction points visible in the evidence.

### Gaps
- I could not get a reliable votes-sorted list for the Markdown subsystem from the tracker API (the subsystem filter returned only zero-vote items and free-text search is noisy), so vote counts above are a sample, not a ranking.
- The August 2022 JetBrains blog post "Some README.md Love: Markdown Support Improvements" (https://blog.jetbrains.com/idea/2022/08/markdown-support-improvements/) returned HTTP 403, so its feature dates and reader comments are not captured.
- No Reddit or Stack Overflow threads were reviewed; version-by-version "added in" dates before 2022.1 (e.g. when TOC generation, drag-and-drop images, the formatter, and run-from-README were introduced) are not sourced here.
