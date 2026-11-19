# VEGA: next plan (everything except Deployment)

_Drafted 2026-11-19 from HANDOFF, ROADMAP, PROMPTS, SETTINGS_SYNC, BRAND and spot checks of the code. Deployment has its own plan (`DEPLOYMENT_PLAN.md`)._

## Open items

| ID | Item | Size | Owner decision? |
|---|---|---|---|
| A1 | Agent thumbs-up/down says it "sends the conversation to the Zed team" (it calls zed.dev's feedback API, which fails without sign-in) | S | no |
| A2 | Reachable "Zed" leftovers: MCP OAuth `client_name` "Zed" (`context_server/src/oauth.rs:743`), "Zed Predictions" settings section, "Upgrade {agent} to work with Zed", the Delta "Made by the Zed team" toast (shows after the first real auto-update), CONTRIBUTING / CODE_OF_CONDUCT / `docs/book.toml` | S–M | small: MCP/ACP client identity |
| A3 | Hidden Zed cloud/collab/plan copy (unreachable without sign-in) | S | leave it + guard test? |
| A4 | Owner checks VEGA Dark (P06) and the AI Provider onboarding (P07) in the app | S | — |
| U1 | `script/vega-upstream-check`: fails on new `.zed` project paths, new `build_zed_*_url` calls, `zed.dev` literals in non-test code, changed VEGA guards; run after each upstream merge and in CI | S | no |
| C1 | Turn off the 7 active upstream workflows (keep the files for cheap merges) | S | yes |
| C2 | CI: clippy on VEGA crates + the check script now; tagged release workflow (macOS DMG, Linux + musl remote server, Windows) later | M | depends on R1 |
| R1 | Repository visibility | — | **yes, first** |
| S1 | Signed + notarized macOS bundle (P11): drop Sentry upload, Zed's team id → env var, rename keychain/DMG; unsigned DMG first | M | Apple Developer account ($99/yr) |
| S2 | Own updates + remote-server binary (P10): GitHub Releases backend with checksums, release notes on GitHub, remove the Delta toast | L | where releases live |
| D1 | Docs on GitHub Pages (move `VEGA_DOCS_BASE_URL`), rebrand docs text, contributor files, one-page site | L | depends on R1 |
| D2 | VEGA's own CIMD document for MCP OAuth (optional) | S | — |
| E1 | Extension catalogue without zed.dev: (a) keep zed.dev, (b) turn the registry off, (c) VEGA static catalogue | L | yes |
| E2 | Settings Sync: installed-extensions list, then close the parent roadmap item | S | after E1 |
| E3 | Settings Sync encryption v2 (passphrase) | M | only if asked for |
| F1 | P09: VEGA-specific AI features (candidates: local BYOK usage/cost meter, model presets, deployment-aware agent tools after D5, synced prompt library) | M–L | which first |
| H1 | Roadmap cleanup (branding sweep split into done/remaining, Settings Sync parent) | S | no |

## Decisions needed

| # | Decision | Recommendation | Blocks |
|---|---|---|---|
| 1 | Make the repo public (after a secrets audit)? | Yes, before the first release | S2, C2 macOS, D1 Pages, in-app doc links |
| 2 | Where updates and the remote server come from | GitHub Releases on the main repo | S2 |
| 3 | Apple Developer account | Owner's personal account now | S1 signing |
| 4 | Extensions | Turn zed.dev off now (b), VEGA catalogue later (c) | E1, E2 |
| 5 | First P09 feature | Usage meter or model presets; deployment agent tools after D5 | F1 |
| 6 | Turn off the 7 upstream workflows | Yes | C1 |
| 7 | Hidden cloud copy | Leave it, add a guard test | A3 |
| 8 | MCP/ACP client identity | MCP → "VEGA"; ACP stays "zed" until tested with agents | A2 |

## Order

**Phase 1 (next 1–2 weeks, no blockers):**
1. A1 agent feedback UI and setting off.
2. A2 reachable branding (OAuth client name, Zed Predictions section, ACP text, Delta toast, contributor files).
3. U1 + C2 part 1: upstream check script, CI job, clippy on VEGA crates.
4. A4 owner's app check, fixes.
5. Weekly upstream merge (P14), gated by the check script.
6. A3 guard test, H1 cleanup, C1 once approved.
7. P09 interview and short spec; E1 option (b) once decided.

**Phase 2 (weeks 3–6, after decisions 1–3):** S1 unsigned DMG then signing; C2 release workflow; S2 GitHub Releases updater; D1 docs on Pages + D2.

**Phase 3 (later):** E1 option (c) catalogue + E2; F1 first feature; full docs rebrand and site; E3 if asked.
