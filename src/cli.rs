use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vein", about = "Agent-focused issue tracker backed by Vikunja")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Discover Vikunja projects and buckets, print env var configuration
    Init,
    /// List available Vikunja projects
    ListProjects,
    /// List views for a Vikunja project
    ListProjectViews {
        /// Project ID
        project_id: i64,
    },
    /// List buckets for a project view
    ListProjectViewBuckets {
        /// Project ID
        project_id: i64,
        /// View ID
        view_id: i64,
    },
    /// Run as MCP stdio server (default if no subcommand given)
    Serve,
    /// Run an MCP tool directly from the CLI
    Tool {
        #[command(subcommand)]
        tool: ToolCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum ToolCommand {
    /// List tasks ready to be worked on (Todo bucket)
    ListReady,
    /// Get full details of a task by ID
    GetTask {
        /// Task ID
        task_id: i64,
    },
    /// List tasks currently in progress
    ListInProgress,
    /// List completed tasks
    ListDone,
    /// Claim a task (move to In Progress)
    Claim {
        /// Task ID
        task_id: i64,
    },
    /// Add a comment to a task
    Comment {
        /// Task ID
        task_id: i64,
        /// Comment text
        comment: String,
    },
    /// Create a new task in the project
    CreateTask {
        /// Task title
        title: String,
        /// Task description
        #[arg(short, long, default_value = "")]
        description: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_init_subcommand() {
        let cli = Cli::parse_from(["vein", "init"]);
        assert!(matches!(cli.command, Some(Command::Init)));
    }

    #[test]
    fn parses_list_projects_subcommand() {
        let cli = Cli::parse_from(["vein", "list-projects"]);
        assert!(matches!(cli.command, Some(Command::ListProjects)));
    }

    #[test]
    fn parses_list_project_views_subcommand() {
        let cli = Cli::parse_from(["vein", "list-project-views", "42"]);
        assert!(matches!(
            cli.command,
            Some(Command::ListProjectViews { project_id: 42 })
        ));
    }

    #[test]
    fn parses_list_project_view_buckets_subcommand() {
        let cli = Cli::parse_from(["vein", "list-project-view-buckets", "5", "10"]);
        assert!(matches!(
            cli.command,
            Some(Command::ListProjectViewBuckets {
                project_id: 5,
                view_id: 10
            })
        ));
    }

    #[test]
    fn parses_tool_list_ready_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "list-ready"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::ListReady
            })
        ));
    }

    #[test]
    fn parses_tool_get_task_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "get-task", "42"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::GetTask { task_id: 42 }
            })
        ));
    }

    #[test]
    fn parses_tool_list_in_progress_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "list-in-progress"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::ListInProgress
            })
        ));
    }

    #[test]
    fn parses_tool_list_done_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "list-done"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::ListDone
            })
        ));
    }

    #[test]
    fn parses_tool_claim_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "claim", "42"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::Claim { task_id: 42 }
            })
        ));
    }

    #[test]
    fn parses_tool_comment_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "comment", "42", "Work in progress"]);
        match cli.command {
            Some(Command::Tool {
                tool: ToolCommand::Comment { task_id, comment },
            }) => {
                assert_eq!(task_id, 42);
                assert_eq!(comment, "Work in progress");
            }
            other => panic!("expected Comment, got {other:?}"),
        }
    }

    #[test]
    fn parses_tool_create_task_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "create-task", "Fix the bug"]);
        match cli.command {
            Some(Command::Tool {
                tool: ToolCommand::CreateTask { title, description },
            }) => {
                assert_eq!(title, "Fix the bug");
                assert_eq!(description, "");
            }
            other => panic!("expected CreateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_tool_create_task_with_description() {
        let cli = Cli::parse_from([
            "vein",
            "tool",
            "create-task",
            "Fix the bug",
            "-d",
            "Something is broken",
        ]);
        match cli.command {
            Some(Command::Tool {
                tool: ToolCommand::CreateTask { title, description },
            }) => {
                assert_eq!(title, "Fix the bug");
                assert_eq!(description, "Something is broken");
            }
            other => panic!("expected CreateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_serve_subcommand() {
        let cli = Cli::parse_from(["vein", "serve"]);
        assert!(matches!(cli.command, Some(Command::Serve)));
    }

    #[test]
    fn defaults_to_none_when_no_subcommand() {
        let cli = Cli::parse_from(["vein"]);
        assert!(cli.command.is_none());
    }
}
