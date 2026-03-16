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

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub project_id: i64,
    pub bucket_id: i64,
    pub priority: i64,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub assignees: Vec<User>,
    #[serde(default)]
    pub related_tasks: HashMap<String, Vec<Task>>,
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
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub is_archived: bool,
}

// --- Trait ---

pub trait VikunjaClient {
    fn list_projects(&self) -> impl Future<Output = Result<Vec<Project>, ClientError>>;
    fn get_user(&self) -> impl Future<Output = Result<User, ClientError>>;
    fn get_task(&self, task_id: i64) -> impl Future<Output = Result<Task, ClientError>>;
    fn list_bucket_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
    ) -> impl Future<Output = Result<Vec<Task>, ClientError>>;
    fn create_task(
        &self,
        project_id: i64,
        title: &str,
        description: &str,
    ) -> impl Future<Output = Result<Task, ClientError>>;
    fn update_task(
        &self,
        task_id: i64,
        updates: TaskUpdate,
    ) -> impl Future<Output = Result<Task, ClientError>>;
    fn create_relation(
        &self,
        task_id: i64,
        other_task_id: i64,
        relation_kind: &str,
    ) -> impl Future<Output = Result<TaskRelation, ClientError>>;
    fn create_comment(
        &self,
        task_id: i64,
        comment: &str,
    ) -> impl Future<Output = Result<TaskComment, ClientError>>;
    fn list_views(
        &self,
        project_id: i64,
    ) -> impl Future<Output = Result<Vec<ProjectView>, ClientError>>;
    fn list_buckets(
        &self,
        project_id: i64,
        view_id: i64,
    ) -> impl Future<Output = Result<Vec<Bucket>, ClientError>>;
}

// --- Update payload ---

#[derive(Debug, Default)]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub bucket_id: Option<i64>,
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

// --- ReqwestClient ---

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
struct CreateTaskPayload<'a> {
    title: &'a str,
    description: &'a str,
}

#[derive(Serialize)]
struct UpdateTaskPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id: Option<i64>,
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

impl VikunjaClient for ReqwestClient {
    async fn list_projects(&self) -> Result<Vec<Project>, ClientError> {
        let resp = self.http.get(self.url("/projects")).send().await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
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

    async fn list_bucket_tasks(
        &self,
        project_id: i64,
        view_id: i64,
        bucket_id: i64,
    ) -> Result<Vec<Task>, ClientError> {
        let resp = self
            .http
            .get(self.url(&format!("/projects/{project_id}/views/{view_id}/tasks")))
            .query(&[("filter", format!("bucket_id = {bucket_id}"))])
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn create_task(
        &self,
        project_id: i64,
        title: &str,
        description: &str,
    ) -> Result<Task, ClientError> {
        let resp = self
            .http
            .put(self.url(&format!("/projects/{project_id}/tasks")))
            .json(&CreateTaskPayload { title, description })
            .send()
            .await?;
        let resp = Self::check_response(resp).await?;
        Ok(resp.json().await?)
    }

    async fn update_task(&self, task_id: i64, updates: TaskUpdate) -> Result<Task, ClientError> {
        let resp = self
            .http
            .post(self.url(&format!("/tasks/{task_id}")))
            .json(&UpdateTaskPayload {
                title: updates.title,
                description: updates.description,
                done: updates.done,
                bucket_id: updates.bucket_id,
            })
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
        async fn get_user(&self) -> Result<User, ClientError> {
            Ok(self.user.clone())
        }
        async fn get_task(&self, _task_id: i64) -> Result<Task, ClientError> {
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
        async fn create_task(
            &self,
            _project_id: i64,
            _title: &str,
            _description: &str,
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
