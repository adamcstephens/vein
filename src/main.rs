use clap::Parser;
use rmcp::ServiceExt;

use vein::cli::{Cli, Command, ToolCommand};
use vein::client::{ReqwestClient, VikunjaClient};
use vein::config::{ConnectionConfig, ProjectConfig};
use vein::init;
use vein::server::{VeinServer, fetch_ready_tasks, format_task_detail, format_task_list};

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
        Some(Command::Tool { tool }) => {
            let conn_config = ConnectionConfig::from_env()?;
            let project_config = ProjectConfig::from_env()?;
            let client = ReqwestClient::new(&conn_config)?;
            match tool {
                ToolCommand::ListReady => {
                    let ready = fetch_ready_tasks(&client, &project_config).await?;
                    println!(
                        "{}",
                        format_task_list(&ready, "No tasks ready to be worked on.")
                    );
                }
                ToolCommand::ListTasks { filter, search } => {
                    let tasks = client
                        .list_view_tasks(
                            project_config.project_id,
                            project_config.view_id,
                            filter.as_deref(),
                            search.as_deref(),
                        )
                        .await?;
                    println!("{}", format_task_list(&tasks, "No tasks found."));
                }
                ToolCommand::ListInProgress => {
                    let tasks = client
                        .list_bucket_tasks(
                            project_config.project_id,
                            project_config.view_id,
                            project_config.inprogress_bucket_id,
                        )
                        .await?;
                    println!(
                        "{}",
                        format_task_list(&tasks, "No tasks currently in progress.")
                    );
                }
                ToolCommand::ListDone => {
                    let tasks = client
                        .list_bucket_tasks(
                            project_config.project_id,
                            project_config.view_id,
                            project_config.done_bucket_id,
                        )
                        .await?;
                    println!("{}", format_task_list(&tasks, "No completed tasks."));
                }
                ToolCommand::GetTask { task_id } => {
                    let task = client.get_task(task_id).await?;
                    println!("{}", format_task_detail(&task));
                }
                ToolCommand::Claim { task_id } => {
                    let task = client.get_task(task_id).await?;
                    client
                        .move_task_to_bucket(
                            project_config.project_id,
                            project_config.view_id,
                            project_config.inprogress_bucket_id,
                            task_id,
                        )
                        .await?;
                    println!("Claimed task #{}: {}", task.id, task.title);
                }
                ToolCommand::Complete { task_id } => {
                    let task = client
                        .update_task(
                            task_id,
                            vein::client::TaskUpdate {
                                done: Some(true),
                                ..Default::default()
                            },
                        )
                        .await?;
                    client
                        .move_task_to_bucket(
                            project_config.project_id,
                            project_config.view_id,
                            project_config.done_bucket_id,
                            task_id,
                        )
                        .await?;
                    println!("Completed task #{}: {}", task.id, task.title);
                }
                ToolCommand::Comment { task_id, comment } => {
                    let result = client.create_comment(task_id, &comment).await?;
                    println!("Added comment #{} to task #{}", result.id, task_id);
                }
                ToolCommand::CreateLabel { title } => {
                    let label = client.create_label(&title).await?;
                    println!("Created label #{}: {}", label.id, label.title);
                }
                ToolCommand::AddLabel { task_id, label_id } => {
                    client.add_label_to_task(task_id, label_id).await?;
                    println!("Added label #{label_id} to task #{task_id}");
                }
                ToolCommand::ListLabels => {
                    let labels = client.list_labels().await?;
                    if labels.is_empty() {
                        println!("No labels found.");
                    } else {
                        for label in labels {
                            println!("- #{}: {}", label.id, label.title);
                        }
                    }
                }
                ToolCommand::AddRelation {
                    task_id,
                    other_task_id,
                    relation_kind,
                } => {
                    let relation = client
                        .create_relation(task_id, other_task_id, &relation_kind)
                        .await?;
                    println!(
                        "Added {} relation: #{} -> #{}",
                        relation.relation_kind, relation.task_id, relation.other_task_id
                    );
                }
                ToolCommand::UpdateTask {
                    task_id,
                    title,
                    description,
                    priority,
                } => {
                    let priority = priority
                        .map(|p| vein::server::parse_priority(&p))
                        .transpose()?;
                    let task = client
                        .update_task(
                            task_id,
                            vein::client::TaskUpdate {
                                title,
                                description,
                                priority,
                                ..Default::default()
                            },
                        )
                        .await?;
                    println!("Updated task #{}: {}", task.id, task.title);
                }
                ToolCommand::CreateTask {
                    title,
                    description,
                    priority,
                } => {
                    let priority = priority
                        .map(|p| vein::server::parse_priority(&p))
                        .transpose()?;
                    let task = client
                        .create_task(project_config.project_id, &title, &description, priority)
                        .await?;
                    println!("Created task #{}: {}", task.id, task.title);
                }
            }
            Ok(())
        }
        Some(Command::Serve) | None => {
            let conn_config = ConnectionConfig::from_env()?;
            let project_config = ProjectConfig::from_env()?;
            let client = ReqwestClient::new(&conn_config)?;
            let server = VeinServer::new(client, project_config);
            let service = server.serve(rmcp::transport::io::stdio()).await?;
            service.waiting().await?;
            Ok(())
        }
    }
}
