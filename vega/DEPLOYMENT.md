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
- **Config.** Same schema as iRemote, in VEGA's own files: `.vega/deployment.json` per project and shared servers in `~/.config/vega/deployment/servers.json`. `.zed/` is ignored (owner, 2026-11-19), so iRemote's `.zed/iremote.json` is no longer imported.
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

D0–D1 is the smallest useful slice: upload on save and the menu actions working with a `.vega/deployment.json` file.

## Status

- **D0 done (2026-11-18).** `crates/deployment` holds iremote-core, renamed and trimmed: no IPC, dialog subprocess, tray, Zed CLI shell-outs or own file watcher (`Engine::on_fs_event` takes VEGA's events instead). Uses VEGA's library versions (zed reqwest fork, rustls with aws-lc-rs, sha/hmac 0.10/0.12, `log`); paths live in VEGA's config/data dirs (`VEGA_DEPLOYMENT_HOME` overrides them). All 63 tests pass, including the Docker server tests for SFTP, FTP/FTPS and WebDAV. Secrets moved to VEGA's credentials provider in D1.

- **D1 in progress (2026-11-18): builds, not tried in the running app yet.**
  - `zed_actions::deployment`: actions `UploadToDefaultServer`, `DownloadFromDefaultServer`, `CompareWithDeployedVersion`, `OpenDeploymentConfig`, `CancelTransfers`, the `ConfiguredRoots` global, and `menu_items` (the PhpStorm-style submenu entries, tested).
  - `crates/deployment_ui/src/`: `deployment_ui.rs` (init, actions, prompts, starter config), `store.rs` (engine per worktree, upload on save, worktree change events, transfer status), `prompter.rs` (engine questions → VEGA dialogs), `secret_modal.rs` (password dialog), `transfers_indicator.rs` (status bar). Wired into the workspace and `crates/zed/src/main.rs`; check, clippy and `script/check-licenses` pass.
  - Secrets (decision 2): `deployment::credentials::Credentials` persists through a `SecretStore`. `deployment_ui/src/secrets.rs` backs it with VEGA's credentials provider (keychain entries `vega-deployment:<account>`; the development provider in dev builds), reached through a foreground task. When VEGA has no entry, iRemote's `iremote` keychain entry is read once, copied over and recorded in `state/imported-iremote-secrets.json` (account names only), so a password later deleted as wrong never comes back. iRemote's entries are left untouched. Tested with fake stores and upstream's `FakeCredentialsProvider`.
  - "Deployment" context submenu (Upload to Default Server, Download from Default Server, Compare with Deployed Version, Configuration…) in the project panel, editor and tab menus, shown only inside a configured project root. Built once in `crates/workspace/src/deployment_menu.rs` (`DeploymentMenuExt`); each upstream menu calls it in one place. The tab menu only offers it on the active tab, because the actions act on the active item.

- **D2 built (2026-11-19), not tried in the app yet.** `deployment: open deployment servers` (and Deployment > Configuration… in the context menus) opens a PhpStorm-style dialog (`deployment_ui/src/servers_modal.rs`): server list with ★ default, Add (SFTP, FTP, FTPS explicit/implicit, WebDAV, Local) and Remove, then Connection (name, type, host, port, user, SFTP auth method, key path, password, root path, web URL), Mappings (local / remote / web rows) and Excluded Paths (one per line) for the selected server. Test Connection runs the engine's step-by-step check; Save writes through `ConfigStore` (the store reloads the engine) and puts a typed password in the keychain. Edits are a draft until Save; `Draft::apply` is tested. The JSON file is still available via `deployment: open deployment config`. An Advanced tab shows the options for the selected type: host-key checking, SSH client, jump host and keep-alive (SFTP); anonymous login, LIST, file name encoding and keep-alive command (FTP); data protection (FTPS); authentication (WebDAV); TLS 1.3, untrusted certificates and a trusted fingerprint (FTPS, WebDAV); max connections. Not in the dialog yet: groups, "in place" servers, and the stored-but-unused options (FTP active mode, rsync), which the engine doesn't apply yet. Settings > General > Deployment has an "Open Servers" link to the dialog.

- **D3 first version built (2026-11-19), not tried in the app yet.** The **Remote Host** dock panel (`deployment_ui/src/remote_host_panel.rs`, Server icon, right dock, `deployment: toggle remote host`) appears once the project has a deployment server. A dropdown picks the server (with "Deployment Servers…"), a refresh button reloads the open folders, and the tree loads folders lazily from `Engine::remote_list`, starting at the first mapping's remote root. Clicking a file opens it via `Engine::remote_open`: the mapped local file when there is one, otherwise a cached server copy, and saving that copy uploads it back (`DeploymentStore::engine_for_remote_edit` routes the save, since the copy lives outside the project). Right-click: Open, Open Server Copy and Download to Project (mapped files), Upload Here… (file picker), New File…, New Folder…, Rename… (`name_modal.rs`), Delete (the engine asks first), Copy Path, Refresh. Excluded paths are dimmed.
- **D3 second pass (2026-11-19), not tried in the app yet.** Everything in the panel is now a `remote_host::*` action (shown with its shortcut in the right-click menu): Cut / Copy / Paste (copies get a free name such as `index copy.php`, moves refuse to overwrite; `Engine::remote_copy` now takes source → destination pairs and copies empty folders too), Change Permissions… (octal chmod via `NameModal::with_validator`), Exclude Path / Include Path (`Engine::exclude_remote`), and keyboard navigation (up/down/home/end, enter opens, left/right collapse/expand; the `RemoteHostPanel` keymap blocks also bind new file/folder, cut/copy/paste, copy path, F2 rename, delete). Drag and drop: rows can be dragged onto a folder (or the blank area = root) to move them; files from Finder (`ExternalPaths`) and from the project panel (`DraggedSelection`) are uploaded into the folder they're dropped on. The dock position is remembered in the key-value store. Multi-select and recursive chmod came with P1–P6 below. Copying between servers (2026-11-20): Copy or Cut on one server, switch the dropdown to another, Paste; items keep their names, a taken name stops the paste (nothing is overwritten), and a Cut is a move that deletes the originals only when every file copied (`Engine::remote_copy_to_server`, `cross_server_plan`).
- **Remote Host PhpStorm layout (2026-11-19).** Header: server dropdown (type icon per server), "…" (Deployment Servers), Collapse All, Refresh, Disconnect, Options. The tree starts with a root node "Server (host)". Options menu (remembered): Show Size / Show Date / Show Permissions, Highlight Symlinks (accent) / Highlight Mappings (green). Files use the project panel's file-type icons; sizes are "40 byte" / "8.6 kB", dates "d/m/yy, h:mm AM" in local time, permissions "rwxr-xr-x".
- **PhpStorm look (2026-11-19).** Deployment dialog: + (type menu with icons) / − / ✓ default toolbar over the server list (type icons, default in bold); tabs Connection, Mappings, Excluded Paths with labels on the left; "Visible only for this project" (= not shared), Host + Port on one row, "Login as anonymous" (FTP), "Password saved permanently · Reset" (keychain), Test Connection in the form, Root path + Autodetect (`Engine::detect_root`), Web URL + open button, collapsible Advanced; Mappings with folder picker and a missing-path warning; Excluded Paths as a list (+ Local / Deployment). Remote Host context menu: New ▸ File/Folder, Change Permissions… (Items Permissions dialog: R/W/X grid + octal for files and folders, optionally recursive via `Engine::remote_chmod_tree`), Rename… (⇧F6), Delete…, Cut/Copy/Paste, Copy Absolute Path (⇧⌘C).
- **D4 Sync with Deployed built (2026-11-19), not tried in the app yet.** `deployment: sync with deployed` (also "Sync with Deployed…" in the Deployment context submenu) opens a **Sync** tab (`deployment_ui/src/sync_view.rs`) for the selected file or folder, or the whole project when nothing (or the project root) is selected. It runs `Engine::sync_compare` and lists every file that differs (state: only in project / only on server / changed in project / changed on server / changed on both sides; "Show identical" adds the rest). Each row has a direction button that cycles through the actions its state allows (Upload, Download, Delete on Server, Delete in Project, Skip) and a Compare button that opens the side-by-side diff (`Engine::compare` → `FileDiffView`). The toolbar picks the server and the compare mode (changes since last transfer, size and timestamp, size, content, content ignoring line endings), sets every direction at once (suggested, upload all, download all, mirror project → server, mirror server → project, skip all), and Synchronize runs `Engine::sync_execute` (which asks before deleting) and compares again. The diff is one file at a time in its own tab, not an inline multi-file preview. Still to do: a Transfers list with retry (the status-bar indicator shows progress only).

- **PhpStorm parity P1–P6 (2026-11-19, see DEPLOYMENT_PLAN.md).** Deployment is its own resizable window (`deployment_window.rs`, size remembered, Esc/⌘W) with a sidebar, server header, underline tabs, sectioned form, Options page, groups and In place servers, Apply/Duplicate/Rename and a server context menu; dialogs share `dialog.rs` (theme-aware solid primary button, field-styled combo boxes). Tools › Deployment menu and "…To…" server pickers (`server_picker.rs`), default server in the status bar, ⌥⌘U / ⌥⇧⌘U. Remote Host matches the project panel (chevrons, file icons, indent guides from ProjectPanelSettings), multi-select, Edit Remote/Local File, Compare/Sync with Local, Download from Here, Open in Browser, SSH session, Select Opened File. Sync with Deployed is a folder diff with filters, an action column and an inline `FileDiffView`. File Transfers panel (`transfers_panel.rs`, bottom dock): progress, Retry, Cancel per transfer (`TransferQueue::cancel_task`), log. Server copies get a bar (`remote_edit_bar.rs`): Upload, Compare, Reload from Server, warning when changed on the server. Opening a project file that changed on the default server since the last transfer shows a warning (`Engine::deployed_copy_changed`). Password prompts are a small movable window. Fixed: empty deployment path mapped to `/` instead of the root path; the dialog dropped empty (root) mappings on save; simultaneous password prompts closed each other.

### Next session

1. Check disk space first (`df -h /`); `~/dev/vega/target` filled the disk once. `rm -r target/debug/incremental` frees most of it.
2. Try D1 and D2 in the app. Without Xcode, run with `cargo run --features gpui_platform/runtime_shaders -- ~/some/test/project`. Start the Docker servers (`docker compose -f crates/deployment/tests/servers/compose.yml up`), run `deployment: open deployment config`, point it at SFTP (`localhost:2222`, `tester`/`pass`), save a file, check the upload, the Deployment submenu (project panel, editor, tab), Compare and the transfers indicator. Then open Deployment > Configuration…, add a second server, Test Connection, Save, and check `.vega/deployment.json`.
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

`Cargo.toml`, `crates/zed/{Cargo.toml, src/main.rs, src/zed.rs, src/zed/app_menus.rs}` (init, panel and status item registration, menu), one `.when(…)` block each in the project panel, editor and tab context menus, one `ActionLink` in `settings_ui`, and one `RemoteHostPanel` block in each `assets/keymaps/default-{macos,linux,windows}.json`. Everything else lives in the two new crates.

## Risks

- **Dependency weight.** russh (a second SSH stack with newer RustCrypto crates), suppaftp and possibly reqwest 0.13 increase build time and binary size. Mitigation: move WebDAV to VEGA's `http_client`; align sha/dirs versions.
- **Runtime.** The core assumes it runs inside tokio (`tokio::spawn`, `Handle::current`, `spawn_blocking`). All entry points must go through `gpui_tokio`, whose 2-worker pool may need more threads. Don't hold `Engine::config()` locks across `.await`.
- **Multi-root and remote projects.** One engine per local worktree; SSH/WSL projects are excluded at first.
- **UI rewrite (~10k lines).** The largest cost. iRemote's UI was never visually reviewed, so treat it as a reference, not a spec.
- **Tests.** No UI tests yet; integration tests need Docker (`tests/servers/compose.yml`). rsync is an unused stub in iRemote; it stays out.

## Decisions (owner, 2026-11-18)

1. **Config:** new VEGA files (`.vega/deployment.json`, `~/.config/vega/deployment/servers.json`). Originally with a one-time import of `.zed/iremote.json`; dropped on 2026-11-19 when all project config moved to `.vega/`.
2. **Passwords:** VEGA's credentials provider, reading iRemote's `iremote` keychain entries once as a fallback.
3. **Source:** iremote-core is vendored into `crates/deployment` and evolves in VEGA.
4. **First slice:** D0–D1.
