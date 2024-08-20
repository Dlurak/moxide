use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct TemplateCli {
    #[command(subcommand)]
    pub action: TemplateCommands,
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    #[command(alias = "ls")]
    List(ListTemplateArgs),
    Start(StartTemplateArgs),
}

#[derive(Parser, Debug)]
pub struct ListTemplateArgs {
    /// Show minimal output for scripts
    #[arg(short, long, default_value_t = false)]
    pub minimal: bool,
    /// Show all templates including hidden ones
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct StartTemplateArgs {
    pub template_name: String,

    /// Start the session detached
    #[arg(short, long, default_value_t = false)]
    pub detached: bool,

    /// The directory to start it in
    #[arg(long, alias = "dir")]
    pub directory: Option<PathBuf>,

    /// Specify the name of the tmux session
    ///
    /// Optionally provide a name for the session. If not provided, it will be either the name from the configuration or from the directory
    #[arg(short, long)]
    pub name: Option<String>,

    /// Always start a new session instead of attaching to an existing session
    #[arg(long, default_value_t = false)]
    pub always_new_session: bool,
}
