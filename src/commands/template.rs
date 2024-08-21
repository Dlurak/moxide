use crate::{
    apply_if,
    cli::template::{ListTemplateArgs, StartTemplateArgs, TemplateCli, TemplateCommands},
    helpers::{absolute_path, dir_name, Exit},
    templates::{apply_windows, parse_template_config},
    tmux,
    widgets::{heading::Heading, table::Table},
};
use std::path::PathBuf;
use tmux_interface::{NewSession, Tmux, TmuxCommand};

pub fn template_handler(args: TemplateCli) {
    match args.action {
        TemplateCommands::List(args) => list_handler(args),
        TemplateCommands::Start(args) => start_handler(args),
    }
}

fn list_handler(args: ListTemplateArgs) {
    let templates = parse_template_config();
    let filtered = if args.all {
        templates
    } else {
        templates
            .into_iter()
            .filter(|t| !t.hidden.unwrap_or(false))
            .collect()
    };

    for template in filtered {
        if args.minimal {
            println!("{}", template.name);
        } else {
            println!("{}", Heading(template.name));
            println!("{}", Table::from_iter(template.windows.iter()));
        }
    }
}

fn start_handler(args: StartTemplateArgs) {
    let templates = parse_template_config();
    let template = templates
        .into_iter()
        .find(|temp| temp.name == args.template_name)
        .exit(1, "No template found");

    let detached = args.detached;
    let resolved_path = args.directory.and_then(|p| absolute_path(&p).ok());
    let name = resolved_path
        .as_ref()
        .map(|p| dir_name(p))
        .unwrap_or(template.name.clone());

    if tmux::session_exists(&name).unwrap_or(false) && !args.always_new_session {
        apply_if!(!detached, Tmux::new(), add_command, tmux::attach(name))
            .output()
            .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let (new_session_cmd, name) = resolve_cmd_name(&resolved_path, args.name, name);

    let initial_tmux = apply_if!(
        !detached,
        Tmux::new().add_command(new_session_cmd),
        add_command,
        tmux::attach(&name)
    );

    let tmux = apply_windows(initial_tmux, &template.windows, &resolved_path);

    tmux.output().exit(1, "Could not start Tmux-session");
}

fn resolve_cmd_name(
    path: &Option<PathBuf>,
    name: Option<String>,
    template_name: String,
) -> (TmuxCommand<'static>, String) {
    if let Some(p) = path {
        let session_name = tmux::get_unused_name(name.unwrap_or_else(|| dir_name(p)));
        return (
            NewSession::new()
                .detached()
                .session_name(session_name.clone())
                .start_directory(p.to_string_lossy().into_owned())
                .into(),
            session_name,
        );
    }

    let session_name = tmux::get_unused_name(name.unwrap_or(template_name));
    (
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .into(),
        session_name,
    )
}
