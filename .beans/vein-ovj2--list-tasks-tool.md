---
# vein-ovj2
title: list_tasks tool
status: completed
type: feature
priority: normal
created_at: 2026-03-16T01:56:27Z
updated_at: 2026-03-16T03:51:38Z
blocked_by:
    - vein-rnzz
---

Generic task listing/search tool for the MCP server. Allows agents to browse tasks broadly — filter by status, label, project, bucket, search text, etc. Unlike list_ready (which is opinionated about actionable work), this is a general-purpose query tool.

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)

## Summary of Changes\n\n- Added `list_tasks` MCP tool with `ListTasksParams` (optional filter expression and search text)\n- Added `list_view_tasks` method to `VikunjaClient` trait and `ReqwestClient`\n- Added `vein tool list-tasks [-f filter] [-s search]` CLI subcommand\n- Updated README and CHANGELOG
