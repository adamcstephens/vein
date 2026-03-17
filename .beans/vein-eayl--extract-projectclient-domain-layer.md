---
# vein-eayl
title: Extract ProjectClient domain layer
status: completed
type: task
priority: normal
created_at: 2026-03-17T04:47:15Z
updated_at: 2026-03-17T13:06:17Z
---

Refactor to eliminate duplicated task resolution and project-scoped operations across server.rs, main.rs, and tests.

## Problem

Task identifier resolution (parsing refs like VEIN-3, resolving to numeric IDs, calling the client) is duplicated in three places:
- `VeinServer::resolve()` in server.rs
- `resolve!()` macro in main.rs  
- Manual calls in integration tests

Higher-level operations (get_task, claim, complete, comment, etc.) are also duplicated between server.rs MCP handlers and main.rs CLI handlers — both parse params, resolve refs, call the same client methods, and format the same output.

## Approach

Extract a `ProjectClient` that wraps `VikunjaClient` + `ProjectConfig` and provides project-scoped operations that accept string task refs:

- `get_task(&str) -> Result<Task>`
- `claim(&str) -> Result<Task>`
- `complete(&str) -> Result<Task>`
- `comment(&str, &str) -> Result<()>`
- `create_task(...) -> Result<Task>`
- `update_task(&str, ...) -> Result<Task>`
- `add_relation(&str, &str, &str) -> Result<TaskRelation>`
- `add_label(&str, i64) -> Result<()>`
- etc.

Resolution lives inside `ProjectClient`. VeinServer and main.rs CLI become thin adapters that parse MCP/CLI params and delegate to `ProjectClient`.

Integration tests can call `ProjectClient` methods directly.

## Todo

- [x] Create `ProjectClient` struct wrapping client + config
- [x] Move `resolve()` into `ProjectClient`
- [x] Move project-scoped operations into `ProjectClient`
- [x] Simplify `VeinServer` to delegate to `ProjectClient`
- [x] Simplify main.rs CLI handlers to delegate to `ProjectClient`
- [x] Update integration tests to use `ProjectClient` where appropriate

## Summary of Changes

Extracted `ProjectClient<C: VikunjaClient>` in `src/project.rs` that wraps a client + config and provides all project-scoped operations. VeinServer and main.rs CLI are now thin adapters. The resolve!() macro and duplicated logic are eliminated. 10 unit tests cover the new module.
