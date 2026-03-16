---
# vein-wx45
title: list_in_progress tool
status: todo
type: feature
priority: normal
created_at: 2026-03-16T02:01:18Z
updated_at: 2026-03-16T02:47:55Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
---

MCP tool: list tasks in the In Progress bucket. Returns task summaries showing what is currently being worked on.

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)
