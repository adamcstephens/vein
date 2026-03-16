---
# vein-5pbi
title: Add dev Vikunja process-compose and provisioning just task
status: completed
type: task
priority: normal
created_at: 2026-03-15T23:36:53Z
updated_at: 2026-03-16T00:11:26Z
---

Create a process-compose configuration for running a dev Vikunja instance, and a just task to start it and provision users.

## Plan

- [x] Create `.services/vikunja/.gitignore` ignoring `vikunja.db` and `files/`
- [x] Create `.services/vikunja/config.yml` with minimal dev config (SQLite at `.services/vikunja/vikunja.db`, public URL `http://localhost:3456`, CORS enabled)
- [x] Create `process-compose.yml` with two processes:
  - `vikunja`: runs `vikunja web` with config pointing to `.services/vikunja/config.yml`
  - `provision`: depends on `vikunja`, checks if admin user exists via `vikunja user list`, creates with `vikunja user create` if missing, then exits (not restarted)
- [x] Add `dev` recipe to justfile that runs `process-compose up`
- [x] Add `PC_CONFIG_FILES` to `.envrc` pointing at `process-compose.yml`
- [x] Update CHANGELOG

## Summary of Changes

- Created `.services/vikunja/config.yml` — minimal dev Vikunja config (SQLite, localhost:3456, CORS)
- Created `.services/vikunja/.gitignore` — ignores `vikunja.db` and `files/`
- Created `process-compose.yml` — runs Vikunja and a provision process that idempotently creates an admin user and a scoped API token (written to `.secret.envrc`)
- Added `just dev` recipe to start everything
- Added `PC_CONFIG_FILES` and `VIKUNJA_URL` to `.envrc`
- Created `README.md` documenting dev setup, commands, and API token scope
- Updated CHANGELOG
