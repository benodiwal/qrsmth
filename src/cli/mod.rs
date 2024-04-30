use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(required=true)]
    pub input: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}