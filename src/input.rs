use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Event {
    /// A key press.
    Key(Key),
    /// A mouse button press, release or wheel use at specific coordinates.
    Mouse(MouseEvent),
    /// An event that cannot currently be evaluated.
    Unsupported(Vec<u8>),
}

impl Event {
    pub fn from_termion_event(termion_event: termion::event::Event) -> Option<Event> {
        use termion::event::Event as TEvent;
        
        match termion_event {
            TEvent::Key(key) => { Key::from_termion_key(key).map(|k| Event::Key(k)) },
            TEvent::Mouse(tme) => { Some(Event::Mouse(MouseEvent::from_termion_mouse_event(tme))) },
            _ => None
        }
    }
}

/// A mouse related event.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseEvent {
    /// A mouse button was pressed.
    ///
    /// The coordinates are one-based.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    ///
    /// The coordinates are one-based.
    Release(u16, u16),
    /// A mouse button is held over the given coordinates.
    ///
    /// The coordinates are one-based.
    Hold(u16, u16),
}

impl MouseEvent {
    pub fn from_termion_mouse_event(tme: termion::event::MouseEvent) -> MouseEvent {
        use MouseEvent::*;
        use termion::event::MouseEvent as TMouseEvent;

        match tme {
            TMouseEvent::Press(b, x, y) => Press(MouseButton::from_termion_mouse_button(b), x, y),
            TMouseEvent::Release(x, y) => Release(x, y),
            TMouseEvent::Hold(x, y) => Hold(x, y),
        }
    } 
}

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,
    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
}

impl MouseButton {
    pub fn from_termion_mouse_button(tmb: termion::event::MouseButton) -> MouseButton {
        use MouseButton::*;
        use termion::event::MouseButton as TMouseButton;

        match tmb {
            TMouseButton::Left => Left,
            TMouseButton::Right => Right,
            TMouseButton::Middle => Middle,
            TMouseButton::WheelUp => WheelUp,
            TMouseButton::WheelDown => WheelDown,
        }
    }
}

/// A key.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
}

impl Key {
    pub fn from_termion_key(termion_key: termion::event::Key) -> Option<Key> {
        use Key::*;
        use termion::event::{Key as TKey};

        match termion_key {
            TKey::Backspace => Some(Backspace),
            TKey::Left => Some(Left),
            TKey::Right => Some(Right),
            TKey::Up => Some(Up),
            TKey::Down => Some(Down),
            TKey::Home => Some(Home),
            TKey::End => Some(End),
            TKey::PageUp => Some(PageUp),
            TKey::PageDown => Some(PageDown),
            TKey::BackTab => Some(BackTab),
            TKey::Delete => Some(Delete),
            TKey::Insert => Some(Insert),
            TKey::F(n) => Some(F(n)),
            TKey::Char(c) => Some(Char(c)),
            TKey::Alt(c) => Some(Alt(c)),
            TKey::Ctrl(c) => Some(Ctrl(c)),
            TKey::Null => Some(Null),
            TKey::Esc => Some(Esc),
            _ => None,
        }
    }
}