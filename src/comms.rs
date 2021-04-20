use serde::{Serialize, Deserialize};
use tui::layout::{Rect};

#[derive(Debug, Clone, Serialize)]
pub enum OutComms {
    InputEvent(termion::event::Event),
    JsonError(String),
    TerminalSize(Rect),
}

#[derive(Debug, Clone, Deserialize)]
pub enum InComms {
    Render(UiRepr)
}

#[derive(Debug, Clone, Deserialize)]
pub enum UiRepr {

}