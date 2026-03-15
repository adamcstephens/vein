---
# vein-uft3
title: list-project-views CLI subcommand
status: completed
type: task
priority: normal
created_at: 2026-03-15T21:22:26Z
updated_at: 2026-03-15T21:33:45Z
parent: vein-rnzz
blocked_by:
    - vein-ndpw
---

Add 'vein list-project-views <project_id>' CLI subcommand. Lists views for a project via GET /projects/{id}/views. Shows view ID, title, and view_kind (list, kanban, gantt, table). Only needs ConnectionConfig.

## Summary of Changes

- Added `list-project-views` CLI subcommand with project_id argument
- Outputs tab-separated view ID, view_kind, and title
