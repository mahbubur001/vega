# VEGA

VEGA ("Code Beyond Limits") is an independent, open-source, AI-powered code editor forked from [Zed](https://github.com/zed-industries/zed). It is a Rust workspace (~250 crates) built on GPUI, Zed's GPU-accelerated UI framework.

**Start of every session: read `vega/HANDOFF.md`** (where work stopped, how the owner wants to work, setup for a new computer) and update it before ending the session.

Product docs live in `vega/`:
- `vega/HANDOFF.md` — restart point: current state, next tasks, pending decisions.
- `vega/ROADMAP.md` — phased plan and current status. Check it before starting work and update it when a phase item is done.
- `vega/BRAND.md` — name, palette, icon rules.
- `vega/PROMPTS.md` — the A–Z prompt sequence used to drive the fork.
- `reports/` — research reports behind features (the Markdown view plan); the notes they were written from are in `research_notes/`.

Coding guidelines for Rust and GPUI (error handling, naming, GPUI contexts, entities, actions, tests) are in `AGENTS.md`. Read the relevant sections before writing Rust code here.

## Git layout

- `upstream` = `https://github.com/zed-industries/zed` (read-only source of truth for the editor core).
- `vega` = our working branch. `main` mirrors upstream; never commit VEGA work to `main`.
- `origin` = `https://github.com/mahbubur001/vega` (public; default branch `vega`).
- Sync upstream regularly: `git fetch upstream && git merge upstream/main` on `vega`. Keep VEGA changes small and localized so merges stay cheap — prefer config, new crates, and thin seams over rewriting upstream files.

## Commands

```sh
cargo run                       # debug build + launch (slow first build, 20–40 min)
cargo run --release             # release build
cargo check -p <crate>          # fast type-check of one crate
cargo test -p <crate>           # test one crate; avoid `--workspace` unless needed
./script/clippy                 # lint the way CI does
script/vega-check-ui-widths         # pickers narrowed the wrong way (rows spill out); skill: vega-ui-overflow
script/vega-clean-deps              # free disk: drop old copies in target/debug/deps
cargo fmt --all                 # format
```

Don't open this repo in a dev build of VEGA itself (`cargo run .`) — rust-analyzer env leakage invalidates the build cache. Use `cargo run ~/some/other/project`.

## Prerequisites (macOS)

- Rust via rustup; the pinned toolchain comes from `rust-toolchain.toml`.
- Full Xcode (not only Command Line Tools), selected with `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`.
- On macOS 26+: `xcodebuild -downloadComponent MetalToolchain`.
- `cmake` (`brew install cmake`).

Full details: `docs/src/development/macos.md`.

## Where branding lives

| What | Where |
|---|---|
| App display name per channel | `crates/release_channel/src/lib.rs` |
| macOS bundle name / identifier / icon | `crates/zed/Cargo.toml` (`[package.metadata.bundle-*]`) |
| App icons | `crates/zed/resources/app-icon*.png`, `Document.icns` |
| Linux desktop entry | `crates/zed/resources/zed.desktop.in` |
| zed.dev URLs / account links | `crates/client/src/zed_urls.rs` |
| Cloud LLM + sign-in | `crates/client`, `crates/cloud_*`, `crates/language_models_cloud` |
| Collaboration server | `crates/collab`, `crates/collab_ui` |

## Licensing — must respect

Zed is GPL-3.0-or-later (editor, including the collab server) with Apache-2.0 for GPUI and some crates; each crate's `Cargo.toml` states its licence. VEGA must stay open source under the same terms: keep `LICENSE-*` files and upstream copyright notices, and don't relicense upstream code. Run `script/check-licenses` after adding dependencies.

## Rules for this fork

- Remove or replace every dependency on Zed Industries' hosted services (sign-in, Zed AI, collab, telemetry, auto-update) — VEGA must never talk to zed.dev servers.
- Don't use the "Zed" name or logo in user-facing VEGA branding (trademark). Internal crate names (`zed`, `zed_actions`, …) stay as-is to keep upstream merges cheap.
- AI features use bring-your-own-key providers (Anthropic, OpenAI, Ollama, …) that already exist in `crates/language_models`.
