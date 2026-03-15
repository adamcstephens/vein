use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vein", about = "Agent-focused issue tracker backed by Vikunja")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Discover Vikunja projects and buckets, print env var configuration
    Init,
    /// List available Vikunja projects
    ListProjects,
    /// Run as MCP stdio server (default if no subcommand given)
    Serve,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_init_subcommand() {
        let cli = Cli::parse_from(["vein", "init"]);
        assert!(matches!(cli.command, Some(Command::Init)));
    }

    #[test]
    fn parses_list_projects_subcommand() {
        let cli = Cli::parse_from(["vein", "list-projects"]);
        assert!(matches!(cli.command, Some(Command::ListProjects)));
    }

    #[test]
    fn parses_serve_subcommand() {
        let cli = Cli::parse_from(["vein", "serve"]);
        assert!(matches!(cli.command, Some(Command::Serve)));
    }

    #[test]
    fn defaults_to_none_when_no_subcommand() {
        let cli = Cli::parse_from(["vein"]);
        assert!(cli.command.is_none());
    }
}
