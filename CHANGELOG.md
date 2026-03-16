# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- `list_in_progress` MCP tool — list tasks currently being worked on
- `list_done` MCP tool — list completed tasks
- `create_task` MCP tool — create a new task with title and optional description
- `list_ready` MCP tool — list tasks in the Todo bucket
- MCP stdio server (run with `vein serve` or just `vein`)
- `vein list-projects` — list available Vikunja projects
- `vein list-project-views <project_id>` — list views for a project
- `vein list-project-view-buckets <project_id> <view_id>` — list buckets for a view
- `vein init` — interactive setup: select project, kanban view, and buckets, then print env vars
- `just dev` — dev Vikunja via process-compose with auto-provisioned admin user
