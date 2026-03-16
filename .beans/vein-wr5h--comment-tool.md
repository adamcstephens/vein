---
# vein-wr5h
title: comment tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:54Z
updated_at: 2026-03-16T03:19:52Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: add a comment to a task for progress notes and status updates.

## API Notes
- PUT /tasks/{taskID}/comments (not POST)
- Body: { comment: "text" }
- Response includes id, comment, author

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `comment` MCP tool with `CommentParams` (task_id + comment text)\n- Added `vein tool comment <task_id> <comment>` CLI subcommand\n- Calls Vikunja PUT /tasks/{id}/comments endpoint
