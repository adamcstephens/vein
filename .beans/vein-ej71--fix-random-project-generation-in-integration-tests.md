---
# vein-ej71
title: Fix random project generation in integration tests
status: completed
type: bug
priority: normal
created_at: 2026-03-16T03:13:29Z
updated_at: 2026-03-16T03:20:46Z
---

Integration tests in tests/mcp_integration.rs use hardcoded project names and rely on bucket index ordering (buckets[0], buckets[1], buckets[2]) which is non-deterministic. Projects should use unique random names and bucket assignment should be deterministic.

## Summary of Changes

- Test projects now use unique names via PID+timestamp suffix
- Bucket assignment uses title matching instead of index position
- Updated CHANGELOG
