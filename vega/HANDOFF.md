# Resume here

The restart point for the next working session, possibly on another computer. Read this first, then `ROADMAP.md`. Update it at the end of every session.

_Last updated: 2026-11-19, end of day. Branch `vega`, everything committed and pushed._

## How the owner wants to work

- Keep going task by task without asking "should I go ahead?": finish, test, commit, push, then take the next recommended task. Ask only for decisions that are genuinely the owner's (product direction, money, publishing).
- Items that need a look in the running app stay "on hold" until the owner checks them or sends screenshots.
- Never add Claude co-author or attribution lines to commits.

## Setting up a new computer

1. `git clone https://github.com/mahbubur001/vega && cd vega && git checkout vega`, then `git remote add upstream https://github.com/zed-industries/zed`.
2. Rust via rustup (the toolchain comes from `rust-toolchain.toml`), `brew install cmake`.
3. With full Xcode, plain `cargo run` works. Without Xcode, add `--features gpui_platform/runtime_shaders` to every build, check, test and clippy command that compiles the app (for crate tests: `cargo test -p <crate> -p gpui_platform --features gpui_platform/runtime_shaders`).
4. Run the app as "VEGA Dev.app" so the Dock shows the VEGA icon: `script/vega-dev-app --build ~/some/test/project` (it picks runtime shaders itself when Xcode is missing). Don't open this repo itself in VEGA.
5. Code graph: `graphify update .` once after cloning (`graph.json` and the cache are not in git).
6. Disk: the debug build cache grows by tens of GB per session. When space runs low, `rm -r target/debug/incremental` (safe; it rebuilds).

## Where things stand

**Newest (2026-11-19, night): PhpStorm External Libraries & Scratches, L1–L4 built** (plan `LIBRARIES_SCRATCHES_PLAN.md`, option A: nodes inside the project panel through a small seam, `crates/project_panel/src/extra_rows.rs`). Crates: `scratches` (Scratches and Consoles: ⇧⌘N New Scratch File with VEGA's built-in language picker, New Scratch Buffer, Show Scratch Files, ⌃⇧R Run, right-click Change Language/Rename/Copy/Move to Project/Delete; checked in the app: create, run, delete-on-close), `external_libraries` (PHP Runtime, Composer, Node.js, include paths; read-only library files with a "Non-project file" bar; optional phpstorm-stubs download), `php_settings` (PHP settings window, `.vega/php.json`, language-server keys in `.vega/settings.json`), `vega_dialog` (shared dialog look, moved out of deployment_ui). L4: Find in Files with *include ignored* also searches include paths and stubs (`crates/project/src/library_worktrees.rs`). Test project with real packages: `~/dev/vega-libs-test` (composer + npm installed).

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

Docker SFTP test server: `crates/deployment/tests/servers` (see `DEPLOYMENT.md` "Running the server tests"; generate `fixtures/id_ed25519` first on a new machine). Test login `tester` / `pass` on `localhost:2222`.

## Next tasks, in order

**Start here next session:**
- Go-to-definition on PHP built-ins (`defined`, `die`…) fails: phpactor answers with paths inside `phpactor.phar`. Fix: when stubs are downloaded, write `worse_reflection.stub_dir` = the stubs folder into phpactor's init options (`php_settings::config::settings_with_lsp_keys`), maybe clear phpactor's stub cache.
- After a reindex, ask phpactor to re-check open files (diagnostics from before the index finished stay until the file changes).
- Owner checks: cmd-click `__DIR__ . '/x.php'` (fixed today), WordPress include path in classified-listing (phpactor index verified to resolve `plugin_dir_path`).
- Restart VEGA Dev with `osascript -e 'tell application "VEGA Dev" to quit'`, never `pkill` (it orphans language servers; 25 phpactor processes piled up).
- Build only with `CARGO_INCREMENTAL=0` (`script/vega-dev-app` does it now); mixing modes filled `target/` to 168 GB, the owner ran `cargo clean`.

0. Owner's check of L1–L4 (table above); fix what it turns up. Open: keyboard navigation of the add-on rows (arrow keys, Enter) in the project panel; JavaScript › Libraries page; include paths/stubs in the file finder.
Deployment: PhpStorm parity P1–P6 built 2026-11-19 (DEPLOYMENT.md); owner is checking it in the app. Everything else: `vega/NEXT_PLAN.md` (phases, decisions needed).


1. Fix whatever the owner's app check turns up (D1, D2, D3, P06, P07).
2. D3 leftovers (small): multi-select, recursive chmod, copy between servers.
3. D4 leftovers: Transfers list with retry; maybe an inline multi-file diff in the Sync view.
4. Branding leftovers: hidden Zed-cloud/collab/plan UI still says Zed (unreachable without sign-in); agent thumbs-up/down text says it "sends the conversation to the Zed team" — check whether it sends anything (telemetry upload is off) and remove or reword.
5. Periodic upstream sync (P14 checklist in `ROADMAP.md`).

## Waiting on the owner's decision

- **P10 updates:** where VEGA downloads updates and the remote-server binary from (GitHub Releases needs a public repo or a separate public releases repo).
- **Extensions:** keep Zed's registry for now (user-initiated only) or plan a replacement catalogue.
- **P09:** which VEGA-specific AI features to build.
- **Upstream workflows:** 7 Zed workflows still registered on the repo (all skipped). Disable with `gh api repos/mahbubur001/vega/actions/workflows --jq '.workflows[] | select(.state=="active") | .id' | xargs -n1 gh workflow disable -R mahbubur001/vega`.
