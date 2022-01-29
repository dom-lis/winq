use clap::Parser;

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {

    #[clap(long, help="path to logfile")]
    pub log: Option<String>,

    #[clap(short, long, group="transport", help="communicate via socket")]
    pub socket: Option<String>,
    
    #[clap(short, long, group="transport", help="communicate via tcp-stream")]
    pub tcp: Option<String>,

    #[clap(short, long, group="format", help="use json for communcation")]
    pub json: bool,

    #[clap(group="transport", help="run this command as child proccess")]
    pub cmd: Option<String>,

    #[clap(requires="cmd", help="arguments for <CMD>")]
    pub cmd_args: Vec<String>,

    #[clap(short, long)]
    pub graphical: bool,

    #[clap(short, long, requires="graphical", conflicts_with_all=&["transport", "cmd-args"], help="send user events to stdout and accept updates on stdin")]
    pub inverse: bool,
}
