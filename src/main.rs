mod cli;
mod client;
mod config;
mod server;

use clap::Parser;
use rmcp::ServiceExt;

use cli::{Cli, Command};
use client::{ReqwestClient, VikunjaClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => {
            eprintln!("vein init: not yet implemented");
            Ok(())
        }
        Some(Command::ListProjects) => {
            let config = config::ConnectionConfig::from_env()?;
            let client = ReqwestClient::new(&config)?;
            let projects = client.list_projects().await?;
            for project in projects {
                if project.is_archived {
                    continue;
                }
                println!("{}\t{}", project.id, project.title);
            }
            Ok(())
        }
        Some(Command::ListProjectViews { project_id }) => {
            let config = config::ConnectionConfig::from_env()?;
            let client = ReqwestClient::new(&config)?;
            let views = client.list_views(project_id).await?;
            for view in views {
                println!("{}\t{}\t{}", view.id, view.view_kind, view.title);
            }
            Ok(())
        }
        Some(Command::ListProjectViewBuckets {
            project_id,
            view_id,
        }) => {
            let config = config::ConnectionConfig::from_env()?;
            let client = ReqwestClient::new(&config)?;
            let buckets = client.list_buckets(project_id, view_id).await?;
            for bucket in buckets {
                println!("{}\t{}", bucket.id, bucket.title);
            }
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
