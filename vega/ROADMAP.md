# VEGA roadmap

Status keys: `[ ]` todo · `[~]` in progress · `[x]` done. Each item maps to a prompt in `PROMPTS.md`.

## Phase 0 — Setup
- [x] Clone Zed into `~/dev/vega`, `upstream` remote, `vega` branch
- [x] `CLAUDE.md`, brand + roadmap docs, icon source SVG
- [x] Install full Xcode + Metal toolchain (manual, see `CLAUDE.md`)
- [x] First successful `cargo run` (P01) — binary is now `vega`
- [x] Create GitHub repo, add `origin`, push `vega` branch (P02) — private: github.com/mahbubur001/vega

## Phase 1 — Independent build
- [x] Rebrand app name, bundle IDs, URL scheme, icons (P03) — bundled .app check pending (P11)
- [x] Disable telemetry, crash upload, auto-update against zed.dev (P04) — release-asset lookups (incl. the SSH/WSL/Docker remote-server download) now refuse zed.dev too; P10 gives them a real host
- [x] Remove/hide Zed sign-in, Zed AI plan, collab UI (P05)
- [x] User-facing "Zed" strings → VEGA (welcome/onboarding, settings descriptions, CLI, updates, providers, extensions); VEGA mark replaces the Zed logo (`assets/images/zed_logo.svg`). Hidden Zed-cloud/collab/plan copy left as is
- [x] Project config in `.vega/` (settings, tasks, debug, deployment), `.zed/` ignored; `vega` CLI (`vega .`), installed by "cli: install" to `/usr/local/bin/vega`
- [~] Default "VEGA Dark" theme (P06) — `assets/themes/vega/vega.json`, default dark theme and first onboarding family; syntax colors ≥ WCAG AA on `#101018` (tested in `theme_settings`). Not yet looked at in the running app

## Phase 2 — AI-first
- [~] Onboarding: pick a BYOK provider (Anthropic, OpenAI, Ollama, …) (P07) — "AI Provider" section on the welcome page (`crates/onboarding/src/ai_provider_setup.rs`): Anthropic, OpenAI, Google, OpenRouter, Ollama, LM Studio; key saved to the OS keychain by the provider itself; Ollama/LM Studio use their own detection view; default-model picker writes `agent.default_model`. Not yet looked at in the running app (`onboarding: open`)
- [x] Sensible default agent + edit-prediction settings without Zed cloud (P08) — see "Changed defaults" below. Onboarding picking the provider is P07
- [ ] VEGA-specific AI features (P09)
- [ ] Settings Sync via GitHub sign-in + secret Gist — see `SETTINGS_SYNC.md`
  - [x] M1 Sign in with GitHub
  - [x] M2 Gist backend + whole-file sync (verified live)
  - [x] M3 Smart merge + never-sync rules
  - [x] M4 All resources, history, settings page — extensions list waits for the extension catalogue
- [ ] Deployment & Remote Host (built-in iRemote) — see `DEPLOYMENT.md`
  - [x] D0 Vendor the core (63 tests incl. real SFTP/FTP/WebDAV servers)
  - [~] D1 Upload on save + actions — built and unit-tested; waiting for a check in the running app (see `HANDOFF.md`)
  - [~] D2 Deployment Servers UI — dialog with Connection / Mappings / Excluded / Advanced tabs, settings-window link; not tried in the app yet
  - [~] D3 Remote Host panel — built (browse, open/edit on server with save-back, new/rename/delete, upload here, download to project, cut/copy/paste, chmod, exclude path, drag and drop from Finder and the project panel, keyboard navigation, remembered dock position); not tried in the app yet
  - [~] D4 Compare & Sync — folder-diff Sync view, File Transfers panel with Retry, remote-change warnings; PhpStorm parity plan P1–P6 done (DEPLOYMENT_PLAN.md). Owner's in-app check ongoing
  - [ ] D5 Extras (SSH, agent tools)
- [ ] External Libraries & Scratches (PhpStorm parity) — see `LIBRARIES_SCRATCHES_PLAN.md`
  - [~] L1 Scratches — "Scratches" node (PhpStorm: "Scratches and Consoles"; renamed 2026-11-21, VEGA has no consoles) at the bottom of the project panel (project-panel seam `crates/project_panel/src/extra_rows.rs`), crate `scratches`: New Scratch File (⇧⌘N, VEGA's built-in language picker, from selection), New Scratch Buffer, Show Scratch Files, Run (⌃⇧R, terminal), Change Language, Rename, Copy, Move to Project, Delete, New File/Directory, empty scratches deleted on close. Waiting for the owner's check
  - [~] L2 External Libraries — node above Scratches (crate `external_libraries`): PHP Runtime (interpreter version + extensions; optional download of JetBrains phpstorm-stubs, Apache-2.0, into the data folder), Composer packages from composer.lock, Node.js packages from package.json with installed versions, lazy folder browsing, Select Opened File for library files, Refresh. Library files (vendor, node_modules, stubs) open read-only with a "Non-project file" bar (Edit This File / Files in This Package / Any Library File, for the session). Waiting for the owner's check
  - [~] L3 Settings › PHP — crate `php_settings`, own resizable window (`php: open php settings`, Settings › Languages & Tools › PHP, right-click External Libraries / PHP Runtime): language level (Auto from composer.json or the interpreter), CLI interpreter (Homebrew, Herd, MAMP, system, or browse), Include Path list, PHP Runtime stubs (choose extensions, sync with interpreter, download), Composer packages as libraries. Saved in `.vega/php.json`; language-server keys written to `.vega/settings.json` (Intelephense includePaths/phpVersion/stubs, phpactor php.version/indexer.stub_paths). External Libraries shows include paths and follows the settings; scratches run with the chosen interpreter. Waiting for the owner's check
  - [~] L4 Scopes — "Project and Libraries": Find in Files with *include ignored* on also searches the include paths and PHP stubs (held as hidden library worktrees; seam `crates/project/src/library_worktrees.rs` + `Search::add_worktrees`); vendor/ and node_modules/ were already covered by that option. Not done: a JavaScript › Libraries page and include paths/stubs in the file finder
- [ ] Markdown view, edit and preview (IntelliJ parity) — see `reports/IntelliJ Markdown view modes.md`; crates `markdown_layout` and `markdown_editing`
  - [~] M1 Layout switcher — Editor Only / Editor and Preview / Preview Only buttons on Markdown editor and preview tabs, plus `markdown_layout: show …` actions; editor and preview stay separate tabs. Built and tested 2026-11-20; waiting for a check in the running app (see `HANDOFF.md`)
  - [~] M2 Default layout — `markdown_preview.default_layout`: `editor` (default), `split`, `preview`; applied once when a file is first opened. Built and tested; not checked in the app
  - [~] M3 Two-way scroll sync — editor and preview keep the same source line at the top; off switches `scroll_preview_with_editor` and `scroll_editor_with_preview`; renderer seam `crates/markdown/src/scroll_sync.rs`. Built and tested; not checked in the app
  - [~] M4 Editing commands — bold, italic, strikethrough, code, insert link, URL paste as link, table Tab / Shift-Tab / Shift-Enter (`markdown_editing`). Built and tested; not checked in the app
  - [ ] M5 Leftovers — preview below the editor, per-file layout memory, one shared preview in the split, selection toolbar, table column commands, table of contents
  - [ ] M6 Math and more HTML tags — wait for upstream

## Phase 3 — Ship
- [ ] Own update server / GitHub Releases auto-update (P10) — must also serve `zed-remote-server` for SSH/WSL/Docker projects (`AutoUpdater::get_release_asset`), release notes (`auto_update_ui`, menu item removed), and the Linux build the WSL sandbox helper downloads (`sandbox::windows_wsl::PROVISION_HELPER_FROM_ZED_CLOUD`)
- [ ] Signed + notarized macOS bundle (P11)
- [~] CI (P12): Linux only for now (`.github/workflows/vega_ci.yml`: fmt + tests of the VEGA crates on `ubuntu-latest`). macOS/Windows CI deferred: 10x/2x minutes on a private repo
- [~] Website, docs, README rewrite (P13) — README rewritten; Settings Sync user docs added
  - [x] MCP OAuth no longer offers Zed's CIMD document (`zed.dev/oauth/client-metadata.json`): `VEGA_CIMD_URL = None` in `crates/context_server/src/oauth.rs`, so VEGA registers via DCR or a pre-registered `client_id`
  - [ ] Optional: host VEGA's own client metadata document and set `VEGA_CIMD_URL` (needed only for CIMD-only auth servers)
  - [x] In-app `zed.dev/docs` links open VEGA's own docs: `gpui::OpenUrlRewriter` hook in `App::open_url`, mapping in `crates/release_channel/src/vega_docs.rs` (pages from `docs/src/SUMMARY.md` + `book.toml` redirects; base URL is one constant, currently the GitHub repo, so links work for others once the repo or docs are public)
  - [x] Every other zed.dev link VEGA opens goes to VEGA's repo (`/releases` → VEGA releases): `rewrite_zed_url` in the same hook. Help menu: feedback and bug reports go to VEGA's GitHub issues; Zed's email, Twitter and jobs items removed
  - [ ] Branding sweep of user-facing "Zed" strings (e.g. "Update Zed" error actions)
- [ ] Extension catalogue without zed.dev — the Extensions page and installs still use `api.zed.dev` (`extension_host::fetch_extensions_from_api`, `install_latest_extension`); only user-initiated, never on startup

## Changed defaults

Every default VEGA changes from Zed (`assets/settings/default.json` unless noted):

| Setting | Zed | VEGA | Why | Item |
|---|---|---|---|---|
| `telemetry.diagnostics` | `true` | `false` | Never send crash reports to zed.dev | P04 |
| `telemetry.metrics` | `true` | `false` | Never send telemetry to zed.dev | P04 |
| `auto_update` | `true` | `false` | No update server until P10 | P04 |
| `collaboration_panel.button` | `true` | `false` | Zed's collab server isn't used | P05 |
| `agent.default_model` | `zed.dev` / `claude-sonnet-4` | `anthropic` / `claude-sonnet-5-5` | Bring your own key. If the user hasn't picked a default and Anthropic has no key, the first provider with a key is used (`agent_ui::built_in_default_or_fallback`) | P05, P08 |
| `agent.enable_feedback` | `true` | `false` | Feedback went to zed.dev | P05 |
| `edit_predictions.provider` | `zed` | `none` | Zed's predictions need a zed.dev account; pick a BYOK provider (or Copilot) in the status bar | P05, P08 |
| `auto_install_extensions` | `{ "html": true }` | `{}` | No registry calls on startup | P05 |
| `theme.dark` | `One Dark` | `VEGA Dark` | Brand default (also `settings_content::DEFAULT_DARK_THEME`) | P06 |

Changed default key (`assets/keymaps/default-*.json`): in `.md` files ⌘B / Ctrl-B is `markdown_editing::ToggleBold` instead of `workspace::ToggleLeftDock` (M4; owner's decision pending, see `HANDOFF.md`). ⌘I, ⌘V, Tab, Shift-Tab and Shift-Enter also go to `markdown_editing` first in `.md` files and fall through to their usual action when the command does not apply.

## Ongoing
- [ ] Weekly upstream sync (P14) — last: 2026-11-19, upstream `dd091a1b4f` (22 commits, no conflicts). Check after each merge: new hard-coded `.zed` project paths (should use `paths::local_settings_folder_name()` / `.vega`), VEGA guards in `client.rs` (`ZED_ACCOUNTS_ENABLED`), `telemetry.rs` (`UPLOAD_EVENTS`), `auto_update.rs` (`is_zed_hosted`), and no new `build_zed_*_url`/zed.dev calls in code. Test `workspace` on its own (combined test runs enable `remote/test-support` and fail to compile)
