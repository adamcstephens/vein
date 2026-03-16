---
# vein-wx45
title: list_in_progress tool
status: completed
type: feature
priority: normal
created_at: 2026-03-16T02:01:18Z
updated_at: 2026-03-16T03:12:50Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: list tasks in the In Progress bucket. Returns task summaries showing what is currently being worked on.

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `list_in_progress` MCP tool and `vein tool list-in-progress` CLI subcommand\n- Queries the In Progress bucket and formats results with `format_task_list`
