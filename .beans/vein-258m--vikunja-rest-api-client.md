---
# vein-258m
title: Vikunja REST API client
status: completed
type: task
priority: normal
created_at: 2026-03-15T20:17:23Z
updated_at: 2026-03-15T20:51:31Z
parent: vein-rnzz
blocked_by:
    - vein-ovz8
---

HTTP client layer for Vikunja REST API. Handles authentication (bearer token), base URL construction, and typed request/response models. Covers endpoints needed by MCP tools: tasks, buckets, relations, comments, user info.

## Design Decisions

- Hand-written client covering only the endpoints our MCP tools need (not generated from swagger)
- Reference swagger spec at https://project.junco.dev/api/v1/docs.json for request/response shapes
- Trait-based: `VikunjaClient` trait with methods per API operation
- `ReqwestClient` struct implements the trait with real HTTP via reqwest
- Tests use hand-written mock impls of the trait — no mocking crate needed
- Add endpoints incrementally as MCP tools are built

## Summary of Changes

- Added `VikunjaClient` trait in `src/client.rs` with 9 async methods
- Added `ReqwestClient` struct implementing the trait with real HTTP via reqwest
- Response types: `User`, `Task`, `Label`, `TaskComment`, `TaskRelation`, `Bucket`, `ProjectView`
- `TaskUpdate` struct for partial updates, `ClientError` enum for errors
- Uses correct HTTP methods per Vikunja swagger spec (PUT for create task/relation/comment, POST for update task)
- Tests: mock impl, JSON deserialization, URL building
- Dependencies added: reqwest 0.13.2 (json, query), serde 1.0.228; dev-deps: tokio 1.50.0, serde_json 1.0.149
