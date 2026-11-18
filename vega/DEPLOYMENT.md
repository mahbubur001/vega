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
- **Config.** Read and write iRemote's existing format unchanged: `.zed/iremote.json` per project, shared servers in `~/.config/iremote/servers.json`. Existing iRemote projects work immediately.
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

## Upstream seams (kept small for merges)

`Cargo.toml`, `crates/zed/{Cargo.toml, src/main.rs, src/zed.rs, src/zed/app_menus.rs}` (init, panel and status item registration, menu), one `.when(…)` block each in the project panel, editor and tab context menus, one `ActionLink` in `settings_ui`, and panel settings in `settings_content` + `default.json`. Everything else lives in the two new crates.

## Risks

- **Dependency weight.** russh (a second SSH stack with newer RustCrypto crates), suppaftp and possibly reqwest 0.13 increase build time and binary size. Mitigation: move WebDAV to VEGA's `http_client`; align sha/dirs versions.
- **Runtime.** The core assumes it runs inside tokio (`tokio::spawn`, `Handle::current`, `spawn_blocking`). All entry points must go through `gpui_tokio`, whose 2-worker pool may need more threads. Don't hold `Engine::config()` locks across `.await`.
- **Multi-root and remote projects.** One engine per local worktree; SSH/WSL projects are excluded at first.
- **UI rewrite (~10k lines).** The largest cost. iRemote's UI was never visually reviewed, so treat it as a reference, not a spec.
- **Tests.** No UI tests yet; integration tests need Docker (`tests/servers/compose.yml`). rsync is an unused stub in iRemote; it stays out.

## Decisions (owner)

1. **Config file.** Keep `.zed/iremote.json` + `~/.config/iremote/servers.json` (compatible with iRemote for Zed/VS Code/Neovim) *(recommended)*, or rename to `.zed/deployment.json`.
2. **Keychain.** Store secrets through VEGA's credentials provider *(recommended)* and fall back to reading iRemote's existing `iremote` keychain entries once, so saved passwords keep working.
3. **Source of truth.** Vendor the core into VEGA and let it evolve there *(recommended)*, or keep one shared crate for both iRemote-for-Zed and VEGA.
4. **First slice.** Start with D0–D1 (upload on save, actions, JSON config) *(recommended)*, or go straight for the Remote Host panel.
