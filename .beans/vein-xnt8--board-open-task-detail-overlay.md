---
# vein-xnt8
title: 'Board: open task detail overlay'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T20:48:32Z
updated_at: 2026-03-31T20:52:19Z
---

Add a keybind (Enter or o) to open a modal overlay showing full task details for the selected task in the board TUI. Use existing format_task_detail() or similar rendering. Esc to close overlay and return to board view.


## Summary of Changes

- Added `detail_task: Option<Task>` and `detail_scroll: u16` to board App state
- Added `selected_task()`, `open_detail()`, `close_detail()` methods
- Renders a centered overlay (75% of terminal) with task detail text, word-wrapped, scrollable with j/k
- Enter/o opens detail from board view, Esc/q closes overlay back to board
- Keybinds are modal: overlay captures all input except Esc/q/j/k
- Updated status bar hints to show `o: open`
