use clap::Parser;
use std::ffi::OsString;

#[derive(Copy, Clone)]
pub enum Protocol {
    Simple,
    Json,
}

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {
    #[clap(long, help="path to logfile")]
    pub log: Option<OsString>,
    #[clap(short, long, group="way", help="communicate via nng uri")]
    pub socket: Option<OsString>,
    #[clap(long, group="protocol", help="use json for communcation")]
    pub json: bool,
    #[clap(group="way", help="run this command as child proccess")]
    pub cmd: Option<OsString>,
    #[clap(requires="cmd", help="arguments for <CMD>")]
    pub cmd_args: Vec<OsString>,
    #[clap(short, long)]
    pub graphical: bool,
    #[clap(short, long, requires="graphical", conflicts_with_all=&["cmd", "cmd-args", "socket"], help="send user events to stdout and accept updates on stdin")]
    pub inverse: bool,
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
