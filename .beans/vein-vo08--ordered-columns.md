---
# vein-vo08
title: ordered columns
status: completed
type: task
priority: normal
created_at: 2026-03-28T04:11:57Z
updated_at: 2026-03-28T04:27:00Z
---

## Context

The Vikunja API returns a `position` field (float64) on tasks when fetched through a view endpoint. This field represents the task's position within its bucket/column in the Kanban view. Tasks can be sorted by position using `sort_by=position&order_by=asc` query parameters.

Currently, vein's list commands (`list-ready`, `list-in-progress`, `list-done`, `list-tasks`) do not capture or use this position field, so items are returned in whatever default order the API provides — not the order the user arranged them in the Kanban board.

## Plan

- [x] Add `position` field (f64) to `Task` struct
- [x] Add `sort_by` and `order_by` query params to `list_bucket_tasks` and `list_view_tasks` API calls, defaulting to `position` + `asc`
- [x] Fix `list_bucket_tasks` deserialization — it uses the view endpoint which returns `Vec<Bucket>`, not `Vec<Task>`, so it should flatten like `list_view_tasks` does
- [x] Sort tasks by position in the returned results (client-side sort as safety net)
- [x] Add tests for position-based ordering
- [x] Update CHANGELOG

## Summary of Changes

- Added `position: f64` field to `Task` struct to capture Vikunja's per-view task ordering
- `list_bucket_tasks` and `list_view_tasks` now request `sort_by=position&order_by=asc` from the API
- Fixed `list_bucket_tasks` deserialization — was incorrectly parsing `Vec<Task>` from an endpoint that returns `Vec<Bucket>`; now correctly flattens bucket-wrapped responses via shared `flatten_buckets` helper
- All list methods (`list_ready`, `list_in_progress`, `list_done`, `list_tasks`) apply client-side `sort_by_position` as a safety net
- Added 5 new tests (1 deserialization, 4 ordering)
