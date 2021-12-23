use termion::event::{Event, MouseEvent, Key, MouseButton};

pub fn repr_event(ev: &Event) -> Option<String> {
    use Event::*;

    match ev {
        Key(key) => repr_key(key).map(|s| format!("key:{}", s)),
        Mouse(mev) => repr_mouse_event(mev).map(|s| format!("mouse:{}", s)),
        _ => None
    }
}

pub fn repr_mouse_event(mev: &MouseEvent) -> Option<String> {
    use MouseEvent::*;

    match mev {
        Press(b, x, y) => repr_mouse_button(b).map(|s| format!("Press:{},{},{}", s, x, y)),
        Release(x, y) => Some(format!("Release:{},{}", x, y)),
        Hold(x, y) => Some(format!("Hold:{},{}", x, y)),
    }
}

pub fn repr_mouse_button(mb: &MouseButton) -> Option<String> {
    use MouseButton::*;

    match mb {
        Left => Some("Left".to_string()),
        Right => Some("Right".to_string()),
        Middle => Some("Middle".to_string()),
        WheelUp => Some("WheelUp".to_string()),
        WheelDown => Some("WheelDown".to_string()),
    }
}

pub fn repr_key(key: &Key) -> Option<String> {
    use Key::*;

    match key {
        Backspace => Some("Backspace".to_string()),
        Left => Some("Left".to_string()),
        Right => Some("Right".to_string()),
        Up => Some("Up".to_string()),
        Down => Some("Down".to_string()),
        Home => Some("Home".to_string()),
        End => Some("End".to_string()),
        PageUp => Some("PageUp".to_string()),
        PageDown => Some("PageDown".to_string()),
        BackTab => Some("BackTab".to_string()),
        Delete => Some("Delete".to_string()),
        Insert => Some("Insert".to_string()),
        F(n) => Some(format!("F{}", n)),
        Char(c) => Some(format!("Char+{}", c)),
        Alt(c) => Some(format!("Alt+{}", c)),
        Ctrl(c) => Some(format!("Ctrl+{}", c)),
        Null => Some("Null".to_string()),
        Esc => Some("Esc".to_string()),
        _ => None,
    }
}
