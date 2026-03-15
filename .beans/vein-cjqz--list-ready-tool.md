---
# vein-cjqz
title: list_ready tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:45Z
updated_at: 2026-03-15T20:51:54Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: list tasks in the Todo bucket that are unassigned. Returns task summaries suitable for an agent to pick work.

## API Notes
- List tasks via GET /projects/{id}/views/{view_id}/tasks with filter query param
- Filter by bucket_id to get Todo bucket tasks
