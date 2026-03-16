---
# vein-7fmg
title: MCP server integration test harness
status: completed
type: task
priority: normal
created_at: 2026-03-16T01:27:50Z
updated_at: 2026-03-16T01:41:56Z
parent: vein-rnzz
---

Set up integration testing for the MCP server using tokio::io::duplex for in-memory MCP transport and real Vikunja API calls.

## Approach
- Use tokio::io::duplex to connect MCP client/server in-process (no stdio spawning)
- VeinServer talks to real Vikunja over HTTP during tests
- Vikunja credentials come from env vars
- Tests create their own projects and tear them down after

## Prerequisites
- [x] Vikunja client needs project creation capability (create_project)
- [x] Vikunja client needs project deletion capability (delete_project) for teardown

## Tasks
- [x] Add create_project and delete_project to Vikunja client
- [x] Add rmcp client feature to dev-dependencies
- [x] Create test helper: spin up VeinServer over duplex, return MCP client
- [x] Create test helper: create ephemeral Vikunja project, teardown on drop
- [x] Write first integration test: initialize MCP server, list tools
- [x] Write integration test: call a tool that hits Vikunja and verify response (create_and_delete_test_project)


## Summary of Changes

- Added `create_project` and `delete_project` to `VikunjaClient` trait and `ReqwestClient` impl
- Extracted `src/lib.rs` to expose modules for integration tests
- Updated `main.rs` to import from the lib crate
- Added `rmcp` client feature and `tokio` io-util to dev-dependencies
- Created `tests/mcp_integration.rs` with:
  - `mcp_client()` helper: spins up VeinServer over `tokio::io::duplex`, returns MCP client
  - `TestProject` helper: creates/deletes ephemeral Vikunja projects
  - 3 passing tests: MCP init+list_tools, tool capabilities, Vikunja project CRUD
