# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- Extract `ProjectClient` domain layer: project-scoped operations (resolve, claim, complete, comment, etc.) are now centralized instead of duplicated between MCP server and CLI

### Added

- Project identifier support: tasks display as `VEIN-3` instead of `#17` when the project has an identifier configured
- All task-referencing tools and CLI commands accept identifiers (e.g. `VEIN-3`) or numeric IDs

### Fixed

- `list_tasks` MCP tool no longer errors when called without filters
- Integration tests use drop guard to clean up projects even on failure
- Provision script: fix login username typo and invalid API token permission group
- Provision script: add error handling to prevent writing empty values to `.secret.envrc`
- Integration tests run sequentially to avoid Vikunja SQLite concurrency issues
- `update_task` no longer zeroes out fields not included in the update (e.g. description, priority)
- Integration tests use unique project names and match buckets by title instead of index

### Added

- Markdown-to-HTML conversion for task descriptions and comments sent to Vikunja
- HTML-to-markdown conversion for task descriptions returned to agents
- `orient` MCP prompt — agent orientation with available tools, workflow guidance, and current task state
- Priority support on `create_task` and `update_task` (none, low, medium, high, urgent)
- `create_label` MCP tool — create a new label
- `add_label` MCP tool — assign a label to a task
- `list_labels` MCP tool — list all available labels
- `list_ready` now filters out tasks blocked by incomplete tasks
- `list_tasks` MCP tool — list and search tasks with optional filters
- `update_task` MCP tool — update a task's title or description
- `add_relation` MCP tool — add a relation between two tasks
- `complete` MCP tool — mark a task as done
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
