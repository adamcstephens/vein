use crate::client::{
    ClientError, Label, Task, TaskComment, TaskRef, TaskRelation, TaskUpdate, VikunjaClient,
    resolve_task_ref,
};
use crate::config::ProjectConfig;
use crate::markdown::markdown_to_html;

/// Project-scoped operations wrapping a Vikunja client.
///
/// Accepts string task references (e.g. "VEIN-3", "42") and handles
/// resolution, markdown conversion, and multi-step workflows internally.
#[derive(Debug, Clone)]
pub struct ProjectClient<C: VikunjaClient> {
    client: C,
    config: ProjectConfig,
}

impl<C: VikunjaClient> ProjectClient<C> {
    pub fn new(client: C, config: ProjectConfig) -> Self {
        ProjectClient { client, config }
    }

    pub fn client(&self) -> &C {
        &self.client
    }

    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }

    /// Resolve a string task reference to a numeric ID.
    pub async fn resolve(&self, task_ref: &str) -> Result<i64, ClientError> {
        let parsed = TaskRef::parse(task_ref).map_err(|e| ClientError::Api {
            status: 400,
            message: e,
        })?;
        resolve_task_ref(&self.client, self.config.project_id, &parsed).await
    }

    /// Get full details of a task by reference.
    pub async fn get_task(&self, task_ref: &str) -> Result<Task, ClientError> {
        let id = self.resolve(task_ref).await?;
        self.client.get_task(id).await
    }

    /// Move a task to a target bucket, auto-managing the done flag.
    pub async fn move_to_column(
        &self,
        task_ref: &str,
        bucket_id: i64,
    ) -> Result<Task, ClientError> {
        let id = self.resolve(task_ref).await?;
        let is_done = bucket_id == self.config.done_bucket_id;
        let task = self
            .client
            .update_task(
                id,
                TaskUpdate {
                    done: Some(is_done),
                    ..Default::default()
                },
            )
            .await?;
        self.client
            .move_task_to_bucket(self.config.project_id, self.config.view_id, bucket_id, id)
            .await?;
        Ok(task)
    }

    /// Claim a task by moving it to the In Progress bucket.
    pub async fn claim(&self, task_ref: &str) -> Result<Task, ClientError> {
        self.move_to_column(task_ref, self.config.inprogress_bucket_id)
            .await
    }

    /// Mark a task as done and move it to the Done bucket.
    pub async fn complete(&self, task_ref: &str) -> Result<Task, ClientError> {
        self.move_to_column(task_ref, self.config.done_bucket_id)
            .await
    }

    /// Add a markdown comment to a task.
    pub async fn comment(&self, task_ref: &str, comment: &str) -> Result<TaskComment, ClientError> {
        let id = self.resolve(task_ref).await?;
        let html = markdown_to_html(comment);
        self.client.create_comment(id, &html).await
    }

    /// Create a new task with an optional markdown description and priority.
    pub async fn create_task(
        &self,
        title: &str,
        description: Option<&str>,
        priority: Option<i64>,
    ) -> Result<Task, ClientError> {
        let html_desc = description.map(markdown_to_html).unwrap_or_default();
        self.client
            .create_task(self.config.project_id, title, &html_desc, priority)
            .await
    }

    /// Update a task's fields. Description is expected as markdown.
    pub async fn update_task(
        &self,
        task_ref: &str,
        title: Option<String>,
        description: Option<String>,
        priority: Option<i64>,
    ) -> Result<Task, ClientError> {
        let id = self.resolve(task_ref).await?;
        let description = description.map(|d| markdown_to_html(&d));
        self.client
            .update_task(
                id,
                TaskUpdate {
                    title,
                    description,
                    priority,
                    ..Default::default()
                },
            )
            .await
    }

    /// Add a relation between two tasks (both specified as string refs).
    pub async fn add_relation(
        &self,
        task_ref: &str,
        other_ref: &str,
        relation_kind: &str,
    ) -> Result<TaskRelation, ClientError> {
        let id = self.resolve(task_ref).await?;
        let other_id = self.resolve(other_ref).await?;
        self.client
            .create_relation(id, other_id, relation_kind)
            .await
    }

    /// Add a label to a task.
    pub async fn add_label(&self, task_ref: &str, label_id: i64) -> Result<(), ClientError> {
        let id = self.resolve(task_ref).await?;
        self.client.add_label_to_task(id, label_id).await
    }

    /// Create a new label.
    pub async fn create_label(&self, title: &str) -> Result<Label, ClientError> {
        self.client.create_label(title).await
    }

    /// List all labels.
    pub async fn list_labels(&self) -> Result<Vec<Label>, ClientError> {
        self.client.list_labels().await
    }

    /// List tasks ready to be worked on (todo bucket, not blocked).
    pub async fn list_ready(&self) -> Result<Vec<Task>, ClientError> {
        let tasks = self
            .client
            .list_bucket_tasks(
                self.config.project_id,
                self.config.view_id,
                self.config.todo_bucket_id,
            )
            .await?;
        let mut tasks: Vec<Task> = tasks.into_iter().filter(|t| !is_blocked(t)).collect();
        sort_by_position(&mut tasks);
        Ok(tasks)
    }

    /// List tasks currently in progress.
    pub async fn list_in_progress(&self) -> Result<Vec<Task>, ClientError> {
        let mut tasks = self
            .client
            .list_bucket_tasks(
                self.config.project_id,
                self.config.view_id,
                self.config.inprogress_bucket_id,
            )
            .await?;
        sort_by_position(&mut tasks);
        Ok(tasks)
    }

    /// List completed tasks.
    pub async fn list_done(&self) -> Result<Vec<Task>, ClientError> {
        let mut tasks = self
            .client
            .list_bucket_tasks(
                self.config.project_id,
                self.config.view_id,
                self.config.done_bucket_id,
            )
            .await?;
        sort_by_position(&mut tasks);
        Ok(tasks)
    }

    /// List/search tasks across all buckets.
    pub async fn list_tasks(
        &self,
        filter: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<Task>, ClientError> {
        let mut tasks = self
            .client
            .list_view_tasks(self.config.project_id, self.config.view_id, filter, search)
            .await?;
        sort_by_position(&mut tasks);
        Ok(tasks)
    }
}

/// Snapshot of all three kanban columns from a single API call.
#[derive(Debug, Clone)]
pub struct BoardState {
    pub ready: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub done: Vec<Task>,
    pub column_names: [String; 3],
}

impl<C: VikunjaClient> ProjectClient<C> {
    /// Fetch all three kanban columns in a single API call.
    pub async fn list_board(&self) -> Result<BoardState, ClientError> {
        let buckets = self
            .client
            .list_buckets(self.config.project_id, self.config.view_id)
            .await?;

        let mut ready = Vec::new();
        let mut in_progress = Vec::new();
        let mut done = Vec::new();
        let mut column_names = [
            String::from("Ready"),
            String::from("In Progress"),
            String::from("Done"),
        ];

        for bucket in buckets {
            if bucket.id == self.config.todo_bucket_id {
                column_names[0] = bucket.title.clone();
                ready.extend(bucket.tasks.into_iter().map(|mut task| {
                    task.bucket_id = bucket.id;
                    task
                }));
            } else if bucket.id == self.config.inprogress_bucket_id {
                column_names[1] = bucket.title.clone();
                in_progress.extend(bucket.tasks.into_iter().map(|mut task| {
                    task.bucket_id = bucket.id;
                    task
                }));
            } else if bucket.id == self.config.done_bucket_id {
                column_names[2] = bucket.title.clone();
                done.extend(bucket.tasks.into_iter().map(|mut task| {
                    task.bucket_id = bucket.id;
                    task
                }));
            }
        }

        ready.retain(|t| !is_blocked(t));
        sort_by_position(&mut ready);
        sort_by_position(&mut in_progress);
        sort_by_position(&mut done);

        Ok(BoardState {
            ready,
            in_progress,
            done,
            column_names,
        })
    }
}

fn sort_by_position(tasks: &mut [Task]) {
    tasks.sort_by(|a, b| {
        a.position
            .partial_cmp(&b.position)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

pub fn is_blocked(task: &Task) -> bool {
    task.related_tasks
        .get("blocked")
        .is_some_and(|blockers| blockers.iter().any(|t| !t.done))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    fn test_config() -> ProjectConfig {
        ProjectConfig {
            project_id: 1,
            view_id: 10,
            todo_bucket_id: 100,
            inprogress_bucket_id: 200,
            done_bucket_id: 300,
        }
    }

    fn make_task(id: i64, title: &str) -> Task {
        Task {
            id,
            identifier: String::new(),
            index: id,
            title: title.to_string(),
            description: String::new(),
            done: false,
            project_id: 1,
            bucket_id: 100,
            priority: 0,
            position: 0.0,
            labels: vec![],
            assignees: vec![],
            related_tasks: HashMap::new(),
        }
    }

    #[derive(Default)]
    struct MockState {
        moved_to_bucket: Vec<(i64, i64)>, // (bucket_id, task_id)
        updated_tasks: Vec<(i64, TaskUpdate)>,
        created_comments: Vec<(i64, String)>,
    }

    struct MockClient {
        tasks: Vec<Task>,
        state: Mutex<MockState>,
    }

    impl MockClient {
        fn new(tasks: Vec<Task>) -> Self {
            MockClient {
                tasks,
                state: Mutex::new(MockState::default()),
            }
        }
    }

    impl VikunjaClient for MockClient {
        async fn list_projects(&self) -> Result<Vec<Project>, ClientError> {
            unimplemented!()
        }
        async fn get_project(&self, _: i64) -> Result<Project, ClientError> {
            unimplemented!()
        }
        async fn get_user(&self) -> Result<User, ClientError> {
            unimplemented!()
        }
        async fn get_task(&self, task_id: i64) -> Result<Task, ClientError> {
            self.tasks
                .iter()
                .find(|t| t.id == task_id)
                .cloned()
                .ok_or(ClientError::Api {
                    status: 404,
                    message: format!("task {task_id} not found"),
                })
        }
        async fn list_project_tasks(&self, _: i64, filter: &str) -> Result<Vec<Task>, ClientError> {
            // Support "index = N" filter for resolve
            if let Some(idx_str) = filter.strip_prefix("index = ") {
                if let Ok(idx) = idx_str.parse::<i64>() {
                    return Ok(self
                        .tasks
                        .iter()
                        .filter(|t| t.index == idx)
                        .cloned()
                        .collect());
                }
            }
            Ok(vec![])
        }
        async fn list_bucket_tasks(
            &self,
            _: i64,
            _: i64,
            bucket_id: i64,
        ) -> Result<Vec<Task>, ClientError> {
            Ok(self
                .tasks
                .iter()
                .filter(|t| t.bucket_id == bucket_id)
                .cloned()
                .collect())
        }
        async fn list_view_tasks(
            &self,
            _: i64,
            _: i64,
            _: Option<&str>,
            _: Option<&str>,
        ) -> Result<Vec<Task>, ClientError> {
            Ok(self.tasks.clone())
        }
        async fn create_task(
            &self,
            _: i64,
            title: &str,
            _: &str,
            _: Option<i64>,
        ) -> Result<Task, ClientError> {
            Ok(make_task(99, title))
        }
        async fn update_task(
            &self,
            task_id: i64,
            updates: TaskUpdate,
        ) -> Result<Task, ClientError> {
            let mut task = self.get_task(task_id).await?;
            if let Some(done) = updates.done {
                task.done = done;
            }
            self.state
                .lock()
                .unwrap()
                .updated_tasks
                .push((task_id, updates));
            Ok(task)
        }
        async fn create_relation(
            &self,
            task_id: i64,
            other_task_id: i64,
            relation_kind: &str,
        ) -> Result<TaskRelation, ClientError> {
            Ok(TaskRelation {
                task_id,
                other_task_id,
                relation_kind: relation_kind.to_string(),
            })
        }
        async fn create_comment(
            &self,
            task_id: i64,
            comment: &str,
        ) -> Result<TaskComment, ClientError> {
            self.state
                .lock()
                .unwrap()
                .created_comments
                .push((task_id, comment.to_string()));
            Ok(TaskComment {
                id: 1,
                comment: comment.to_string(),
                author: User {
                    id: 1,
                    username: "test".to_string(),
                    name: "Test".to_string(),
                },
            })
        }
        async fn list_views(&self, _: i64) -> Result<Vec<ProjectView>, ClientError> {
            unimplemented!()
        }
        async fn list_buckets(&self, _: i64, _: i64) -> Result<Vec<Bucket>, ClientError> {
            let mut bucket_map: HashMap<i64, Vec<Task>> = HashMap::new();
            for task in &self.tasks {
                bucket_map
                    .entry(task.bucket_id)
                    .or_default()
                    .push(task.clone());
            }
            Ok(bucket_map
                .into_iter()
                .map(|(id, tasks)| Bucket {
                    id,
                    title: format!("Bucket {id}"),
                    tasks,
                })
                .collect())
        }
        async fn create_label(&self, title: &str) -> Result<Label, ClientError> {
            Ok(Label {
                id: 1,
                title: title.to_string(),
            })
        }
        async fn add_label_to_task(&self, _: i64, _: i64) -> Result<(), ClientError> {
            Ok(())
        }
        async fn list_labels(&self) -> Result<Vec<Label>, ClientError> {
            Ok(vec![])
        }
        async fn create_project(
            &self,
            _: &str,
            _: &str,
            _: Option<&str>,
        ) -> Result<Project, ClientError> {
            unimplemented!()
        }
        async fn delete_project(&self, _: i64) -> Result<(), ClientError> {
            unimplemented!()
        }
        async fn move_task_to_bucket(
            &self,
            _: i64,
            _: i64,
            bucket_id: i64,
            task_id: i64,
        ) -> Result<(), ClientError> {
            self.state
                .lock()
                .unwrap()
                .moved_to_bucket
                .push((bucket_id, task_id));
            Ok(())
        }
        async fn update_task_position(
            &self,
            _task_id: i64,
            _view_id: i64,
            _position: f64,
        ) -> Result<(), ClientError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn resolve_bare_numeric_by_index() {
        let mut task = make_task(99, "Test");
        task.index = 7;
        let pc = ProjectClient::new(MockClient::new(vec![task]), test_config());
        assert_eq!(pc.resolve("7").await.unwrap(), 99);
    }

    #[tokio::test]
    async fn resolve_hash_numeric_by_index() {
        let mut task = make_task(99, "Test");
        task.index = 7;
        let pc = ProjectClient::new(MockClient::new(vec![task]), test_config());
        assert_eq!(pc.resolve("#7").await.unwrap(), 99);
    }

    #[tokio::test]
    async fn resolve_identifier() {
        let mut task = make_task(42, "Test");
        task.index = 3;
        task.identifier = "PROJ-3".to_string();
        let pc = ProjectClient::new(MockClient::new(vec![task]), test_config());
        assert_eq!(pc.resolve("PROJ-3").await.unwrap(), 42);
    }

    #[tokio::test]
    async fn resolve_invalid_ref() {
        let pc = ProjectClient::new(MockClient::new(vec![]), test_config());
        assert!(pc.resolve("not-valid-ref").await.is_err());
    }

    #[tokio::test]
    async fn get_task_resolves_and_fetches() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "My Task")]),
            test_config(),
        );
        let task = pc.get_task("42").await.unwrap();
        assert_eq!(task.id, 42);
        assert_eq!(task.title, "My Task");
    }

    #[tokio::test]
    async fn claim_moves_to_inprogress_bucket() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "Claim Me")]),
            test_config(),
        );
        let task = pc.claim("42").await.unwrap();
        assert_eq!(task.title, "Claim Me");

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.moved_to_bucket, vec![(200, 42)]); // inprogress_bucket_id
    }

    #[tokio::test]
    async fn complete_marks_done_and_moves_to_done_bucket() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "Finish Me")]),
            test_config(),
        );
        let task = pc.complete("42").await.unwrap();
        assert!(task.done);

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.moved_to_bucket, vec![(300, 42)]); // done_bucket_id
        assert_eq!(state.updated_tasks.len(), 1);
        assert_eq!(state.updated_tasks[0].1.done, Some(true));
    }

    #[tokio::test]
    async fn comment_converts_markdown_to_html() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "Commented")]),
            test_config(),
        );
        pc.comment("42", "**bold**").await.unwrap();

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.created_comments.len(), 1);
        assert_eq!(state.created_comments[0].0, 42);
        assert!(
            state.created_comments[0]
                .1
                .contains("<strong>bold</strong>")
        );
    }

    #[tokio::test]
    async fn list_ready_filters_blocked_tasks() {
        let mut blocked = make_task(1, "Blocked");
        blocked.related_tasks.insert(
            "blocked".to_string(),
            vec![make_task(2, "Blocker")], // done: false
        );
        let ready = make_task(3, "Ready");

        let pc = ProjectClient::new(MockClient::new(vec![blocked, ready]), test_config());
        let tasks = pc.list_ready().await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Ready");
    }

    #[tokio::test]
    async fn list_ready_returns_tasks_ordered_by_position() {
        let mut t1 = make_task(1, "Third");
        t1.position = 30.0;
        let mut t2 = make_task(2, "First");
        t2.position = 10.0;
        let mut t3 = make_task(3, "Second");
        t3.position = 20.0;

        let pc = ProjectClient::new(MockClient::new(vec![t1, t2, t3]), test_config());
        let tasks = pc.list_ready().await.unwrap();
        assert_eq!(tasks[0].title, "First");
        assert_eq!(tasks[1].title, "Second");
        assert_eq!(tasks[2].title, "Third");
    }

    #[tokio::test]
    async fn list_in_progress_returns_tasks_ordered_by_position() {
        let mut t1 = make_task(1, "Second");
        t1.bucket_id = 200;
        t1.position = 20.0;
        let mut t2 = make_task(2, "First");
        t2.bucket_id = 200;
        t2.position = 10.0;

        let pc = ProjectClient::new(MockClient::new(vec![t1, t2]), test_config());
        let tasks = pc.list_in_progress().await.unwrap();
        assert_eq!(tasks[0].title, "First");
        assert_eq!(tasks[1].title, "Second");
    }

    #[tokio::test]
    async fn list_done_returns_tasks_ordered_by_position() {
        let mut t1 = make_task(1, "Second");
        t1.bucket_id = 300;
        t1.position = 5.0;
        let mut t2 = make_task(2, "First");
        t2.bucket_id = 300;
        t2.position = 1.0;

        let pc = ProjectClient::new(MockClient::new(vec![t1, t2]), test_config());
        let tasks = pc.list_done().await.unwrap();
        assert_eq!(tasks[0].title, "First");
        assert_eq!(tasks[1].title, "Second");
    }

    #[tokio::test]
    async fn list_tasks_returns_tasks_ordered_by_position() {
        let mut t1 = make_task(1, "Third");
        t1.position = 30.0;
        let mut t2 = make_task(2, "First");
        t2.position = 10.0;
        let mut t3 = make_task(3, "Second");
        t3.position = 20.0;

        let pc = ProjectClient::new(MockClient::new(vec![t1, t2, t3]), test_config());
        let tasks = pc.list_tasks(None, None).await.unwrap();
        assert_eq!(tasks[0].title, "First");
        assert_eq!(tasks[1].title, "Second");
        assert_eq!(tasks[2].title, "Third");
    }

    #[tokio::test]
    async fn add_relation_resolves_both_refs() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(1, "A"), make_task(2, "B")]),
            test_config(),
        );
        let rel = pc.add_relation("1", "2", "blocked").await.unwrap();
        assert_eq!(rel.task_id, 1);
        assert_eq!(rel.other_task_id, 2);
        assert_eq!(rel.relation_kind, "blocked");
    }

    #[tokio::test]
    async fn list_board_splits_by_bucket() {
        let mut ready_task = make_task(1, "Ready");
        ready_task.bucket_id = 100; // todo_bucket_id
        let mut wip_task = make_task(2, "WIP");
        wip_task.bucket_id = 200; // inprogress_bucket_id
        let mut done_task = make_task(3, "Done");
        done_task.bucket_id = 300; // done_bucket_id

        let pc = ProjectClient::new(
            MockClient::new(vec![ready_task, wip_task, done_task]),
            test_config(),
        );
        let board = pc.list_board().await.unwrap();
        assert_eq!(board.ready.len(), 1);
        assert_eq!(board.ready[0].title, "Ready");
        assert_eq!(board.in_progress.len(), 1);
        assert_eq!(board.in_progress[0].title, "WIP");
        assert_eq!(board.done.len(), 1);
        assert_eq!(board.done[0].title, "Done");
    }

    #[tokio::test]
    async fn list_board_filters_blocked_from_ready() {
        let mut blocked = make_task(1, "Blocked");
        blocked.bucket_id = 100;
        blocked.related_tasks.insert(
            "blocked".to_string(),
            vec![make_task(2, "Blocker")], // done: false
        );
        let mut ready = make_task(3, "Ready");
        ready.bucket_id = 100;

        let pc = ProjectClient::new(MockClient::new(vec![blocked, ready]), test_config());
        let board = pc.list_board().await.unwrap();
        assert_eq!(board.ready.len(), 1);
        assert_eq!(board.ready[0].title, "Ready");
    }

    #[tokio::test]
    async fn list_board_sorts_by_position() {
        let mut t1 = make_task(1, "Third");
        t1.bucket_id = 100;
        t1.position = 30.0;
        let mut t2 = make_task(2, "First");
        t2.bucket_id = 100;
        t2.position = 10.0;
        let mut t3 = make_task(3, "Second");
        t3.bucket_id = 100;
        t3.position = 20.0;

        let pc = ProjectClient::new(MockClient::new(vec![t1, t2, t3]), test_config());
        let board = pc.list_board().await.unwrap();
        assert_eq!(board.ready[0].title, "First");
        assert_eq!(board.ready[1].title, "Second");
        assert_eq!(board.ready[2].title, "Third");
    }

    #[tokio::test]
    async fn list_board_ignores_unknown_buckets() {
        let mut task = make_task(1, "Unknown");
        task.bucket_id = 999; // not in config

        let pc = ProjectClient::new(MockClient::new(vec![task]), test_config());
        let board = pc.list_board().await.unwrap();
        assert!(board.ready.is_empty());
        assert!(board.in_progress.is_empty());
        assert!(board.done.is_empty());
    }

    #[tokio::test]
    async fn move_to_column_moves_to_inprogress() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "Move Me")]),
            test_config(),
        );
        let task = pc.move_to_column("42", 200).await.unwrap();
        assert_eq!(task.title, "Move Me");

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.moved_to_bucket, vec![(200, 42)]);
        // Moving to inprogress should set done = false
        assert_eq!(state.updated_tasks.len(), 1);
        assert_eq!(state.updated_tasks[0].1.done, Some(false));
    }

    #[tokio::test]
    async fn move_to_column_sets_done_when_moving_to_done_bucket() {
        let pc = ProjectClient::new(
            MockClient::new(vec![make_task(42, "Finish Me")]),
            test_config(),
        );
        let task = pc.move_to_column("42", 300).await.unwrap();
        assert!(task.done);

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.moved_to_bucket, vec![(300, 42)]);
        assert_eq!(state.updated_tasks[0].1.done, Some(true));
    }

    #[tokio::test]
    async fn move_to_column_unsets_done_when_moving_to_todo() {
        let mut task = make_task(42, "Reopen Me");
        task.done = true;
        let pc = ProjectClient::new(MockClient::new(vec![task]), test_config());
        let result = pc.move_to_column("42", 100).await.unwrap();
        // The mock returns the task after update_task, which should set done=false
        assert!(!result.done);

        let state = pc.client().state.lock().unwrap();
        assert_eq!(state.moved_to_bucket, vec![(100, 42)]);
        assert_eq!(state.updated_tasks[0].1.done, Some(false));
    }
}
