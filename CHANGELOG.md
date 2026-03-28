# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- List commands return tasks ordered by column position (matching Kanban board order)

### Fixed

- `list_bucket_tasks` now correctly deserializes bucket-wrapped API responses

## [0.3.0] - 2026-03-21

### Changed

- Flatten CLI: all commands are now top-level (e.g. `vein list-ready` instead of `vein tool list-ready`)
- Extract `ProjectClient` domain layer for project-scoped operations

### Added

- Shell completion support via `vein completions <shell>` (fish, bash, zsh)
- Nix package installs shell completions automatically
- Project identifier support: tasks display as `VEIN-3` instead of `#17`
- All task-referencing commands accept identifiers (e.g. `VEIN-3`) or numeric IDs
- Markdown-to-HTML and HTML-to-markdown conversion for task descriptions and comments
- `orient` MCP prompt for agent orientation
- Priority support on `create_task` and `update_task`
- MCP tools: `create_label`, `add_label`, `list_labels`, `list_tasks`, `update_task`, `add_relation`
- `list_ready` filters out tasks blocked by incomplete tasks

### Fixed

- `list_tasks` no longer errors when called without filters
- `update_task` no longer zeroes out fields not included in the update
- Provision script fixes for login and error handling

## [0.2.0] - 2026-03-18

### Added

- `complete` MCP tool — mark a task as done
- `claim` MCP tool — claim a task by moving it to In Progress
- `comment` MCP tool — add a comment to a task
- `get_task` MCP tool — get full task details by ID
- `list_in_progress` MCP tool — list tasks currently being worked on
- `list_done` MCP tool — list completed tasks
- `create_task` MCP tool — create a new task with title and optional description
- `list_ready` MCP tool — list tasks in the Todo bucket
- MCP stdio server (run with `vein serve` or just `vein`)
- `vein list-projects`, `vein list-project-views`, `vein list-project-view-buckets`
- `vein init` — interactive setup
- `just dev` — dev Vikunja via process-compose
