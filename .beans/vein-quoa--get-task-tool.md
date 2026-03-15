---
# vein-quoa
title: get_task tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:47Z
updated_at: 2026-03-15T20:52:02Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: get full details of a task by ID, including description, labels, relations, and comments.

## API Notes
- GET /tasks/{id}
- Response includes related_tasks (map keyed by relation kind), labels, assignees, comments (with expand=comments)
