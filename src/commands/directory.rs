use crate::{
    cli::{DirectoryCli, DirectoryCommands},
    directories,
    widgets::{heading::Heading, table::fmt_table},
};

pub fn directory_handler(args: DirectoryCli) {
    match args.action {
        DirectoryCommands::List(ls_args) => {
            let config = directories::parse_directory_config().expect("Invalid directory config");
            let categories = config.categories;

            if ls_args.minimal {
                let categories_formatted: Vec<_> = categories
                    .iter()
                    .map(move |(key, dirs)| {
                        let dirs_formatted: Vec<_> = dirs
                            .iter()
                            .map(move |x| match &x.name {
                                Some(name) => format!("{} {}", x.path.display(), name),
                                None => format!("{}", x.path.display()),
                            })
                            .collect();

                        let dirs = dirs_formatted.join("\n");
                        format!("{}\n{}", key, dirs)
                    })
                    .collect();

                println!("{}", categories_formatted.join("\n\n"));
                return;
            }

            for (key, value) in categories {
                println!("{}", Heading(key));
                println!("{}", fmt_table(value));
            }
        }
        DirectoryCommands::Pick => {}
    }
}
