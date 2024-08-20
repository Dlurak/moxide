use crate::{
    cli::project::{ProjectCli, ProjectCommands, ProjectListArgs},
    projects::parse_project_config,
    widgets::{heading::Heading, table::Table},
};

pub fn project_handler(args: ProjectCli) {
    match args.action {
        ProjectCommands::List(args) => list_handler(args),
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
