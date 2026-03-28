use clap::{CommandFactory, Parser};
use clap_complete::generate;
use rmcp::ServiceExt;

use vein::cli::{Cli, Command};
use vein::client::{ReqwestClient, VikunjaClient};
use vein::config::{ConnectionConfig, ProjectConfig};
use vein::init;
use vein::project::ProjectClient;
use vein::server::{VeinServer, format_task_detail, format_task_list, parse_priority};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init) => {
            let config = ConnectionConfig::from_env()?;
            let client = init::make_client(&config)?;
            init::run(&client).await
        }
        Some(Command::ListProjects) => {
            let config = ConnectionConfig::from_env()?;
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
            let config = ConnectionConfig::from_env()?;
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
            let config = ConnectionConfig::from_env()?;
            let client = ReqwestClient::new(&config)?;
            let buckets = client.list_buckets(project_id, view_id).await?;
            for bucket in buckets {
                println!("{}\t{}", bucket.id, bucket.title);
            }
            Ok(())
        }
        Some(Command::Completions { shell }) => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "vein", &mut std::io::stdout());
            Ok(())
        }
        Some(
            Command::ListReady
            | Command::ListTasks { .. }
            | Command::ListInProgress
            | Command::ListDone
            | Command::GetTask { .. }
            | Command::Claim { .. }
            | Command::Complete { .. }
            | Command::Comment { .. }
            | Command::CreateLabel { .. }
            | Command::AddLabel { .. }
            | Command::ListLabels
            | Command::AddRelation { .. }
            | Command::UpdateTask { .. }
            | Command::CreateTask { .. },
        ) => {
            let conn_config = ConnectionConfig::from_env()?;
            let project_config = ProjectConfig::from_env()?;
            let client = ReqwestClient::new(&conn_config)?;
            let project = ProjectClient::new(client, project_config);
            match cli.command.unwrap() {
                Command::ListReady => {
                    let ready = project.list_ready().await?;
                    println!(
                        "{}",
                        format_task_list(&ready, "No tasks ready to be worked on.")
                    );
                }
                Command::ListTasks { filter, search } => {
                    let tasks = project
                        .list_tasks(filter.as_deref(), search.as_deref())
                        .await?;
                    println!("{}", format_task_list(&tasks, "No tasks found."));
                }
                Command::ListInProgress => {
                    let tasks = project.list_in_progress().await?;
                    println!(
                        "{}",
                        format_task_list(&tasks, "No tasks currently in progress.")
                    );
                }
                Command::ListDone => {
                    let tasks = project.list_done().await?;
                    println!("{}", format_task_list(&tasks, "No completed tasks."));
                }
                Command::GetTask { task_id } => {
                    let task = project.get_task(&task_id).await?;
                    println!("{}", format_task_detail(&task));
                }
                Command::Claim { task_id } => {
                    let task = project.claim(&task_id).await?;
                    println!("Claimed task {}: {}", task.display_id(), task.title);
                }
                Command::Complete { task_id } => {
                    let task = project.complete(&task_id).await?;
                    println!("Completed task {}: {}", task.display_id(), task.title);
                }
                Command::Comment { task_id, comment } => {
                    project.comment(&task_id, &comment).await?;
                    println!("Added comment to task {task_id}");
                }
                Command::CreateLabel { title } => {
                    let label = project.create_label(&title).await?;
                    println!("Created label #{}: {}", label.id, label.title);
                }
                Command::AddLabel { task_id, label_id } => {
                    project.add_label(&task_id, label_id).await?;
                    println!("Added label #{label_id} to task {task_id}");
                }
                Command::ListLabels => {
                    let labels = project.list_labels().await?;
                    if labels.is_empty() {
                        println!("No labels found.");
                    } else {
                        for label in labels {
                            println!("- #{}: {}", label.id, label.title);
                        }
                    }
                }
                Command::AddRelation {
                    task_id,
                    other_task_id,
                    relation_kind,
                } => {
                    let relation = project
                        .add_relation(&task_id, &other_task_id, &relation_kind)
                        .await?;
                    println!(
                        "Added {} relation: {} -> {}",
                        relation.relation_kind, task_id, other_task_id
                    );
                }
                Command::UpdateTask {
                    task_id,
                    title,
                    description,
                    priority,
                } => {
                    let priority = priority.map(|p| parse_priority(&p)).transpose()?;
                    let task = project
                        .update_task(&task_id, title, description, priority)
                        .await?;
                    println!("Updated task {}: {}", task.display_id(), task.title);
                }
                Command::CreateTask {
                    title,
                    description,
                    priority,
                } => {
                    let priority = priority.map(|p| parse_priority(&p)).transpose()?;
                    let task = project
                        .create_task(&title, Some(&description), priority)
                        .await?;
                    println!("Created task {}: {}", task.display_id(), task.title);
                }
                _ => unreachable!(),
            }
            Ok(())
        }
        Some(Command::Serve) => {
            let conn_config = ConnectionConfig::from_env()?;
            let project_config = ProjectConfig::from_env()?;
            let client = ReqwestClient::new(&conn_config)?;
            let server = VeinServer::new(client, project_config);
            let service = server.serve(rmcp::transport::io::stdio()).await?;
            service.waiting().await?;
            Ok(())
        }
        None => unreachable!("clap enforces subcommand_required"),
    }
}
