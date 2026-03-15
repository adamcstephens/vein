---
# vein-c4z5
title: list_mine tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:46Z
updated_at: 2026-03-15T20:51:56Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
    - vein-rbmw
---

MCP tool: list tasks in the In Progress bucket claimed by this agent instance. Uses agent identity from startup.

## API Notes
- List tasks via GET /projects/{id}/views/{view_id}/tasks with filter query param
- Filter by bucket_id for In Progress bucket, then filter by assignee matching agent identity
