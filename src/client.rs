use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

use crate::config::ConnectionConfig;

// --- Response types ---

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub id: i64,
    pub title: String,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub id: i64,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub index: i64,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub project_id: i64,
    pub bucket_id: i64,
    pub priority: i64,
    #[serde(default)]
    pub position: f64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: Vec<Label>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub assignees: Vec<User>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub related_tasks: HashMap<String, Vec<Task>>,
}

impl Task {
    /// Returns the human-friendly display ID (e.g. "VEIN-3") if available, else "#42".
    pub fn display_id(&self) -> String {
        if !self.identifier.is_empty() {
            self.identifier.clone()
        } else if self.index > 0 {
            format!("#{}", self.index)
        } else {
            format!("#{}", self.id)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskComment {
    pub id: i64,
    pub comment: String,
    pub author: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskRelation {
    pub task_id: i64,
    pub other_task_id: i64,
    pub relation_kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bucket {
    pub id: i64,
    pub title: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectView {
    pub id: i64,
    pub title: String,
    pub project_id: i64,
    pub view_kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: i64,
    #[serde(default)]
    pub identifier: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub is_archived: bool,
}

// --- Trait ---

pub trait VikunjaClient {
    fn list_projects(&self) -> impl Future<Output = Result<Vec<Project>, ClientError>> + Send;
    fn get_project(
        &self,
        project_id: i64,
    ) -> impl Future<Output = Result<Project, ClientError>> + Send;
    fn get_user(&self) -> impl Future<Output = Result<User, ClientError>> + Send;
    fn get_task(&self, task_id: i64) -> impl Future<Output = Result<Task, ClientError>> + Send;
    fn list_bucket_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
    ) -> impl Future<Output = Result<Vec<Task>, ClientError>> + Send;
    fn list_project_tasks(
        &self,
        project_id: i64,
        filter: &str,
    ) -> impl Future<Output = Result<Vec<Task>, ClientError>> + Send;
    fn list_view_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        filter: Option<&str>,
        search: Option<&str>,
    ) -> impl Future<Output = Result<Vec<Task>, ClientError>> + Send;
    fn create_task(
        &self,
        project_id: i64,
        title: &str,
        description: &str,
        priority: Option<i64>,
    ) -> impl Future<Output = Result<Task, ClientError>> + Send;
    fn update_task(
        &self,
        task_id: i64,
        updates: TaskUpdate,
    ) -> impl Future<Output = Result<Task, ClientError>> + Send;
    fn create_relation(
        &self,
        task_id: i64,
        other_task_id: i64,
        relation_kind: &str,
    ) -> impl Future<Output = Result<TaskRelation, ClientError>> + Send;
    fn create_comment(
        &self,
        task_id: i64,
        comment: &str,
    ) -> impl Future<Output = Result<TaskComment, ClientError>> + Send;
    fn list_views(
        &self,
        project_id: i64,
    ) -> impl Future<Output = Result<Vec<ProjectView>, ClientError>> + Send;
    fn list_buckets(
        &self,
        project_id: i64,
        view_id: i64,
    ) -> impl Future<Output = Result<Vec<Bucket>, ClientError>> + Send;
    fn create_label(&self, title: &str) -> impl Future<Output = Result<Label, ClientError>> + Send;
    fn add_label_to_task(
        &self,
        task_id: i64,
        label_id: i64,
    ) -> impl Future<Output = Result<(), ClientError>> + Send;
    fn list_labels(&self) -> impl Future<Output = Result<Vec<Label>, ClientError>> + Send;
    fn create_project(
        &self,
        title: &str,
        description: &str,
        identifier: Option<&str>,
    ) -> impl Future<Output = Result<Project, ClientError>> + Send;
    fn delete_project(
        &self,
        project_id: i64,
    ) -> impl Future<Output = Result<(), ClientError>> + Send;
    fn move_task_to_bucket(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
        task_id: i64,
    ) -> impl Future<Output = Result<(), ClientError>> + Send;
}

// --- Task reference resolution ---

/// Parsed task reference: either a numeric ID or a project identifier like "VEIN-3".
#[derive(Debug, Clone, PartialEq)]
pub enum TaskRef {
    Id(i64),
    Index(i64),
    Identifier { prefix: String, index: i64 },
}

impl TaskRef {
    /// Parse a task reference string. Accepts "VEIN-3" style identifiers or plain numeric IDs.
    pub fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if let Some(after_hash) = trimmed.strip_prefix('#')
            && let Ok(index) = after_hash.parse::<i64>()
        {
            return Ok(TaskRef::Index(index));
        }
        if let Ok(id) = trimmed.parse::<i64>() {
            return Ok(TaskRef::Id(id));
        }
        if let Some((prefix, index_str)) = trimmed.rsplit_once('-')
            && let Ok(index) = index_str.parse::<i64>()
            && !prefix.is_empty()
        {
            return Ok(TaskRef::Identifier {
                prefix: prefix.to_string(),
                index,
            });
        }
        Err(format!(
            "Invalid task reference '{trimmed}'. Use a numeric ID (42), #index (#3), or identifier (VEIN-3)."
        ))
    }
}

/// Resolve a task reference to a numeric task ID.
pub async fn resolve_task_ref(
    client: &impl VikunjaClient,
    project_id: i64,
    task_ref: &TaskRef,
) -> Result<i64, ClientError> {
    match task_ref {
        TaskRef::Id(id) => Ok(*id),
        TaskRef::Index(index) | TaskRef::Identifier { index, .. } => {
            let tasks = client
                .list_project_tasks(project_id, &format!("index = {index}"))
                .await?;
            tasks.first().map(|t| t.id).ok_or_else(|| ClientError::Api {
                status: 404,
                message: format!("no task found with index {index}"),
            })
        }
    }
}

// --- Update payload ---

#[derive(Debug, Default)]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub priority: Option<i64>,
}

// --- Errors ---

#[derive(Debug)]
pub enum ClientError {
    Http(reqwest::Error),
    Api { status: u16, message: String },
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Http(e) => write!(f, "HTTP error: {e}"),
            ClientError::Api { status, message } => write!(f, "API error {status}: {message}"),
        }
    }
}

impl std::error::Error for ClientError {}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::Http(e)
    }
}

// --- Helpers ---

fn flatten_buckets(buckets: Vec<Bucket>) -> Vec<Task> {
    buckets
        .into_iter()
        .flat_map(|bucket| {
            let bucket_id = bucket.id;
            bucket.tasks.into_iter().map(move |mut task| {
                task.bucket_id = bucket_id;
                task
            })
        })
        .collect()
}

// --- ReqwestClient ---

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    http: reqwest::Client,
    base_url: String,
}

impl ReqwestClient {
    pub fn new(config: &ConnectionConfig) -> Result<Self, ClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        let auth_value =
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", config.vikunja_api_token))
                .map_err(|e| ClientError::Api {
                    status: 0,
                    message: format!("invalid API token: {e}"),
                })?;
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(ReqwestClient {
            http,
            base_url: config.vikunja_url.trim_end_matches('/').to_string(),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api/v1{path}", self.base_url)
    }

    async fn check_response(response: reqwest::Response) -> Result<reqwest::Response, ClientError> {
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            Err(ClientError::Api { status, message })
        }
    }
}

#[derive(Serialize)]
struct CreateLabelPayload<'a> {
    title: &'a str,
}

#[derive(Serialize)]
struct AddLabelPayload {
    label_id: i64,
}

#[derive(Serialize)]
struct CreateProjectPayload<'a> {
    title: &'a str,
    description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<&'a str>,
}

#[derive(Serialize)]
struct CreateTaskPayload<'a> {
    title: &'a str,
    description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<i64>,
}

#[derive(Serialize)]
struct UpdateTaskPayload {
    title: String,
    description: String,
    done: bool,
    priority: i64,
}

impl UpdateTaskPayload {
    fn from_task_with_updates(task: &Task, updates: TaskUpdate) -> Self {
        UpdateTaskPayload {
            title: updates.title.unwrap_or_else(|| task.title.clone()),
            description: updates
                .description
                .unwrap_or_else(|| task.description.clone()),
            done: updates.done.unwrap_or(task.done),
            priority: updates.priority.unwrap_or(task.priority),
        }
    }
}

#[derive(Serialize)]
struct CreateRelationPayload<'a> {
    other_task_id: i64,
    relation_kind: &'a str,
}

#[derive(Serialize)]
struct CreateCommentPayload<'a> {
    comment: &'a str,
}

#[derive(Serialize)]
struct MoveTaskToBucketPayload {
    task_id: i64,
}

impl VikunjaClient for ReqwestClient {
    async fn list_projects(&self) -> Result<Vec<Project>, ClientError> {
        let resp = self.http.get(self.url("/projects")).send().await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn get_project(&self, project_id: i64) -> Result<Project, ClientError> {
        let projects = self.list_projects().await?;
        projects
            .into_iter()
            .find(|p| p.id == project_id)
            .ok_or_else(|| ClientError::Api {
                status: 404,
                message: format!("project {project_id} not found"),
            })
    }

    async fn get_user(&self) -> Result<User, ClientError> {
        let resp = self.http.get(self.url("/user")).send().await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn get_task(&self, task_id: i64) -> Result<Task, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/tasks/{task_id}")))
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn list_project_tasks(
        &self,
        project_id: i64,
        filter: &str,
    ) -> Result<Vec<Task>, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/tasks")))
            .query(&[("filter", filter)])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn list_bucket_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
    ) -> Result<Vec<Task>, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/views/{view_id}/tasks")))
            .query(&[
                ("filter", format!("bucket_id = {bucket_id}")),
                ("sort_by", "position".to_string()),
                ("order_by", "asc".to_string()),
            ])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;

        // Vikunja returns Vec<Bucket> where each bucket contains a tasks array.
        // We flatten these into a single Vec<Task>, setting bucket_id from the
        // containing bucket.
        let buckets: Vec<Bucket> = resp.json().await?;
        Ok(flatten_buckets(buckets))
    }

    async fn list_view_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<Task>, ClientError> {
        let mut query: Vec<(&str, String)> = vec![
            ("sort_by", "position".to_string()),
            ("order_by", "asc".to_string()),
        ];
        if let Some(f) = filter {
            query.push(("filter", f.to_string()));
        }
        if let Some(s) = search {
            query.push(("s", s.to_string()));
        }
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/views/{view_id}/tasks")))
            .query(&query)
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;

        // Vikunja returns Vec<Bucket> where each bucket contains a tasks array.
        // We flatten these into a single Vec<Task>, setting bucket_id from the
        // containing bucket.
        let buckets: Vec<Bucket> = resp.json().await?;
        Ok(flatten_buckets(buckets))
    }

    async fn create_task(
        &self,
        project_id: i64,
        title: &str,
        description: &str,
        priority: Option<i64>,
    ) -> Result<Task, ClientError> {
        let resp = self
            .http
            .put(self.url(&format!("/projects/{project_id}/tasks")))
            .json(&CreateTaskPayload {
                title,
                description,
                priority,
            })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn update_task(&self, task_id: i64, updates: TaskUpdate) -> Result<Task, ClientError> {
        let current = self.get_task(task_id).await?;
        let payload = UpdateTaskPayload::from_task_with_updates(&current, updates);
        let resp = self
            .http
            .post(self.url(&format!("/tasks/{task_id}")))
            .json(&payload)
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn create_relation(
        &self,
        task_id: i64,
        other_task_id: i64,
        relation_kind: &str,
    ) -> Result<TaskRelation, ClientError> {
        let resp = self
            .http
            .put(self.url(&format!("/tasks/{task_id}/relations")))
            .json(&CreateRelationPayload {
                other_task_id,
                relation_kind,
            })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn create_comment(
        &self,
        task_id: i64,
        comment: &str,
    ) -> Result<TaskComment, ClientError> {
        let resp = self
            .http
            .put(self.url(&format!("/tasks/{task_id}/comments")))
            .json(&CreateCommentPayload { comment })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn list_views(&self, project_id: i64) -> Result<Vec<ProjectView>, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/views")))
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn list_buckets(
        &self,
        project_id: i64,
        view_id: i64,
    ) -> Result<Vec<Bucket>, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/views/{view_id}/tasks")))
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn create_label(&self, title: &str) -> Result<Label, ClientError> {
        let resp = self
            .http
            .put(self.url("/labels"))
            .json(&CreateLabelPayload { title })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn add_label_to_task(&self, task_id: i64, label_id: i64) -> Result<(), ClientError> {
        let resp = self
            .http
            .put(self.url(&format!("/tasks/{task_id}/labels")))
            .json(&AddLabelPayload { label_id })
            .send()
            .await?;
        Self::check_response(resp).await?;
        Ok(())
    }

    async fn list_labels(&self) -> Result<Vec<Label>, ClientError> {
        let resp = self.http.get(self.url("/labels")).send().await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn create_project(
        &self,
        title: &str,
        description: &str,
        identifier: Option<&str>,
    ) -> Result<Project, ClientError> {
        let resp = self
            .http
            .put(self.url("/projects"))
            .json(&CreateProjectPayload {
                title,
                description,
                identifier,
            })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn delete_project(&self, project_id: i64) -> Result<(), ClientError> {
        let resp = self
            .http
            .delete(self.url(&format!("/projects/{project_id}")))
            .send()
            .await?;
        Self::check_response(resp).await?;
        Ok(())
    }

    async fn move_task_to_bucket(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
        task_id: i64,
    ) -> Result<(), ClientError> {
        let resp = self
            .http
            .post(self.url(&format!(
                "/projects/{project_id}/views/{view_id}/buckets/{bucket_id}/tasks"
            )))
            .json(&MoveTaskToBucketPayload { task_id })
            .send()
            .await?;
        Self::check_response(resp).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient {
        user: User,
    }

    impl VikunjaClient for MockClient {
        async fn list_projects(&self) -> Result<Vec<Project>, ClientError> {
            unimplemented!()
        }
        async fn get_project(&self, _project_id: i64) -> Result<Project, ClientError> {
            unimplemented!()
        }
        async fn get_user(&self) -> Result<User, ClientError> {
            Ok(self.user.clone())
        }
        async fn get_task(&self, _task_id: i64) -> Result<Task, ClientError> {
            unimplemented!()
        }
        async fn list_project_tasks(
            &self,
            _project_id: i64,
            _filter: &str,
        ) -> Result<Vec<Task>, ClientError> {
            unimplemented!()
        }
        async fn list_bucket_tasks(
            &self,
            _project_id: i64,
            _view_id: i64,
            _bucket_id: i64,
        ) -> Result<Vec<Task>, ClientError> {
            unimplemented!()
        }
        async fn list_view_tasks(
            &self,
            _project_id: i64,
            _view_id: i64,
            _filter: Option<&str>,
            _search: Option<&str>,
        ) -> Result<Vec<Task>, ClientError> {
            unimplemented!()
        }
        async fn create_task(
            &self,
            _project_id: i64,
            _title: &str,
            _description: &str,
            _priority: Option<i64>,
        ) -> Result<Task, ClientError> {
            unimplemented!()
        }
        async fn update_task(
            &self,
            _task_id: i64,
            _updates: TaskUpdate,
        ) -> Result<Task, ClientError> {
            unimplemented!()
        }
        async fn create_relation(
            &self,
            _task_id: i64,
            _other_task_id: i64,
            _relation_kind: &str,
        ) -> Result<TaskRelation, ClientError> {
            unimplemented!()
        }
        async fn create_comment(
            &self,
            _task_id: i64,
            _comment: &str,
        ) -> Result<TaskComment, ClientError> {
            unimplemented!()
        }
        async fn list_views(&self, _project_id: i64) -> Result<Vec<ProjectView>, ClientError> {
            unimplemented!()
        }
        async fn list_buckets(
            &self,
            _project_id: i64,
            _view_id: i64,
        ) -> Result<Vec<Bucket>, ClientError> {
            unimplemented!()
        }
        async fn create_label(&self, _title: &str) -> Result<Label, ClientError> {
            unimplemented!()
        }
        async fn add_label_to_task(
            &self,
            _task_id: i64,
            _label_id: i64,
        ) -> Result<(), ClientError> {
            unimplemented!()
        }
        async fn list_labels(&self) -> Result<Vec<Label>, ClientError> {
            unimplemented!()
        }
        async fn create_project(
            &self,
            _title: &str,
            _description: &str,
            _identifier: Option<&str>,
        ) -> Result<Project, ClientError> {
            unimplemented!()
        }
        async fn delete_project(&self, _project_id: i64) -> Result<(), ClientError> {
            unimplemented!()
        }
        async fn move_task_to_bucket(
            &self,
            _project_id: i64,
            _view_id: i64,
            _bucket_id: i64,
            _task_id: i64,
        ) -> Result<(), ClientError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn mock_client_returns_user() {
        let client = MockClient {
            user: User {
                id: 1,
                username: "agent".to_string(),
                name: "Test Agent".to_string(),
            },
        };

        let user = client.get_user().await.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "agent");
    }

    #[test]
    fn project_deserializes_from_json() {
        let json = r#"{
            "id": 5,
            "title": "My Project",
            "description": "A test project",
            "is_archived": false
        }"#;

        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, 5);
        assert_eq!(project.title, "My Project");
        assert!(!project.is_archived);
    }

    #[test]
    fn project_deserializes_with_missing_optional_fields() {
        let json = r#"{
            "id": 1,
            "title": "Minimal",
            "description": ""
        }"#;

        let project: Project = serde_json::from_str(json).unwrap();
        assert!(!project.is_archived);
    }

    #[test]
    fn task_deserializes_from_json() {
        let json = r#"{
            "id": 42,
            "title": "Fix the bug",
            "description": "Something is broken",
            "done": false,
            "project_id": 1,
            "bucket_id": 2,
            "priority": 0,
            "labels": [{"id": 1, "title": "urgent"}],
            "assignees": [],
            "related_tasks": {}
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, 42);
        assert_eq!(task.title, "Fix the bug");
        assert_eq!(task.labels.len(), 1);
        assert_eq!(task.labels[0].title, "urgent");
    }

    fn make_client(base_url: &str) -> ReqwestClient {
        ReqwestClient {
            http: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    #[test]
    fn reqwest_client_builds_urls() {
        let client = make_client("http://localhost:59123/");
        assert_eq!(client.url("/user"), "http://localhost:59123/api/v1/user");
        assert_eq!(
            client.url("/tasks/42"),
            "http://localhost:59123/api/v1/tasks/42"
        );
    }

    #[test]
    fn reqwest_client_strips_trailing_slash() {
        let client = make_client("http://localhost:59123///");
        assert_eq!(client.url("/user"), "http://localhost:59123/api/v1/user");
    }

    #[test]
    fn task_deserializes_with_related_tasks() {
        let json = r#"{
            "id": 5,
            "title": "Blocked task",
            "description": "",
            "done": false,
            "project_id": 1,
            "bucket_id": 2,
            "priority": 0,
            "labels": [],
            "assignees": [],
            "related_tasks": {
                "blocked": [
                    {
                        "id": 4,
                        "title": "Blocker",
                        "description": "",
                        "done": false,
                        "project_id": 1,
                        "bucket_id": 0,
                        "priority": 0,
                        "labels": null,
                        "assignees": null,
                        "related_tasks": null
                    }
                ]
            }
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.related_tasks.len(), 1);
        let blocked = task.related_tasks.get("blocked").unwrap();
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].id, 4);
        assert!(!blocked[0].done);
    }

    fn make_task() -> Task {
        Task {
            id: 1,
            identifier: String::new(),
            index: 0,
            title: "Original title".to_string(),
            description: "Original description".to_string(),
            done: false,
            project_id: 1,
            bucket_id: 10,
            priority: 3,
            position: 0.0,
            labels: vec![],
            assignees: vec![],
            related_tasks: HashMap::new(),
        }
    }

    #[test]
    fn task_ref_parses_numeric_id() {
        assert_eq!(TaskRef::parse("42"), Ok(TaskRef::Id(42)));
    }

    #[test]
    fn task_ref_parses_hash_numeric_as_index() {
        assert_eq!(TaskRef::parse("#42"), Ok(TaskRef::Index(42)));
    }

    #[test]
    fn task_ref_parses_identifier() {
        assert_eq!(
            TaskRef::parse("VEIN-3"),
            Ok(TaskRef::Identifier {
                prefix: "VEIN".to_string(),
                index: 3,
            })
        );
    }

    #[test]
    fn task_ref_rejects_invalid() {
        assert!(TaskRef::parse("not-valid-ref").is_err());
    }

    #[test]
    fn task_ref_rejects_empty() {
        assert!(TaskRef::parse("").is_err());
    }

    #[test]
    fn display_id_uses_identifier_when_present() {
        let mut task = make_task();
        task.identifier = "VEIN-3".to_string();
        assert_eq!(task.display_id(), "VEIN-3");
    }

    #[test]
    fn display_id_uses_index_when_no_identifier() {
        let mut task = make_task();
        task.index = 5;
        assert_eq!(task.display_id(), "#5");
    }

    #[test]
    fn display_id_falls_back_to_hash_id_when_no_index() {
        let task = make_task(); // index = 0
        assert_eq!(task.display_id(), "#1");
    }

    #[test]
    fn update_payload_preserves_fields_not_in_update() {
        let task = make_task();
        let updates = TaskUpdate {
            done: Some(true),
            ..Default::default()
        };
        let payload = UpdateTaskPayload::from_task_with_updates(&task, updates);
        assert_eq!(payload.title, "Original title");
        assert_eq!(payload.description, "Original description");
        assert!(payload.done);
        assert_eq!(payload.priority, 3);
    }

    #[test]
    fn update_payload_applies_all_updates() {
        let task = make_task();
        let updates = TaskUpdate {
            title: Some("New title".to_string()),
            description: Some("New description".to_string()),
            done: Some(true),
            priority: Some(1),
        };
        let payload = UpdateTaskPayload::from_task_with_updates(&task, updates);
        assert_eq!(payload.title, "New title");
        assert_eq!(payload.description, "New description");
        assert!(payload.done);
        assert_eq!(payload.priority, 1);
    }

    #[test]
    fn task_deserializes_position_from_json() {
        let json = r#"{
            "id": 1,
            "title": "Positioned",
            "description": "",
            "done": false,
            "project_id": 1,
            "bucket_id": 2,
            "priority": 0,
            "position": 42.5
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();
        assert!((task.position - 42.5).abs() < f64::EPSILON);
    }

    #[test]
    fn task_deserializes_with_missing_optional_fields() {
        let json = r#"{
            "id": 1,
            "title": "Minimal",
            "description": "",
            "done": false,
            "project_id": 1,
            "bucket_id": 0,
            "priority": 0
        }"#;

        let task: Task = serde_json::from_str(json).unwrap();
        assert!(task.labels.is_empty());
        assert!(task.assignees.is_empty());
        assert!(task.related_tasks.is_empty());
    }
}
