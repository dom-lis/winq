use serde::{Serialize, Deserialize};

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
pub enum Key {
    Button,
    Backspace,
    Tab,
    IsoKey,
    Enter,
    Pause,
    ScrollLock,
    Esc,
    Kana,
    Eisu,
    Yen,
    JISUnderscore,
    Home,
    Left,
    Up,
    Right,
    Down,
    PageUp,
    PageDown,
    End,
    Print,
    Insert,
    Menu,
    Help,
    NumLock,
    KP,
    KPEnter,
    KPLast,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    FLast,
    Shift,
    RShift,
    Ctrl,
    RCtrl,
    CapsLock,
    Meta,
    RMeta,
    Alt,
    RAlt,
    Delete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    MouseMove {
        x: i32,
        y: i32,
    },
    MouseDown {
        x: i32,
        y: i32,
        button: i32,
    },
    MouseDrag {
        x: i32,
        y: i32,
        button: i32,
    },
    MouseUp {
        x: i32,
        y: i32,
        button: i32,
    },
    MouseWheel {
        dx: i32,
        dy: i32,
    },
    KeyDown {
        code: Option<i32>,
        key: Option<Key>,
        text: Option<String>,
        mods: Mods,
    },
    KeyUp {
        code: Option<i32>,
        key: Option<Key>,
        text: Option<String>,
        mods: Mods,
    },
    Close,
    Deactivate,
    Activate,
    Hide,
    Show,
    Paste,
    SelectionClear,
    DndEnter,
    DndDrag,
    DndLeave,
    DndRelease,
    ScreenConfigChanged,
    Fullscreen,
    ZoomGesture,
    ZoomEvent,
    Resize,
    Enter,
    Leave,
    Focus,
    Unfocus,
}
