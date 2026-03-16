---
# vein-ybfg
title: blocked-by awareness in list_ready
status: completed
type: feature
priority: normal
created_at: 2026-03-16T04:25:03Z
updated_at: 2026-03-16T12:31:06Z
---

list_ready currently just reads the Todo bucket. It should filter out tasks that have blocked_by relations pointing to incomplete tasks, so agents don't pick up blocked work.

## Summary of Changes

list_ready now filters out tasks that have `blocked` relations pointing to incomplete tasks. The Vikunja view endpoint already returns related_tasks with full task objects, so no extra API calls needed — filtering is done in-memory via `is_blocked()`. Shared `fetch_ready_tasks()` function used by both MCP tool and CLI.
