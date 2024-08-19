use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct DirectoryCli {
    #[command(subcommand)]
    pub action: DirectoryCommands,
}

#[derive(Subcommand, Debug)]
pub enum DirectoryCommands {
    #[command(alias = "ls")]
    List(ListDirectoryArgs),
    Start(StartDirectoryArgs),
}

#[derive(Parser, Debug)]
pub struct ListDirectoryArgs {
    /// Show minimal output for scripts
    #[arg(short, long, default_value_t = false)]
    pub minimal: bool,
}

#[derive(Parser, Debug)]
pub struct StartDirectoryArgs {
    /// The directory to start the session in
    pub directory: String,

    /// Start the session detached
    #[arg(short, long, default_value_t = false)]
    pub detached: bool,

    /// Specify the name of the tmux session
    ///
    /// Optionally provide a name for the session. If not provided, it will be either the name from the configuration or from the directory
    #[arg(short, long)]
    pub name: Option<String>,

    /// Always start a new session instead of attaching to an existing session
    #[arg(long, default_value_t = false)]
    pub always_new_session: bool,
}
