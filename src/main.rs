mod cli;
mod commands;
mod directories;
mod helpers;
mod init;
mod macros;
mod widgets;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    match args.cmd {
        cli::Commands::Init => commands::init::init_handler(),
        cli::Commands::Directory(args) => commands::directory::directory_handler(args),
    }
}
