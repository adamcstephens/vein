# vein

Agent-focused issue tracker built on [Vikunja](https://vikunja.io).

## Setup

### 1. Create a Vikunja API token

In your Vikunja instance, go to **Settings > API Tokens** and create a token with these permissions:

| Group | Permissions |
|-------|-------------|
| `projects` | `read_one`, `views_buckets`, `views_buckets_tasks` |
| `tasks` | `read_one`, `create`, `update` |
| `projects_views_tasks` | `read_all` |
| `tasks_relations` | `create` |
| `tasks_comments` | `create` |
| `labels` | `read_all`, `create` |
| `tasks_labels` | `create` |

`vein init` also needs `projects` (`read_all`) and `projects_views` (`read_all`) to discover your project and bucket IDs. You can add these temporarily or use a broader token for setup.

### 2. Configure environment

Set your Vikunja connection, then run `vein init` to discover project and bucket IDs:

```sh
export VIKUNJA_URL="https://your-vikunja-instance.example.com"
export VIKUNJA_API_TOKEN="tk_..."
vein init
```

Then set all the variables it prints:

```sh
export VIKUNJA_PROJECT_ID="1"
export VIKUNJA_VIEW_ID="10"
export VIKUNJA_TODO_BUCKET_ID="2"
export VIKUNJA_INPROGRESS_BUCKET_ID="3"
export VIKUNJA_DONE_BUCKET_ID="4"
```

### 3. Connect to Claude Code

Add vein as an MCP server in your project's `.mcp.json`:

```json
{
  "mcpServers": {
    "vein": {
      "command": "vein",
      "args": ["serve"],
      "env": {
        "VIKUNJA_URL": "https://your-vikunja-instance.example.com",
        "VIKUNJA_API_TOKEN": "tk_...",
        "VIKUNJA_PROJECT_ID": "1",
        "VIKUNJA_VIEW_ID": "10",
        "VIKUNJA_TODO_BUCKET_ID": "2",
        "VIKUNJA_INPROGRESS_BUCKET_ID": "3",
        "VIKUNJA_DONE_BUCKET_ID": "4"
      }
    }
  }
}
```

If you use direnv to manage these environment variables, you can omit the `env` block:

```json
{
  "mcpServers": {
    "vein": {
      "command": "vein",
      "args": ["serve"]
    }
  }
}
```

### Agent orientation

Vein exposes an `orient` MCP prompt that gives agents a complete orientation: available tools, workflow guidance, and current task state. To have your agent invoke it automatically, add this to your project's `CLAUDE.md`:

```markdown
## Agent workflow
- **IMPORTANT**: before you do anything else, invoke the vein `orient` MCP prompt and heed its output with `/mcp__vein__orient`.
```

Or use tools directly from the CLI:

```sh
vein tool list-ready
vein tool get-task 42
vein tool list-tasks -f "done = false" -s "login"
vein tool create-task "Fix the bug" -d "Login is broken"
vein tool claim 42
vein tool complete 42
vein tool comment 42 "Started working on this"
vein tool update-task 42 -t "New title"
vein tool add-relation 1 2 blocked
vein tool create-label "bug"
vein tool add-label 42 1
vein tool list-labels
```

## Development

### Prerequisites

- [Nix](https://nixos.org/) with flakes enabled

### Setup

```sh
nix develop    # or use direnv
just dev       # starts dev Vikunja via process-compose
direnv allow   # reload env after first run to pick up the generated API token
```

`just dev` starts a local Vikunja instance on `http://127.0.0.1:3456` and provisions:

- An admin user (`admin` / `admin`)
- A scoped API token written to `.secret.envrc` as `VIKUNJA_API_TOKEN`
- A `vein-dev` project with a kanban view (To-Do, Doing, Done buckets), with `VIKUNJA_PROJECT_ID` and `VIKUNJA_VIEW_ID` written to `.secret.envrc`

### API token scope

The provisioned token grants access to project-scoped Vikunja endpoints:

| Group | Permissions |
|-------|-------------|
| `projects` | `read_all`, `read_one`, `create`, `update`, `delete`, `views_buckets`, `views_buckets_tasks` |
| `tasks` | `read_all`, `read_one`, `create`, `update`, `delete` |
| `labels` | `read_all`, `create` |
| `tasks_labels` | `create` |
| `projects_views_tasks` | `read_all` |

Global endpoints like `/api/v1/tasks/all` and `/api/v1/user` are **not** accessible with scoped tokens — use JWT auth (`POST /api/v1/login`) for those.

### Commands

| Command | Description |
|---------|-------------|
| `just dev` | Start dev Vikunja (detached) |
| `just test` | Run tests |
| `just lint` | Run clippy |
| `just format` | Format code |

### Config

- Vikunja config: `.services/vikunja/config.yml`
- Secrets: `.secret.envrc` (gitignored, loaded by direnv)
- Process-compose: `process-compose.yml`
