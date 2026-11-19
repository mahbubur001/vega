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

Done and pushed today: D4 Sync with Deployed view (`deployment: sync with deployed`, see `DEPLOYMENT.md`); D3 second pass (cut/copy/paste, chmod, Exclude Path, drag and drop, keyboard navigation, remembered dock position; see `DEPLOYMENT.md`). Earlier: Linux-only CI (`.github/workflows/vega_ci.yml`, green, about 22 minutes uncached), MCP OAuth without Zed's CIMD document, every zed.dev link VEGA opens now goes to VEGA's docs or repo (`crates/release_channel/src/vega_docs.rs` + `gpui::OpenUrlRewriter`), Help menu feedback to VEGA's GitHub, Deployment Servers dialog Advanced tab and settings link, `script/vega-dev-app`, and the first version of the **Remote Host panel (D3)**.

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
| App icon | Launch with `script/vega-dev-app` and look at the Dock |

Docker SFTP test server: `crates/deployment/tests/servers` (see `DEPLOYMENT.md` "Running the server tests"; generate `fixtures/id_ed25519` first on a new machine). Test login `tester` / `pass` on `localhost:2222`.

## Next tasks, in order

1. Fix whatever the owner's app check turns up (D1, D2, D3, P06, P07).
2. D3 leftovers (small): multi-select, recursive chmod, copy between servers.
3. D4 leftovers: Transfers list with retry; maybe an inline multi-file diff in the Sync view.
4. Branding sweep of user-facing "Zed" strings (e.g. "Update Zed" error actions), keeping internal crate names.
5. Periodic upstream sync (P14 checklist in `ROADMAP.md`).

## Waiting on the owner's decision

- **P10 updates:** where VEGA downloads updates and the remote-server binary from (GitHub Releases needs a public repo or a separate public releases repo).
- **Extensions:** keep Zed's registry for now (user-initiated only) or plan a replacement catalogue.
- **P09:** which VEGA-specific AI features to build.
- **Upstream workflows:** 7 Zed workflows still registered on the repo (all skipped). Disable with `gh api repos/mahbubur001/vega/actions/workflows --jq '.workflows[] | select(.state=="active") | .id' | xargs -n1 gh workflow disable -R mahbubur001/vega`.
