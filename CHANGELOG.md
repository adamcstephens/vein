# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Configuration module split into two structs (`src/config.rs`)
  - `ConnectionConfig`: `VIKUNJA_URL`, `VIKUNJA_API_TOKEN` — sufficient for CLI discovery commands
  - `ProjectConfig`: `VIKUNJA_PROJECT_ID`, `VIKUNJA_VIEW_ID`, `VIKUNJA_TODO_BUCKET_ID`, `VIKUNJA_INPROGRESS_BUCKET_ID`, `VIKUNJA_DONE_BUCKET_ID` — needed for MCP server mode
  - Both have `from_env()` and `load()` with testable lookup function
  - Reports all missing/invalid variables at once instead of failing on the first error
- Vikunja REST API client (`src/client.rs`)
  - `VikunjaClient` trait with methods: `get_user`, `get_task`, `list_bucket_tasks`, `create_task`, `update_task`, `create_relation`, `create_comment`, `list_views`, `list_buckets`
  - `ReqwestClient` implementation using reqwest with bearer token auth
  - Response types: `User`, `Task`, `Label`, `TaskComment`, `TaskRelation`, `Bucket`, `ProjectView`
  - `ClientError` enum for HTTP and API errors
- MCP stdio server transport (`src/server.rs`)
  - `VeinServer` struct with rmcp `ToolRouter` and `ServerHandler` implementation
  - Stdio transport via `rmcp::transport::io::stdio()`
  - `main.rs` wired up with tokio `current_thread` runtime
- CLI framework with clap (`src/cli.rs`)
  - `vein init` — discover Vikunja projects/buckets (stub)
  - `vein serve` — run as MCP stdio server
  - No subcommand defaults to serve mode
- `list_projects` method on `VikunjaClient` trait and `ReqwestClient` impl
  - `Project` response type with id, title, description, is_archived
- `vein list-projects` CLI subcommand — lists non-archived Vikunja projects
