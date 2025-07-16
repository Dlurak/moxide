use crate::{
    cli::list::ListCli,
    directories,
    helpers::{format_name, ExitErr},
    projects, templates,
};

pub fn list_handler(args: ListCli) {
    let projects = projects::parse_project_config();
    let templates = templates::parse_template_config();
    let dirs = directories::parse_directory_config().exit_err(1);

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

    for (name, _) in dirs {
        println!("{}", format_name(args.format_directory.as_deref(), &name));
    }
}
