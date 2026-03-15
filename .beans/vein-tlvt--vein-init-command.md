---
# vein-tlvt
title: vein init command
status: todo
type: feature
priority: normal
created_at: 2026-03-15T20:18:01Z
updated_at: 2026-03-15T21:10:17Z
parent: vein-rnzz
blocked_by:
    - vein-ovz8
    - vein-258m
    - vein-yjct
    - vein-vvb1
    - vein-xltj
---

CLI subcommand: connect to Vikunja using VIKUNJA_URL + VIKUNJA_API_TOKEN, list available projects and their buckets, and print the env var block the user needs to configure the MCP server. MVP: no creation, just discovery and output.

## API Notes
- GET /projects/{project}/views to list views (find kanban view)
- GET /projects/{id}/views/{view}/buckets to list buckets
- Bucket fields: id, title
- View has view_kind field (look for 'kanban')
