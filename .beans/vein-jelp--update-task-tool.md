---
# vein-jelp
title: update_task tool
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:17:54Z
updated_at: 2026-03-16T02:47:56Z
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

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)
