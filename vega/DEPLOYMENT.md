# Deployment & Remote Host — plan

Bring iRemote (`~/dev/toolsmith`, MIT) into VEGA as a built-in feature: PhpStorm-style **Deployment** (SFTP, FTP/FTPS, WebDAV, local servers with path mappings, upload on save, compare and sync) and a **Remote Host** panel to browse and edit files on the server.

Built in, it no longer needs iRemote's workarounds for Zed's extension limits (fake language server, menu-bar icon, separate windows over a socket, MCP shim). Everything becomes real VEGA panels, actions, menus and settings.

## Naming

Zed already uses "Remote Development", "Remote Project" and "Remote Servers" for SSH projects (`crates/remote`, which proxies the whole project over RPC and doesn't sync files). To avoid confusion, this feature is called **Deployment**, with the **Remote Host** panel, **Deployment Servers**, and the `deployment::` action namespace.

## What we reuse

| iRemote part | Lines | In VEGA |
|---|---|---|
| `iremote-core` (config, protocols, pool, queue, mapper, deployer, sync, state, known_hosts, TLS) | ~9.9k | **Vendored** as `crates/deployment`, ~90% unchanged |
| `iremote-app` (gpui-kit windows) | ~10k | **Rewritten** on VEGA's `ui` crate; settings model and tree logic reused |
| `iremote-lsp`, IPC, dialog subprocess, tray, `zed` CLI shell-outs, WASM extension | ~3.5k | **Dropped**, replaced by direct hooks |

The `RemoteClient` protocol trait, the `Engine` facade and the async `Prompter` trait (secret, host key, certificate, conflict, confirm, diff, notify) map cleanly onto VEGA. Its tokio code runs on VEGA's `gpui_tokio` runtime.

## Architecture

```
crates/deployment      (no UI)   vendored iremote-core: config, protocols, Engine, queue, sync
crates/deployment_ui   (GPUI)    Remote Host panel, transfers status item, server settings,
                                 Prompter on VEGA modals, upload-on-save, compare/sync views
```

- **Engine per worktree.** A `DeploymentStore` entity per workspace holds one `Engine` per local worktree root. SSH/remote projects are out of scope at first.
- **Upload on save.** Subscribe to `BufferStore` → `BufferEvent::Saved`, then call `Engine::on_saved`. Worktree entry updates replace iRemote's own file watcher for external changes, deletes and renames.
- **Config.** Same schema as iRemote, in VEGA's own files: `.zed/deployment.json` per project and shared servers in `~/.config/vega/deployment/servers.json`. A project's existing `.zed/iremote.json` is imported once (the original is left untouched).
- **Secrets.** In the OS keychain only, never in JSON or logs. See decision 2.
- **Security.** Unchanged from iRemote: `known_hosts` checks with strict-host-key modes, TLS verified against the system store, explicit fingerprint trust.

## UI

| Feature | Where in VEGA |
|---|---|
| Deployment Servers (Connection / Mappings / Excluded Paths, default ★, Test Connection, options) | Settings window link + a servers editor tab |
| Remote Host browser (open, edit on server, create, rename, delete, chmod, copy/paste, upload here) | **Remote Host** dock panel (tree like the project panel) |
| Drag files from the project panel or Finder onto the server | Drop targets on the Remote Host panel |
| Upload to… / Download from… / Compare with Deployed Version | Project panel, editor and tab context menus, command palette, keybindings |
| Compare with Deployed Version | Built-in side-by-side diff (`FileDiffView`), editable |
| Sync with Deployed (compare modes, per-file direction, mirror) | Sync view with a multi-file diff preview |
| Transfers with progress, retry, cancel | Status-bar indicator + Transfers list |
| Password, host-key, certificate, overwrite prompts | VEGA modals and notifications |
| Remote changes noticed | Notifications |
| SSH session / install public key | VEGA terminal |
| Agent tools (list, upload, download) | Later, as built-in agent tools |

## Milestones

| | Milestone | Done when |
|---|---|---|
| D0 | **Vendor the core.** `crates/deployment`: workspace deps and lints, drop IPC/dialog/tray/CLI shims, run on `gpui_tokio`, keep its 65 tests (Docker integration tests behind a feature) | `cargo test -p deployment` passes |
| D1 | **Upload on save + actions.** `DeploymentStore`, save hook, `Prompter` on VEGA modals, transfers status item, Upload/Download/Compare actions and menu entries. Config edited as JSON (with schema) for now | Saving a file uploads it to a Docker SFTP server |
| D2 | **Deployment Servers UI.** Server list + Connection / Mappings / Excluded tabs, Test Connection, keychain passwords, options | Configure a server with no JSON editing |
| D3 | **Remote Host panel.** Browse, edit on server with save-back, file operations, drag and drop | Edit a file on the server from the panel |
| D4 | **Compare & Sync.** Compare with Deployed (diff view), Sync view, Transfers list with retry | Sync a folder both ways with a preview |
| D5 | **Extras.** SSH session in the terminal, ssh-copy-id, upload changed files (git), agent tools, docs | |

D0–D1 is the smallest useful slice: upload on save and the menu actions working with existing `.zed/iremote.json` files.

## Status

- **D0 done (2026-11-18).** `crates/deployment` holds iremote-core, renamed and trimmed: no IPC, dialog subprocess, tray, Zed CLI shell-outs or own file watcher (`Engine::on_fs_event` takes VEGA's events instead). Uses VEGA's library versions (zed reqwest fork, rustls with aws-lc-rs, sha/hmac 0.10/0.12, `log`); paths live in VEGA's config/data dirs (`VEGA_DEPLOYMENT_HOME` overrides them). All 63 tests pass, including the Docker server tests for SFTP, FTP/FTPS and WebDAV. Secrets moved to VEGA's credentials provider in D1.

- **D1 in progress (2026-11-18): builds, not tried in the running app yet.**
  - `zed_actions::deployment`: actions `UploadToDefaultServer`, `DownloadFromDefaultServer`, `CompareWithDeployedVersion`, `OpenDeploymentConfig`, `CancelTransfers`, the `ConfiguredRoots` global, and `menu_items` (the PhpStorm-style submenu entries, tested).
  - `crates/deployment_ui/src/`: `deployment_ui.rs` (init, actions, prompts, starter config), `store.rs` (engine per worktree, upload on save, worktree change events, transfer status), `prompter.rs` (engine questions → VEGA dialogs), `secret_modal.rs` (password dialog), `transfers_indicator.rs` (status bar). Wired into the workspace and `crates/zed/src/main.rs`; check, clippy and `script/check-licenses` pass.
  - Secrets (decision 2): `deployment::credentials::Credentials` persists through a `SecretStore`. `deployment_ui/src/secrets.rs` backs it with VEGA's credentials provider (keychain entries `vega-deployment:<account>`; the development provider in dev builds), reached through a foreground task. When VEGA has no entry, iRemote's `iremote` keychain entry is read once, copied over and recorded in `state/imported-iremote-secrets.json` (account names only), so a password later deleted as wrong never comes back. iRemote's entries are left untouched. Tested with fake stores and upstream's `FakeCredentialsProvider`.
  - "Deployment" context submenu (Upload to Default Server, Download from Default Server, Compare with Deployed Version, Configuration…) in the project panel, editor and tab menus, shown only inside a configured project root. Built once in `crates/workspace/src/deployment_menu.rs` (`DeploymentMenuExt`); each upstream menu calls it in one place. The tab menu only offers it on the active tab, because the actions act on the active item.

- **D2 built (2026-11-19), not tried in the app yet.** `deployment: open deployment servers` (and Deployment > Configuration… in the context menus) opens a PhpStorm-style dialog (`deployment_ui/src/servers_modal.rs`): server list with ★ default, Add (SFTP, FTP, FTPS explicit/implicit, WebDAV, Local) and Remove, then Connection (name, type, host, port, user, SFTP auth method, key path, password, root path, web URL), Mappings (local / remote / web rows) and Excluded Paths (one per line) for the selected server. Test Connection runs the engine's step-by-step check; Save writes through `ConfigStore` (the store reloads the engine) and puts a typed password in the keychain. Edits are a draft until Save; `Draft::apply` is tested. The JSON file is still available via `deployment: open deployment config`. Not in the dialog yet: groups, "in place" servers, the less common options (jump host, host-key policy, FTP passive/LIST, TLS), and a link from the settings window.

### Next session

1. Check disk space first (`df -h /`); `~/dev/vega/target` filled the disk once. `rm -r target/debug/incremental` frees most of it.
2. Try D1 and D2 in the app. Without Xcode, run with `cargo run --features gpui_platform/runtime_shaders -- ~/some/test/project`. Start the Docker servers (`docker compose -f crates/deployment/tests/servers/compose.yml up`), run `deployment: open deployment config`, point it at SFTP (`localhost:2222`, `tester`/`pass`), save a file, check the upload, the Deployment submenu (project panel, editor, tab), Compare and the transfers indicator. Then open Deployment > Configuration…, add a second server, Test Connection, Save, and check `.zed/deployment.json`.
3. Mark D1 done once step 2 works. Known issue: `cargo clippy -p deployment --tests --features integration` flags `std::process::Command::output` in `tests/sftp.rs` (pre-existing; normal clippy doesn't build that test).

### Running the server tests

```sh
cd crates/deployment/tests/servers
[ -f fixtures/id_ed25519 ] || ssh-keygen -q -t ed25519 -N '' -f fixtures/id_ed25519
docker compose up -d --build --wait
cd - && cargo test -p deployment --features integration
docker compose -f crates/deployment/tests/servers/compose.yml down -v
```

## Upstream seams (kept small for merges)

`Cargo.toml`, `crates/zed/{Cargo.toml, src/main.rs, src/zed.rs, src/zed/app_menus.rs}` (init, panel and status item registration, menu), one `.when(…)` block each in the project panel, editor and tab context menus, one `ActionLink` in `settings_ui`, and panel settings in `settings_content` + `default.json`. Everything else lives in the two new crates.

## Risks

- **Dependency weight.** russh (a second SSH stack with newer RustCrypto crates), suppaftp and possibly reqwest 0.13 increase build time and binary size. Mitigation: move WebDAV to VEGA's `http_client`; align sha/dirs versions.
- **Runtime.** The core assumes it runs inside tokio (`tokio::spawn`, `Handle::current`, `spawn_blocking`). All entry points must go through `gpui_tokio`, whose 2-worker pool may need more threads. Don't hold `Engine::config()` locks across `.await`.
- **Multi-root and remote projects.** One engine per local worktree; SSH/WSL projects are excluded at first.
- **UI rewrite (~10k lines).** The largest cost. iRemote's UI was never visually reviewed, so treat it as a reference, not a spec.
- **Tests.** No UI tests yet; integration tests need Docker (`tests/servers/compose.yml`). rsync is an unused stub in iRemote; it stays out.

## Decisions (owner, 2026-11-18)

1. **Config:** new VEGA files (`.zed/deployment.json`, `~/.config/vega/deployment/servers.json`), with a one-time import of `.zed/iremote.json`.
2. **Passwords:** VEGA's credentials provider, reading iRemote's `iremote` keychain entries once as a fallback.
3. **Source:** iremote-core is vendored into `crates/deployment` and evolves in VEGA.
4. **First slice:** D0–D1.
