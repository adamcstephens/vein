---
# vein-542b
title: Treat bare numeric task refs as project index
status: completed
type: task
priority: normal
created_at: 2026-03-28T22:12:36Z
updated_at: 2026-03-28T22:15:39Z
---

Make bare numeric IDs (e.g. '42') resolve by project index, same as '#42'. Removes global ID escape hatch to prevent cross-project contamination. TaskRef::Id variant gets removed.

## Summary of Changes\n\nRemoved `TaskRef::Id` variant. Both bare numeric (`42`) and hash-prefixed (`#42`) references now parse as `TaskRef::Index` and resolve by project index. No global Vikunja IDs are exposed.
