use crate::{
    cli::list::ListCli,
    directories,
    helpers::{format_name, ExitErr},
    projects, templates,
    tmux::session_exists,
};

pub fn list_handler(args: ListCli) {
    let mut projects = projects::parse_project_config();
    let mut templates = templates::parse_template_config();
    let dirs = directories::parse_directory_config().exit_err(1);
    let dirs = dirs
        .names()
        .filter(|name| !args.running || session_exists(*name).unwrap_or(false));

    if args.running {
        projects.retain(|project| session_exists(&project.name).unwrap_or(false));
        templates.retain(|template| session_exists(&template.name).unwrap_or(false));
    }

    for project in projects {
        println!(
            "{}",
            format_name(args.format_project.as_deref(), &project.name)
        );
    }

    for template in templates {
        let is_hidden = template.hidden.unwrap_or(false);
        if args.all || !is_hidden {
            println!(
                "{}",
                format_name(args.format_template.as_deref(), &template.name)
            );
        }
    }

    for name in dirs {
        println!("{}", format_name(args.format_directory.as_deref(), name));
    }
}
