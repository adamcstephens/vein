---
# vein-vvb1
title: Add VIKUNJA_VIEW_ID to config
status: scrapped
type: task
priority: normal
created_at: 2026-03-15T21:10:11Z
updated_at: 2026-03-15T21:20:42Z
parent: vein-rnzz
---

Add VIKUNJA_VIEW_ID environment variable to Config struct. The kanban view ID is needed to list tasks by bucket via GET /projects/{id}/views/{view_id}/tasks.

## Reasons for Scrapping

Absorbed into vein-ndpw (split config into connection and project config), which includes VIKUNJA_VIEW_ID as part of ProjectConfig.
