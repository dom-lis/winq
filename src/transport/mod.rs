pub mod tcp;
pub mod child;
pub mod stdio;
#[cfg(not(windows))]
pub mod socket;

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
    Size((u16, u16)),
    Event(Event)
}

pub use OutComm::*;
