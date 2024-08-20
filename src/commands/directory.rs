use crate::{
    cli::directory::{DirectoryCli, DirectoryCommands, ListDirectoryArgs, StartDirectoryArgs},
    conditional_command,
    directories::{self, Directory},
    helpers::{absolute_path, dir_name, Exit},
    tmux::{attach, session_exists},
    widgets::{heading::Heading, table::Table},
};
use std::{collections::HashMap, path::PathBuf};
use tmux_interface::{NewSession, Tmux, TmuxCommand};

pub fn directory_handler(args: DirectoryCli) {
    match args.action {
        DirectoryCommands::List(args) => list_handler(args),
        DirectoryCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(args: ListDirectoryArgs) {
    let config = directories::parse_directory_config();
    let categories = config.categories;

    if args.minimal {
        println!("{}", format_categories_minimal(&categories));
        return;
    }

    for (key, value) in categories {
        println!("{}", Heading(key));
        println!("{}", Table::from_iter(value.iter()));
    }
}

fn format_categories_minimal(categories: &HashMap<String, Vec<Directory>>) -> String {
    let categories_formatted: Vec<_> = categories
        .iter()
        .map(move |(key, dirs)| {
            let dirs_formatted: Vec<_> = dirs.iter().map(|x| x.to_string()).collect();
            let dirs = dirs_formatted.join("\n");
            format!("{}\n{}", key, dirs)
        })
        .collect();

    categories_formatted.join("\n\n")
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
    let config = directories::parse_directory_config();
    let dirs: Vec<_> = config.categories.values().flatten().collect();
    let dir = dirs.iter().find(|&&d| d.get_name() == cli_args.directory);
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
