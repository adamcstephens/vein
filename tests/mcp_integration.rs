use rmcp::ServiceExt;
use vein::client::{ClientError, ReqwestClient, VikunjaClient};
use vein::config::ConnectionConfig;
use vein::server::VeinServer;

/// Create a ReqwestClient from env vars, or panic if not set.
fn vikunja_client() -> ReqwestClient {
    let config = ConnectionConfig::from_env()
        .expect("VIKUNJA_URL and VIKUNJA_API_TOKEN must be set for integration tests");
    ReqwestClient::new(&config).expect("failed to create Vikunja client")
}

/// Spin up a VeinServer and an MCP client connected over an in-memory duplex.
/// Returns the MCP client's RunningService (which derefs to Peer<RoleClient>).
async fn mcp_client() -> rmcp::service::RunningService<rmcp::RoleClient, ()> {
    let (server_transport, client_transport) = tokio::io::duplex(4096);

    tokio::spawn(async move {
        let server = VeinServer::new();
        let service = server.serve(server_transport).await.unwrap();
        service.waiting().await.unwrap();
    });

    let client: rmcp::service::RunningService<rmcp::RoleClient, ()> =
        ().serve(client_transport).await.unwrap();
    client
}

/// Create a test project and return its id; call cleanup() to delete.
struct TestProject {
    pub id: i64,
    client: ReqwestClient,
}

impl TestProject {
    async fn create(client: ReqwestClient) -> Result<Self, ClientError> {
        let project = client
            .create_project("vein-integration-test", "Auto-created by integration tests")
            .await?;
        Ok(TestProject {
            id: project.id,
            client,
        })
    }

    async fn cleanup(self) -> Result<(), ClientError> {
        self.client.delete_project(self.id).await
    }
}

#[tokio::test]
async fn initialize_and_list_tools() {
    let client = mcp_client().await;
    let tools = client.list_all_tools().await.expect("failed to list tools");

    // Server should report its capabilities — even if no tools are registered yet,
    // the list call should succeed.
    drop(tools);
}

#[tokio::test]
async fn server_reports_tool_capabilities() {
    let client = mcp_client().await;

    let result = client
        .list_tools(None)
        .await
        .expect("server should support tools/list");

    // Even with zero tools, the response should be valid
    drop(result.tools);
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

    // Verify it's gone
    let projects = vikunja
        .list_projects()
        .await
        .expect("failed to list projects");
    assert!(
        !projects.iter().any(|p| p.id == project.id),
        "project should be deleted"
    );
}
