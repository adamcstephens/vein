---
# vein-eghl
title: add_relation tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:54Z
updated_at: 2026-03-16T03:33:46Z
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

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `add_relation` MCP tool with `AddRelationParams` (task_id, other_task_id, relation_kind)\n- Added `vein tool add-relation <task_id> <other_task_id> <relation_kind>` CLI subcommand\n- Calls Vikunja PUT /tasks/{id}/relations endpoint
