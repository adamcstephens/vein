---
# vein-ybfg
title: blocked-by awareness in list_ready
status: todo
type: feature
created_at: 2026-03-16T04:25:03Z
updated_at: 2026-03-16T04:25:03Z
---

list_ready currently just reads the Todo bucket. It should filter out tasks that have blocked_by relations pointing to incomplete tasks, so agents don't pick up blocked work.
