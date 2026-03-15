---
# vein-ovz8
title: Configuration & environment variables
status: completed
type: task
priority: normal
created_at: 2026-03-15T20:17:19Z
updated_at: 2026-03-15T20:33:13Z
parent: vein-rnzz
---

Set up configuration loading from environment variables: VIKUNJA_URL, VIKUNJA_API_TOKEN, VIKUNJA_PROJECT_ID, VIKUNJA_TODO_BUCKET_ID, VIKUNJA_INPROGRESS_BUCKET_ID, VIKUNJA_DONE_BUCKET_ID. Validate at startup.

## Summary of Changes

- Added `Config` struct in `src/config.rs` with 6 fields (url, token as String; project/bucket IDs as i64)
- `Config::from_env()` reads from real environment variables
- `Config::load(lookup)` accepts a closure for testability without unsafe env mutation
- Collects and reports all missing/invalid variables at once
- 3 tests: valid config, all missing, invalid integers
