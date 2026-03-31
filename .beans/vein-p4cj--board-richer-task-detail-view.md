---
# vein-p4cj
title: 'Board: richer task detail view'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T22:22:13Z
updated_at: 2026-03-31T22:27:07Z
---

The task detail overlay currently shows format_task_detail() output as plain text. Enhance it to show more structured data: comments, assignees with highlighting, relation links, timestamps, and better visual formatting using ratatui styling rather than raw markdown text.


## Summary of Changes

- Replaced format_task_detail() plain text with build_detail_lines() producing styled ratatui Lines
- Status: green for Done, yellow for Open
- Priority: red+bold for Urgent, red for High, yellow for Medium, blue for Low
- Labels: magenta
- Assignees: cyan, shows name (falls back to username)
- Relations: bold section header, completed relations dimmed with checkmark
- Description: bold header, HTML converted to markdown, rendered line-by-line
- Title bar shows task ID and name instead of generic "Task Detail"
- Added e keybind from detail view to jump directly to edit form
- Removed unused format_task_detail import
