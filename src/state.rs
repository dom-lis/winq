use tui::style::{Color, Style, Modifier};
use crate::aux::parse_color;
use crate::mode::Mode;

pub struct State {
    text: Vec<String>,
    fg: Vec<String>,
    bg: Vec<String>,
    style: Vec<String>,
}

pub fn get_at(ss: &[String], x: usize, y: usize) -> String {
    ss.get(y).and_then(|s| s.chars().nth(x)).map(|c| c.to_string()).unwrap_or_else(|| " ".to_string())
}

impl State {
    pub fn push(&mut self, mode: Mode, line: String) {
        match mode {
            Mode::Text => {
                self.text.push(line);
            }
            Mode::Fg => {
                self.fg.push(line);
            }
            Mode::Bg => {
                self.bg.push(line);
            }
            Mode::Style => {
                self.style.push(line);
            }
        }
    }
    
    pub fn get_string(&self, x: usize, y: usize) -> String {
        get_at(&self.text, x, y)
    }
    
    pub fn get_fg(&self, x: usize, y: usize) -> Color {
        let fg = get_at(&self.fg, x, y);
        parse_color(&fg)
    }

    pub fn get_bg(&self, x: usize, y: usize) -> Color {
        let bg = get_at(&self.bg, x, y);
        parse_color(&bg)
    }
    
    pub fn get_mod(&self, x: usize, y: usize) -> Modifier {
        let m = get_at(&self.style, x ,y);
        match m.as_str() {
            "1" => Modifier::BOLD,
            "2" => Modifier::ITALIC,
            "3" => Modifier::BOLD | Modifier::ITALIC,
             _ => Modifier::empty(),
        }
    }

    pub fn get_style(&self, x: usize, y: usize) -> Style {
        Style::default()
            .fg(self.get_fg(x, y))
            .bg(self.get_bg(x, y))
            .add_modifier(self.get_mod(x, y))
    }
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
