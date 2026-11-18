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
- [~] Default "VEGA Dark" theme (P06) — `assets/themes/vega/vega.json`, default dark theme and first onboarding family; syntax colors ≥ WCAG AA on `#101018` (tested in `theme_settings`). Not yet looked at in the running app

## Phase 2 — AI-first
- [~] Onboarding: pick a BYOK provider (Anthropic, OpenAI, Ollama, …) (P07) — "AI Provider" section on the welcome page (`crates/onboarding/src/ai_provider_setup.rs`): Anthropic, OpenAI, Google, OpenRouter, Ollama, LM Studio; key saved to the OS keychain by the provider itself; Ollama/LM Studio use their own detection view; default-model picker writes `agent.default_model`. Not yet looked at in the running app (`onboarding: open`)
- [x] Sensible default agent + edit-prediction settings without Zed cloud (P08) — see "Changed defaults" below. Onboarding picking the provider is P07
- [ ] VEGA-specific AI features (P09)
- [ ] Settings Sync via GitHub sign-in + secret Gist — see `SETTINGS_SYNC.md`
  - [x] M1 Sign in with GitHub
  - [x] M2 Gist backend + whole-file sync (verified live)
  - [x] M3 Smart merge + never-sync rules
  - [ ] M4 All resources, history, settings page
- [ ] Deployment & Remote Host (built-in iRemote) — see `DEPLOYMENT.md`
  - [x] D0 Vendor the core (63 tests incl. real SFTP/FTP/WebDAV servers)
  - [~] D1 Upload on save + actions — code written, not built yet (see DEPLOYMENT.md "Next session")
  - [ ] D2 Deployment Servers UI
  - [ ] D3 Remote Host panel
  - [ ] D4 Compare & Sync
  - [ ] D5 Extras (SSH, agent tools)

## Phase 3 — Ship
- [ ] Own update server / GitHub Releases auto-update (P10) — must also serve `zed-remote-server` for SSH/WSL/Docker projects (`AutoUpdater::get_release_asset`), release notes (`auto_update_ui`, menu item removed), and the Linux build the WSL sandbox helper downloads (`sandbox::windows_wsl::PROVISION_HELPER_FROM_ZED_CLOUD`)
- [ ] Signed + notarized macOS bundle (P11)
- [ ] Linux + Windows builds in CI (P12)
- [ ] Website, docs, README rewrite (P13)
  - [ ] Host VEGA's OAuth client metadata (CIMD) and point `crates/context_server/src/oauth.rs` `CIMD_URL` at it — MCP OAuth currently identifies as Zed via `zed.dev/oauth/client-metadata.json`
  - [ ] Repoint ~60 in-app `zed.dev/docs` links (error toasts, help menu, status/merch links) at VEGA docs
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

## Ongoing
- [ ] Weekly upstream sync (P14)
