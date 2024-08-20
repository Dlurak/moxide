pub mod directory;
pub mod project;
pub mod template;

use self::{directory::DirectoryCli, project::ProjectCli, template::TemplateCli};
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
    /// Manage directories in the context of muxmate and tmux
    ///
    /// This command provides functionalities to interact with tmux sessions based on directories.
    #[command(alias = "dir", alias = "dirs", alias = "directories")]
    Directory(DirectoryCli),
    /// Manage templates in the context of muxmate and tmux
    ///
    /// This command provides functionalities to interact with tmux sessions based on templates
    #[command(alias = "temp", alias = "templ")]
    Template(TemplateCli),

    #[command(alias = "proj", alias = "projects")]
    Project(ProjectCli),
}
