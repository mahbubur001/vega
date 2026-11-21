# Resume here

The restart point for the next working session, possibly on another computer. Read this first, then `ROADMAP.md`. Update it at the end of every session.

_Last updated: 2026-11-21 (built-in PHP, auto-install fixed). Branch `vega`, everything committed and pushed._

## How the owner wants to work

- Keep going task by task without asking "should I go ahead?": finish, test, commit, push, then take the next recommended task. Ask only for decisions that are genuinely the owner's (product direction, money, publishing).
- Items that need a look in the running app stay "on hold" until the owner checks them or sends screenshots.
- Never add Claude co-author or attribution lines to commits.

## Setting up a new computer

1. `git clone https://github.com/mahbubur001/vega && cd vega && git checkout vega`, then `git remote add upstream https://github.com/zed-industries/zed`. Before the first `git fetch upstream`, limit it to `main` with no tags, so Zed's ~1,800 branches and ~1,500 tags stay out of the clone: `git config remote.upstream.fetch '+refs/heads/main:refs/remotes/upstream/main' && git config remote.upstream.tagOpt --no-tags`.
2. Rust via rustup (the toolchain comes from `rust-toolchain.toml`), `brew install cmake`.
3. With full Xcode, plain `cargo run` works. Without Xcode, add `--features gpui_platform/runtime_shaders` to every build, check, test and clippy command that compiles the app (for crate tests: `cargo test -p <crate> -p gpui_platform --features gpui_platform/runtime_shaders`).
4. Run the app as "VEGA Dev.app" so the Dock shows the VEGA icon: `script/vega-dev-app --build ~/some/test/project` (it picks runtime shaders itself when Xcode is missing). Don't open this repo itself in VEGA.
5. Disk: the debug build cache grows by tens of GB per session. When space runs low, `rm -r target/debug/incremental` (safe; it rebuilds). Each new dependency also leaves the old app and test binaries behind in `target/debug/deps` (`vega-<hash>`, about 1.2 GB each, and `<crate>-<hash>` test binaries); all but the newest of each name can be deleted.

## Where things stand

**Upstream sync, background fetch, Check TODO (2026-11-21, evening):** merged upstream `2f808b3c2d` (39 commits). Background fetch is upstream's `git.auto_fetch`, now on by default every 300 s, so the branch popup's ↓/↑ counts stay current. Check TODO before commit: the git panel runs registered before-commit checks (`git_ui::vega_commit_check`, a small seam in `GitPanel::commit_changes`); `vega_todo::commit_check` diffs the commit with `git diff -U0` (`--cached` when something is staged, else `HEAD`), scans the changed files and, when TODO items are on added lines, asks Commit / Review / Cancel. Review opens the TODO panel on a "Changed Lines" tab. Setting `todo.check_before_commit` (default true). Not checked in the app.

**External Libraries like PhpStorm (2026-11-21):** "PHP" (include paths outside the project) and "Project @types/*" (node_modules/@types); PHP Runtime, Composer and Node.js nodes removed. Shown automatically for PHP projects (composer.json, `.vega/php.json`, include paths) or projects with @types; Tools › Show/Hide External Libraries (action `php::ToggleExternalLibraries`) saves a per-project choice in VEGA's database. The scratches node is now just "Scratches".

**TODO tool window and scratch autosave (2026-11-21):** new crate `vega_todo`: a dock panel (Project / Current File, grouped by file, click to jump), TODO and FIXME highlighting in editors, and a `todo` settings section (patterns, case_sensitive, highlight). The comment test uses the syntax tree (`override_name() == "comment"`) and falls back to comment markers. Scratch files save as you type and never ask on close: `workspace::item::AlwaysAutosaveFolders`, a small seam in item.rs and pane.rs. The repo is now public, so CI minutes are free.

**UI overflow guard (2026-11-21):** rows spilling past a popup's edge come from narrowing a `Picker` with a wrapper `.w(...)`. Use `.initial_width(...)` instead. Run `script/vega-check-ui-widths`; the skill `.claude/skills/vega-ui-overflow` covers the other causes.

**Newest (2026-11-21, later): PhpStorm-style title bar and branch popup** (new crate `vega_branch_popup`; title bar change in `crates/title_bar/src/title_bar.rs`, one call swapped from `git_ui_core::build_branch_picker`).
- Title bar: the worktree button shows only when the repository has several worktrees; the branch button shows ↓incoming ↑outgoing and a ▾.
- Popup: search over branches and actions; Update Project (`git::Pull`), Commit… (`git::ExpandCommitEditor`), Push… (`git::Push`), New Branch…, Checkout Tag or Revision…; groups Recent (from the reflog, unfolded) / Local / Remote / Tags (folded; any query unfolds all). Rows: ★ current, ↓/↑ counts, upstream, › menu: Checkout, New Branch from…, Rebase current onto, Merge into current, Update/Push (current), Rename…, Delete. Enter on a branch checks it out; Enter on a header folds it.
- Tags, recent, remote checkout (`git checkout --track`), revision checkout, merge and rebase run the `git` CLI (local projects only); the rest uses the repository API. Errors show as toasts.
- Not built: Compare with current, favourites (☆), Update for a non-current branch, PhpStorm's ⌘T/⌘K/⇧⌘K keys (⌘K is Zed's chord prefix). Not checked in the app.
- Markdown layout buttons moved to the right end of the tab bar (`Pane::add_tab_bar_end_item`, a small VEGA seam in `crates/workspace/src/pane.rs`). Editor toolbar: breadcrumbs, quick actions and selections menu off by default.
- phpactor: downloads `phpactor.phar` through the direct latest-release link when GitHub's API is rate limited; restarts only after the new include paths are loaded (settings reload is awaited, not timed); every project reindexes once (marker v3).

 (it was the `php` extension from Zed's registry, so a fresh VEGA had no PHP at all).
- Grammar: `tree-sitter-php` 0.24 (the exact revision the extension used); queries copied from the extension into `crates/grammars/src/php` (Apache-2.0, `LICENSE-APACHE` there, header in each file). Not ported: phpdoc highlighting in `/** */` comments (its grammar needs tree-sitter 0.20), Intelephense's custom completion labels, the phptools and phpantom servers.
- Language servers in `crates/languages/src/php.rs`: phpactor (VEGA's default: `phpactor` on PATH, else the latest `phpactor.phar` from GitHub run with `php`) and Intelephense (npm). Tasks: phpunit, pest, run selection.
- Xdebug debug adapter back in core: `crates/dap_adapters/src/xdebug.rs`, name "Xdebug" like the extension.
- The `php` extension is suppressed (`SUPPRESSED_EXTENSIONS`), so an installed copy is removed and it can't be installed again.
- Extension auto-install works again: VEGA installs what `auto_install_extensions` lists, at startup and when the setting changes (VEGA's default list is empty, so it never contacts the registry by itself; update checks stay off). Before, auto-install never ran, and the install prompt was hidden for listed extensions, so a `.php` file opened without highlighting and without a prompt.
- Templates: keymap template says VEGA. The owner's `~/.config/vega/settings.json` got the VEGA header and their Zed extension list (backup in the session scratchpad).
- Known failing test, not from this work: `extensions_ui` `test_suggestion_returns_after_failed_install` (fails on the previous commit too).
- Owner check: open a `.php` file: highlighting, outline, phpactor starts (status bar), go-to-definition; debug a script with Xdebug.

, steps 1 to 5 of 6** (plan and research: `reports/IntelliJ Markdown view modes.md`, "Build the switcher first…" table). Steps 1 to 4 are in the new crate `markdown_layout`, step 5 in the new crate `markdown_editing`.
- Step 1, switcher: three buttons on every Markdown editor and preview tab (Editor Only / Editor and Preview / Preview Only, like IntelliJ) and the actions `markdown_layout: show editor only`, `show editor and preview`, `show preview only` (no default keys). Editor and preview stay separate tabs; the layout is worked out from what is open, nothing is stored. Preview Only keeps an editor tab that has unsaved changes.
- Step 2, setting `markdown_preview.default_layout`: `editor` (default, no change), `split` or `preview`. Applied once, when a Markdown file is first opened (not on restore, not when a tab moves); `split` and `preview` win over `open_markdown_files_in_preview`. Also in the settings window ("Markdown Default Layout").
- Steps 3 and 4, two-way scroll sync: scrolling a Markdown editor puts the same source line at the top of its previews (with the fraction of a line), and scrolling a preview by wheel, scrollbar or page keys scrolls its editor to the line at the preview's top. A scroll that one side caused on the other is not sent back. Off switches: `markdown_preview.scroll_preview_with_editor` and `markdown_preview.scroll_editor_with_preview` (both default true). The renderer seam is `crates/markdown/src/scroll_sync.rs` (new file): the event `MarkdownScrolled` and `Markdown::scroll_source_index_to_top`.
- Step 5, editing commands (`markdown_editing`, actions in the `markdown_editing` namespace): Toggle Bold (⌘B), Toggle Italic (⌘I), Toggle Strikethrough, Toggle Code and Insert Link (no default keys: IntelliJ's collide with Save As and others), Paste (⌘V: a URL pasted over selected text becomes `[text](url)`, anything else is the normal paste), and in tables Next / Previous Table Cell (Tab / Shift-Tab) and Insert Table Row (Shift-Enter). The keys are bound only in `.md` files, and a command that does not apply hands the key on, so Tab outside a table still indents. In `.md` files ⌘B no longer toggles the left dock. Pasting an image was already in upstream. The text logic is plain functions in `crates/markdown_editing/src/text_edits.rs`. Upstream files touched for step 5: `crates/zed/src/main.rs` (init) and six bindings in the `Editor && extension == md` block of each `assets/keymaps/default-*.json`.
- Upstream files touched: `crates/zed/src/zed.rs` (toolbar item), `crates/zed/src/main.rs` (init), two added functions in `crates/markdown_preview/src/markdown_preview_view.rs` (`suppress_auto_preview_for`, `markdown`), `crates/markdown/src/markdown.rs` (module line, one field, and the prepaint call now goes through `autoscroll_and_track_scroll`), three keys in `crates/settings_content/src/settings_content.rs` and `assets/settings/default.json`, three entries in `crates/settings_ui/src/page_data.rs` (the section's array length changes too) and one renderer line in `crates/settings_ui/src/settings_ui.rs`.
- Known limits: in split, each Markdown file gets its own preview tab in the side pane, and closing the editor leaves its preview open. Scrolling the preview through a tall image or diagram holds the editor at the next text line. Not built: IntelliJ's selection toolbar, table column commands, table of contents, and step 6 (math; wait for upstream).
- Also today: Zed's branches and tags removed from the local clone (`upstream` fetches only `main`, no tags) and the 12 Zed workflows disabled on GitHub.

**Earlier (2026-11-19, night): PhpStorm External Libraries & Scratches, L1–L4 built** (plan `LIBRARIES_SCRATCHES_PLAN.md`, option A: nodes inside the project panel through a small seam, `crates/project_panel/src/extra_rows.rs`). Crates: `scratches` (Scratches and Consoles: ⇧⌘N New Scratch File with VEGA's built-in language picker, New Scratch Buffer, Show Scratch Files, ⌃⇧R Run, right-click Change Language/Rename/Copy/Move to Project/Delete; checked in the app: create, run, delete-on-close), `external_libraries` (PHP Runtime, Composer, Node.js, include paths; read-only library files with a "Non-project file" bar; optional phpstorm-stubs download), `php_settings` (PHP settings window, `.vega/php.json`, language-server keys in `.vega/settings.json`), `vega_dialog` (shared dialog look, moved out of deployment_ui). L4: Find in Files with *include ignored* also searches include paths and stubs (`crates/project/src/library_worktrees.rs`). Test project with real packages: `~/dev/vega-libs-test` (composer + npm installed).

Done and pushed today: branding sweep of ~100 visible "Zed" strings, VEGA logo on the welcome/onboarding pages, onboarding telemetry switches hidden (VEGA never uploads); project config folder is `.vega/` instead of `.zed/` (settings, tasks, debug, deployment; `.zed/` is ignored on purpose); the CLI is `vega` (`vega .` opens a folder; `script/vega-dev-app --build` builds it, then `ln -sf "$PWD/target/debug/VEGA Dev.app/Contents/MacOS/cli" ~/.local/bin/vega` or run "cli: install"); D4 Sync with Deployed view (`deployment: sync with deployed`, see `DEPLOYMENT.md`); D3 second pass (cut/copy/paste, chmod, Exclude Path, drag and drop, keyboard navigation, remembered dock position; see `DEPLOYMENT.md`). Earlier: Linux-only CI (`.github/workflows/vega_ci.yml`, green, about 22 minutes uncached), MCP OAuth without Zed's CIMD document, every zed.dev link VEGA opens now goes to VEGA's docs or repo (`crates/release_channel/src/vega_docs.rs` + `gpui::OpenUrlRewriter`), Help menu feedback to VEGA's GitHub, Deployment Servers dialog Advanced tab and settings link, `script/vega-dev-app`, and the first version of the **Remote Host panel (D3)**.

Built but **not yet checked in the running app** (owner's manual check, screenshots welcome):

| Item | How to check |
|---|---|
| P06 VEGA Dark theme | It's the default dark theme |
| P07 AI Provider onboarding | `onboarding: open` → AI Provider section |
| D1 upload on save, Deployment submenu | See `DEPLOYMENT.md` "Next session" (Docker SFTP test server) |
| D2 Deployment Servers dialog | Right-click → Deployment → Configuration…, or Settings → General → Deployment |
| D3 Remote Host panel | Server icon in the right dock (shown once a server is configured), or `deployment: toggle remote host`. Browse, open a file, edit and save (should upload), right-click menu, cut/copy/paste, Change Permissions…, Exclude Path, arrow keys, drag rows onto folders, drag files from Finder and the project panel, move the dock and restart |
| D4 Sync with Deployed | Deployment → Sync with Deployed… on a folder (or `deployment: sync with deployed`). Change some files on both sides, check the states, flip directions, Compare, Synchronize |
| Settings Sync page | Settings → General → Settings Sync |
| `vega` CLI | `vega .` in a project folder opens it in VEGA Dev |
| App icon | Launch with `script/vega-dev-app` and look at the Dock |
| L1 Scratches (mouse parts) | Right-click a scratch: Run, Change Language, Rename, Copy, Move to Project, Delete; fold/unfold the node |
| L2 External Libraries | `script/vega-dev-app ~/dev/vega-libs-test`: unfold External Libraries, open a vendor file (read-only bar), right-click PHP Runtime › Download PHP Stubs |
| L3 PHP settings | `php: open php settings` or Settings › Languages & Tools › PHP; change interpreter, add an include path, OK; check `.vega/php.json` and `.vega/settings.json` |
| L4 Project and Libraries | Find in Files with "include ignored" on finds text inside include paths and stubs |
| Markdown layout switcher | Open any `.md` file: three buttons at the right of the editor toolbar. Try all three from the editor tab and from the preview tab; check the icons read well next to the upstream eye button, and whether a split made of two tabs feels acceptable |
| Markdown default layout | Set `"markdown_preview": { "default_layout": "split" }` in settings (or Settings window › "Markdown Default Layout"), then open a `.md` file that is not open yet: editor left, preview right. Try `preview` too |
| Markdown scroll sync | In the split, scroll a long `.md` file in the editor: the preview should keep the same line at its top. Then scroll the preview: the editor should follow. Watch for jitter or the two sides fighting each other |
| Markdown editing commands | In a `.md` file: select a word, ⌘B and ⌘I (press again to undo the style); copy a URL, select some text, ⌘V; in a table press Tab, Shift-Tab and Shift-Enter; outside a table check Tab still indents and Shift-Enter still makes a new line. Check ⌘B still toggles the left dock in other files |

Docker SFTP test server: `crates/deployment/tests/servers` (see `DEPLOYMENT.md` "Running the server tests"; generate `fixtures/id_ed25519` first on a new machine). Test login `tester` / `pass` on `localhost:2222`.

## Next tasks, in order

**Start here next session:**
- Markdown (built 2026-11-20, steps 1 to 5; nothing has been looked at in the running app by the agent, only by tests): the owner checks the five "Markdown …" rows in the table above. Likely follow-ups, in the order the checks will show them: icons of the three layout buttons next to the upstream eye button; whether a split made of two tabs is acceptable (if not, the alternative is one composite tab, see the report, "Build the switcher first…"); jitter in the two-way scroll sync; key choices for the editing commands.
- Markdown, not built: a preview below the editor (the split always opens to the right), per-file memory of the layout, one shared preview tab for all files in the split, IntelliJ's selection toolbar, table column commands (insert, move, align, remove), table of contents, default keys for strikethrough, code and insert link. Math is left to upstream (two community PRs open).
- `research_notes/IntelliJ Markdown view modes/` holds the four research notes behind the report (IntelliJ view modes, IntelliJ editing features, VS Code and others, Zed's preview and seams).
- Go-to-definition on PHP built-ins (`defined`, `die`…), **fix built 2026-11-20, not checked in the app**: once the stubs are downloaded (External Libraries › PHP Runtime › Download PHP Stubs), phpactor's init options in `.vega/settings.json` get `worse_reflection.stub_dir` = the stubs folder and `worse_reflection.cache_dir` = `<data dir>/php-stubs-phpactor-cache` (a fresh stub map; a new download clears it), and phpactor restarts. Projects opened later get the keys if they already have a `.vega/settings.json`. Code: `php_settings::config::settings_with_stub_keys`, `phpactor::sync_stub_settings`. Owner check: in a PHP project, download the stubs, wait for phpactor to restart, cmd-click `defined(`; it should open `…/php-stubs/Core/Core.php`.
- After phpactor builds its index (at startup or after a reindex), **built 2026-11-20, not checked in the app**: when its "Indexing workspace" progress ends, VEGA sends `didSave` for each open file phpactor handles, and phpactor re-runs its diagnostics (`language_server.diagnostics_on_save`, on by default). Code: `php_settings::phpactor::recheck_open_files`. Owner check: open a file that uses a class from `vendor/`, run `php: reindex`; a "class not found" shown while indexing should clear by itself when indexing ends.
- Owner checks: cmd-click `__DIR__ . '/x.php'` (fixed today), WordPress include path in classified-listing (phpactor index verified to resolve `plugin_dir_path`).
- Restart VEGA Dev with `osascript -e 'tell application "VEGA Dev" to quit'`, never `pkill` (it orphans language servers; 25 phpactor processes piled up).
- Build only with `CARGO_INCREMENTAL=0` (`script/vega-dev-app` does it now); mixing modes filled `target/` to 168 GB, the owner ran `cargo clean`.

0. Owner's check of L1–L4 (table above); fix what it turns up. Open: keyboard navigation of the add-on rows (arrow keys, Enter) in the project panel; JavaScript › Libraries page; include paths/stubs in the file finder.
Deployment: PhpStorm parity P1–P6 built 2026-11-19 (DEPLOYMENT.md); owner is checking it in the app. Everything else: `vega/NEXT_PLAN.md` (phases, decisions needed).


1. Fix whatever the owner's app check turns up (D1, D2, D3, P06, P07).
2. D3 leftovers: done. Multi-select and recursive chmod came with P1–P6; copy between servers built 2026-11-20 (Copy or Cut in Remote Host, switch server in the dropdown, Paste: names are kept, a taken name stops the paste, Cut + Paste deletes the originals only when every file copied; `Engine::remote_copy_to_server`). Not checked in the app (needs two servers; the Docker SFTP test server can be added twice with different root paths).
3. D4 leftovers: Transfers list with Retry came with P1–P6 (`transfers_panel.rs`). Maybe an inline multi-file diff in the Sync view.
4. Branding leftovers: hidden Zed-cloud/collab/plan UI still says Zed (unreachable without sign-in). Done 2026-11-20: agent thread rating (thumbs up/down and `/helpful`, `/not-helpful`) is off, because it uploads the whole conversation to Zed's cloud (`submit_agent_feedback`; `is_thread_feedback_enabled` in `crates/agent_ui/src/conversation_view/thread_view.rs` returns false). The inline assistant's rating stays (it only writes a local telemetry event). The "Introducing Delta" announcement after an update is off (`SHOW_ZED_ANNOUNCEMENTS` in `crates/auto_update_ui`).
5. Periodic upstream sync (P14 checklist in `ROADMAP.md`).

## Waiting on the owner's decision

- **Markdown ⌘B:** in `.md` files ⌘B is now Toggle Bold (as in IntelliJ) instead of Toggle Left Dock. Keep, or move bold to another key.
- **Markdown default layout:** `markdown_preview.default_layout` ships as `editor` (no change from Zed). IntelliJ's default is `split`. Change VEGA's default or leave it.
- **AGENTS.md README banner rule:** upstream's `AGENTS.md` says to put a "remove this line to confirm you've reviewed this PR" banner at the top of `README.md` whenever source changes. It was never applied in this fork and was skipped again on 2026-11-20. Either delete the rule from `AGENTS.md` or say it should be followed.
- **P10 updates:** where VEGA downloads updates and the remote-server binary from (GitHub Releases needs a public repo or a separate public releases repo).
- **Extensions:** keep Zed's registry for now (user-initiated only) or plan a replacement catalogue.
- **P09:** which VEGA-specific AI features to build.
