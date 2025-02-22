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
        cli::Commands::Directory(args) => commands::directory::directory_handler(args),
        cli::Commands::Template(args) => commands::template::template_handler(args),
        cli::Commands::Project(args) => commands::project::project_handler(args),
        cli::Commands::List(args) => commands::list::list_handler(args),
        cli::Commands::Freeze {
            name,
            force,
            file_name,
            stdout,
        } => commands::freeze::freeze_handler(name, force, file_name, stdout),
    }
}
