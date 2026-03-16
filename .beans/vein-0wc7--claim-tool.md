---
# vein-0wc7
title: claim tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:49Z
updated_at: 2026-03-16T02:47:55Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: claim a task by moving it to the In Progress bucket.

## API Notes
- Move to bucket: POST /tasks/{id} with bucket_id field

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)
