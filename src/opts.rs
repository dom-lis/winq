use clap::Parser;
use std::ffi::OsString;

#[derive(Copy, Clone)]
pub enum Protocol {
    Simple,
    Json,
}

#[derive(Parser)]
#[clap(name="tulip", help="terminal text-user interface proxy (or client)")]
pub struct Opts {
    #[clap(long, help="path to logfile")]
    pub log: Option<OsString>,
    #[clap(long, group="way", help="communicate via nng uri")]
    pub nng: Option<OsString>,
    #[clap(long, group="protocol", help="use json for communcation")]
    pub json: bool,
    #[clap(group="way", help="run this command as child proccess")]
    pub cmd: Option<OsString>,
    #[clap(requires="cmd", help="arguments for <CMD>")]
    pub cmd_args: Vec<OsString>,
}

impl Opts {
    pub fn protocol(&self) -> Protocol {
        if self.json {
            Protocol::Json
        } else {
            Protocol::Simple
        }
    }
}
