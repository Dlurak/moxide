use crate::{
    apply_if,
    cli::template::{ListTemplateArgs, StartTemplateArgs, TemplateCli, TemplateCommands},
    helpers::{absolute_path, dir_name, Exit},
    templates::{apply_template, parse_template_config},
    tmux::{attach, session_exists},
    widgets::{heading::Heading, table::fmt_table},
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
            println!("{}", fmt_table(template.windows));
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

    if session_exists(&name).unwrap_or(false) && !args.always_new_session {
        apply_if!(!detached, Tmux::new(), add_command, attach(name))
            .output()
            .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let (new_session_cmd, name) = resolve_cmd_name(&resolved_path, args.name, name);

    let initial_tmux = apply_if!(
        !detached,
        Tmux::new().add_command(new_session_cmd),
        add_command,
        attach(&name)
    );

    let tmux = apply_template(initial_tmux, &template, &resolved_path);

    tmux.output().exit(1, "Could not start Tmux-session");
}

fn get_unused_name(name: String, used: Option<u8>) -> String {
    let new_name = match used {
        Some(counter) => format!("{}({})", name, counter),
        None => name.clone(),
    };

    if session_exists(&new_name).unwrap_or(false) {
        let next_counter = used.unwrap_or(0) + 1;
        get_unused_name(name, Some(next_counter))
    } else {
        new_name
    }
}

fn resolve_cmd_name(
    path: &Option<PathBuf>,
    name: Option<String>,
    template_name: String,
) -> (TmuxCommand<'static>, String) {
    if let Some(p) = path {
        let session_name = get_unused_name(name.unwrap_or_else(|| dir_name(p)), None);
        return (
            NewSession::new()
                .detached()
                .session_name(session_name.clone())
                .start_directory(p.to_string_lossy().into_owned())
                .into(),
            session_name,
        );
    }

    let session_name = get_unused_name(name.unwrap_or(template_name), None);
    (
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .into(),
        session_name,
    )
}
