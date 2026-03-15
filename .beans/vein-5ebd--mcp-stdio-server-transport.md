---
# vein-5ebd
title: MCP stdio server transport
status: completed
type: task
priority: normal
created_at: 2026-03-15T20:17:38Z
updated_at: 2026-03-15T21:07:50Z
parent: vein-rnzz
---

Set up the MCP server using stdio transport. Handle JSON-RPC message framing, tool registration, and request routing. This is the server skeleton that tools plug into.

## Summary of Changes

- Added `VeinServer` struct in `src/server.rs` with rmcp `ToolRouter` and `ServerHandler`
- Server reports capabilities (tools enabled) and instructions
- `main.rs` wired up with `tokio::main(flavor = "current_thread")` and stdio transport
- Added rmcp 1.2.0 dependency with server and transport-io features
- Empty tool router ready for tools to be added via `#[tool]` macros
