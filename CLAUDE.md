# VEGA

VEGA ("Code Beyond Limits") is an independent, open-source, AI-powered code editor forked from [Zed](https://github.com/zed-industries/zed). It is a Rust workspace (~250 crates) built on GPUI, Zed's GPU-accelerated UI framework.

Product docs live in `vega/`:
- `vega/ROADMAP.md` — phased plan and current status. Check it before starting work and update it when a phase item is done.
- `vega/BRAND.md` — name, palette, icon rules.
- `vega/PROMPTS.md` — the A–Z prompt sequence used to drive the fork.

Coding guidelines for Rust and GPUI (error handling, naming, GPUI contexts, entities, actions, tests) are in `AGENTS.md`. Read the relevant sections before writing Rust code here.

## Git layout

- `upstream` = `https://github.com/zed-industries/zed` (read-only source of truth for the editor core).
- `vega` = our working branch. `main` mirrors upstream; never commit VEGA work to `main`.
- `origin` = `https://github.com/mahbubur001/vega` (private for now; default branch `vega`).
- Sync upstream regularly: `git fetch upstream && git merge upstream/main` on `vega`. Keep VEGA changes small and localized so merges stay cheap — prefer config, new crates, and thin seams over rewriting upstream files.

## Commands

```sh
cargo run                       # debug build + launch (slow first build, 20–40 min)
cargo run --release             # release build
cargo check -p <crate>          # fast type-check of one crate
cargo test -p <crate>           # test one crate; avoid `--workspace` unless needed
./script/clippy                 # lint the way CI does
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

Zed is GPL-3.0-or-later (editor), AGPL-3.0 (collab server), Apache-2.0 (GPUI and some crates). VEGA must stay open source under the same terms: keep `LICENSE-*` files and upstream copyright notices, and don't relicense upstream code. Run `script/check-licenses` after adding dependencies.

## Rules for this fork

- Remove or replace every dependency on Zed Industries' hosted services (sign-in, Zed AI, collab, telemetry, auto-update) — VEGA must never talk to zed.dev servers.
- Don't use the "Zed" name or logo in user-facing VEGA branding (trademark). Internal crate names (`zed`, `zed_actions`, …) stay as-is to keep upstream merges cheap.
- AI features use bring-your-own-key providers (Anthropic, OpenAI, Ollama, …) that already exist in `crates/language_models`.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

Rules:
- When graphify-out/graph.json exists, run `graphify query "<question>"` before your first grep, search or source-file read for a codebase question, and put this rule in every subagent prompt that explores code. Read raw files only for lines the graph or an earlier read already located. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. When the answer is under a Markdown heading, `graphify section "<heading>"` prints that section's text, so the .md need not be opened. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
- A committed graphify-out/graph.json is rewritten whole on every build, so a large diff to it in a pull request is expected. Do not flag it as churn or suggest skipping the graph rebuild or its hooks.
