use serde::{Serialize, Deserialize};
use crate::state::State;
use crate::event::Event;

pub const CHAN_BOUND: usize = 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuiMsg {
    Quit,
    BadComm(String),
    State(State),
}

pub use GuiMsg::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    Size((i32, i32)),
    Event(Event)
}

pub use ClientMsg::*;

