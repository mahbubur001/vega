# Deployment: PhpStorm parity plan (2026-11-19)

Based on a study of PhpStorm's Deployment / Remote Host docs and an inventory of VEGA's `deployment` + `deployment_ui` crates. Preview: `vega/mockups/deployment.html` (open in a browser). Status 2026-11-19: P1–P6 built (see DEPLOYMENT.md); P7 later.

## What is missing today (summary)

| Area | PhpStorm | VEGA now |
|---|---|---|
| Deployment dialog | Own resizable window, OK / Cancel / **Apply**, Duplicate, Rename, server groups, In place, SSH config picker, Rsync | Fixed-size modal inside the editor, no Apply/Duplicate, no groups/In place |
| Deployment › Options | Auto upload (Always / On explicit save / Never), exclude by name, overwrite up-to-date, temp file, preserve timestamps, delete missing, empty dirs, confirmations, default permissions, warn on newer remote, notify remote changes, logging | **No UI** (options exist only in `.vega/deployment.json`) |
| Actions | Upload to… / Download from… / Compare with Deployed To… / Sync with Deployed To… (server picker), Edit Remote File, Automatic Upload toggle, Tools › Deployment menu, default-server status widget | Default server only, no menu bar entries, no shortcuts |
| Remote Host | Select Opened File, Edit Remote / Edit Local File, Compare with Local, Sync with Local, Download from Here, SSH session here, hidden files | Browse, edit, file ops, chmod, drag and drop. "Open" never opens the local mapped file (bug) |
| Sync with Deployed | Folder diff: table Name / Size / Date / action, filters, filter field, diff pane for the selected file, Synchronize Selected / All | List with per-file direction; diff opens in a separate tab |
| File Transfer | Log tool window with Retry for failed transfers | Status-bar spinner only; cancel all |
| Remote editing | Banner "modified / identical", Upload, Compare, Revert; confirm on close | Saves upload silently; no banner |

## Plan, in order

**P1. Deployment window (resizable).** Open Deployment as its own OS window like VEGA's Settings window: resizable, minimum 720×420, remembers its size, reused if open; Esc / ⌘W close. Prompts (password, host key) raised from it show in it. Left list gets Add / Remove / Duplicate / Use as Default / Rename and server groups (drag servers in). Footer: Cancel / Apply / OK. Add "In place" and "Server group" types.

**P2. Options page.** "Options" entry at the bottom of the left list (PhpStorm keeps it as a sibling page). Every engine option gets a control (see mockup). Tools › Deployment › Automatic Upload toggle and a status-bar default-server picker.

**P3. Actions and menus.** Tools › Deployment menu (Upload to…, Download from…, Compare with Deployed To…, Sync with Deployed To…, Edit Remote File, Browse Remote Host, Automatic Upload, Configuration…, Options…). "…To…" actions show a server picker. Fix: mapped files open locally; explicit saves vs autosave; local renames mirrored on the server; multi-file selection.

**P4. Remote Host completions.** Toolbar: Select Opened File. Context menu: Edit Remote File / Edit Local File, Compare with Local Version, Sync with Local, Download from Here, Start SSH Session Here (SFTP), Open in Browser. Multi-select (⌘/⇧-click). Show hidden files per server. The panel already resizes by dragging the dock edge; VEGA docks can't float, so instead the panel gets "Open in Window" later if wanted.

**P5. Sync with Deployed as a folder diff.** Table (Name / Size / Date / action icon → ← = ≠ ×), filter toggles (new local, different, equal, new remote, hide excluded) and a filter field, Space cycles an action, row menu (Copy to Server / Copy to Project / Delete / Do Nothing / Mirror), Synchronize Selected / All, and a split diff of the selected file below the table.

**P6. File Transfer panel and remote-change awareness.** Bottom-dock "File Transfers" panel: live log (Errors / Brief / Details), per-file progress, Retry failed, Cancel per item, Clear. Notifications when a deployed file changed on the server; banner on remote-edited files (Identical / Modified, Upload, Compare, Revert).

**P7. Later.** SSH configurations shared with Remote Development, Rsync transfers, time offset, D5 items (SSH session, ssh-copy-id, upload changed files from git).

Each phase ends with tests, clippy, a build, and the owner's check in the app.
