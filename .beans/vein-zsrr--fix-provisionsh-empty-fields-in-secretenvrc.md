---
# vein-zsrr
title: 'Fix provision.sh: empty fields in .secret.envrc'
status: completed
type: bug
priority: critical
created_at: 2026-03-16T13:37:33Z
updated_at: 2026-03-16T13:50:00Z
---

The provision script writes empty values to .secret.envrc because the login curl on line 16 uses username 'admin1' instead of 'admin' (which is what's created on line 10). The JWT comes back empty, cascading into all subsequent API calls returning empty values.


## Summary of Changes

Three bugs found and fixed in provision.sh:

1. **Username typo**: Login payload used `admin1` instead of `admin` (line 16) — JWT always came back empty
2. **Invalid permission group**: Token requested `projects_views_buckets_tasks` which doesn't exist in Vikunja — should be `views_buckets_tasks` under the `projects` group
3. **Silent failures**: Added `set -euo pipefail`, JWT validation, token validation, and project/bucket validation so empty values are caught early instead of written to .secret.envrc

Also added:
- `read_one` and `views_buckets` to projects permissions
- `projects_views_tasks` (`read_all`) for task listing by view
- Stale envrc clearing when a new user is created
- Updated README permission tables to match
