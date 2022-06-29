use clap::Parser;

#[derive(Default, Parser)]
#[clap(name="winq", about="graphical text-user interface utility")]
pub struct Opts {
    #[clap(long, help="path to log file")]
    pub log: Option<String>,

    #[clap(short, long, help="dump default config to output and exit")]
    pub dump_conf: bool,

    #[clap(short, long, help="use json for communication")]
    pub json: bool,

    #[clap(short, long, requires="height", help="initial window width")]
    pub width: Option<i32>,

    #[clap(short, long, requires="width", help="initial window height")]
    pub height: Option<i32>,

    #[clap(short, long, requires="rows", help="initial window width in columns")]
    pub cols: Option<i32>,

    #[clap(short, long, requires="cols", help="initial window height in rows")]
    pub rows: Option<i32>,

    #[clap(short='C', long, requires_all=&["width", "height"], help="center window on screen")]
    pub center: bool,

    #[clap(short, long, requires="y", conflicts_with="center", help="initial window horizontal position")]
    pub x: Option<i32>,

    #[clap(short, long, requires="x", conflicts_with="center", help="initial window vertical position")]
    pub y: Option<i32>,

    #[clap(short, long, help="window title")]
    pub title: Option<String>,
}

