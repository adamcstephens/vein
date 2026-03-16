use rmcp::ServiceExt;
use vein::client::{ClientError, ReqwestClient, VikunjaClient};
use vein::config::{ConnectionConfig, ProjectConfig};
use vein::server::VeinServer;

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
    client: ReqwestClient,
}

impl TestProject {
    async fn create(client: ReqwestClient) -> Result<Self, Box<dyn std::error::Error>> {
        let project = client
            .create_project("vein-integration-test", "Auto-created by integration tests")
            .await?;

        let views = client.list_views(project.id).await?;
        let kanban_view = views
            .iter()
            .find(|v| v.view_kind == "kanban")
            .ok_or("no kanban view found on new project")?;

        let buckets = client.list_buckets(project.id, kanban_view.id).await?;
        if buckets.len() < 3 {
            return Err(format!("expected at least 3 buckets, found {}", buckets.len()).into());
        }

        let config = ProjectConfig {
            project_id: project.id,
            view_id: kanban_view.id,
            todo_bucket_id: buckets[0].id,
            inprogress_bucket_id: buckets[1].id,
            done_bucket_id: buckets[2].id,
        };

        Ok(TestProject {
            id: project.id,
            config,
            client,
        })
    }

    async fn cleanup(self) -> Result<(), ClientError> {
        self.client.delete_project(self.id).await
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
async fn initialize_and_list_tools() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;
    let tools = client.list_all_tools().await.expect("failed to list tools");

    assert!(
        tools.iter().any(|t| t.name == "list_ready"),
        "list_ready tool should be registered"
    );

    test_project
        .cleanup()
        .await
        .expect("failed to clean up test project");
}

#[tokio::test]
async fn server_reports_tool_capabilities() {
    let test_project = TestProject::create(vikunja_client())
        .await
        .expect("failed to create test project");

    let client = mcp_client(&test_project).await;

    let result = client
        .list_tools(None)
        .await
        .expect("server should support tools/list");

    assert!(!result.tools.is_empty(), "should have at least one tool");

    test_project
        .cleanup()
        .await
        .expect("failed to clean up test project");
}

#[tokio::test]
async fn create_and_delete_test_project() {
    let vikunja = vikunja_client();

    let project = vikunja
        .create_project("vein-integration-test", "Temp project for testing")
        .await
        .expect("failed to create project");

    assert_eq!(project.title, "vein-integration-test");

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
