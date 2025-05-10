use crate::{
    cli::template::{StartTemplateArgs, TemplateCommands},
    helpers::{absolute_path, apply_if_some, dir_name, Exit},
    templates::{apply_windows, find_template, parse_template_config},
    tmux,
    widgets::{heading::Heading, table::Table},
};
use std::path::PathBuf;
use tmux_interface::{NewSession, Tmux, TmuxCommand};

pub fn template_handler(action: TemplateCommands) {
    match action {
        TemplateCommands::List { minimal, all } => list_handler(minimal, all),
        TemplateCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(minimal: bool, all: bool) {
    let templates = parse_template_config();
    let filtered = if all {
        templates
    } else {
        templates
            .into_iter()
            .filter(|t| !t.hidden.unwrap_or(false))
            .collect()
    };

    for template in filtered {
        if minimal {
            println!("{}", template.name);
        } else {
            println!("{}", Heading(template.name));
            println!("{}", template.windows.iter().collect::<Table<_, _>>());
        }
    }
}

fn start_handler(args: StartTemplateArgs) {
    let template = find_template(&args.template_name).exit(1, "No template found");

    let detached = args.detached;
    let resolved_path = args.directory.and_then(|p| absolute_path(&p).ok());
    let name = resolved_path
        .as_ref()
        .map_or(template.name, |p| dir_name(p));

    if tmux::session_exists(&name).unwrap_or(false) && !args.always_new_session {
        apply_if_some(
            Tmux::new(),
            (!detached).then(|| tmux::attach(&name)),
            |tmux, cmd| tmux.add_command(cmd),
        )
        .output()
        .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let (new_session_cmd, name) = resolve_cmd_name(resolved_path.as_ref(), args.name, name);

    let initial_tmux = apply_if_some(
        Tmux::new().add_command(new_session_cmd),
        (!detached).then(|| tmux::attach(&name)),
        |tmux, cmd| tmux.add_command(cmd),
    );

    let tmux = apply_windows(initial_tmux, &template.windows, resolved_path.as_ref());

    tmux.output().exit(1, "Could not start Tmux-session");
}

fn resolve_cmd_name(
    path: Option<&PathBuf>,
    name: Option<String>,
    template_name: String,
) -> (TmuxCommand<'static>, String) {
    if let Some(p) = path {
        let session_name = tmux::get_unused_name(&name.unwrap_or_else(|| dir_name(p)));
        return (
            NewSession::new()
                .detached()
                .session_name(session_name.clone())
                .start_directory(p.to_string_lossy().into_owned())
                .into(),
            session_name,
        );
    }

    let session_name = tmux::get_unused_name(&name.unwrap_or(template_name));
    (
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .into(),
        session_name,
    )
}
