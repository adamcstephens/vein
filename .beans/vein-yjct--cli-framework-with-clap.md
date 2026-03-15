---
# vein-yjct
title: CLI framework with clap
status: completed
type: task
priority: normal
created_at: 2026-03-15T21:10:10Z
updated_at: 2026-03-15T21:12:04Z
parent: vein-rnzz
---

Add clap for CLI argument parsing. Distinguish between 'vein init' (discovery mode) and 'vein serve' or default (MCP stdio server mode).

## Summary of Changes

- Added `src/cli.rs` with clap derive: `Cli` struct and `Command` enum (Init, Serve)
- `main.rs` dispatches on subcommand: init (stub), serve/default (MCP stdio)
- Added clap 4.6.0 with derive feature
- 3 tests: parse init, parse serve, default to none
