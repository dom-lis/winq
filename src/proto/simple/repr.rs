use crate::comms::{OutComm};
use crate::event::{Event, Key, Mods};

pub fn repr_mods(mods: &Mods) -> String {
    let mut ms = vec![];
    if mods.shift {
        ms.push("Shift");
    }
    if mods.caps_lock {
        ms.push("CapsLock");
    }
    if mods.ctrl {
        ms.push("Ctrl");
    }
    if mods.alt {
        ms.push("Alt");
    }
    if mods.meta {
        ms.push("Meta");
    }
    if mods.command {
        ms.push("Command");
    }
    if mods.control {
        ms.push("Control");
    }
    ms.join("+")
}

pub fn repr_key(key: &Key) -> String {
    match key {
        Key::Button => "Button".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::IsoKey => "IsoKey".to_string(),
        Key::Enter => "Enter".to_string(),
        Key::Pause => "Pause".to_string(),
        Key::ScrollLock => "ScrollLock".to_string(),
        Key::Esc => "Esc".to_string(),
        Key::Kana => "Kana".to_string(),
        Key::Eisu => "Eisu".to_string(),
        Key::Yen => "Yen".to_string(),
        Key::JISUnderscore => "JISUnderscore".to_string(),
        Key::Home => "Home".to_string(),
        Key::Left => "Left".to_string(),
        Key::Up => "Up".to_string(),
        Key::Right => "Right".to_string(),
        Key::Down => "Down".to_string(),
        Key::PageUp => "PageUp".to_string(),
        Key::PageDown => "PageDown".to_string(),
        Key::End => "End".to_string(),
        Key::Print => "Print".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::Menu => "Menu".to_string(),
        Key::Help => "Help".to_string(),
        Key::NumLock => "NumLock".to_string(),
        Key::KP => "KP".to_string(),
        Key::KPEnter => "KPEnter".to_string(),
        Key::KPLast => "KPLast".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        Key::FLast => "FLast".to_string(),
        Key::Shift => "Shift".to_string(),
        Key::RShift => "RShift".to_string(),
        Key::Ctrl => "Ctrl".to_string(),
        Key::RCtrl => "RCtrl".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::Meta => "Meta".to_string(),
        Key::RMeta => "RMeta".to_string(),
        Key::Alt => "Alt".to_string(),
        Key::RAlt => "RAlt".to_string(),
        Key::Delete => "Delete".to_string(),
    }
}

pub fn repr_text(text: &str) -> String {
    text
        .replace("&", "&amp;")
        .replace(",", "&comma;")
}

pub fn repr_event(ev: &Event) -> Option<String> {
    use Event::*;

    match ev {
        MouseMove { x, y } => Some(format!("MouseMove:{},{}", x, y)),
        MouseDown { x, y, button } => Some(format!("MouseDown:{},{},{}", x, y, button)),
        MouseDrag { x, y, button } => Some(format!("MouseDrag:{},{},{}", x, y, button)),
        MouseUp { x, y, button } => Some(format!("MouseUp:{},{},{}", x, y, button)),
        MouseWheel { dx, dy } => Some(format!("MouseWheel:{},{}", dx, dy)),
        KeyDown { code, key, text, mods } => {
            let code = code.unwrap_or(0);
            let key = key.as_ref().map(|k| repr_key(k)).unwrap_or("".to_string());
            let text = text.as_ref().map(|s| repr_text(s)).unwrap_or("".to_string());
            let mods = repr_mods(mods);
            Some(format!("KeyDown:{},{},{},{}", code, key, text, mods))
        },
        KeyUp { code, key, text, mods } => {
            let code = code.unwrap_or(0);
            let key = key.as_ref().map(|k| repr_key(k)).unwrap_or("".to_string());
            let text = text.as_ref().map(|s| repr_text(s)).unwrap_or("".to_string());
            let mods = repr_mods(mods);
            Some(format!("KeyUp:{},{},{},{}", code, key, text, mods))
        },
        Close => Some("Close".to_string()),
        Deactivate => Some("Deactivate".to_string()),
        Activate => Some("Activate".to_string()),
        Hide => Some("Hide".to_string()),
        Show => Some("Show".to_string()),
        Paste => Some("Paste".to_string()),
        SelectionClear => Some("SelectionClear".to_string()),
        DndEnter => Some("DndEnter".to_string()),
        DndDrag => Some("DndDrag".to_string()),
        DndLeave => Some("DndLeave".to_string()),
        DndRelease => Some("DndRelease".to_string()),
        ScreenConfigChanged => Some("ScreenConfigChanged".to_string()),
        Fullscreen => Some("Fullscreen".to_string()),
        ZoomGesture => Some("ZoomGesture".to_string()),
        ZoomEvent => Some("ZoomEvent".to_string()),
        Resize => Some("Resize".to_string()),
        Enter => Some("Enter".to_string()),
        Leave => Some("Leave".to_string()),
        Focus => Some("Focus".to_string()),
        Unfocus => Some("Unfocus".to_string()),
    }
}

pub fn repr_comm(comm: &OutComm) -> Option<String> {
    use crate::comms::OutComm::*;

    match comm {
        Size((x, y)) => Some(format!("Size:{},{}", x, y)),
        Event(e) => repr_event(&e),
    }
}
