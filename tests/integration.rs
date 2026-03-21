use std::time::{SystemTime, UNIX_EPOCH};

use rmcp::ServiceExt;
use vein::client::{ReqwestClient, TaskUpdate, VikunjaClient};
use vein::config::{ConnectionConfig, ProjectConfig};
use vein::project::ProjectClient;
use vein::server::VeinServer;

fn unique_project_name() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_millis();
    format!("vein-test-{}-{}", std::process::id(), ts)
}

fn random_identifier() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    let mut val = ts ^ (std::process::id() as u128);
    let mut chars = Vec::with_capacity(4);
    for _ in 0..4 {
        chars.push(b'A' + (val % 26) as u8);
        val /= 26;
    }
    String::from_utf8(chars).expect("valid ascii")
}

/// Create a ReqwestClient from env vars, or panic if not set.
fn vikunja_client() -> ReqwestClient {
    let config = ConnectionConfig::from_env()
        .expect("VIKUNJA_URL and VIKUNJA_API_TOKEN must be set for integration tests");
    ReqwestClient::new(&config).expect("failed to create Vikunja client")
}

/// Create a test project, discover its views/buckets, and return the project
/// along with a ready-to-use ProjectConfig.
struct TestProject {
    pub id: i64,
    pub config: ProjectConfig,
}

impl TestProject {
    async fn create(client: ReqwestClient) -> Result<Self, Box<dyn std::error::Error>> {
        Self::create_with_identifier(client, None).await
    }

    async fn create_with_identifier(
        client: ReqwestClient,
        identifier: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let name = unique_project_name();
        let project = client
            .create_project(&name, "Auto-created by integration tests", identifier)
            .await?;

        let views = client.list_views(project.id).await?;
        let kanban_view = views
            .iter()
            .find(|v| v.view_kind == "kanban")
            .ok_or("no kanban view found on new project")?;

        let buckets = client.list_buckets(project.id, kanban_view.id).await?;

        let find_bucket = |needle: &str| -> Result<i64, Box<dyn std::error::Error>> {
            buckets
                .iter()
                .find(|b| b.title.eq_ignore_ascii_case(needle))
                .map(|b| b.id)
                .ok_or_else(|| format!("no bucket named '{}' found", needle).into())
        };

        let config = ProjectConfig {
            project_id: project.id,
            view_id: kanban_view.id,
            todo_bucket_id: find_bucket("To-Do")?,
            inprogress_bucket_id: find_bucket("Doing")?,
            done_bucket_id: find_bucket("Done")?,
        };

        Ok(TestProject {
            id: project.id,
            config,
        })
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let client = vikunja_client();
        let id = self.id;
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to create cleanup runtime");
            rt.block_on(async {
                let _ = client.delete_project(id).await;
            });
        });
        let _ = handle.join();
    }
}

/// Spin up a VeinServer backed by a real test project, connected to an MCP
/// client over an in-memory duplex.
async fn mcp_client(
    test_project: &TestProject,
) -> rmcp::service::RunningService<rmcp::RoleClient, ()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);

    let server = VeinServer::new(vikunja_client(), test_project.config.clone());

    tokio::spawn(async move {
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
    });

    let client: rmcp::service::RunningService<rmcp::RoleClient, ()> =
        ().serve(client_transport).await.unwrap();
    client
}

#[tokio::test]
async fn mcp_initialize_and_list_tools() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;
    let tools = client.list_all_tools().await.expect("failed to list tools");

    assert!(
        tools.iter().any(|t| t.name == "list_ready"),
        "list_ready tool should be registered"
    );
}

#[tokio::test]
async fn mcp_server_reports_prompt_capabilities() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;

    let result = client
        .list_prompts(None)
        .await
        .expect("server should support prompts/list");

    assert!(
        result.prompts.iter().any(|p| p.name == "orient"),
        "orient prompt should be registered"
    );
}

#[tokio::test]
async fn mcp_orient_prompt_returns_orientation() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;

    let result = client
        .get_prompt(rmcp::model::GetPromptRequestParams::new("orient"))
        .await
        .expect("should be able to get orient prompt");

    assert!(!result.messages.is_empty(), "orient should return messages");
    let text = match &result.messages[0].content {
        rmcp::model::PromptMessageContent::Text { text } => text,
        _ => panic!("expected text content"),
    };
    assert!(
        text.contains("Available Tools"),
        "orient should list available tools"
    );
    assert!(
        text.contains("Workflow"),
        "orient should include workflow guidance"
    );
}

#[tokio::test]
async fn mcp_server_reports_tool_capabilities() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;

    let result = client
        .list_tools(None)
        .await
        .expect("server should support tools/list");

    assert!(!result.tools.is_empty(), "should have at least one tool");
}

#[tokio::test]
async fn create_and_delete_test_project() {
    let vikunja = vikunja_client();

    let name = unique_project_name();
    let project = vikunja
        .create_project(&name, "Temp project for testing", None)
        .await
        .expect("failed to create project");

    assert_eq!(project.title, name);

    vikunja
        .delete_project(project.id)
        .await
        .expect("failed to delete project");

    let projects = vikunja
        .list_projects()
        .await
        .expect("failed to list projects");
    assert!(
        !projects.iter().any(|p| p.id == project.id),
        "project should be deleted"
    );
}

#[tokio::test]
async fn update_task_preserves_fields_not_in_update() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = vikunja_client();

    let task = client
        .create_task(
            test_project.config.project_id,
            "Preserve me",
            "This description must survive updates",
            Some(3),
        )
        .await
        .expect("failed to create task");

    assert_eq!(task.description, "This description must survive updates");
    assert_eq!(task.priority, 3);

    // Partial update: only change done (simulates `complete` without bucket move)
    let updated = client
        .update_task(
            task.id,
            TaskUpdate {
                done: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("failed to update task");

    assert_eq!(
        updated.description, "This description must survive updates",
        "description was wiped by partial update"
    );
    assert_eq!(updated.priority, 3, "priority was wiped by partial update");
    assert!(updated.done);
}

#[tokio::test]
async fn claim_moves_task_to_in_progress_bucket() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = vikunja_client();

    // Create a task (lands in todo bucket by default)
    let task = client
        .create_task(test_project.config.project_id, "Move me", "", None)
        .await
        .expect("failed to create task");

    // Move it to in-progress bucket
    client
        .move_task_to_bucket(
            test_project.config.project_id,
            test_project.config.view_id,
            test_project.config.inprogress_bucket_id,
            task.id,
        )
        .await
        .expect("failed to move task to in-progress bucket");

    // Verify it appears in the in-progress bucket
    let in_progress = client
        .list_bucket_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            test_project.config.inprogress_bucket_id,
        )
        .await
        .expect("failed to list in-progress tasks");

    assert!(
        in_progress.iter().any(|t| t.id == task.id),
        "task should be in the in-progress bucket after move"
    );

    // Verify it's no longer in todo
    let todo = client
        .list_bucket_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            test_project.config.todo_bucket_id,
        )
        .await
        .expect("failed to list todo tasks");

    assert!(
        !todo.iter().any(|t| t.id == task.id),
        "task should not be in the todo bucket after move"
    );
}

#[tokio::test]
async fn list_view_tasks_without_filter_returns_all_tasks() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = vikunja_client();

    // Create tasks in different buckets
    let task1 = client
        .create_task(test_project.config.project_id, "Todo task", "", None)
        .await
        .expect("failed to create task 1");

    let task2 = client
        .create_task(test_project.config.project_id, "Doing task", "", None)
        .await
        .expect("failed to create task 2");

    // Move task2 to in-progress bucket
    client
        .move_task_to_bucket(
            test_project.config.project_id,
            test_project.config.view_id,
            test_project.config.inprogress_bucket_id,
            task2.id,
        )
        .await
        .expect("failed to move task to in-progress bucket");

    // list_view_tasks with no filter should return all tasks
    let tasks = client
        .list_view_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            None,
            None,
        )
        .await
        .expect("list_view_tasks without filter should not error");

    assert!(
        tasks.iter().any(|t| t.id == task1.id),
        "should contain the todo task"
    );
    assert!(
        tasks.iter().any(|t| t.id == task2.id),
        "should contain the in-progress task"
    );

    // Verify bucket_id is correctly populated
    let found1 = tasks.iter().find(|t| t.id == task1.id).unwrap();
    assert_eq!(
        found1.bucket_id, test_project.config.todo_bucket_id,
        "todo task should have todo bucket_id"
    );

    let found2 = tasks.iter().find(|t| t.id == task2.id).unwrap();
    assert_eq!(
        found2.bucket_id, test_project.config.inprogress_bucket_id,
        "in-progress task should have inprogress bucket_id"
    );

    // list_view_tasks with a filter should also work
    let filtered = client
        .list_view_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            Some("done = false"),
            None,
        )
        .await
        .expect("list_view_tasks with filter should not error");

    assert!(
        filtered.iter().any(|t| t.id == task1.id),
        "filtered results should contain undone task"
    );

    // list_view_tasks with search should work
    let searched = client
        .list_view_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            None,
            Some("Doing"),
        )
        .await
        .expect("list_view_tasks with search should not error");

    assert!(
        searched.iter().any(|t| t.id == task2.id),
        "search results should contain matching task"
    );
    assert!(
        !searched.iter().any(|t| t.id == task1.id),
        "search results should not contain non-matching task"
    );

    // list_view_tasks with both filter and search should work
    let combined = client
        .list_view_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            Some("done = false"),
            Some("Todo"),
        )
        .await
        .expect("list_view_tasks with filter and search should not error");

    assert!(
        combined.iter().any(|t| t.id == task1.id),
        "combined filter+search should find matching task"
    );
    assert!(
        !combined.iter().any(|t| t.id == task2.id),
        "combined filter+search should exclude non-matching task"
    );

    // list_view_tasks with a filter matching nothing should return empty
    let empty = client
        .list_view_tasks(
            test_project.config.project_id,
            test_project.config.view_id,
            Some("priority = 99"),
            None,
        )
        .await
        .expect("list_view_tasks with no-match filter should not error");

    assert!(empty.is_empty(), "no-match filter should return empty vec");
}

#[tokio::test]
async fn task_identifier_resolves_to_correct_task() {
    let client = vikunja_client();
    let ident = random_identifier();
    let test_project = TestProject::create_with_identifier(client, Some(&ident))
        .await
        .expect("failed to create project with identifier");

    let project = ProjectClient::new(vikunja_client(), test_project.config.clone());

    // Create two tasks — they should get {IDENT}-1 and {IDENT}-2
    let task1 = project
        .create_task("First task", None, None)
        .await
        .expect("failed to create task 1");
    let task2 = project
        .create_task("Second task", None, None)
        .await
        .expect("failed to create task 2");

    assert_eq!(task1.identifier, format!("{ident}-1"));
    assert_eq!(task2.identifier, format!("{ident}-2"));
    assert_eq!(task1.display_id(), format!("{ident}-1"));

    // Resolve {IDENT}-2 via ProjectClient
    let ref_str = format!("{ident}-2");
    let resolved_id = project
        .resolve(&ref_str)
        .await
        .expect("failed to resolve identifier");

    assert_eq!(
        resolved_id, task2.id,
        "identifier should resolve to task2's ID"
    );

    // Fetch by identifier and verify it's the right task
    let fetched = project
        .get_task(&ref_str)
        .await
        .expect("failed to get task by identifier");

    assert_eq!(fetched.title, "Second task");
    assert_eq!(fetched.identifier, format!("{ident}-2"));
}
