---
# vein-eghl
title: add_relation tool
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

MCP tool: add a relation between two tasks using Vikunja's native relation types (blocked, blocking, related, subtask, parenttask, etc.). Uses PUT /tasks/{taskID}/relations.

## API Notes
- PUT /tasks/{taskID}/relations
- Body: { other_task_id, relation_kind }
- Native relation kinds: blocked, blocking, related, subtask, parenttask, duplicateof, duplicates, precedes, follows, copiedfrom, copiedto
