use clap::{Parser, Subcommand};

// TODO: Make a `cli`directory with multiple files

/// A CLI for tmux session management
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize the config
    ///
    /// This command will initialize your config directories.
    Init,
    /// Manage tmux sessions related to directories
    ///
    /// This command provides functionalities to interact with tmux sessions based on directories.
    #[command(alias = "dir", alias = "dirs", alias = "directories")]
    Directory(DirectoryCli),
}

#[derive(Parser, Debug)]
pub struct DirectoryCli {
    #[command(subcommand)]
    pub action: DirectoryCommands,
}

#[derive(Subcommand, Debug)]
pub enum DirectoryCommands {
    #[command(alias = "ls")]
    List(ListArgs),
    Start(StartDirectoryArgs),
}

#[derive(Parser, Debug)]
pub struct ListArgs {
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
}
