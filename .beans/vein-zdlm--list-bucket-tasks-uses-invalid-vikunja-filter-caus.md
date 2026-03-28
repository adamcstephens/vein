---
# vein-zdlm
title: list_bucket_tasks uses invalid Vikunja filter, causing flaky test
status: completed
type: bug
priority: normal
created_at: 2026-03-28T21:54:16Z
updated_at: 2026-03-28T21:55:42Z
---

list_bucket_tasks passes filter=bucket_id={id} to Vikunja task filter DSL, but bucket_id is a view-level concept not a task filter field. Fix: fetch all view buckets and filter client-side.

## Summary of Changes\n\nRemoved invalid `bucket_id` filter from `list_bucket_tasks` API call. Now fetches all view buckets and filters to the target bucket client-side. Fixes flaky `claim_moves_task_to_in_progress_bucket` integration test.
