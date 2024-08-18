mod cli;
mod commands;
mod directories;
mod helpers;
mod init;
mod macros;
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
    }
}
