use clap::Parser;

#[derive(Parser)]
#[clap(name="tulip", about="terminal text-user interface proxy (or client)")]
pub struct Opts {
    #[clap(long, help="path to log file")]
    pub log: Option<String>,

    #[clap(short, long, help="use json for communication")]
    pub json: bool,

    #[clap(short, long, requires="height", help="initial window width")]
    pub width: Option<i32>,

    #[clap(short, long, requires="width", help="initial window height")]
    pub height: Option<i32>,

    #[clap(short, long, requires_all=&["width", "height"], help="center window on screen")]
    pub center: bool,

    #[clap(short, long, requires="y", conflicts_with="center", help="initial window horizontal position")]
    pub x: Option<i32>,

    #[clap(short, long, requires="x", conflicts_with="center", help="initial window vertical position")]
    pub y: Option<i32>,

    #[clap(short, long, help="window title")]
    pub title: Option<String>,
}

