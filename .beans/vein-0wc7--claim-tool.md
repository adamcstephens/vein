---
# vein-0wc7
title: claim tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:49Z
updated_at: 2026-03-16T03:27:00Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: claim a task by moving it to the In Progress bucket.

## API Notes
- Move to bucket: POST /tasks/{id} with bucket_id field

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `claim` MCP tool using `TaskIdParams`, moves task to In Progress bucket\n- Added `vein tool claim <task_id>` CLI subcommand\n- Uses `update_task` with `bucket_id` set to `inprogress_bucket_id`
