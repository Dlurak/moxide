use crate::{cli::list::ListCli, directories, helpers::format_name, projects, templates};

pub fn list_handler(args: ListCli) {
    let projects = projects::parse_project_config();
    let templates = templates::parse_template_config();
    let dirs = directories::parse_directory_config();

    for project in projects {
        println!("{}", format_name(&args.format_project, &project.name));
    }

    for template in templates {
        let is_hidden = template.hidden.unwrap_or(false);
        if args.all || !is_hidden {
            println!("{}", format_name(&args.format_template, &template.name));
        }
    }

    for dir in dirs {
        if let Some(name) = dir.name {
            println!("{}", format_name(&args.format_directory, &name));
        }
    }
}
