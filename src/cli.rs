use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(name = "vein", about = "Agent-focused issue tracker backed by Vikunja")]
#[command(subcommand_required = true, arg_required_else_help = true)]
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
    /// Run as MCP stdio server
    Serve,
    /// Generate shell completion script
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
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
    /// Get full details of a task by index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
    GetTask {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
    },
    /// List tasks currently in progress
    ListInProgress,
    /// List completed tasks
    ListDone,
    /// Claim a task (move to In Progress)
    Claim {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
    },
    /// Mark a task as done
    Complete {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
    },
    /// Add a comment to a task
    Comment {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
        /// Comment text
        comment: String,
    },
    /// Update an existing task's title, description, or priority
    UpdateTask {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
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
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
        /// Label ID
        label_id: i64,
    },
    /// List all available labels
    ListLabels,
    /// Add a relation between two tasks
    AddRelation {
        /// Task index (e.g. 3 or #3) or identifier (e.g. VEIN-3)
        task_id: String,
        /// Other task identifier (e.g. VEIN-3) or numeric ID
        other_task_id: String,
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
    fn parses_completions_subcommand() {
        let cli = Cli::parse_from(["vein", "completions", "fish"]);
        assert!(matches!(
            cli.command,
            Some(Command::Completions { shell: Shell::Fish })
        ));
    }

    #[test]
    fn parses_completions_bash() {
        let cli = Cli::parse_from(["vein", "completions", "bash"]);
        assert!(matches!(
            cli.command,
            Some(Command::Completions { shell: Shell::Bash })
        ));
    }

    #[test]
    fn parses_completions_zsh() {
        let cli = Cli::parse_from(["vein", "completions", "zsh"]);
        assert!(matches!(
            cli.command,
            Some(Command::Completions { shell: Shell::Zsh })
        ));
    }

    #[test]
    fn parses_list_ready_subcommand() {
        let cli = Cli::parse_from(["vein", "list-ready"]);
        assert!(matches!(cli.command, Some(Command::ListReady)));
    }

    #[test]
    fn parses_list_tasks_subcommand() {
        let cli = Cli::parse_from(["vein", "list-tasks"]);
        assert!(matches!(
            cli.command,
            Some(Command::ListTasks {
                filter: None,
                search: None
            })
        ));
    }

    #[test]
    fn parses_list_tasks_with_filter_and_search() {
        let cli = Cli::parse_from(["vein", "list-tasks", "-f", "done = false", "-s", "login"]);
        match cli.command {
            Some(Command::ListTasks {
                filter: Some(f),
                search: Some(s),
            }) => {
                assert_eq!(f, "done = false");
                assert_eq!(s, "login");
            }
            other => panic!("expected ListTasks, got {other:?}"),
        }
    }

    #[test]
    fn parses_get_task_subcommand() {
        let cli = Cli::parse_from(["vein", "get-task", "42"]);
        match cli.command {
            Some(Command::GetTask { task_id }) => assert_eq!(task_id, "42"),
            other => panic!("expected GetTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_get_task_with_identifier() {
        let cli = Cli::parse_from(["vein", "get-task", "VEIN-3"]);
        match cli.command {
            Some(Command::GetTask { task_id }) => assert_eq!(task_id, "VEIN-3"),
            other => panic!("expected GetTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_list_in_progress_subcommand() {
        let cli = Cli::parse_from(["vein", "list-in-progress"]);
        assert!(matches!(cli.command, Some(Command::ListInProgress)));
    }

    #[test]
    fn parses_list_done_subcommand() {
        let cli = Cli::parse_from(["vein", "list-done"]);
        assert!(matches!(cli.command, Some(Command::ListDone)));
    }

    #[test]
    fn parses_complete_subcommand() {
        let cli = Cli::parse_from(["vein", "complete", "42"]);
        match cli.command {
            Some(Command::Complete { task_id }) => assert_eq!(task_id, "42"),
            other => panic!("expected Complete, got {other:?}"),
        }
    }

    #[test]
    fn parses_claim_subcommand() {
        let cli = Cli::parse_from(["vein", "claim", "VEIN-3"]);
        match cli.command {
            Some(Command::Claim { task_id }) => assert_eq!(task_id, "VEIN-3"),
            other => panic!("expected Claim, got {other:?}"),
        }
    }

    #[test]
    fn parses_comment_subcommand() {
        let cli = Cli::parse_from(["vein", "comment", "42", "Work in progress"]);
        match cli.command {
            Some(Command::Comment { task_id, comment }) => {
                assert_eq!(task_id, "42");
                assert_eq!(comment, "Work in progress");
            }
            other => panic!("expected Comment, got {other:?}"),
        }
    }

    #[test]
    fn parses_update_task_subcommand() {
        let cli = Cli::parse_from([
            "vein",
            "update-task",
            "42",
            "-t",
            "New title",
            "-d",
            "New desc",
        ]);
        match cli.command {
            Some(Command::UpdateTask {
                task_id,
                title,
                description,
                ..
            }) => {
                assert_eq!(task_id, "42");
                assert_eq!(title.as_deref(), Some("New title"));
                assert_eq!(description.as_deref(), Some("New desc"));
            }
            other => panic!("expected UpdateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_add_relation_subcommand() {
        let cli = Cli::parse_from(["vein", "add-relation", "VEIN-1", "VEIN-2", "blocked"]);
        match cli.command {
            Some(Command::AddRelation {
                task_id,
                other_task_id,
                relation_kind,
            }) => {
                assert_eq!(task_id, "VEIN-1");
                assert_eq!(other_task_id, "VEIN-2");
                assert_eq!(relation_kind, "blocked");
            }
            other => panic!("expected AddRelation, got {other:?}"),
        }
    }

    #[test]
    fn parses_create_task_subcommand() {
        let cli = Cli::parse_from(["vein", "create-task", "Fix the bug"]);
        match cli.command {
            Some(Command::CreateTask {
                title, description, ..
            }) => {
                assert_eq!(title, "Fix the bug");
                assert_eq!(description, "");
            }
            other => panic!("expected CreateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_create_task_with_description() {
        let cli = Cli::parse_from([
            "vein",
            "create-task",
            "Fix the bug",
            "-d",
            "Something is broken",
        ]);
        match cli.command {
            Some(Command::CreateTask {
                title, description, ..
            }) => {
                assert_eq!(title, "Fix the bug");
                assert_eq!(description, "Something is broken");
            }
            other => panic!("expected CreateTask, got {other:?}"),
        }
    }

    #[test]
    fn parses_create_task_with_priority() {
        let cli = Cli::parse_from(["vein", "create-task", "Urgent bug", "-p", "high"]);
        match cli.command {
            Some(Command::CreateTask {
                title, priority, ..
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
    fn no_subcommand_prints_help() {
        // With subcommand_required, parsing with no args should fail (prints help)
        let result = Cli::try_parse_from(["vein"]);
        assert!(result.is_err());
    }
}
