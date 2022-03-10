use serde::{Serialize, Deserialize};
use crate::state::State;
use crate::event::Event;

pub const CHAN_BOUND: usize = 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InComm {
    Quit(),
    BadComm(String),
    State(State),
}

pub use InComm::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutComm {
    Size((i32, i32)),
    Event(Event)
}

pub use OutComm::*;

use std::io;
use std::io::{Read, Write};

pub fn open() -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    Ok((
        Box::new(io::stdin()),
        Box::new(io::stdout())
    ))
}
