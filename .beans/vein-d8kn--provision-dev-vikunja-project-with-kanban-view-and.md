---
# vein-d8kn
title: Provision dev Vikunja project with kanban view and buckets
status: completed
type: task
priority: normal
created_at: 2026-03-16T00:14:52Z
updated_at: 2026-03-16T00:16:47Z
---

Extend the process-compose provision process to create a dev project with a kanban view and standard buckets (Backlog, In Progress, Done) via the API.

## Summary of Changes

- Extended process-compose provision to create a `vein-dev` project with kanban view via the API
- Writes `VIKUNJA_PROJECT_ID` and `VIKUNJA_VIEW_ID` to `.secret.envrc`
- Idempotent — skips if env vars already present
- Updated README with project provisioning details
