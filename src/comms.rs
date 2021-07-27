use serde::{Serialize, Deserialize};
use crate::input;

#[derive(Debug, Clone, Serialize)]
pub enum OutComms {
    InputEvent(input::Event),
    JsonError(String),
    TerminalSize((u16, u16)),
}

#[derive(Debug, Clone, Deserialize)]
pub enum InComms {
    Draw(String)
}
