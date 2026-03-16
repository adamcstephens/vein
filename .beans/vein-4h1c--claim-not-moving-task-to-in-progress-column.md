---
# vein-4h1c
title: claim not moving task to in-progress column
status: completed
type: bug
priority: high
created_at: 2026-03-16T13:10:36Z
updated_at: 2026-03-16T13:53:42Z
---

claim tool sends bucket_id for in-progress but task stays in the todo column in Vikunja kanban view.


## Summary of Changes

Root cause was the same as vein-zsrr: the API token requested the non-existent `projects_views_buckets_tasks` permission group, so the `POST /projects/:id/views/:view/buckets/:bucket/tasks` endpoint returned 401. Fixed by using `views_buckets_tasks` under the `projects` permission group in provision.sh. Integration test `claim_moves_task_to_in_progress_bucket` now passes.
