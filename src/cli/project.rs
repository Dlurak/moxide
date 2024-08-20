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
}

#[derive(Debug, Parser)]
pub struct ProjectListArgs {
    /// Show minimal output for scripts
    #[arg(short, long, default_value_t = false)]
    pub minimal: bool,
}
