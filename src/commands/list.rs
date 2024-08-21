use crate::{cli::list::ListCli, directories, helpers::format_name, projects, templates};

pub fn list_handler(args: ListCli) {
    let projects = projects::parse_project_config();
    let templates = templates::parse_template_config();
    let dirs = directories::parse_directory_config();

    for i in projects {
        println!("{}", format_name(&args.format_project, &i.name));
    }
    for i in templates.into_iter().filter(|x| !x.hidden.unwrap_or(false)) {
        println!("{}", format_name(&args.format_template, &i.name));
    }

    for i in dirs {
        if let Some(name) = &i.name {
            println!("{}", format_name(&args.format_directory, name));
        }
    }
}
