# VEGA — Claude Code prompts, A to Z

Run these in order with `claude` from `~/dev/vega`. Each prompt is self-contained; finish (and commit) one before starting the next. Claude reads `CLAUDE.md` automatically, so prompts don't repeat project context.

---

## P01 — First build

```
Build VEGA for the first time with `cargo run`. If the build fails, diagnose using
docs/src/development/macos.md (Xcode selection, Metal toolchain, cmake,
BINDGEN_EXTRA_CLANG_ARGS) and tell me any command that needs sudo instead of running it.
When the app launches, confirm and mark P01 done in vega/ROADMAP.md.
```

## P02 — GitHub repo

```
Create a public GitHub repo for VEGA with `gh repo create`, ask me for the owner/name
first. Add it as `origin`, push the `vega` branch and make it the default branch.
Do not push `main`.
```

## P03 — Rebrand

```
Rebrand the user-facing app from Zed to VEGA without renaming internal crates:
- display names in crates/release_channel/src/lib.rs ("VEGA", "VEGA Dev", "VEGA Nightly", "VEGA Preview")
- bundle metadata in crates/zed/Cargo.toml (name, identifier dev.vega.Vega*, url scheme `vega`)
- Linux desktop entry, Windows resources
- app icons generated from vega/brand/vega-icon.svg as described in vega/BRAND.md
List any other user-visible "Zed" strings you find (menus, About, window titles) and
fix them too. Keep the diff minimal so upstream merges stay easy. Build and launch to verify.
```

## P04 — Cut ties to zed.dev services

```
Find every place VEGA contacts zed.dev / Zed Industries servers: telemetry, crash
reporting, auto-update, extension registry, docs links, cloud LLM. Produce a table
(feature, crate, endpoint, proposal: remove / disable / point to VEGA infra).
Wait for my approval, then make telemetry and crash upload off-by-default and unreachable,
and disable auto-update for now.
```

## P05 — Remove Zed account UI

```
Hide or remove Zed sign-in, Zed AI plan/trial prompts and collaboration (channels,
calls) from the UI, so a fresh install never asks the user to sign in to Zed.
Prefer feature flags or settings defaults over deleting crates. Verify the app still
builds and the agent panel works with a BYOK provider.
```

## P06 — VEGA Dark theme

```
Create a "VEGA Dark" theme in assets/themes using the palette in vega/BRAND.md for
chrome (backgrounds, surfaces, text, accents). Choose syntax colors for readability and
WCAG AA contrast against #101018. Make it the default dark theme.
```

## P07 — AI onboarding

```
Redesign first-run onboarding: after launch, let the user pick an AI provider that
already exists in crates/language_models (Anthropic, OpenAI, Google, Ollama, LM Studio,
OpenRouter…), paste a key or detect a local Ollama, and pick a default model.
Store keys in the OS keychain the same way existing providers do.
```

## P08 — AI defaults

```
Set VEGA's default settings (assets/settings/default.json) so the agent panel and
edit predictions work without Zed cloud: default provider = whatever onboarding chose,
edit predictions via a BYOK-capable provider or off. Document every changed default
in vega/ROADMAP.md.
```

## P09 — Signature feature

```
Let's design VEGA's first signature AI feature. Interview me one question at a time
about the problem I want to solve, then write a short spec in vega/specs/, then a plan,
then implement it in a new crate.
```

## P10 — Updates

```
Implement auto-update for VEGA using GitHub Releases on our repo instead of zed.dev.
Reuse crates/auto_update structure; point it at the GitHub Releases API, verify
signatures/checksums, and keep it behind a setting.
```

## P11 — macOS release bundle

```
Adapt script/bundle-mac for VEGA: bundle name, identifier, icon, DMG name. Remove
Sentry/zed.dev upload steps. Leave signing + notarization configurable via env vars
(APPLE_ID, APPLE_TEAM_ID, certificate). Produce a local unsigned DMG to verify.
```

## P12 — CI

```
Replace upstream's GitHub Actions with a lean VEGA pipeline: fmt + clippy + tests on PR
(Linux + macOS), and a tagged-release workflow building macOS (DMG), Linux (tar.gz),
and Windows artifacts and attaching them to a GitHub Release. Delete workflows that
target Zed's infrastructure.
```

## P13 — README, docs, site

```
Rewrite README.md for VEGA (what it is, credit + link to Zed, license, build steps,
screenshots placeholder). Update CONTRIBUTING.md and CODE_OF_CONDUCT.md contacts.
Draft a one-page landing site in vega/site/ using the brand palette.
```

## P14 — Upstream sync (repeat weekly)

```
Sync with Zed: `git fetch upstream`, merge upstream/main into vega, resolve conflicts
in favor of keeping VEGA branding/service changes, build, run tests for crates touched
by the merge, and summarize what changed upstream that matters to VEGA.
```
