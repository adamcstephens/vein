---
# vein-ovj2
title: list_tasks tool
status: todo
type: feature
priority: normal
created_at: 2026-03-16T01:56:27Z
updated_at: 2026-03-16T02:47:56Z
blocked_by:
    - vein-rnzz
---

Generic task listing/search tool for the MCP server. Allows agents to browse tasks broadly — filter by status, label, project, bucket, search text, etc. Unlike list_ready (which is opinionated about actionable work), this is a general-purpose query tool.

\n\n## Implementation Notes\n- Add a `vein tool <tool-name>` CLI subcommand that exercises the same logic as the MCP tool\n- Use the CLI subcommand for testing against the dev Vikunja instance (`just dev`)
