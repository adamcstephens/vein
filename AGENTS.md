## Agent workflow
- **IMPORTANT**: before you do anything else, run the `beans prime` command and heed its output.
- When asked to work with veins, invoke the vein `orient` MCP prompt and heed its output with `/mcp__vein__orient`.
- Always use red/green TDD when implementing
- Always format code with `just format`
- Always check code linting with `just lint`
- Always use commit messages following conventional commits, with ticket id in the body
- When new context is decided, always update the associated work ticket with the details

## Definition of done
- tests pass
- lints pass
- you've checked any updated capabilities using `cargo run -- tool <name>`
- nix build passes (background recommended)
- changelog updated (`just changelog` to preview, git-cliff generates CHANGELOG.md during release)
- code committed with all ticket changes included
- stop and ask for approval
- ticket marked done

## Code style
- No `unwrap()` outside of tests — propagate with `?` or `ok_or_else`
- Keep `Option`/`Result` as long as possible — don't collapse to sentinel values (e.g. `unwrap_or(0)` then `> 0`)
- Functions that can fail should return `Result`, not log-and-continue
- Avoid unsafe code, ask before adding.
- Construct structs with direct literal syntax (`Foo { field: value, .. }`) instead of builder patterns or multi-argument `new()` functions
- Canonicalize relative paths to absolute paths as early as possible

## Dependencies
- Rust deps are in `.cargo-home` — read code from there for correct versions without needing the internet.
- *Always* ask before adding dependencies.
- When adding dependencies, *always* check the internet for the latest version first.
- For new dependencies, fetch the crate before trying to read source.

## Testing
- Unit tests: `just test`
- Integration tests: `just test-integration`
