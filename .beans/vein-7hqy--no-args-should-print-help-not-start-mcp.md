---
# vein-7hqy
title: no args should print help, not start mcp
status: completed
type: task
priority: normal
created_at: 2026-03-28T17:45:33Z
updated_at: 2026-03-28T22:02:58Z
---

## Summary of Changes\n\nAdded `subcommand_required` and `arg_required_else_help` to the CLI struct so running `vein` with no arguments prints help text instead of silently starting the MCP server. The `serve` subcommand remains the explicit way to start the MCP server.
