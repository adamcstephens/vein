use rmcp::{
    ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::client::{ReqwestClient, Task, VikunjaClient};
use crate::config::ProjectConfig;

#[derive(Debug, Clone)]
pub struct VeinServer {
    client: ReqwestClient,
    project_config: ProjectConfig,
    tool_router: ToolRouter<Self>,
}

impl VeinServer {
    pub fn new(client: ReqwestClient, project_config: ProjectConfig) -> Self {
        VeinServer {
            client,
            project_config,
            tool_router: Self::tool_router(),
        }
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

#[tool_handler]
impl ServerHandler for VeinServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
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
