---
# vein-u2tw
title: create_task tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:51Z
updated_at: 2026-03-16T02:47:55Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: create a new task in the Todo bucket with title, description, and optional labels.

## API Notes
- PUT /projects/{id}/tasks (not POST) to create
- Send title, description in body

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)
