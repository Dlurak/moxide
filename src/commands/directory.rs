use crate::{
    cli::directory::{DirectoryCli, DirectoryCommands, ListDirectoryArgs, StartDirectoryArgs},
    conditional_command,
    directories::{self, Directory},
    helpers::{absolute_path, dir_name, Exit},
    tmux::{attach, session_exists},
    widgets::table::Table,
};
use std::path::PathBuf;
use tmux_interface::{NewSession, Tmux};

pub fn directory_handler(args: DirectoryCli) {
    match args.action {
        DirectoryCommands::List(args) => list_handler(args),
        DirectoryCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(args: ListDirectoryArgs) {
    let dirs = directories::parse_directory_config();

    if args.minimal {
        println!("{}", format_dirs_minimal(dirs));
        return;
    }

    let tables = dirs.into_iter().map(Table::from);
    let table: Table<_, _> = tables.collect();
    println!("{}", table);
}

fn format_dirs_minimal(dirs: Vec<Directory>) -> String {
    let dirs_formatted: Vec<_> = dirs
        .into_iter()
        .map(|dir| {
            let name = dir.name.unwrap_or("No name".to_string());
            format!("\"{}\" {}", name, dir.path.display())
        })
        .collect();

    dirs_formatted.join("\n")
}

fn start_handler(args: StartDirectoryArgs) {
    let (name, path) = resolve_dir_path(&args);
    let exists = session_exists(&name).unwrap_or(false);

    let new_session_cmd = conditional_command!(
        args.always_new_session || !exists,
        NewSession::new()
            .start_directory(path.to_string_lossy())
            .detached()
            .session_name(&name)
            .window_name(&name)
    );
    let attach_cmd = conditional_command!(!args.detached, attach(&name));

    Tmux::new()
        .add_command(new_session_cmd)
        .add_command(attach_cmd)
        .output()
        .exit(1, "Could not switch to the Tmux session");
}

fn resolve_dir_path(cli_args: &StartDirectoryArgs) -> (String, PathBuf) {
    let dirs = directories::parse_directory_config();
    let dir = dirs.iter().find(|d| d.get_name() == cli_args.directory);
    let user_name = cli_args.name.clone();

    match dir {
        Some(dir) => (
            user_name.unwrap_or(dir.get_name().to_string()),
            absolute_path(&dir.path).exit(1, "The path could not be generated"),
        ),
        None => {
            let relative_path = PathBuf::from(&cli_args.directory);
            let path = absolute_path(&relative_path).exit(1, "The path could not be generated");
            let name = user_name.unwrap_or(dir_name(&path));

            (name, path)
        }
    }
}
