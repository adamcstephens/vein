---
# vein-lvy1
title: set priority on create/update
status: completed
type: feature
priority: normal
created_at: 2026-03-16T04:25:01Z
updated_at: 2026-03-16T04:55:06Z
---

Allow setting task priority via `create_task` and `update_task` MCP tools and CLI subcommands. Agents need to triage work.

## Summary of Changes

Added optional priority parameter (none/low/medium/high/urgent) to create_task and update_task MCP tools and CLI subcommands. Priority strings are parsed to Vikunja integer values (0-4). Also added priority to the TaskUpdate and CreateTaskPayload structs in the client layer.
