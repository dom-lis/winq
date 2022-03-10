use std::fs::File;
use std::error::Error;
use clap::Parser;
use crate::opts::Opts;

mod gui;
mod opts;
mod event;
mod state;
mod utils;
mod format;
mod config;
mod transport;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let opts = Opts::parse();
    
    if let Some(log) = &opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let (r, w) = transport::open()?;
    
    let (tx, rx) = if opts.json {
        format::json::open(r, w)
    } else {
        format::simple::open(r, w)
    };
    
    gui::run(tx, rx)?;

    Ok(())

}
