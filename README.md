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
