---
# vein-0hc9
title: complete tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:49Z
updated_at: 2026-03-16T03:30:15Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: mark a task as done by moving it to the Done bucket.

## API Notes
- POST /tasks/{id} with done: true and bucket_id set to Done bucket
- Vikunja uses POST (not PUT/PATCH) for task updates

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `complete` MCP tool using `TaskIdParams`, sets done=true and moves to Done bucket\n- Added `vein tool complete <task_id>` CLI subcommand
