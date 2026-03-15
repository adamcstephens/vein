---
# vein-rbmw
title: Agent identity resolution
status: todo
type: task
created_at: 2026-03-15T20:17:34Z
updated_at: 2026-03-15T20:17:34Z
parent: vein-rnzz
blocked_by:
    - vein-ovz8
---

At startup, call GET /user with the configured API token to resolve the agent's user identity. Store in process state for use by claim/list_mine tools.
