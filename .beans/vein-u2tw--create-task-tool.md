---
# vein-u2tw
title: create_task tool
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:17:51Z
updated_at: 2026-03-16T03:02:37Z
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

## Summary of Changes\n\n- Added `create_task` MCP tool to VeinServer with `CreateTaskParams` (title + optional description)\n- Added `vein tool create-task` CLI subcommand\n- Wired up CLI to call the Vikunja API client's `create_task` method\n- Unit tests for CLI parsing (with and without description flag)\n- All existing tests pass
