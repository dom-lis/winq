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
        Left => Some("Left".to_owned()),
        Right => Some("Right".to_owned()),
        Middle => Some("Middle".to_owned()),
        WheelUp => Some("WheelUp".to_owned()),
        WheelDown => Some("WheelDown".to_owned()),
    }
}

pub fn repr_key(key: &Key) -> Option<String> {
    use Key::*;

    match key {
        Backspace => Some("Backspace".to_owned()),
        Left => Some("Left".to_owned()),
        Right => Some("Right".to_owned()),
        Up => Some("Up".to_owned()),
        Down => Some("Down".to_owned()),
        Home => Some("Home".to_owned()),
        End => Some("End".to_owned()),
        PageUp => Some("PageUp".to_owned()),
        PageDown => Some("PageDown".to_owned()),
        BackTab => Some("BackTab".to_owned()),
        Delete => Some("Delete".to_owned()),
        Insert => Some("Insert".to_owned()),
        F(n) => Some(format!("F{}", n)),
        Char(c) => Some(format!("Char+{}", c)),
        Alt(c) => Some(format!("Alt+{}", c)),
        Ctrl(c) => Some(format!("Ctrl+{}", c)),
        Null => Some("Null".to_owned()),
        Esc => Some("Esc".to_owned()),
        _ => None,
    }
}
