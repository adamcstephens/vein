---
# vein-xltj
title: Add list_projects to VikunjaClient
status: completed
type: task
priority: normal
created_at: 2026-03-15T21:10:13Z
updated_at: 2026-03-15T21:15:51Z
parent: vein-rnzz
---

Add list_projects method to VikunjaClient trait and ReqwestClient impl. GET /projects returns available projects. Needed by vein init for project discovery.

## Summary of Changes

- Added `Project` struct (id, title, description, is_archived)
- Added `list_projects` to `VikunjaClient` trait and `ReqwestClient` (GET /projects)
- 2 tests: JSON deserialization with and without optional fields
