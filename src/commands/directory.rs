use crate::{
    cli::{DirectoryCli, DirectoryCommands, ListArgs, StartDirectoryArgs},
    conditional_apply,
    directories::{self, Directory},
    helpers::{absolute_path, Exit},
    widgets::{heading::Heading, table::fmt_table},
};
use std::{collections::HashMap, path::PathBuf};
use tmux_interface::{NewSession, Tmux};

pub fn directory_handler(args: DirectoryCli) {
    match args.action {
        DirectoryCommands::List(args) => list_handler(args),
        DirectoryCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(args: ListArgs) {
    let config = directories::parse_directory_config();
    let categories = config.categories;

    if args.minimal {
        println!("{}", format_categories_minimal(&categories));
        return;
    }

    for (key, value) in categories {
        println!("{}", Heading(key));
        println!("{}", fmt_table(value));
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
    let name = args.name.unwrap_or(name);

    let new_session_cmd = NewSession::new()
        .start_directory(path.to_string_lossy())
        .session_name(&name)
        .window_name(&name);
    let new_session_cmd = conditional_apply!(new_session_cmd, args.detached, detached);

    Tmux::new()
        .add_command(new_session_cmd)
        .output()
        .exit(1, "Could not start Tmux-session");
}

fn resolve_dir_path(cli_args: &StartDirectoryArgs) -> (String, PathBuf) {
    let config = directories::parse_directory_config();
    let dirs: Vec<_> = config.categories.values().flatten().collect();
    let dir = dirs.iter().find(|&&d| d.get_name() == cli_args.directory);

    match dir {
        Some(dir) => (
            dir.get_name().to_string(),
            absolute_path(&dir.path).exit(1, "The path could not be generated"),
        ),
        None => {
            let relative_path = PathBuf::from(&cli_args.directory);
            let path = absolute_path(&relative_path).exit(1, "The path could not be generated");
            let name = path
                .file_name()
                .and_then(|os_string| os_string.to_str())
                .map(|str| str.to_string())
                .unwrap_or("".to_string());

            (name, path)
        }
    }
}
