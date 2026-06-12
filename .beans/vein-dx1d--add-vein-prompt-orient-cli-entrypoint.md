---
# vein-dx1d
title: Add vein prompt orient CLI entrypoint
status: in-progress
type: feature
priority: normal
created_at: 2026-06-12T19:10:03Z
updated_at: 2026-06-12T19:17:07Z
---

Add a 'vein prompt orient' CLI subcommand that prints the same priming orientation text as the MCP 'orient' prompt, so agents without MCP access can be primed via the CLI.

## Todo
- [x] Extract orient text generation so MCP prompt and CLI share it
- [x] Add 'prompt' subcommand with 'orient' nested subcommand to CLI
- [x] Wire up handler in main.rs
- [x] Tests pass, lints pass
- [x] Verify with cargo run -- prompt orient
- [x] nix build passes

## Summary of Changes

- Added `Prompt` subcommand with nested `PromptCommand::Orient` to the CLI (`vein prompt orient`)
- Extracted orient text generation from the MCP prompt handler into shared `orient_text` (fetches task state) and `format_orient` (pure formatter) functions in server.rs; both the MCP prompt and the CLI now use them
- Wired up the handler in main.rs alongside the other project-scoped commands
- Tests added for CLI parsing and orient text formatting; verified live output against running vikunja
- Separate chore commit: ignore sqlite WAL sidecar files in .services/vikunja
