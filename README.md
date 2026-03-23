# Chops Core

A local-first Tauri + React clone of the core Chops workflow:

- scan local AI skill directories
- normalize different file layouts into one indexed model
- search across name, description, and content
- edit source files directly from a desktop UI

## Stack

- Tauri 2
- React 19 + TypeScript + Vite
- Rust
- SQLite via `rusqlite`

## Current MVP

Supported sources:

- `~/.claude/skills`
- `~/.agents/skills`
- `~/.cursor/skills`
- `~/.cursor/rules`
- `~/.codex`
- `~/.openclaw/skills`

Implemented capabilities:

- full scan into SQLite index
- frontmatter and heading parsing
- tool filtering
- full-text list search
- detail loading
- save back to source file

Not implemented yet:

- file watching / auto-refresh
- collections / favorites
- remote SSH skill servers
- registry install flow

## Development

Install dependencies:

```bash
npm install
```

Run the desktop app:

```bash
npm run tauri dev
```

Run the frontend build:

```bash
npm run build
```

Run Rust tests:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

## macOS note

The current macOS release build is not notarized yet.
Because of that, Gatekeeper may show:

`"Skills Manager" is damaged and can’t be opened. You should move it to the Trash.`

This does not mean the DMG is corrupt. It means macOS is blocking an unsigned app.

You can open it in either of these ways:

1. Finder method

- Open the DMG
- Drag `Skills Manager.app` into `Applications`
- In `Applications`, right-click `Skills Manager`
- Choose `Open`
- Click `Open` again in the confirmation dialog

2. Terminal method

Remove the quarantine flag, then open the app:

```bash
xattr -dr com.apple.quarantine "/Applications/Skills Manager.app"
open "/Applications/Skills Manager.app"
```

If you do not trust the downloaded app, do not bypass Gatekeeper.

## Architecture

Disk files are the source of truth. SQLite is only the query/index layer.

- `src-tauri/src/scanner.rs`
  scans supported directories and converts files into normalized `ScannedSkill` values
- `src-tauri/src/parser.rs`
  extracts metadata from frontmatter or falls back to markdown headings
- `src-tauri/src/db.rs`
  stores indexed skills and their installation paths
- `src-tauri/src/commands.rs`
  exposes `scan_all`, `list_skills`, `get_skill`, and `save_skill` to the frontend
- `src/`
  renders the desktop UI and calls backend commands through Tauri invoke
