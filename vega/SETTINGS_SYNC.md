# Settings Sync — design and plan

Sign in with GitHub on any machine and your VEGA settings follow you. No VEGA or Zed server is involved: the app talks only to `github.com` / `api.github.com`.

## Decisions

| Topic | Decision | Why |
|---|---|---|
| Sign-in | GitHub **OAuth App + device flow** ("enter this code on github.com"), scope **`gist`** only | No client secret needed, so it's safe in an open-source binary. Same flow as `gh` and Copilot. `gist` is the narrowest scope that can store data. |
| Storage | One **secret Gist** per user, found by a marker file `vega-sync.json` | Free version history (restore), no extra repo, no `repo` scope (which would grant access to *all* private repos). |
| Token | OS keychain via `credentials_provider`, never logged, never in settings | Dev builds use `development_credentials` in the config dir, which is never synced. |
| Merge | Three-way merge per resource, against a locally stored *base* (last synced content). `settings.json` merges per top-level key as JSONC text edits, so comments survive | Same model as VS Code's `settingsMerge.ts`. |
| Conflicts | Never silently overwrite. Keep local, pause that resource, ask: **Use Local / Use Remote**. Always back up before overwriting | "Silent data loss" is the top complaint about Gist-based sync tools. |
| Secrets | Never-sync denylist (keys matching `api_key|token|secret|password|auth|credential`, any `env` object, `ssh_connections`, `proxy`, `server_url`), plus a user `settings_sync.ignored_settings` list and a pre-upload scan that blocks values that look like keys (`sk-`, `ghp_`, `github_pat_`, …) | A secret gist is *unlisted, not private*: anyone with the URL can read it. |
| Keymaps | Synced per OS (`keymap.macos.json`, …) | cmd vs ctrl isn't portable; VS Code does the same. |
| Encryption | Not in v1. v2 option: passphrase → Argon2id → XChaCha20-Poly1305 | Adds a passphrase to every machine and makes history unreadable on github.com. |
| Backend | Behind a `SyncBackend` trait, so a private-repo backend (GitHub App, single repo) can come later | |

## What syncs

| Resource | Gist file | Notes |
|---|---|---|
| `settings.json` | `settings.json` | Three-way merge per top-level key. `"macos"`/`"linux"`/`"windows"` blocks already handle per-OS values |
| `keymap.json` | `keymap.<os>.json` | Whole-file |
| `tasks.json`, `debug.json` | same names | Whole-file |
| `snippets/*` | `snippets__<name>` | Gists are flat |
| `themes/*` | `themes__<name>` | User themes only |
| `AGENTS.md` | `AGENTS.md` | Global agent rules |
| Installed extensions | `extensions.json` (list of ids) | Installs missing ones; never auto-uninstalls. Depends on the extension registry decision (see ROADMAP) |
| Never | `global_settings.json`, backups, `development_credentials`, prompts DB, anything in the data dir | |

## When it syncs

On startup and on sign-in; on local change (existing file watchers, debounced ~3 s); a poll every 5 min using `If-None-Match` (a 304 is free); and on window focus (at most once a minute). Everything runs through one serial queue. Writes made by sync are hashed so they don't echo back as uploads.

**First sync on a new machine:** if local is untouched, pull silently. Otherwise ask: *Merge (cloud wins conflicts)* / *Replace local with cloud* / *Replace cloud with local*. Local files are backed up to `<data>/sync-backups/<timestamp>/` first.

**Concurrency:** gists have no `If-Match`. Before a PATCH, do a conditional GET (200 = changed, so merge first). After it, check that the response's previous `history` version is the base we merged against; if not, another machine wrote in between, so re-merge and retry (bounded).

## GitHub calls

Headers: `Authorization: Bearer <token>`, `Accept: application/vnd.github+json`, `X-GitHub-Api-Version: 2022-11-28`, `User-Agent: VEGA/<version>`.

| Purpose | Call |
|---|---|
| Device code | `POST https://github.com/login/device/code` (`client_id`, `scope=gist`, `Accept: application/json`) |
| Token | `POST https://github.com/login/oauth/access_token` (`client_id`, `device_code`, `grant_type=urn:ietf:params:oauth:grant-type:device_code`), honouring `slow_down`/`expired_token`/`access_denied` |
| Who am I | `GET /user` (login, avatar; check `X-OAuth-Scopes`) |
| Find gist | `GET /gists?per_page=100` (paginate), match marker file |
| Create | `POST /gists` with `public: false`, then verify `public == false` |
| Read / poll | `GET /gists/{id}` with `If-None-Match`; `raw_url` if `truncated` |
| Write | `PATCH /gists/{id}` (`null` deletes a file) |
| History / restore | `GET /gists/{id}/commits`, `GET /gists/{id}/{sha}` |
| Delete cloud data | `DELETE /gists/{id}` (confirmed by user) |

Limits: GitHub keeps ≤10 tokens per user per app (an 11th machine revokes the oldest, so a 401 means "sign in again"). Rate limits: serial requests, ≥1 s between writes, honour `retry-after` / `x-ratelimit-reset`. Sign-out deletes the local token and links to `github.com/settings/applications` to revoke (a public app can't revoke via API).

## Status

- **M1/M2 built.** `settings.json` and the per-OS keymap sync with base tracking, conflict prompts (Keep This Machine's / Use GitHub's) and backups in `<data>/settings_sync/backups/`. Local state is `<data>/settings_sync/state.json`.
- **M3 built** (`merge.rs`, `engine.rs`):
  - `settings.json` merges per top-level key against the base, so different settings changed on two machines merge silently; only keys changed differently on both sides conflict, and the prompt names them. The local file is edited key by key with `settings_json`, so comments and formatting survive. The keymap stays whole-file.
  - Never-sync rules: key names containing the words `token`, `secret`, `password`, `passphrase`, `auth`, `credential(s)` or `api_key`, any `env` object, and top-level `ssh_connections`, `proxy`, `server_url` are removed before comparing or uploading, and kept on each machine when its file is updated. Words are matched whole, so `max_tokens` still syncs.
  - Secret scan: a value that looks like a key (`sk-`, `ghp_`, `github_pat_`, `xox…`, `AKIA`, `AIza`, PEM private keys) stops the sync with the path of the value; nothing is uploaded.
  - First sync on a machine with its own settings: keys only one side has merge; keys both have with different values are asked about (Keep This Machine's / Use GitHub's), with a backup first.
  - Writes racing between machines: after each PATCH, the response's previous history version must be the one this machine merged against. If not, the other machine's version is fetched and merged in with a second PATCH; incompatible changes give the other machine's copy back and are asked about.
  - `settings_sync.ignored_settings` (top-level keys or dotted paths, e.g. `"terminal.font_size"`): this machine neither uploads those settings nor takes them from GitHub, and GitHub keeps other machines' values for them. The `settings_sync` block itself never syncs. Schema in `settings_content::SettingsSyncContent`, default in `default.json`.
- **Not yet:** M4.

## Code layout

New crate `crates/settings_sync` (thin seams into upstream):

- `github_auth.rs` — device flow + `GET /user` + keychain. Adapted from `copilot_chat::copilot_oauth` with our own `client_id` and scope.
- `gist_backend.rs` — `SyncBackend` trait + Gist implementation (ETag, history check, rate-limit backoff).
- `merge.rs` — three-way merge; JSONC edits via `settings_json`.
- `resources.rs` — what syncs, path ↔ gist-file mapping, never-sync rules, secret scan.
- `sync_engine.rs` — global entity: state (`SignedOut / SigningIn / Idle / Syncing / Conflict / Error`), serial queue, watchers, poll timer, local state + backups in `<data>/settings_sync/`.
- `ui/` — device-code modal (adapted from `copilot_ui::sign_in`), conflict prompts, title-bar account menu replacing Zed's sign-in (GitHub avatar, *Sync now*, *Turn off sync*, *Sign out*), settings page.
- Settings: `settings_sync: { enabled, ignored_settings, resources: {…} }` in `default.json`.
- Actions: `settings_sync::{SignIn, SignOut, SyncNow, TurnOff, ShowHistory, DeleteCloudData}`.

`client_id` is compiled in with an env override (`VEGA_GITHUB_CLIENT_ID`) so forks can use their own app.

## Milestones

1. **Sign in with GitHub.** Crate skeleton, settings, device-flow modal, keychain, title-bar account menu. Verify: sign in/out, token survives restart.
2. **Gist backend + whole-file sync** for `settings.json` and keymap. Find-or-create gist, push/pull with base tracking, conflict prompt, backups. Verify with two data dirs (`--user-data-dir`) on one machine.
3. **Smart merge + safety.** Per-key three-way merge with comments preserved, never-sync rules, secret scan, first-sync prompt. Unit tests for the merge and for the scrubber.
4. **Everything else.** tasks, debug, snippets, themes, `AGENTS.md`, extensions list; status/last-sync indicator; history + restore; settings page; docs.

## Needs from the owner

- Register a GitHub OAuth App: <https://github.com/settings/applications/new>. Name `VEGA`, homepage `https://github.com/mahbubur001/vega`, callback URL `https://github.com/mahbubur001/vega` (unused by device flow), then tick **Enable Device Flow**. Send the **Client ID** (not a secret). Move it to a VEGA org later if one is created.
