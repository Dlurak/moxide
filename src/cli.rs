use clap::{Parser, Subcommand};

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
    /// Directory related stuff/// Directory related stuff
    #[command(alias = "dir", alias = "directories")]
    Directory(DirectoryCli),
}

#[derive(Parser, Debug)]
pub struct DirectoryCli {
        #[command(subcommand)]
        pub action: DirectoryCommands,
}

#[derive(Subcommand, Debug)]
pub enum DirectoryCommands {
    List(ListArgs),
    Pick
}

#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Show minimal output for scripts
    #[arg(short, long, default_value_t = false)]
    pub minimal: bool,
}
