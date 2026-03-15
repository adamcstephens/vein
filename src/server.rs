use rmcp::{
    ServerHandler,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    tool_router,
};

#[derive(Debug, Clone)]
pub struct VeinServer {
    tool_router: ToolRouter<Self>,
}

impl VeinServer {
    pub fn new() -> Self {
        VeinServer {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl VeinServer {}

impl ServerHandler for VeinServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Vikunja-backed issue tracker for AI agents")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_creates_with_empty_tool_router() {
        let server = VeinServer::new();
        let info = server.get_info();
        assert!(info.capabilities.tools.is_some());
    }
}
