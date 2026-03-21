---
# vein-8oev
title: list_tasks errors out
status: completed
type: bug
priority: high
created_at: 2026-03-21T18:52:44Z
updated_at: 2026-03-21T19:09:22Z
---

## Description

The `list_tasks` MCP tool errors with "Failed to list tasks: HTTP error: error decoding response body" when called without filters.

### Root cause

The Vikunja API endpoint `GET /projects/{id}/views/{id}/tasks` returns **different response shapes** depending on whether a filter is provided:

- **With a filter** (e.g. `filter=bucket_id = 4`): returns `Vec<Task>` (flat task list)
- **Without a filter**: returns `Vec<Bucket>` where each bucket contains a `tasks` array:

```json
[
  {
    "id": 4,
    "title": "To-Do",
    "project_view_id": 8,
    "tasks": [ { "id": 5, "title": "test", ... } ],
    "limit": 0,
    "count": 1,
    ...
  },
  ...
]
```

The `list_view_tasks` method in `src/client.rs:466` always deserializes the response as `Vec<Task>`, which fails when the response is the bucket-grouped format.

### Affected code path

1. MCP tool `list_tasks` (`src/server.rs:406-418`)
2. → `ProjectClient::list_tasks` (`src/project.rs:194-202`)
3. → `ReqwestClient::list_view_tasks` (`src/client.rs:445-467`)
4. → `resp.json::<Vec<Task>>()` fails at line 466

### Why `list_bucket_tasks` works

`list_bucket_tasks` (`src/client.rs:429-443`) hits the same endpoint but always passes a `bucket_id` filter, so Vikunja returns the flat `Vec<Task>` format.

### Fix

`list_view_tasks` needs to handle the unfiltered case by either:
- Deserializing as `Vec<Bucket>` and flattening the tasks out, or
- Always passing a filter (e.g. `done = false && done = true` as a no-op) to force the flat response format


## Summary of Changes

- Added `tasks` field to `Bucket` struct in `src/client.rs`
- Updated `list_view_tasks` to detect whether a filter is present: with filter it deserializes the flat `Vec<Task>` response; without filter it deserializes the bucket-grouped `Vec<Bucket>` response and flattens tasks out, setting `bucket_id` from the containing bucket
- Added integration test `list_view_tasks_without_filter_returns_all_tasks` that creates tasks in different buckets and verifies the unfiltered response returns all tasks with correct `bucket_id` values
- Updated CHANGELOG
