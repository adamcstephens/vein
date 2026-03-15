---
# vein-ndpw
title: Split config into connection and project config
status: in-progress
type: task
priority: normal
created_at: 2026-03-15T21:19:30Z
updated_at: 2026-03-15T21:23:17Z
parent: vein-rnzz
---

Split Config into ConnectionConfig (VIKUNJA_URL, VIKUNJA_API_TOKEN) and ProjectConfig (VIKUNJA_PROJECT_ID, VIKUNJA_VIEW_ID, bucket IDs). CLI commands like list-projects only need ConnectionConfig. MCP server mode needs both. ReqwestClient::new should take ConnectionConfig.
