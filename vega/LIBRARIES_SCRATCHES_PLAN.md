# External Libraries and Scratches: plan (2026-11-19)

PhpStorm/IntelliJ parity for two Project-view features. Based on the JetBrains docs and a survey of VEGA's project panel, worktrees and language servers. Preview: `vega/mockups/libraries-scratches.html`. Owner chose "go" on 2026-11-19 → option A (nodes inside the project panel). L1 built 2026-11-19.

## What PhpStorm does (summary)

**External Libraries** (node at the bottom of the Project view)
- Children: **PHP Runtime** (one entry per enabled stub extension: Core, standard, pdo, json, mbstring…), every **Include Path** folder, Composer **vendor** packages (library roots: excluded from the project but indexed), JavaScript libraries (**Node.js Core**, `@types/…`, CDN/custom libraries), **HTML**.
- Files open read-only; editing triggers **Non-Project Files Protection** ("This file / All files in this directory / Any non-project file", for this session). Libraries are indexed (completion, navigation, Find Usages in the "Project and Libraries" scope) but skip inspections.
- Configured in **Settings › PHP**: language level, CLI interpreter, **Include Path** tab (Add, Remove, Exclude Under This Path), **PHP Runtime** tab (stub checkboxes, Sync Extensions with Interpreter); **Settings › PHP › Composer** ("Add packages as libraries", sync with composer.json); **JavaScript › Libraries** (enable, Add, Download, scopes).

**Scratches and Consoles** (node at the bottom of the Project view)
- Scratch files live outside projects in the IDE config folder and are visible in every project.
- **File › New › Scratch File (⇧⌘N)**: language popup (4 recent languages on top, type to filter), files named `scratch.php`, `scratch_1.php`…; from a selection, the scratch starts with that code. **New Scratch Buffer**: plain text `buffer1.txt`…`buffer5.txt`, rotating.
- Actions: Change Language, Rename (⇧F6), Copy (F5), Move into the project (F6, drag), Delete, New Directory, Run / Debug (PHP scratches use the project's PHP interpreter), Show Scratch Files. An empty scratch is deleted when its tab closes.
- Consoles: database query consoles (VEGA has no database tools, so out of scope).

## How it fits VEGA

| Piece | Approach |
|---|---|
| Where the nodes show | **Option A (recommended for parity):** two nodes at the bottom of the project panel, as in PhpStorm. Needs one small seam in the upstream project panel ("extra roots" chained after the project folders, read-only guard). **Option B:** a separate "Libraries & Scratches" dock panel, no project-panel changes. |
| Scratches folder | `~/.config/vega/scratches`, opened as one invisible worktree per window, so scratches get the project's language servers and task variables. |
| Libraries | `vendor/` and `node_modules/` are already inside the project (shown by reference, loaded on expand). Include paths and PHP stubs outside the project become invisible worktrees held by VEGA. |
| Read-only | Library files open read-only with a "Non-project file" bar: Edit anyway (this file / this folder / all, for this session). |
| Completion into libraries | Include paths are passed to the PHP language server (Intelephense `environment.includePaths`, Phpactor init options) through project settings; vendor and node_modules are already seen by the servers. |
| Run a scratch | Runs in the terminal: `php`, `node`, `npx tsx`, `python3`, `bash`, by language; the PHP command comes from Settings › PHP › CLI interpreter. |

## Plan, in order

**L1. Scratches.** New `scratches` crate. Scratches and Consoles › Scratches node (or panel section); New Scratch File (⇧⌘N) with the language picker, recent languages, numbering, from-selection; New Scratch Buffer (rotating 5); Change Language, Rename, Copy, Move to Project, Delete, New Directory, Reveal in Finder, Run; Show Scratch Files popup; empty scratches removed on close.

**L2. External Libraries (browse).** New `external_libraries` crate. Node with PHP Runtime (stubs from the PHP language server), Include Paths, Composer packages (name + version from `composer.lock`), Node.js packages (dependencies from `package.json`, versions from the lock file), `@types`. Read-only opening with the Non-project file bar. Select Opened File reveals library files.

**L3. Settings › PHP page.** PHP language level, CLI interpreter (detected `php` binaries + Add…), Include Path list (Add, Remove, Exclude Under This Path), PHP Runtime stubs (checkboxes, Sync with Interpreter), Composer: "Add packages as libraries" and "Sync with composer.json". Settings are written to `.vega/settings.json` and passed to the PHP language server.

**L4. Scopes.** "Project and Libraries" option in Find in Files and the file finder; JavaScript › Libraries page (enable Node.js Core / @types, custom library folders).

Each phase ends with tests, clippy, a build, screenshots and the owner's check.

## Decision for the owner

**A or B?** Option A looks exactly like PhpStorm (nodes inside the project tree) but adds a small, recurring merge cost in Zed's project panel. Option B keeps upstream untouched and shows the same content in its own panel. Recommendation: **A**, since matching PhpStorm is the goal; the seam is about 200–350 lines in one upstream file.
