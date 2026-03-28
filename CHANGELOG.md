# Changelog

All notable changes to this project will be documented in this file.
## [unreleased]

### Added

- Order list results by column position
- Print help when invoked with no arguments

### Fixed

- Resolve #N task refs by project index instead of global ID
- Filter list_bucket_tasks by bucket client-side
- Treat bare numeric task refs as project index
## [0.3.0] - 2026-03-21

### Added

- *(mcp)* Add orient prompt for agent orientation
- *(tools)* Add priority support to create_task and update_task
- *(tools)* Add label management tools
- *(tools)* Filter blocked tasks from list_ready
- *(markdown)* Convert descriptions between markdown and HTML for Vikunja
- Support project identifiers (e.g. VEIN-3) for task references
- *(nix)* Add named package to flake
- Add shell completion support

### Fixed

- *(client)* Fetch task before update to prevent field zeroing
- *(provision)* Fix empty fields in .secret.envrc
- *(test)* Use drop guard for integration test project cleanup
- Buckets can have limits, give advice on this
- Handle bucket-grouped response in list_view_tasks

### Changed

- Extract ProjectClient domain layer
- *(test)* Rename integration test file and prefix MCP tests
- Flatten CLI by removing tool subcommand nesting

### Miscellaneous

- *(dev)* Move provision script to standalone
- *(dev)* Split tests task
## [0.2.0] - 2026-03-16

### Added

- *(config)* Add configuration module for Vikunja env vars
- *(client)* Add Vikunja REST API client with trait abstraction
- *(server)* Add MCP stdio server transport using rmcp
- *(cli)* Add clap CLI framework with init and serve subcommands
- *(client)* Add list_projects to VikunjaClient
- *(cli)* Add list-projects subcommand
- *(cli)* Add list-project-views subcommand
- *(cli)* Add list-project-view-buckets subcommand
- *(init)* Interactive project/bucket setup with dialoguer
- *(dev)* Add process-compose dev environment with auto-provisioning
- *(test)* Add MCP server integration test harness
- *(tools)* Add list_ready MCP tool and CLI subcommand
- *(tools)* Add create_task MCP tool and CLI subcommand
- *(tools)* Add list_in_progress and list_done MCP tools
- *(tools)* Add get_task MCP tool and CLI subcommand
- *(tools)* Add comment MCP tool and CLI subcommand
- *(tools)* Add claim MCP tool and CLI subcommand
- *(tools)* Add complete MCP tool and CLI subcommand
- *(tools)* Add add_relation MCP tool and CLI subcommand
- *(tools)* Add update_task MCP tool and CLI subcommand
- *(tools)* Add list_tasks MCP tool and CLI subcommand

### Fixed

- *(init)* Default selector to first item
- *(nix)* Provide SSL certs for nix build sandbox
- *(test)* Use unique project names and match buckets by title

### Changed

- *(config)* Split into ConnectionConfig and ProjectConfig

### Documentation

- Document required API token permissions and user setup

### Miscellaneous

- *(beans)* Scrap agent identity tracking, simplify claim tool
- *(beans)* Mark vein-rnzz epic completed
