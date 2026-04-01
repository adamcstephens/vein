---
# vein-9or4
title: 'Board: move tasks between columns'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T20:48:27Z
updated_at: 2026-04-01T14:06:31Z
---

Add keybinds to move tasks between kanban columns in the board TUI. Use existing ProjectClient::claim() and complete() to move tasks to In Progress and Done respectively. Likely m or Enter to trigger, with column context determining the action.


## Implementation Plan

### 1. Add `move_to_column()` to ProjectClient
- `move_to_column(task_ref, bucket_id)` resolves task, auto-manages `done` flag based on target bucket (done=true for done_bucket_id, done=false otherwise), moves task
- Refactor `claim()` and `complete()` to use it

### 2. Board move mode
- `m` enters move mode, Esc/Enter exits
- `h/l` moves selected task left/right across columns via `move_to_column()`
- Visual indicator (yellow borders) when in move mode
- Immediate refresh after move


## Summary of Changes

- Added `move_to_column(task_ref, bucket_id)` to ProjectClient — auto-manages done flag, refactored claim/complete to use it
- Added `update_task_position(task_id, view_id, position)` to VikunjaClient — POST /tasks/{id}/position for kanban view positioning
- Board move mode: m to enter, j/k reorder locally, h/l move across columns, Enter commits (midpoint position calc), Esc reverts
- Yellow column borders in move mode, task follows cursor on cross-column move
- Replaced broken tokio::spawn background poller with inline timer (current_thread runtime fix)
- Added tasks_position permission to API token provisioning
- Integration tests: position reorder (adjacent swap + middle insert), move_to_column (bucket + done flag)
- Unit tests: move_to_column with mock (inprogress, done, todo transitions)
