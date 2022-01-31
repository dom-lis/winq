use crate::transport::InComm;
use std::sync::mpsc::Receiver;
use crate::transport::OutComm;
use std::sync::mpsc::SyncSender;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;
use clap::Parser;
use crate::opts::Opts;

mod utils;
mod opts;
mod state;
mod event;
mod format;
mod frontend;
mod transport;

#[cfg(windows)]
fn open_transport(opts: &Opts) -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    if let Some(cmd) = &opts.cmd {
        let cmd = cmd.into();
        let cmd_args = opts.cmd_args.iter().map(|s| s.into()).collect::<Vec<_>>();
        transport::child::open(cmd, &cmd_args)
    } else if opts.inverse {
        transport::stdio::open()
    } else if let Some(addr) = &opts.tcp {
        transport::tcp::open(addr)
    } else {
        unreachable!()
    }
}

#[cfg(not(windows))]
fn open_transport(opts: &Opts) -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    if let Some(cmd) = &opts.cmd {
        let cmd = cmd.into();
        let cmd_args = opts.cmd_args.iter().map(|s| s.into()).collect::<Vec<_>>();
        transport::child::open(cmd, &cmd_args)
    } else if opts.inverse {
        transport::stdio::open()
    } else if let Some(addr) = &opts.tcp {
        transport::tcp::open(addr)
    } else if let Some(path) = &opts.socket {
        transport::socket::open(path)
    } else {
        unreachable!()
    }
}

#[cfg(windows)]
fn run_frontend(opts: &Opts, tx: SyncSender<OutComm>, rx: Receiver<InComm>) -> Result<(), Box<dyn Error + Send + Sync>> {
    frontend::fltk::run(tx, rx)
}

#[cfg(not(windows))]
fn run_frontend(opts: &Opts, tx: SyncSender<OutComm>, rx: Receiver<InComm>) -> Result<(), Box<dyn Error + Send + Sync>> {
    if opts.graphical {
        frontend::fltk::run(tx, rx)
    } else {
        frontend::tui::run(tx, rx)
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let opts = Opts::parse();
    
    if let Some(log) = &opts.log {
        use log::LevelFilter;
        use simplelog::{WriteLogger, Config};

        WriteLogger::init(LevelFilter::max(), Config::default(), File::create(log).unwrap()).unwrap()
    }
    
    let (r, w) = open_transport(&opts)?;
    
    let (tx, rx) = if opts.json {
        format::json::open(r, w)
    } else {
        format::simple::open(r, w)
    };
    
    run_frontend(&opts, tx, rx)?;

    Ok(())

}
