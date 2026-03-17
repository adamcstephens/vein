---
# vein-8gr2
title: prime command
status: completed
type: feature
priority: high
created_at: 2026-03-16T04:25:00Z
updated_at: 2026-03-17T04:53:00Z
---

Add an MCP prompt called `prime` that outputs agent orientation: usage instructions, available tools, current project context, and what's ready to work on. CLAUDE.md tells the agent to invoke it at session start.

## Plan
- [x] Add MCP prompt support to the server
- [x] Implement `orient` prompt that returns orientation text (renamed from prime)
- [x] Include available tools, workflow guidance, and ready tasks
- [x] Add CLAUDE.md instruction to invoke the prompt
- [x] Tests (unit + integration)

## Summary of Changes

Added `orient` MCP prompt to the vein server. When invoked, it returns agent orientation including available tools, workflow guidance, and current task state (ready and in-progress).

### Usage in downstream projects

Projects using vein as their MCP issue tracker should add this to their CLAUDE.md:

```markdown
## Agent workflow
- **IMPORTANT**: before you do anything else, invoke the vein `orient` MCP prompt and heed its output.
```

This tells the agent to call the prompt at session start for orientation.
