use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListCli {
    #[arg(long = "format-project", short = 'p', alias = "fmt-proj")]
    pub format_project: Option<String>,

    #[arg(long = "format-template", short = 't', alias = "fmt-temp")]
    pub format_template: Option<String>,

    #[arg(long = "format-directory", short = 'd', alias = "fmt-dir")]
    pub format_directory: Option<String>,
}
