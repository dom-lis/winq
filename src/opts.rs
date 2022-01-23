use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {
    #[clap(long, about="path to logfile")]
    pub log: Option<OsString>,
    #[clap(long, group="way", about="communicate via nng uri")]
    pub nng: Option<OsString>,
    #[clap(long, group="protocol", about="use json for communcation")]
    pub json: bool,
    #[clap(group="way", about="run this command as child proccess")]
    pub cmd: Option<OsString>,
    #[clap(requires="cmd", about="arguments for <CMD>")]
    pub cmd_args: Vec<OsString>,
}
