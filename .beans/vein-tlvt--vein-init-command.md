---
# vein-tlvt
title: vein init command
status: completed
type: feature
priority: normal
created_at: 2026-03-15T20:18:01Z
updated_at: 2026-03-15T21:48:08Z
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

## Design

- Interactive selection using dialoguer
- Flow: select project → auto-find kanban view → select buckets (todo, in-progress, done)
- Print env var block at the end

## Summary of Changes

- Interactive init flow using dialoguer Select prompts
- Auto-filters to kanban views, prompts only if multiple
- Prints env var block for VIKUNJA_PROJECT_ID, VIKUNJA_VIEW_ID, and bucket IDs
