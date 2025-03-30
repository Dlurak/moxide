use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// List all projects
    #[command(alias = "ls")]
    List {
        /// Show minimal output for scripts
        #[arg(short, long, default_value_t = false)]
        minimal: bool,
    },
    /// Start a specific project
    Start(ProjectStartArgs),
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
