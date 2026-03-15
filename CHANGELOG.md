# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Configuration module for loading Vikunja settings from environment variables
  - `Config::from_env()` reads `VIKUNJA_URL`, `VIKUNJA_API_TOKEN`, `VIKUNJA_PROJECT_ID`, `VIKUNJA_TODO_BUCKET_ID`, `VIKUNJA_INPROGRESS_BUCKET_ID`, `VIKUNJA_DONE_BUCKET_ID`
  - `Config::load()` accepts a custom lookup function for testability
  - Reports all missing/invalid variables at once instead of failing on the first error
- Vikunja REST API client (`src/client.rs`)
  - `VikunjaClient` trait with methods: `get_user`, `get_task`, `list_bucket_tasks`, `create_task`, `update_task`, `create_relation`, `create_comment`, `list_views`, `list_buckets`
  - `ReqwestClient` implementation using reqwest with bearer token auth
  - Response types: `User`, `Task`, `Label`, `TaskComment`, `TaskRelation`, `Bucket`, `ProjectView`
  - `ClientError` enum for HTTP and API errors
