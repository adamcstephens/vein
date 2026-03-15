# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Configuration module for loading Vikunja settings from environment variables
  - `Config::from_env()` reads `VIKUNJA_URL`, `VIKUNJA_API_TOKEN`, `VIKUNJA_PROJECT_ID`, `VIKUNJA_TODO_BUCKET_ID`, `VIKUNJA_INPROGRESS_BUCKET_ID`, `VIKUNJA_DONE_BUCKET_ID`
  - `Config::load()` accepts a custom lookup function for testability
  - Reports all missing/invalid variables at once instead of failing on the first error
