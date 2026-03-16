---
# vein-c4z5
title: list_mine tool
status: scrapped
type: feature
priority: normal
created_at: 2026-03-15T20:17:46Z
updated_at: 2026-03-16T01:48:32Z
parent: vein-rnzz
blocked_by:
    - vein-258m
    - vein-5ebd
    - vein-rbmw
---

MCP tool: list tasks in the In Progress bucket claimed by this agent instance. Uses agent identity from startup.

## API Notes
- List tasks via GET /projects/{id}/views/{view_id}/tasks with filter query param
- Filter by bucket_id for In Progress bucket, then filter by assignee matching agent identity

## Reasons for Scrapping
Agent identity tracking deferred — single shared Vikunja user means there's no per-agent identity to resolve or filter by.
