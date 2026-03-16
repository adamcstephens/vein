---
# vein-j69k
title: label management
status: completed
type: feature
priority: normal
created_at: 2026-03-16T04:25:02Z
updated_at: 2026-03-16T05:47:08Z
---

Add MCP tools and CLI subcommands to create and assign labels to tasks. Vikunja labels serve the role of beans' types (bug, feature, task, etc).

## Summary of Changes

Added label management: create_label, add_label, and list_labels MCP tools and CLI subcommands. Updated the Vikunja client trait with create_label, add_label_to_task, and list_labels methods. Updated dev provisioning to include labels and tasks_labels permissions on the API token. Updated README with new permission requirements.
