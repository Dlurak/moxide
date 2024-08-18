use crate::{
    apply_if,
    cli::{ListArgs, StartTemplateArgs, TemplateCli, TemplateCommands},
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

fn list_handler(args: ListArgs) {
    let templates = parse_template_config();

    for template in templates {
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
    let name = &template.name;

    if session_exists(name).unwrap_or(false) {
        apply_if!(!detached, Tmux::new(), add_command, attach(name))
            .output()
            .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let resolved_path = args.directory.and_then(|p| absolute_path(&p).ok());
    let (new_session_cmd, name) = resolve_cmd_name(&resolved_path, args.name, name.to_string());

    let initial_tmux = Tmux::new().add_command(new_session_cmd);
    let initial_tmux = apply_if!(!detached, initial_tmux, add_command, attach(&name));

    let tmux = apply_template(initial_tmux, &template, &resolved_path);

    tmux.output().exit(1, "Could not start Tmux-session");
}

fn resolve_cmd_name(
    path: &Option<PathBuf>,
    name: Option<String>,
    template_name: String,
) -> (TmuxCommand<'static>, String) {
    if let Some(p) = path {
        let session_name = name.unwrap_or_else(|| dir_name(p));
        return (
            NewSession::new()
                .detached()
                .session_name(session_name.clone())
                .start_directory(p.to_string_lossy().into_owned())
                .into(),
            session_name,
        );
    }

    let session_name = name.unwrap_or(template_name);
    (
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .into(),
        session_name,
    )
}
