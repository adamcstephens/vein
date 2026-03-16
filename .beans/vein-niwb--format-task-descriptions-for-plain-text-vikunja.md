---
# vein-niwb
title: Format task descriptions for plain-text Vikunja
status: completed
type: task
priority: normal
created_at: 2026-03-16T13:54:24Z
updated_at: 2026-03-16T14:24:52Z
---

Vikunja task descriptions don't appear to support markdown rendering. When vein writes descriptions (e.g. via create_task or update_task), the output should be formatted for plain text — no markdown syntax that won't render properly.

## Summary of Changes\n\nAdded bidirectional markdown/HTML conversion for Vikunja compatibility:\n- Descriptions and comments written via create_task, update_task, and comment are converted from markdown to HTML using pulldown-cmark\n- Descriptions read back via get_task are converted from HTML to markdown using htmd\n- Both MCP server and CLI paths are covered\n- Round-trip tests verify conversion fidelity
