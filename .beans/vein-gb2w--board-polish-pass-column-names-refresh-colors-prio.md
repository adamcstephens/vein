---
# vein-gb2w
title: 'Board: polish pass — column names, refresh, colors, priorities'
status: completed
type: task
priority: normal
created_at: 2026-03-31T23:54:21Z
updated_at: 2026-04-01T00:01:43Z
---

1. Use Vikunja bucket names instead of hardcoded Ready/In Progress/Done. 2. Ensure create/edit triggers board refresh. 3. Fix highlight colors for light themes. 4. Colorize priority indicators in board list items.


## Summary of Changes

- Added column_names to BoardState, populated from Vikunja bucket titles
- Colorized priority indicators: red+bold (urgent), red (high), yellow (medium), blue (low)
- Styled task IDs in DarkGray, labels in magenta using Spans
- Highlight style: black fg on dark gray bg with bold, removed shifting caret symbol
- Added r keybind for manual board refresh
- Updated status bar hints
