use clap::Parser;

#[derive(Parser, Debug)]
pub struct InitCli {
    #[arg(long, alias = "gen-config", default_value_t = false)]
    pub config: bool,
    #[arg(
        long,
        alias = "gen-completion",
        alias = "gen-comp",
        alias = "gen-complete",
        default_value_t = false
    )]
    pub completion: bool,
}
