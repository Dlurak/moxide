use crate::{
    cli::{ListArgs, StartTemplateArgs, TemplateCli, TemplateCommands},
    helpers::Exit,
    templates::{parse_template_config, Template},
    tmux::{attach, session_exists},
    widgets::{heading::Heading, table::fmt_table},
};
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
        .iter()
        .find(|&temp| temp.name == args.name)
        .exit(1, "No template found");

    let name = &template.name;
    if session_exists(name).unwrap_or(false) {
        Tmux::new()
            .add_command(attach(name))
            .output()
            .exit(1, "Could not attach to the Tmux-session");
        return;
    }

    let initial_tmux = Tmux::new()
        .add_command(NewSession::new().detached().session_name(name))
        .add_command(attach(name));
    let tmux = apply_template(initial_tmux, template);

    tmux.output().exit(1, "Could not start Tmux-session");
}

fn apply_template<'a>(tmux: Tmux<'a>, template: &'a Template) -> Tmux<'a> {
    let enumerated = template.windows.iter().enumerate();
    enumerated.fold(tmux, |tmux, (window_idx, window)| {
        let cmd = match (window_idx, &window.name) {
            (0, Some(name)) => TmuxCommand::rename_window().new_name(name).into(),
            (0, None) => TmuxCommand::new(),
            (_, Some(name)) => TmuxCommand::new_window().window_name(name).into(),
            (_, None) => TmuxCommand::new_window().into(),
        };

        let tmux = tmux.add_command(cmd);
        add_panes_to_tmux(tmux, &window.panes)
    })
}

fn add_panes_to_tmux<'a>(tmux: Tmux<'a>, panes: &[String]) -> Tmux<'a> {
    let enumerated = panes.iter().enumerate();

    enumerated.fold(tmux, |tmux, (pane_idx, command)| {
        let tmux = if pane_idx > 0 {
            tmux.add_command(TmuxCommand::split_window())
        } else {
            tmux
        };
        tmux.add_command(TmuxCommand::send_keys().key(format!("{}\r", command)))
    })
}
