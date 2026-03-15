use dialoguer::Select;

use crate::client::{Bucket, Project, ProjectView, VikunjaClient};
use crate::config::ConnectionConfig;

pub async fn run(client: &impl VikunjaClient) -> Result<(), Box<dyn std::error::Error>> {
    let projects: Vec<Project> = client
        .list_projects()
        .await?
        .into_iter()
        .filter(|p| !p.is_archived)
        .collect();

    if projects.is_empty() {
        eprintln!("No projects found.");
        return Ok(());
    }

    let project_labels: Vec<String> = projects
        .iter()
        .map(|p| format!("{} (id: {})", p.title, p.id))
        .collect();

    let project_idx = Select::new()
        .with_prompt("Select a project")
        .items(&project_labels)
        .interact()?;

    let project = &projects[project_idx];

    let views: Vec<ProjectView> = client.list_views(project.id).await?;
    let kanban_views: Vec<&ProjectView> =
        views.iter().filter(|v| v.view_kind == "kanban").collect();

    let view = match kanban_views.len() {
        0 => {
            eprintln!("No kanban views found for project '{}'.", project.title);
            return Ok(());
        }
        1 => kanban_views[0],
        _ => {
            let view_labels: Vec<String> = kanban_views
                .iter()
                .map(|v| format!("{} (id: {})", v.title, v.id))
                .collect();

            let view_idx = Select::new()
                .with_prompt("Multiple kanban views found — select one")
                .items(&view_labels)
                .interact()?;

            kanban_views[view_idx]
        }
    };

    let buckets: Vec<Bucket> = client.list_buckets(project.id, view.id).await?;

    if buckets.is_empty() {
        eprintln!("No buckets found for view '{}'.", view.title);
        return Ok(());
    }

    let bucket_labels: Vec<String> = buckets
        .iter()
        .map(|b| format!("{} (id: {})", b.title, b.id))
        .collect();

    let todo_idx = Select::new()
        .with_prompt("Select the Todo bucket")
        .items(&bucket_labels)
        .interact()?;

    let inprogress_idx = Select::new()
        .with_prompt("Select the In Progress bucket")
        .items(&bucket_labels)
        .interact()?;

    let done_idx = Select::new()
        .with_prompt("Select the Done bucket")
        .items(&bucket_labels)
        .interact()?;

    println!();
    println!("# Add these to your environment:");
    println!("VIKUNJA_PROJECT_ID={}", project.id);
    println!("VIKUNJA_VIEW_ID={}", view.id);
    println!("VIKUNJA_TODO_BUCKET_ID={}", buckets[todo_idx].id);
    println!(
        "VIKUNJA_INPROGRESS_BUCKET_ID={}",
        buckets[inprogress_idx].id
    );
    println!("VIKUNJA_DONE_BUCKET_ID={}", buckets[done_idx].id);

    Ok(())
}

pub fn make_client(
    config: &ConnectionConfig,
) -> Result<crate::client::ReqwestClient, crate::client::ClientError> {
    crate::client::ReqwestClient::new(config)
}
