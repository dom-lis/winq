mod aux;
mod opts;
mod comms;
mod state;
mod event;
mod proto;
mod frontend;

use std::fs::File;
use std::error::Error;
use clap::Parser;

use crate::opts::Opts;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let opts = Opts::parse();
    let proto = opts.protocol();
    
    if let Some(log) = opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let (tx, rx) = {
        let cmd = opts.cmd;
        let cmd_args = opts.cmd_args;
        if let Some(cmd) = cmd {
            comms::child::open_comms(cmd, &cmd_args, proto)
        } else if opts.inverse {
            comms::stdio::open_comms(proto)
        } else if let Some(_socket) = opts.socket {
            unimplemented!()
        } else {
            unreachable!()
        }
    }?;
    
    if opts.graphical {
        frontend::fltk::run(tx, rx)?;
    } else {
        frontend::tui::run(tx, rx)?;
    }

    Ok(())

}
