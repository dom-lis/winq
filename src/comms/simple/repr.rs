use crate::event::{Event, Mods};

pub fn repr_mods(mods: &Mods) -> String {
    let mut ms = vec![];
    if mods.shift {
        ms.push("shift");
    }
    if mods.caps_lock {
        ms.push("caps_lock");
    }
    if mods.ctrl {
        ms.push("ctrl");
    }
    if mods.alt {
        ms.push("alt");
    }
    if mods.meta {
        ms.push("meta");
    }
    if mods.command {
        ms.push("command");
    }
    if mods.control {
        ms.push("control");
    }
    ms.join("+")
}

pub fn repr_event(ev: &Event) -> Option<String> {
    use Event::*;

    match ev {
        KeyDown { code, key, text, mods } => {
            let code = code.unwrap_or(0);
            let key = key.unwrap_or("".to_string());
            let text = text.unwrap_or("".to_string());
            let mods = repr_mods(mods);
            format!("KeyDown:{},{},{},{}", code, key, text, mods)
        },
        MouseButtonDown { x, y, button } => format!("MouseButtonDown:{},{},{}", x, y, button),
        MouseButtonUp { x, y, button } => format!("MouseButtonUp:{},{},{}", x, y, button),
        MouseMove { x, y, button } => format!("MouseMove:{},{},{}", x, y, button),
        MouseWheel { dx, dy } => format!("MouseWheel:{},{}", dx, dy),
    }
}
