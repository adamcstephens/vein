---
# vein-m2jw
title: Implement delete_project tool
status: completed
type: feature
priority: normal
created_at: 2026-03-16T13:16:40Z
updated_at: 2026-03-16T23:39:22Z
---

Add a delete_project capability to vein and use it in integration tests for cleanup. This will allow tests to properly tear down test projects instead of leaving them around.

## Plan\n- [x] Add delete_project function to client\n- [x] Use it in integration test cleanup\n- [x] Tests pass, lints pass

## Summary of Changes\n\nReplaced manual cleanup() calls in integration tests with a Drop guard on TestProject. Now test project cleanup runs even when tests panic or fail mid-assertion, preventing stale projects from accumulating in Vikunja.
