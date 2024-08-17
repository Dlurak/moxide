mod cli;
mod commands;
mod widgets;
mod directories;
mod helpers;
mod init;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    match args.cmd {
        cli::Commands::Init => commands::init::init_handler(),
        cli::Commands::Directory(args) => commands::directory::directory_handler(args),
    }
}
