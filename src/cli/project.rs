use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct ProjectCli {
    #[command(subcommand)]
    pub action: ProjectCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// List all projects
    #[command(alias = "ls")]
    List(ProjectListArgs),
    /// Start a specific project
    Start(ProjectStartArgs),
}

#[derive(Debug, Parser)]
pub struct ProjectListArgs {
    /// Show minimal output for scripts
    #[arg(short, long, default_value_t = false)]
    pub minimal: bool,
}

#[derive(Debug, Parser)]
pub struct ProjectStartArgs {
    /// The name of the project as it's defined in the config
    pub name: String,

    /// Start the session detached
    #[arg(short, long, default_value_t = false)]
    pub detached: bool,

    /// Always start a new session instead of attaching to an existing session
    #[arg(short = 'n', long, default_value_t = false)]
    pub always_new_session: bool,
}
