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
- [x] Disable telemetry, crash upload, auto-update against zed.dev (P04)
- [x] Remove/hide Zed sign-in, Zed AI plan, collab UI (P05)
- [ ] Default "VEGA Dark" theme (P06)

## Phase 2 — AI-first
- [ ] Onboarding: pick a BYOK provider (Anthropic, OpenAI, Ollama, …) (P07)
- [ ] Sensible default agent + edit-prediction settings without Zed cloud (P08)
- [ ] VEGA-specific AI features (P09)
- [ ] Settings Sync via GitHub sign-in + secret Gist — see `SETTINGS_SYNC.md`
  - [~] M1 Sign in with GitHub — built; needs the OAuth App Client ID to go live
  - [ ] M2 Gist backend + whole-file sync
  - [ ] M3 Smart merge + never-sync rules
  - [ ] M4 All resources, history, settings page

## Phase 3 — Ship
- [ ] Own update server / GitHub Releases auto-update (P10)
- [ ] Signed + notarized macOS bundle (P11)
- [ ] Linux + Windows builds in CI (P12)
- [ ] Website, docs, README rewrite (P13)

## Ongoing
- [ ] Weekly upstream sync (P14)
