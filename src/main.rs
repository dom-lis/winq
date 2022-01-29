mod utils;
mod opts;
mod state;
mod event;
mod format;
mod frontend;
mod transport;

use std::fs::File;
use std::error::Error;
use clap::Parser;

use crate::opts::Opts;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let opts = Opts::parse();
    
    if let Some(log) = opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let (r, w) = {
        if let Some(cmd) = opts.cmd {
            let cmd = cmd.into();
            let cmd_args = opts.cmd_args.iter().map(|s| s.into()).collect::<Vec<_>>();
            transport::child::open(cmd, &cmd_args)
        } else if opts.inverse {
            transport::stdio::open()
        } else if let Some(path) = opts.socket {
            transport::socket::open(path)
        } else if let Some(addr) = opts.tcp {
            transport::tcp::open(addr)
        } else {
            unreachable!()
        }
    }?;
    
    let (tx, rx) = if opts.json {
        format::json::open(r, w)
    } else {
        format::simple::open(r, w)
    };
    
    if opts.graphical {
        frontend::fltk::run(tx, rx)?;
    } else {
        frontend::tui::run(tx, rx)?;
    }

    Ok(())

}
