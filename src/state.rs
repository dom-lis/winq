use serde::{Serialize, Deserialize};

pub fn get_at(ss: &[String], x: usize, y: usize) -> String {
    ss.get(y).and_then(|s| s.chars().nth(x)).map(|c| c.to_string()).unwrap_or_else(|| " ".to_string())
}

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

