---
# vein-lfng
title: Support project identifiers (e.g. VEIN-3) for task references
status: completed
type: feature
priority: normal
created_at: 2026-03-17T04:01:49Z
updated_at: 2026-03-17T04:20:35Z
---

Add support for Vikunja's project identifier system. Currently vein uses raw numeric task IDs everywhere, but Vikunja assigns human-friendly identifiers like VEIN-3 (project identifier + index).

## Changes needed

- [x] Add `identifier` field to Project struct
- [x] Add `identifier` and `index` fields to Task struct
- [x] Display identifiers in task list and detail output (e.g. VEIN-3 instead of #17)
- [x] Accept identifier strings (e.g. VEIN-3) as task references in CLI and MCP tools
- [x] Resolve identifiers to numeric IDs via `filter=index=N` API query
- [x] Keep backward compat: also accept raw numeric IDs

## API details

- Project has `identifier` field (e.g. "VEIN")
- Task has `identifier` (e.g. "VEIN-3") and `index` (e.g. 3) fields
- No direct lookup by identifier; resolve via `GET /projects/{id}/tasks?filter=index=N`
- Tasks without a project identifier have empty `identifier` field

## Summary of Changes

Added project identifier support throughout vein. Tasks now display as VEIN-3 instead of #17 when the project has an identifier. All task-referencing CLI commands and MCP tools accept both identifier strings (VEIN-3) and numeric IDs (42). Resolution uses the Vikunja project tasks API with index filter.
