---
# vein-g9mh
title: 'Board: full edit screen for task creation'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T21:20:20Z
updated_at: 2026-03-31T21:39:11Z
---

Replace the single-field title input with a multi-field form for task creation in the board TUI. Should support title, description, and label selection. Design TBD — could be full-screen form or stacked overlay with tab navigation.


## Summary of Changes

- Replaced simple title input with full `CreateForm` struct: title, description, priority, labels
- Added `FormField` enum and Tab/Shift+Tab navigation between fields
- Priority field cycles with left/right arrows through None/Low/Medium/High/Urgent
- Labels fetched from Vikunja on form open, toggled with space, applied after task creation
- Ctrl+S saves (creates task + adds labels + refreshes board)
- Esc prompts "Discard changes? y/n" if form is non-empty, immediate cancel if empty
- Added `ConfirmDiscard` mode for the discard dialog
