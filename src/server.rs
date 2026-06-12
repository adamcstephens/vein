use rmcp::{
    RoleServer, ServerHandler,
    handler::server::wrapper::Parameters,
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
use crate::markdown::html_to_markdown;
use crate::project::ProjectClient;

pub fn parse_priority(s: &str) -> Result<i64, String> {
    match s.to_lowercase().as_str() {
        "none" => Ok(0),
        "low" => Ok(1),
        "medium" => Ok(2),
        "high" => Ok(3),
        "urgent" => Ok(4),
        other => Err(format!(
            "Unknown priority '{other}'. Valid values: none, low, medium, high, urgent"
        )),
    }
}

#[derive(Debug, Clone)]
pub struct VeinServer {
    project: ProjectClient<ReqwestClient>,
}

impl VeinServer {
    pub fn new(client: ReqwestClient, project_config: ProjectConfig) -> Self {
        VeinServer {
            project: ProjectClient::new(client, project_config),
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
    #[schemars(description = "Task index (e.g. \"3\" or \"#3\") or identifier (e.g. \"VEIN-3\")")]
    pub task_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CommentParams {
    #[schemars(description = "Task index (e.g. \"3\" or \"#3\") or identifier (e.g. \"VEIN-3\")")]
    pub task_id: String,
    #[schemars(description = "Comment text")]
    pub comment: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateTaskParams {
    #[schemars(description = "Task index (e.g. \"3\" or \"#3\") or identifier (e.g. \"VEIN-3\")")]
    pub task_id: String,
    #[schemars(description = "New title (optional)")]
    pub title: Option<String>,
    #[schemars(description = "New description (optional)")]
    pub description: Option<String>,
    #[schemars(description = "Priority: none, low, medium, high, urgent (optional)")]
    pub priority: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateLabelParams {
    #[schemars(description = "Label title")]
    pub title: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddLabelParams {
    #[schemars(description = "Task index (e.g. \"3\" or \"#3\") or identifier (e.g. \"VEIN-3\")")]
    pub task_id: String,
    #[schemars(description = "Label ID to assign")]
    pub label_id: i64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddRelationParams {
    #[schemars(description = "Task index (e.g. \"3\" or \"#3\") or identifier (e.g. \"VEIN-3\")")]
    pub task_id: String,
    #[schemars(
        description = "Other task identifier (e.g. \"VEIN-3\") or numeric ID (e.g. \"42\")"
    )]
    pub other_task_id: String,
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
    #[schemars(
        description = "Priority: none, low, medium, high, urgent (optional, defaults to none)"
    )]
    pub priority: Option<String>,
}

#[prompt_router]
impl VeinServer {
    /// Agent orientation: available tools, workflow, and ready tasks
    #[prompt(
        name = "orient",
        description = "Agent orientation: available tools, workflow guidance, and ready tasks"
    )]
    async fn orient(&self) -> Vec<PromptMessage> {
        let text = orient_text(&self.project).await;

        vec![PromptMessage::new_text(PromptMessageRole::User, text)]
    }
}

/// Build the orient prompt text, fetching current task state from the project
pub async fn orient_text<C: VikunjaClient>(project: &ProjectClient<C>) -> String {
    let ready_tasks = project
        .list_ready()
        .await
        .map(|tasks| format_task_list(&tasks, "No tasks ready to be worked on."))
        .unwrap_or_else(|e| format!("(failed to fetch ready tasks: {e})"));

    let in_progress = project
        .list_in_progress()
        .await
        .map(|tasks| format_task_list(&tasks, "No tasks currently in progress."))
        .unwrap_or_else(|e| format!("(failed to fetch in-progress tasks: {e})"));

    format_orient(&ready_tasks, &in_progress)
}

/// Format the orient prompt text from pre-rendered task list sections
pub fn format_orient(ready_tasks: &str, in_progress: &str) -> String {
    format!(
        r#"# Vein — Agent Orientation

You are connected to a Vikunja-backed issue tracker. Use the tools below to manage your work.

## Available Tools

- **list_ready** — List tasks ready to be worked on (Todo bucket)
- **list_tasks** — List/search tasks across all buckets (supports filter and search params)
- **list_in_progress** — List tasks currently being worked on
- **list_done** — List completed tasks
- **get_task** — Get full task details (description, labels, relations, assignees)
- **create_task** — Create a new task (with optional priority)
- **update_task** — Update a task's title, description, or priority
- **claim** — Move a task to In Progress
- **complete** — Mark a task as done
- **comment** — Add a progress note to a task
- **add_relation** — Add a relation between tasks (blocked, blocking, subtask, etc.)
- **create_label** — Create a new label
- **add_label** — Assign a label to a task
- **list_labels** — List all available labels

## Workflow

1. **Before starting work**: Check for existing tasks with `list_ready` or `list_tasks`. If one matches your work, `claim` it. If not, `create_task` first. If claiming fails due to bucket limit, stop.
2. **While working**: Use `comment` to log progress, decisions, and blockers.
3. **When done**: Use `complete` to mark the task finished. Add a final `comment` summarizing what was done.
4. **Task descriptions**: Use `update_task` to rewrite the description when the plan changes. Use `comment` for incremental progress notes.
5. **Labels** Use labels for bugs, features, tests, ci, etc, including existing project labels.
6. **Relations** Add relations between related tickets, ensuring dependent tasks are properly blocked.

## Current State

### Ready to work on
{ready_tasks}

### In progress
{in_progress}
"#
    )
}

#[tool_router]
impl VeinServer {
    /// List tasks that are ready to be worked on (in the Todo bucket, excluding blocked tasks)
    #[tool(name = "list_ready")]
    async fn list_ready(&self) -> Result<String, String> {
        let ready = self
            .project
            .list_ready()
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;
        Ok(format_task_list(&ready, "No tasks ready to be worked on."))
    }

    /// Create a new task in the project
    #[tool(name = "create_task")]
    async fn create_task(
        &self,
        Parameters(params): Parameters<CreateTaskParams>,
    ) -> Result<String, String> {
        let priority = params.priority.map(|p| parse_priority(&p)).transpose()?;
        let task = self
            .project
            .create_task(&params.title, params.description.as_deref(), priority)
            .await
            .map_err(|e| format!("Failed to create task: {e}"))?;

        Ok(format!(
            "Created task {}: {}",
            task.display_id(),
            task.title
        ))
    }

    /// List tasks currently being worked on (in the In Progress bucket)
    #[tool(name = "list_in_progress")]
    async fn list_in_progress(&self) -> Result<String, String> {
        let tasks = self
            .project
            .list_in_progress()
            .await
            .map_err(|e| format!("Failed to list tasks: {e}"))?;

        Ok(format_task_list(&tasks, "No tasks currently in progress."))
    }

    /// List completed tasks (in the Done bucket)
    #[tool(name = "list_done")]
    async fn list_done(&self) -> Result<String, String> {
        let tasks = self
            .project
            .list_done()
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
            .project
            .get_task(&params.task_id)
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
        self.project
            .comment(&params.task_id, &params.comment)
            .await
            .map_err(|e| format!("Failed to add comment: {e}"))?;

        Ok(format!("Added comment to task {}", params.task_id))
    }

    /// Claim a task by moving it to the In Progress bucket
    #[tool(name = "claim")]
    async fn claim(&self, Parameters(params): Parameters<TaskIdParams>) -> Result<String, String> {
        let task = self
            .project
            .claim(&params.task_id)
            .await
            .map_err(|e| format!("Failed to claim task: {e}"))?;

        Ok(format!(
            "Claimed task {}: {}",
            task.display_id(),
            task.title
        ))
    }

    /// Mark a task as done by moving it to the Done bucket
    #[tool(name = "complete")]
    async fn complete(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<String, String> {
        let task = self
            .project
            .complete(&params.task_id)
            .await
            .map_err(|e| format!("Failed to complete task: {e}"))?;

        Ok(format!(
            "Completed task {}: {}",
            task.display_id(),
            task.title
        ))
    }

    /// Add a relation between two tasks
    #[tool(name = "add_relation")]
    async fn add_relation(
        &self,
        Parameters(params): Parameters<AddRelationParams>,
    ) -> Result<String, String> {
        let relation = self
            .project
            .add_relation(
                &params.task_id,
                &params.other_task_id,
                &params.relation_kind,
            )
            .await
            .map_err(|e| format!("Failed to add relation: {e}"))?;

        Ok(format!(
            "Added {} relation: {} -> {}",
            relation.relation_kind, params.task_id, params.other_task_id
        ))
    }

    /// Update an existing task's title, description, or priority
    #[tool(name = "update_task")]
    async fn update_task(
        &self,
        Parameters(params): Parameters<UpdateTaskParams>,
    ) -> Result<String, String> {
        let priority = params.priority.map(|p| parse_priority(&p)).transpose()?;
        let task = self
            .project
            .update_task(&params.task_id, params.title, params.description, priority)
            .await
            .map_err(|e| format!("Failed to update task: {e}"))?;

        Ok(format!(
            "Updated task {}: {}",
            task.display_id(),
            task.title
        ))
    }

    /// Create a new label
    #[tool(name = "create_label")]
    async fn create_label(
        &self,
        Parameters(params): Parameters<CreateLabelParams>,
    ) -> Result<String, String> {
        let label = self
            .project
            .create_label(&params.title)
            .await
            .map_err(|e| format!("Failed to create label: {e}"))?;

        Ok(format!("Created label #{}: {}", label.id, label.title))
    }

    /// Add a label to a task
    #[tool(name = "add_label")]
    async fn add_label(
        &self,
        Parameters(params): Parameters<AddLabelParams>,
    ) -> Result<String, String> {
        self.project
            .add_label(&params.task_id, params.label_id)
            .await
            .map_err(|e| format!("Failed to add label: {e}"))?;

        Ok(format!(
            "Added label #{} to task {}",
            params.label_id, params.task_id,
        ))
    }

    /// List all available labels
    #[tool(name = "list_labels")]
    async fn list_labels(&self) -> Result<String, String> {
        let labels = self
            .project
            .list_labels()
            .await
            .map_err(|e| format!("Failed to list labels: {e}"))?;

        if labels.is_empty() {
            return Ok("No labels found.".to_string());
        }

        let lines: Vec<String> = labels
            .iter()
            .map(|l| format!("- #{}: {}", l.id, l.title))
            .collect();
        Ok(lines.join("\n"))
    }

    /// List and search tasks across all buckets with optional filters
    #[tool(name = "list_tasks")]
    async fn list_tasks(
        &self,
        Parameters(params): Parameters<ListTasksParams>,
    ) -> Result<String, String> {
        let tasks = self
            .project
            .list_tasks(params.filter.as_deref(), params.search.as_deref())
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
            "- {}: {}{}{}",
            task.display_id(),
            task.title,
            priority_str,
            label_str
        ));
    }

    lines.join("\n")
}

pub fn format_task_detail(task: &Task) -> String {
    let mut lines = vec![format!("# {}: {}", task.display_id(), task.title)];

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
                lines.push(format!("  - {kind}: {} {}", t.display_id(), t.title));
            }
        }
    }

    if !task.description.is_empty() {
        lines.push(String::new());
        let description =
            html_to_markdown(&task.description).unwrap_or_else(|_| task.description.clone());
        lines.push(description.trim_end().to_string());
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
    use crate::project::is_blocked;
    use std::collections::HashMap;

    fn make_task(id: i64, title: &str, priority: i64, labels: Vec<&str>) -> Task {
        Task {
            id,
            identifier: String::new(),
            index: 0,
            title: title.to_string(),
            description: String::new(),
            done: false,
            project_id: 1,
            bucket_id: 2,
            priority,
            position: 0.0,
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
                identifier: String::new(),
                index: 0,
                title: "Reset flow".to_string(),
                description: String::new(),
                done: false,
                project_id: 1,
                bucket_id: 2,
                priority: 0,
                position: 0.0,
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
    fn format_task_detail_converts_html_description_to_markdown() {
        let mut task = make_task(1, "HTML desc", 0, vec![]);
        task.description = "<p>some <strong>bold</strong> text</p>".to_string();
        let result = format_task_detail(&task);
        assert!(result.contains("some **bold** text"));
        assert!(!result.contains("<strong>"));
    }

    #[test]
    fn is_blocked_returns_true_when_blocked_by_incomplete_task() {
        let mut task = make_task(1, "Blocked", 0, vec![]);
        task.related_tasks.insert(
            "blocked".to_string(),
            vec![make_task(2, "Blocker", 0, vec![])], // done: false
        );
        assert!(is_blocked(&task));
    }

    #[test]
    fn is_blocked_returns_false_when_blocked_by_completed_task() {
        let mut task = make_task(1, "Not blocked", 0, vec![]);
        let mut blocker = make_task(2, "Done blocker", 0, vec![]);
        blocker.done = true;
        task.related_tasks
            .insert("blocked".to_string(), vec![blocker]);
        assert!(!is_blocked(&task));
    }

    #[test]
    fn is_blocked_returns_false_when_no_relations() {
        let task = make_task(1, "Free", 0, vec![]);
        assert!(!is_blocked(&task));
    }

    #[test]
    fn parse_priority_maps_strings_to_integers() {
        assert_eq!(parse_priority("none"), Ok(0));
        assert_eq!(parse_priority("low"), Ok(1));
        assert_eq!(parse_priority("medium"), Ok(2));
        assert_eq!(parse_priority("high"), Ok(3));
        assert_eq!(parse_priority("urgent"), Ok(4));
        assert_eq!(parse_priority("High"), Ok(3)); // case insensitive
        assert!(parse_priority("invalid").is_err());
    }

    #[test]
    fn create_task_params_accepts_priority() {
        let json = r#"{"title": "Test", "priority": "high"}"#;
        let params: CreateTaskParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.priority.as_deref(), Some("high"));
    }

    #[test]
    fn update_task_params_accepts_priority() {
        let json = r#"{"task_id": "VEIN-1", "priority": "urgent"}"#;
        let params: UpdateTaskParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.task_id, "VEIN-1");
        assert_eq!(params.priority.as_deref(), Some("urgent"));
    }

    #[test]
    fn format_orient_contains_sections_and_task_lists() {
        let text = format_orient("- VEIN-1: Ready task", "- VEIN-2: Active task");
        assert!(text.contains("# Vein — Agent Orientation"));
        assert!(text.contains("## Available Tools"));
        assert!(text.contains("## Workflow"));
        assert!(text.contains("### Ready to work on\n- VEIN-1: Ready task"));
        assert!(text.contains("### In progress\n- VEIN-2: Active task"));
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
    fn format_task_list_uses_identifier_when_present() {
        let mut task = make_task(42, "Fix bug", 3, vec!["auth"]);
        task.identifier = "VEIN-5".to_string();
        task.index = 5;
        let result = format_task_list(&[task], "No tasks.");
        assert!(result.contains("- VEIN-5: Fix bug (high) [auth]"));
        assert!(!result.contains("#42"));
    }

    #[test]
    fn format_task_detail_uses_identifier_when_present() {
        let mut task = make_task(42, "Fix bug", 3, vec![]);
        task.identifier = "VEIN-5".to_string();
        task.index = 5;
        let result = format_task_detail(&task);
        assert!(result.contains("# VEIN-5: Fix bug"));
        assert!(!result.contains("#42"));
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
