---
# vein-uwfh
title: 'Board: edit existing task via form'
status: completed
type: feature
priority: normal
created_at: 2026-03-31T21:39:36Z
updated_at: 2026-03-31T22:22:05Z
---

Add a keybind (e) to open the form overlay pre-populated with the selected task's current title, description, priority, and labels. Ctrl+S saves changes via ProjectClient::update_task() and add_label(). Reuses the CreateForm structure.


## Summary of Changes

- Renamed CreateForm to TaskForm, added editing_task_id and original_label_ids fields
- Added TaskForm::from_task() to pre-populate from existing task (HTML→markdown for description)
- Added Mode::EditTask, e keybind, generalized close_form()/try_close_form()
- Ctrl+S in edit mode calls update_task() + adds new labels
- ConfirmDiscard returns to correct form mode
- Form title dynamically shows "New Task" or "Edit Task"
