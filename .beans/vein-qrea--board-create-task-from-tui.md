---
# vein-qrea
title: 'Board: create task from TUI'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T20:59:41Z
updated_at: 2026-03-31T21:13:09Z
---

Add a keybind (c or n) to create a new task from within the board TUI. Needs to capture a title (and optionally description/priority) via an input prompt, then call ProjectClient::create_task(). New task appears in the Ready column on next poll.


## Summary of Changes

- Added `Mode` enum (`Board`, `Detail`, `CreateTask`) to replace ad-hoc mode checks
- Added `create_input` field and `start_create()`/`cancel_create()` methods to App
- Create overlay: centered input box with cursor, green border, title prompt
- `c` keybind opens input, Enter submits (async via tokio::spawn), Esc cancels
- Empty titles are rejected (no API call)
- Board refreshes immediately after successful creation via `AppEvent::TaskCreated`
- Errors shown in status bar via `AppEvent::CreateError`
- Updated status bar hints to show `c: create`
