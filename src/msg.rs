use serde::{Serialize, Deserialize};
use crate::state::State;
use crate::event::Event;

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

