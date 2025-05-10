mod cli;
mod commands;
mod directories;
mod helpers;
mod init;
mod projects;
mod templates;
mod tmux;
mod widgets;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    match args.cmd {
        cli::Commands::Init => commands::init::init_handler(),
        cli::Commands::Directory { action } => commands::directory::directory_handler(action),
        cli::Commands::Template { action } => commands::template::template_handler(action),
        cli::Commands::Project { action } => commands::project::project_handler(action),
        cli::Commands::List(args) => commands::list::list_handler(args),
        cli::Commands::Freeze {
            name,
            force,
            file_name,
            stdout,
        } => commands::freeze::freeze_handler(name, force, file_name.as_deref(), stdout),
    }
}
