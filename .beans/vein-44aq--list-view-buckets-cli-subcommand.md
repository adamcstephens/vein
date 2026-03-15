---
# vein-44aq
title: list-view-buckets CLI subcommand
status: completed
type: task
priority: normal
created_at: 2026-03-15T21:22:44Z
updated_at: 2026-03-15T21:37:28Z
parent: vein-rnzz
blocked_by:
    - vein-ndpw
---

Add 'vein list-project-view-buckets <project_id> <view_id>' CLI subcommand. Lists buckets for a kanban view via GET /projects/{id}/views/{view_id}/buckets. Shows bucket ID and title. Only needs ConnectionConfig.

## Summary of Changes

- Added `list-project-view-buckets` CLI subcommand with project_id and view_id arguments
- Outputs tab-separated bucket ID and title
