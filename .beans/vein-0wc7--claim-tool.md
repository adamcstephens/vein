---
# vein-0wc7
title: claim tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:49Z
updated_at: 2026-03-15T20:52:08Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
    - vein-rbmw
---

MCP tool: claim a task by moving it to the In Progress bucket and applying an agent label (e.g. agent:run-id). Assigns the agent user.

## API Notes
- Move to bucket: POST /tasks/{id} with bucket_id field
- Assign agent: POST /tasks/{id} with assignees
- Add label: separate endpoint for labels
- Uses agent identity from vein-rbmw
