---
# vein-cjqz
title: list_ready tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:45Z
updated_at: 2026-03-16T02:17:37Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: list tasks in the Todo bucket. Returns task summaries suitable for an agent to pick work.

## Plan
- [x] Keep VeinServer concrete with ReqwestClient (rmcp macros incompatible with generics)
- [x] Add ProjectConfig to VeinServer
- [x] Implement list_ready tool method
- [x] Wire up server construction in main.rs with real client + config
- [x] Update tests

## API Notes
- List tasks via GET /projects/{id}/views/{view_id}/tasks with filter query param
- Filter by bucket_id to get Todo bucket tasks

## Summary of Changes

- Added `list_ready` MCP tool that lists tasks in the Todo bucket
- `VeinServer::new` now takes `ReqwestClient` and `ProjectConfig`
- Extracted `format_task_list` for testable task formatting (priority, labels)
- Added `+ Send` bounds to `VikunjaClient` trait futures
- Integration tests now create dedicated test projects and discover views/buckets
- Updated CHANGELOG
