---
# vein-5uj5
title: Document required Vikunja API token permissions
status: completed
type: task
priority: low
created_at: 2026-03-15T21:41:59Z
updated_at: 2026-03-16T03:44:35Z
parent: vein-rnzz
---

Enumerate the minimum Vikunja API token permissions needed for vein. Document in README or vein init output. Permissions not listed in swagger spec — check Vikunja UI/docs.

## Summary of Changes\n\n- Added user-facing Setup section to README.md documenting required API token permissions\n- Permissions mapped from actual HTTP calls: tasks (read_one, create, update), projects_views_tasks (read_all), tasks_relations (create), tasks_comments (create)\n- Noted vein init needs additional projects and projects_views read_all permissions
