use crate::{
    cli::project::{ProjectCli, ProjectCommands, ProjectListArgs, ProjectStartArgs},
    helpers::{self, apply_if_some, Exit},
    projects::parse_project_config,
    templates::apply_windows,
    tmux,
    widgets::{heading::Heading, table::Table},
};
use tmux_interface::{NewSession, Tmux};

pub fn project_handler(args: ProjectCli) {
    match args.action {
        ProjectCommands::List(args) => list_handler(args),
        ProjectCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(args: ProjectListArgs) {
    for proj in parse_project_config() {
        if args.minimal {
            println!("{}", proj.name);
        } else {
            println!("{}", Heading(proj.name));
            println!("{}", Table::from(proj.setup));
        }
    }
}

fn start_handler(args: ProjectStartArgs) {
    let projects = parse_project_config();
    let project = projects
        .into_iter()
        .find(|proj| proj.name == args.name)
        .exit(1, "Project could not be found");

    let detached = args.detached;

    if tmux::session_exists(&project.name).unwrap_or(false) && !args.always_new_session {
        apply_if_some(
            Tmux::new(),
            (!detached).then(|| tmux::attach(&project.name)),
            |tmux, cmd| tmux.add_command(cmd),
        )
        .output()
        .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let name = tmux::get_unused_name(project.name);
    let windows = Vec::from(project.setup);
    let path = helpers::absolute_path(&project.root_dir).exit(1, "The path could not be found");

    let new_session_cmd = NewSession::new()
        .detached()
        .session_name(&name)
        .start_directory(path.to_string_lossy().into_owned());

    let initial_tmux = apply_if_some(
        Tmux::new().add_command(new_session_cmd),
        (!detached).then(|| tmux::attach(&name)),
        |tmux, cmd| tmux.add_command(cmd),
    );

    apply_windows(initial_tmux, &windows, &Some(path))
        .output()
        .exit(1, "Could not start Tmux-session");
}
