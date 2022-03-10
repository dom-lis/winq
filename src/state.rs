use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub text: Vec<String>,
    pub fg: Vec<String>,
    pub bg: Vec<String>,
    pub style: Vec<String>,
}

impl Default for State {
    fn default() -> State {
        State {
            text: vec![],
            fg: vec![],
            bg: vec![],
            style: vec![],
        }
    }
}

