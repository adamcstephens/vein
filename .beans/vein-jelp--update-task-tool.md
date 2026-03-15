---
# vein-jelp
title: update_task tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:54Z
updated_at: 2026-03-15T20:52:09Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: update an existing task's title, description, or labels.

## API Notes
- POST /tasks/{id} (not PUT/PATCH) for updates
- Send only fields to change; skip_serializing_if for None fields
- labels are read-only on task; separate endpoint needed to modify them
