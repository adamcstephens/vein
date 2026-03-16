# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed

- Integration tests use unique project names and match buckets by title instead of index

### Added

- `claim` MCP tool — claim a task by moving it to In Progress
- `comment` MCP tool — add a comment to a task
- `get_task` MCP tool — get full task details by ID
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
