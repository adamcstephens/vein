---
# vein-ndpw
title: Split config into connection and project config
status: completed
type: task
priority: normal
created_at: 2026-03-15T21:19:30Z
updated_at: 2026-03-15T21:25:55Z
parent: vein-rnzz
---

Split Config into ConnectionConfig (VIKUNJA_URL, VIKUNJA_API_TOKEN) and ProjectConfig (VIKUNJA_PROJECT_ID, VIKUNJA_VIEW_ID, bucket IDs). CLI commands like list-projects only need ConnectionConfig. MCP server mode needs both. ReqwestClient::new should take ConnectionConfig.

## Summary of Changes

- Split `Config` into `ConnectionConfig` (url, token) and `ProjectConfig` (project_id, view_id, bucket IDs)
- Added `VIKUNJA_VIEW_ID` to ProjectConfig (absorbs vein-vvb1)
- `ReqwestClient::new` now takes `&ConnectionConfig`
- CLI `list-projects` only requires ConnectionConfig
- 5 tests covering both config types
