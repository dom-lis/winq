use serde::{Serialize, Deserialize};
use termion::event::{
    Key as TermionKey,
    Event as TermionEvent,
    MouseEvent as TermionMouseEvent,
    MouseButton as TermionMouseButton,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mods {
    pub shift: bool,
    pub caps_lock: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub command: bool,
    pub control: bool,
}

impl Default for Mods {
    fn default() -> Mods {
        Mods {
            shift: false,
            caps_lock: false,
            ctrl: false,
            alt: false,
            meta: false,
            command: false,
            control: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    KeyEvent {
        code: Option<i32>,
        key: Option<String>,
        text: Option<String>,
        mods: Mods,
    },
    Push {
        x: i32,
        y: i32,
        button: i32,
    },
    Move {
        x: i32,
        y: i32,
        button: i32,
    },
    Release {
        x: i32,
        y: i32,
    },
    MouseWheel {
        dx: i32,
        dy: i32,
    },
}

impl Event {
    pub fn interp_termion_event(te: TermionEvent) -> Option<Event> {
        match te {
            TermionEvent::Key(k) => Event::interp_termion_key(k),
            TermionEvent::Mouse(m) => Event::interp_termion_mouse(m),
            _ => None,
        }
    }

    pub fn interp_termion_key(tk: TermionKey) -> Option<Event> {
        match tk {
            TermionKey::Backspace => Some(Event::KeyEvent {
                code: None,
                key: Some("Backspace".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Left => Some(Event::KeyEvent {
                code: None,
                key: Some("Left".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Right => Some(Event::KeyEvent {
                code: None,
                key: Some("Right".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Up => Some(Event::KeyEvent {
                code: None,
                key: Some("Up".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Down => Some(Event::KeyEvent {
                code: None,
                key: Some("Down".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Home => Some(Event::KeyEvent {
                code: None,
                key: Some("Home".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::End => Some(Event::KeyEvent {
                code: None,
                key: Some("End".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::PageUp => Some(Event::KeyEvent {
                code: None,
                key: Some("PageUp".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::PageDown => Some(Event::KeyEvent {
                code: None,
                key: Some("PageDown".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::BackTab => Some(Event::KeyEvent {
                code: None,
                key: Some("BackTab".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Delete => Some(Event::KeyEvent {
                code: None,
                key: Some("Delete".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Insert => Some(Event::KeyEvent {
                code: None,
                key: Some("Insert".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::F(n) => Some(Event::KeyEvent {
                code: None,
                key: Some(format!("F{}", n)),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Char(c) => Some(Event::KeyEvent {
                code: None,
                key: Some("Button".to_string()),
                text: Some(c.to_string()),
                mods: Mods { shift: c.is_uppercase(), ..Mods::default() },
            }),
            TermionKey::Alt(c) => Some(Event::KeyEvent {
                code: None,
                key: None,
                text: Some(c.to_string()),
                mods: Mods { alt: true, ..Mods::default() }
            }),
            TermionKey::Ctrl(c) => Some(Event::KeyEvent {
                code: None,
                key: None,
                text: Some(c.to_string()),
                mods: Mods { ctrl: true, ..Mods::default() },
            }),
            TermionKey::Esc => Some(Event::KeyEvent {
                code: None,
                key: Some("Esc".to_string()),
                text: None,
                mods: Mods::default(),
            }),
            _ => None,
        }
    }
    
    pub fn interp_termion_mouse(tme: TermionMouseEvent) -> Option<Event> {
        match tme {
            TermionMouseEvent::Press(m, x, y) => match m {
                TermionMouseButton::WheelDown => Some(Event::MouseWheel { dx: 0, dy: 1 }),
                TermionMouseButton::WheelUp => Some(Event::MouseWheel { dx: 0, dy: -1 }),
                _ => Some(Event::Push {
                    x: x as i32,
                    y: y as i32,
                    button: match m {
                        TermionMouseButton::Left => 1,
                        TermionMouseButton::Right => 2,
                        TermionMouseButton::Middle => 3,
                        _ => 0,
                    }
                }),
            },
            TermionMouseEvent::Hold(x, y) => Some(Event::Move {
                x: x as i32,
                y: y as i32,
                button: 0,
            }),
            TermionMouseEvent::Release(x, y) => Some(Event::Move {
                x: x as i32,
                y: y as i32,
                button: 0,
            }),
        }
    }
}
