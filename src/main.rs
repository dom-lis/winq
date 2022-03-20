use std::io;
use std::fs::File;
use std::error::Error;
use clap::Parser;
use crate::opts::Opts;

mod msg;
mod gui;
mod opts;
mod event;
mod state;
mod utils;
mod format;
mod config;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let opts = Opts::parse();
    
    if let Some(log) = &opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let r = Box::new(io::stdin());
    let w = Box::new(io::stdout());
    
    let (tx, rx) = if opts.json {
        format::json::open(r, w)
    } else {
        format::simple::open(r, w)
    };
    
    gui::run(&opts, tx, rx)?;

    Ok(())

}

