use rmcp::{
    RoleServer, ServerHandler,
    handler::server::{
        router::prompt::PromptRouter, router::tool::ToolRouter, wrapper::Parameters,
    },
    model::{
        GetPromptRequestParams, GetPromptResult, ListPromptsResult, PaginatedRequestParams,
        PromptMessage, PromptMessageRole, ServerCapabilities, ServerInfo,
    },
    prompt, prompt_handler, prompt_router, schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

use crate::client::{ReqwestClient, Task, VikunjaClient};
use crate::config::ProjectConfig;

#[derive(Debug, Clone)]
pub struct VeinServer {
    client: ReqwestClient,
    project_config: ProjectConfig,
    tool_router: ToolRouter<Self>,
    prompt_router: PromptRouter<Self>,
}

impl VeinServer {
    pub fn new(client: ReqwestClient, project_config: ProjectConfig) -> Self {
        VeinServer {
            client,
            project_config,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListTasksParams {
    #[schemars(description = "Filter expression (e.g. \"done = false\", \"priority >= 3\")")]
    pub filter: Option<String>,
    #[schemars(description = "Search text to match against task titles")]
    pub search: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TaskIdParams {
    #[schemars(description = "Task ID")]
    pub task_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CommentParams {
    #[schemars(description = "Task ID")]
    pub task_id: i64,
    #[schemars(description = "Comment text")]
    pub comment: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateTaskParams {
    #[schemars(description = "Task ID")]
    pub task_id: i64,
    #[schemars(description = "New title (optional)")]
    pub title: Option<String>,
    #[schemars(description = "New description (optional)")]
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddRelationParams {
    #[schemars(description = "Task ID")]
    pub task_id: i64,
    #[schemars(description = "Other task ID to relate to")]
    pub other_task_id: i64,
    #[schemars(
        description = "Relation kind: blocked, blocking, related, subtask, parenttask, duplicateof, duplicates, precedes, follows, copiedfrom, copiedto"
    )]
    pub relation_kind: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateTaskParams {
    #[schemars(description = "Task title")]
    pub title: String,
    #[schemars(description = "Task description (optional)")]
    pub description: Option<String>,
}

#[prompt_router]
impl VeinServer {
    /// Agent orientation: available tools, workflow, and ready tasks
    #[prompt(
        name = "orient",
        description = "Agent orientation: available tools, workflow guidance, and ready tasks"
    )]
    async fn orient(&self) -> Vec<PromptMessage> {
        let ready_tasks = self
            .client
            .list_bucket_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                self.project_config.todo_bucket_id,
            )
            .await
            .map(|tasks| format_task_list(&tasks, "No tasks ready to be worked on."))
            .unwrap_or_else(|e| format!("(failed to fetch ready tasks: {e})"));

        let in_progress = self
            .client
            .list_bucket_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                self.project_config.inprogress_bucket_id,
            )
            .await
            .map(|tasks| format_task_list(&tasks, "No tasks currently in progress."))
            .unwrap_or_else(|e| format!("(failed to fetch in-progress tasks: {e})"));

        let text = format!(
            r#"# Vein — Agent Orientation

You are connected to a Vikunja-backed issue tracker. Use the tools below to manage your work.

## Available Tools

- **list_ready** — List tasks ready to be worked on (Todo bucket)
- **list_tasks** — List/search tasks across all buckets (supports filter and search params)
- **list_in_progress** — List tasks currently being worked on
- **list_done** — List completed tasks
- **get_task** — Get full task details (description, labels, relations, assignees)
- **create_task** — Create a new task
- **update_task** — Update a task's title or description
- **claim** — Move a task to In Progress
- **complete** — Mark a task as done
- **comment** — Add a progress note to a task
- **add_relation** — Add a relation between tasks (blocked, blocking, subtask, etc.)

## Workflow

1. **Before starting work**: Check for existing tasks with `list_ready` or `list_tasks`. If one matches your work, `claim` it. If not, `create_task` first.
2. **While working**: Use `comment` to log progress, decisions, and blockers.
3. **When done**: Use `complete` to mark the task finished. Add a final `comment` summarizing what was done.
4. **Task descriptions**: Use `update_task` to rewrite the description when the plan changes. Use `comment` for incremental progress notes.

## Current State

### Ready to work on
{ready_tasks}

### In progress
{in_progress}
"#
        );

        vec![PromptMessage::new_text(PromptMessageRole::User, text)]
    }
}

#[tool_router]
impl VeinServer {
    /// List tasks that are ready to be worked on (in the Todo bucket)
    #[tool(name = "list_ready")]
    async fn list_ready(&self) -> Result<String, String> {
        let tasks = self
            .client
            .list_bucket_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                self.project_config.todo_bucket_id,
            )
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;

        Ok(format_task_list(&tasks, "No tasks ready to be worked on."))
    }

    /// Create a new task in the project
    #[tool(name = "create_task")]
    async fn create_task(
        &self,
        Parameters(params): Parameters<CreateTaskParams>,
    ) -> Result<String, String> {
        let description = params.description.unwrap_or_default();
        let task = self
            .client
            .create_task(self.project_config.project_id, &params.title, &description)
            .await
            .map_err(|e| format!("Failed to create task: {e}"))?;

        Ok(format!("Created task #{}: {}", task.id, task.title))
    }

    /// List tasks currently being worked on (in the In Progress bucket)
    #[tool(name = "list_in_progress")]
    async fn list_in_progress(&self) -> Result<String, String> {
        let tasks = self
            .client
            .list_bucket_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                self.project_config.inprogress_bucket_id,
            )
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;

        Ok(format_task_list(&tasks, "No tasks currently in progress."))
    }

    /// List completed tasks (in the Done bucket)
    #[tool(name = "list_done")]
    async fn list_done(&self) -> Result<String, String> {
        let tasks = self
            .client
            .list_bucket_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                self.project_config.done_bucket_id,
            )
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;

        Ok(format_task_list(&tasks, "No completed tasks."))
    }

    /// Get full details of a task by ID, including description, labels, relations, and assignees
    #[tool(name = "get_task")]
    async fn get_task(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<String, String> {
        let task = self
            .client
            .get_task(params.task_id)
            .await
            .map_err(|e| format!("Failed to get task: {e}"))?;

        Ok(format_task_detail(&task))
    }

    /// Add a comment to a task for progress notes and status updates
    #[tool(name = "comment")]
    async fn comment(
        &self,
        Parameters(params): Parameters<CommentParams>,
    ) -> Result<String, String> {
        let comment = self
            .client
            .create_comment(params.task_id, &params.comment)
            .await
            .map_err(|e| format!("Failed to add comment: {e}"))?;

        Ok(format!(
            "Added comment #{} to task #{}",
            comment.id, params.task_id
        ))
    }

    /// Claim a task by moving it to the In Progress bucket
    #[tool(name = "claim")]
    async fn claim(&self, Parameters(params): Parameters<TaskIdParams>) -> Result<String, String> {
        let task = self
            .client
            .update_task(
                params.task_id,
                crate::client::TaskUpdate {
                    bucket_id: Some(self.project_config.inprogress_bucket_id),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| format!("Failed to claim task: {e}"))?;

        Ok(format!("Claimed task #{}: {}", task.id, task.title))
    }

    /// Mark a task as done by moving it to the Done bucket
    #[tool(name = "complete")]
    async fn complete(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<String, String> {
        let task = self
            .client
            .update_task(
                params.task_id,
                crate::client::TaskUpdate {
                    done: Some(true),
                    bucket_id: Some(self.project_config.done_bucket_id),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| format!("Failed to complete task: {e}"))?;

        Ok(format!("Completed task #{}: {}", task.id, task.title))
    }

    /// Add a relation between two tasks
    #[tool(name = "add_relation")]
    async fn add_relation(
        &self,
        Parameters(params): Parameters<AddRelationParams>,
    ) -> Result<String, String> {
        let relation = self
            .client
            .create_relation(params.task_id, params.other_task_id, &params.relation_kind)
            .await
            .map_err(|e| format!("Failed to add relation: {e}"))?;

        Ok(format!(
            "Added {} relation: #{} -> #{}",
            relation.relation_kind, relation.task_id, relation.other_task_id
        ))
    }

    /// Update an existing task's title or description
    #[tool(name = "update_task")]
    async fn update_task(
        &self,
        Parameters(params): Parameters<UpdateTaskParams>,
    ) -> Result<String, String> {
        let task = self
            .client
            .update_task(
                params.task_id,
                crate::client::TaskUpdate {
                    title: params.title,
                    description: params.description,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| format!("Failed to update task: {e}"))?;

        Ok(format!("Updated task #{}: {}", task.id, task.title))
    }

    /// List and search tasks across all buckets with optional filters
    #[tool(name = "list_tasks")]
    async fn list_tasks(
        &self,
        Parameters(params): Parameters<ListTasksParams>,
    ) -> Result<String, String> {
        let tasks = self
            .client
            .list_view_tasks(
                self.project_config.project_id,
                self.project_config.view_id,
                params.filter.as_deref(),
                params.search.as_deref(),
            )
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;

        Ok(format_task_list(&tasks, "No tasks found."))
    }
}

pub fn format_task_list(tasks: &[Task], empty_message: &str) -> String {
    if tasks.is_empty() {
        return empty_message.to_string();
    }

    let mut lines = Vec::with_capacity(tasks.len());
    for task in tasks {
        let labels: Vec<&str> = task.labels.iter().map(|l| l.title.as_str()).collect();
        let label_str = if labels.is_empty() {
            String::new()
        } else {
            format!(" [{}]", labels.join(", "))
        };
        let priority_str = match task.priority {
            0 => "",
            1 => " (low)",
            2 => " (medium)",
            3 => " (high)",
            4 => " (urgent)",
            _ => " (unknown priority)",
        };
        lines.push(format!(
            "- #{}: {}{}{}",
            task.id, task.title, priority_str, label_str
        ));
    }

    lines.join("\n")
}

pub fn format_task_detail(task: &Task) -> String {
    let mut lines = vec![format!("# #{}: {}", task.id, task.title)];

    let status = if task.done { "Done" } else { "Open" };
    lines.push(format!("Status: {status}"));

    let priority = match task.priority {
        0 => "None",
        1 => "Low",
        2 => "Medium",
        3 => "High",
        4 => "Urgent",
        _ => "Unknown",
    };
    if task.priority > 0 {
        lines.push(format!("Priority: {priority}"));
    }

    if !task.labels.is_empty() {
        let label_names: Vec<&str> = task.labels.iter().map(|l| l.title.as_str()).collect();
        lines.push(format!("Labels: {}", label_names.join(", ")));
    }

    if !task.assignees.is_empty() {
        let names: Vec<&str> = task.assignees.iter().map(|u| u.username.as_str()).collect();
        lines.push(format!("Assignees: {}", names.join(", ")));
    }

    if !task.related_tasks.is_empty() {
        lines.push("Relations:".to_string());
        for (kind, related) in &task.related_tasks {
            for t in related {
                lines.push(format!("  - {kind}: #{} {}", t.id, t.title));
            }
        }
    }

    if !task.description.is_empty() {
        lines.push(String::new());
        lines.push(task.description.clone());
    }

    lines.join("\n")
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for VeinServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
        )
        .with_instructions("Vikunja-backed issue tracker for AI agents")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{Label, Task};
    use std::collections::HashMap;

    fn make_task(id: i64, title: &str, priority: i64, labels: Vec<&str>) -> Task {
        Task {
            id,
            title: title.to_string(),
            description: String::new(),
            done: false,
            project_id: 1,
            bucket_id: 2,
            priority,
            labels: labels
                .into_iter()
                .enumerate()
                .map(|(i, t)| Label {
                    id: i as i64,
                    title: t.to_string(),
                })
                .collect(),
            assignees: vec![],
            related_tasks: HashMap::new(),
        }
    }

    #[test]
    fn format_task_list_returns_empty_message_when_no_tasks() {
        let result = format_task_list(&[], "No tasks ready.");
        assert_eq!(result, "No tasks ready.");
    }

    #[test]
    fn format_task_list_returns_task_summaries() {
        let tasks = vec![
            make_task(1, "Fix login bug", 3, vec!["auth"]),
            make_task(2, "Add search", 0, vec![]),
        ];
        let result = format_task_list(&tasks, "No tasks.");
        assert!(result.contains("- #1: Fix login bug (high) [auth]"));
        assert!(result.contains("- #2: Add search"));
        assert!(!result.contains("#2: Add search ("));
    }

    #[test]
    fn format_task_list_shows_multiple_labels() {
        let tasks = vec![make_task(5, "Refactor", 0, vec!["tech-debt", "backend"])];
        let result = format_task_list(&tasks, "No tasks.");
        assert!(result.contains("[tech-debt, backend]"));
    }

    #[test]
    fn format_task_detail_shows_full_info() {
        let mut task = make_task(42, "Fix login bug", 3, vec!["auth", "urgent"]);
        task.description = "Users cannot log in after password reset.".to_string();
        task.assignees = vec![crate::client::User {
            id: 1,
            username: "agent".to_string(),
            name: "Test Agent".to_string(),
        }];
        task.related_tasks.insert(
            "blocked".to_string(),
            vec![Task {
                id: 10,
                title: "Reset flow".to_string(),
                description: String::new(),
                done: false,
                project_id: 1,
                bucket_id: 2,
                priority: 0,
                labels: vec![],
                assignees: vec![],
                related_tasks: HashMap::new(),
            }],
        );

        let result = format_task_detail(&task);
        assert!(result.contains("# #42: Fix login bug"));
        assert!(result.contains("Status: Open"));
        assert!(result.contains("Priority: High"));
        assert!(result.contains("Labels: auth, urgent"));
        assert!(result.contains("Assignees: agent"));
        assert!(result.contains("blocked: #10 Reset flow"));
        assert!(result.contains("Users cannot log in after password reset."));
    }

    #[test]
    fn format_task_detail_minimal_task() {
        let task = make_task(1, "Simple task", 0, vec![]);
        let result = format_task_detail(&task);
        assert!(result.contains("# #1: Simple task"));
        assert!(result.contains("Status: Open"));
        assert!(!result.contains("Priority:"));
        assert!(!result.contains("Labels:"));
        assert!(!result.contains("Assignees:"));
        assert!(!result.contains("Relations:"));
    }

    #[test]
    fn orient_prompt_contains_orientation() {
        let attr = VeinServer::orient_prompt_attr();
        assert_eq!(attr.name, "orient");
        assert!(attr.description.as_deref().unwrap().contains("orientation"),);
        assert!(
            attr.arguments.is_none() || attr.arguments.as_ref().unwrap().is_empty(),
            "prime prompt should have no arguments"
        );
    }

    #[test]
    fn format_task_list_shows_all_priority_levels() {
        let tasks = vec![
            make_task(1, "Low", 1, vec![]),
            make_task(2, "Medium", 2, vec![]),
            make_task(3, "High", 3, vec![]),
            make_task(4, "Urgent", 4, vec![]),
        ];
        let result = format_task_list(&tasks, "No tasks.");
        assert!(result.contains("#1: Low (low)"));
        assert!(result.contains("#2: Medium (medium)"));
        assert!(result.contains("#3: High (high)"));
        assert!(result.contains("#4: Urgent (urgent)"));
    }
}
