# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- MCP stdio server (run with `vein serve` or just `vein`)
- `vein list-projects` — list available Vikunja projects
- `vein list-project-views <project_id>` — list views for a project
- `vein list-project-view-buckets <project_id> <view_id>` — list buckets for a view
- `vein init` — interactive setup: select project, kanban view, and buckets, then print env vars
- `just dev` — dev Vikunja via process-compose with auto-provisioned admin user
