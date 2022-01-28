use clap::Parser;

#[derive(Copy, Clone)]
pub enum Protocol {
    Simple,
    Json,
}

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {

    #[clap(long, help="path to logfile")]
    pub log: Option<String>,

    #[clap(short, long, group="way", help="communicate via socket")]
    pub socket: Option<String>,
    
    #[clap(short, long, group="way", help="communicate via tcp-stream")]
    pub tcp: Option<String>,

    #[clap(long, group="protocol", help="use json for communcation")]
    pub json: bool,

    #[clap(group="way", help="run this command as child proccess")]
    pub cmd: Option<String>,

    #[clap(requires="cmd", help="arguments for <CMD>")]
    pub cmd_args: Vec<String>,

    #[clap(short, long)]
    pub graphical: bool,

    #[clap(short, long, requires="graphical", conflicts_with_all=&["way", "cmd-args"], help="send user events to stdout and accept updates on stdin")]
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
