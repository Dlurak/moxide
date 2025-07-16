use crate::{
    cli::directory::{DirectoryCommands, StartDirectoryArgs},
    directories::{parse_directory_config, Directories},
    helpers::{absolute_path, dir_name, Exit, ExitErr},
    tmux::{attach, session_exists},
    widgets::table::Table,
};
use std::path::PathBuf;
use tmux_interface::{NewSession, Tmux};

pub fn directory_handler(action: DirectoryCommands) {
    match action {
        DirectoryCommands::List { minimal } => list_handler(minimal),
        DirectoryCommands::Start(args) => start_handler(&args),
    }
}

fn list_handler(minimal: bool) {
    let dirs = parse_directory_config().exit_err(1);

    if minimal {
        println!("{}", format_dirs_minimal(dirs));
        return;
    }

    let table: Table<_, _> = dirs.into();
    println!("{table}");
}

fn format_dirs_minimal(dirs: Directories) -> String {
    let dirs_formatted: Vec<_> = dirs
        .into_iter()
        .map(|(name, path)| format!("\"{}\" {}", name, path.display()))
        .collect();

    dirs_formatted.join("\n")
}

fn start_handler(args: &StartDirectoryArgs) {
    let (name, path) = resolve_dir_path(args);
    let exists = session_exists(&name).unwrap_or(false);

    let mut tmux = Tmux::new();
    if args.always_new_session || !exists {
        let cmd = NewSession::new()
            .start_directory(path.to_string_lossy())
            .detached()
            .session_name(&name)
            .window_name(&name);
        tmux = tmux.add_command(cmd);
    }
    if !args.detached {
        tmux = tmux.add_command(attach(&name));
    }

    tmux.output()
        .exit(1, "Could not switch to the Tmux session");
}

fn resolve_dir_path(cli_args: &StartDirectoryArgs) -> (String, PathBuf) {
    let name = &cli_args.directory;

    let dirs = parse_directory_config().exit_err(1);
    let dir = dirs.get(name);
    let user_name = cli_args.name.clone();

    match dir {
        Some(dir) => (
            user_name.unwrap_or_else(|| name.clone()),
            absolute_path(dir).exit(1, "The path could not be generated"),
        ),
        None => {
            let relative_path = PathBuf::from(&cli_args.directory);
            let path = absolute_path(&relative_path).exit(1, "The path could not be generated");
            let name = user_name.unwrap_or_else(|| dir_name(&path));

            (name, path)
        }
    }
}
