---
title: Settings Sync - VEGA
description: Sync your VEGA settings, keymap, tasks, snippets, themes and agent rules across machines with a secret GitHub Gist.
---

# Settings Sync

Settings Sync keeps your VEGA setup the same on every machine. It stores your files in one secret [GitHub Gist](https://gist.github.com) in your own account. VEGA talks only to GitHub; no VEGA or Zed server is involved.

## Turn it on

1. Run {#action settings_sync::SignIn} (or click **Sign In** in the title bar).
2. Enter the code VEGA shows on the GitHub page that opens. VEGA asks only for the `gist` permission.
3. That's it. VEGA creates the gist on your first machine and finds it on the others.

The account menu in the title bar shows when this machine last synced, and has **Sync Now**, **Sync History…** and **Turn Off Settings Sync**.

## What syncs

| On your machine | Notes |
|---|---|
| `settings.json` | Merged setting by setting, see below |
| `keymap.json` | One per operating system, because `cmd` and `ctrl` don't carry over |
| `tasks.json`, `debug.json` | Whole file |
| `AGENTS.md` (your global agent rules) | Whole file |
| `snippets/*.json` | Each file on its own |
| `themes/*.json` | Your own themes; themes from extensions aren't synced |

Deleting one of these files on one machine deletes it on the others.

## How changes merge

VEGA syncs on startup, a few seconds after you change a synced file, and every few minutes.

`settings.json` merges one top-level setting at a time. If you change `tab_size` on one machine and `theme` on another, both changes arrive everywhere. Your comments and formatting stay as they are.

If the same setting was changed differently on two machines, VEGA stops and asks: **Keep This Machine's Settings** or **Use Settings from GitHub**. It never overwrites silently, and it backs up this machine's files before replacing them.

## What never syncs

A secret gist is unlisted, not private: anyone with its link can read it. So VEGA never uploads:

- settings whose name contains `token`, `secret`, `password`, `passphrase`, `auth`, `credential` or `api_key`, at any depth
- `env` blocks, `ssh_connections`, `proxy` and `server_url`

These stay on each machine and are kept when synced settings arrive. If a value anywhere in `settings.json` looks like an API key or token (for example `sk-…` or `ghp_…`), VEGA stops and tells you where it is instead of uploading it. API keys belong in each provider's key field in Settings > AI, which stores them in your OS keychain.

To keep other settings different on this machine, list them in `settings_sync.ignored_settings`:

```json [settings]
{
  "settings_sync": {
    "ignored_settings": ["buffer_font_size", "terminal.font_size"]
  }
}
```

This machine neither uploads those settings nor takes them from GitHub, and the other machines keep their own values. The `settings_sync` block itself never syncs.

## History and restore

GitHub keeps every version of your gist. Run {#action settings_sync::ShowHistory} to see them and pick one to restore. VEGA writes it back as the newest version, backs up this machine's files, and your other machines pick it up on their next sync.

## Turn it off or sign out

- {#action settings_sync::TurnOffSync} stops syncing on this machine. Your settings stay on GitHub.
- {#action settings_sync::SignOut} also removes the GitHub sign-in from this machine. To revoke VEGA's access completely, visit [github.com/settings/applications](https://github.com/settings/applications).

Local backups are in VEGA's data folder under `settings_sync/backups/`.
