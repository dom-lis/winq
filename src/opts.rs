use clap::Parser;

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {
    #[clap(long, help="path to logfile")]
    pub log: Option<String>,

    #[clap(short, long, help="use json for communcation")]
    pub json: bool,
}

