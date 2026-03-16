# vein

Agent-focused issue tracker built on [Vikunja](https://vikunja.io).

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
| `projects` | `read_all`, `create`, `update`, `delete` |
| `tasks` | `read_all`, `create`, `update`, `delete` |

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
