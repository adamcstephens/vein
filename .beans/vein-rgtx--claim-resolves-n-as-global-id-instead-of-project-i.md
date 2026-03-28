---
# vein-rgtx
title: 'claim resolves #N as global ID instead of project index'
status: completed
type: bug
priority: normal
created_at: 2026-03-28T21:26:33Z
updated_at: 2026-03-28T21:30:22Z
---

When a project has no identifier prefix, display_id() shows #{id} (global Vikunja ID). TaskRef::parse("#2") treats this as global ID 2. But in a multi-project instance, global ID 2 may be a different task. Fix: display_id() should use index when available, and #N should resolve by project index (like VEIN-N does).

## Summary of Changes\n\nAdded `TaskRef::Index` variant so `#N` resolves by project index instead of global Vikunja ID. Updated `display_id()` to prefer index over global ID when no project identifier is set. Updated CLI and MCP tool descriptions to document the `#N` index format.
