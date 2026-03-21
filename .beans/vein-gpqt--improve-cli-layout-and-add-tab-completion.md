---
# vein-gpqt
title: 'flatten cli: remove tool subcommand nesting'
status: completed
type: task
priority: normal
created_at: 2026-03-17T03:07:07Z
updated_at: 2026-03-21T19:25:04Z
---

## Description

Restructure the CLI layout by flattening the `tool` subcommand hierarchy — all tool subcommands become top-level commands. Remove the `tool` subcommand entirely.

## Tasks

- [x] Move all `ToolCommand` variants to top-level `Command` enum
- [x] Remove `Tool` variant and `ToolCommand` enum
- [x] Update dispatch logic in main
- [x] Update all tests referencing `tool` subcommand
- [x] Update CHANGELOG

## Summary of Changes

Flattened CLI by merging all `ToolCommand` variants into the top-level `Command` enum. Removed the `Tool` wrapper variant and `ToolCommand` enum entirely. Updated dispatch in `main.rs` and all CLI parser tests. Commands like `vein tool list-ready` are now just `vein list-ready`.
