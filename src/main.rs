mod cli;
mod client;
mod config;
mod server;

use clap::Parser;
use rmcp::ServiceExt;

use cli::{Cli, Command};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => {
            eprintln!("vein init: not yet implemented");
            Ok(())
        }
        Some(Command::Serve) | None => {
            let server = server::VeinServer::new();
            let service = server.serve(rmcp::transport::io::stdio()).await?;
            service.waiting().await?;
            Ok(())
        }
    }
}
