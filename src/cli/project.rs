use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct ProjectCli {
    #[command(subcommand)]
    pub action: ProjectCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    #[command(alias = "ls")]
    List(ProjectListArgs),
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
    pub name: String,

    /// Start the session detached
    #[arg(short, long, default_value_t = false)]
    pub detached: bool,

    /// Always start a new session instead of attaching to an existing session
    #[arg(long, default_value_t = false)]
    pub always_new_session: bool,
}
