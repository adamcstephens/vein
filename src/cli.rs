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
    /// List and search tasks across all buckets
    ListTasks {
        /// Filter expression (e.g. "done = false", "priority >= 3")
        #[arg(short, long)]
        filter: Option<String>,
        /// Search text to match against task titles
        #[arg(short, long)]
        search: Option<String>,
    },
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
    /// Mark a task as done
    Complete {
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
    /// Update an existing task's title, description, or priority
    UpdateTask {
        /// Task ID
        task_id: i64,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// Priority: none, low, medium, high, urgent
        #[arg(short, long)]
        priority: Option<String>,
    },
    /// Create a new label
    CreateLabel {
        /// Label title
        title: String,
    },
    /// Add a label to a task
    AddLabel {
        /// Task ID
        task_id: i64,
        /// Label ID
        label_id: i64,
    },
    /// List all available labels
    ListLabels,
    /// Add a relation between two tasks
    AddRelation {
        /// Task ID
        task_id: i64,
        /// Other task ID to relate to
        other_task_id: i64,
        /// Relation kind (blocked, blocking, related, subtask, parenttask, etc.)
        relation_kind: String,
    },
    /// Create a new task in the project
    CreateTask {
        /// Task title
        title: String,
        /// Task description
        #[arg(short, long, default_value = "")]
        description: String,
        /// Priority: none, low, medium, high, urgent
        #[arg(short, long)]
        priority: Option<String>,
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
    fn parses_tool_list_tasks_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "list-tasks"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::ListTasks {
                    filter: None,
                    search: None
                }
            })
        ));
    }

    #[test]
    fn parses_tool_list_tasks_with_filter_and_search() {
        let cli = Cli::parse_from([
            "vein",
            "tool",
            "list-tasks",
            "-f",
            "done = false",
            "-s",
            "login",
        ]);
        match cli.command {
            Some(Command::Tool {
                tool:
                    ToolCommand::ListTasks {
                        filter: Some(f),
                        search: Some(s),
                    },
            }) => {
                assert_eq!(f, "done = false");
                assert_eq!(s, "login");
            }
            other => panic!("expected ListTasks, got {other:?}"),
        }
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
    fn parses_tool_complete_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "complete", "42"]);
        assert!(matches!(
            cli.command,
            Some(Command::Tool {
                tool: ToolCommand::Complete { task_id: 42 }
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
    fn parses_tool_update_task_subcommand() {
        let cli = Cli::parse_from([
            "vein",
            "tool",
            "update-task",
            "42",
            "-t",
            "New title",
            "-d",
            "New desc",
        ]);
        match cli.command {
            Some(Command::Tool {
                tool:
                    ToolCommand::UpdateTask {
                        task_id,
                        title,
                        description,
                        ..
                    },
            }) => {
                assert_eq!(task_id, 42);
                assert_eq!(title.as_deref(), Some("New title"));
                assert_eq!(description.as_deref(), Some("New desc"));
            }
            other => panic!("expected UpdateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_tool_add_relation_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "add-relation", "1", "2", "blocked"]);
        match cli.command {
            Some(Command::Tool {
                tool:
                    ToolCommand::AddRelation {
                        task_id,
                        other_task_id,
                        relation_kind,
                    },
            }) => {
                assert_eq!(task_id, 1);
                assert_eq!(other_task_id, 2);
                assert_eq!(relation_kind, "blocked");
            }
            other => panic!("expected AddRelation, got {other:?}"),
        }
    }

    #[test]
    fn parses_tool_create_task_subcommand() {
        let cli = Cli::parse_from(["vein", "tool", "create-task", "Fix the bug"]);
        match cli.command {
            Some(Command::Tool {
                tool:
                    ToolCommand::CreateTask {
                        title, description, ..
                    },
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
                tool:
                    ToolCommand::CreateTask {
                        title, description, ..
                    },
            }) => {
                assert_eq!(title, "Fix the bug");
                assert_eq!(description, "Something is broken");
            }
            other => panic!("expected CreateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_tool_create_task_with_priority() {
        let cli = Cli::parse_from(["vein", "tool", "create-task", "Urgent bug", "-p", "high"]);
        match cli.command {
            Some(Command::Tool {
                tool:
                    ToolCommand::CreateTask {
                        title, priority, ..
                    },
            }) => {
                assert_eq!(title, "Urgent bug");
                assert_eq!(priority.as_deref(), Some("high"));
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
