use std::convert::TryFrom;
use std::io;
use std::sync::mpsc::{channel, SyncSender, Receiver};
use std::error::Error;
use std::thread;
use tui::Terminal;
use tui::backend::TermionBackend;
use crate::state::State;
use crate::comms;
use crate::comms::{InComm, OutComm};
use crate::event::{Event, Key, Mods};
use termion::event::{
    Key as TermionKey,
    Event as TermionEvent,
    MouseEvent as TermionMouseEvent,
    MouseButton as TermionMouseButton,
};

impl TryFrom<TermionKey> for Event {
    type Error = ();

    fn try_from(tk: TermionKey) -> Result<Event, Self::Error> {
        match tk {
            TermionKey::Backspace => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Backspace),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Left => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Left),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Right => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Right),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Up => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Up),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Down => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Down),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Home => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Home),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::End => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::End),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::PageUp => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::PageUp),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::PageDown => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::PageDown),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Delete => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Delete),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::Insert => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Insert),
                text: None,
                mods: Mods::default(),
            }),
            TermionKey::F(n) => Ok({
                let key = match n {
                    1 => Some(Key::F1),
                    2 => Some(Key::F2),
                    3 => Some(Key::F3),
                    4 => Some(Key::F4),
                    5 => Some(Key::F5),
                    6 => Some(Key::F6),
                    7 => Some(Key::F7),
                    8 => Some(Key::F8),
                    9 => Some(Key::F9),
                    10 => Some(Key::F10),
                    11 => Some(Key::F11),
                    12 => Some(Key::F12),
                    _ => None,
                };
                Event::KeyDown {
                    code: None,
                    key: key,
                    text: None,
                    mods: Mods::default(),
                }
            }),
            TermionKey::Char(c) => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Button),
                text: Some(c.to_string()),
                mods: Mods { shift: c.is_uppercase(), ..Mods::default() },
            }),
            TermionKey::Alt(c) => Ok(Event::KeyDown {
                code: None,
                key: None,
                text: Some(c.to_string()),
                mods: Mods { alt: true, ..Mods::default() }
            }),
            TermionKey::Ctrl(c) => Ok(Event::KeyDown {
                code: None,
                key: None,
                text: Some(c.to_string()),
                mods: Mods { ctrl: true, ..Mods::default() },
            }),
            TermionKey::Esc => Ok(Event::KeyDown {
                code: None,
                key: Some(Key::Esc),
                text: None,
                mods: Mods::default(),
            }),
            // TermionKey::BackTab => Ok(Event::KeyDown {
            //     code: None,
            //     key: Some(Key::BackTab),
            //     text: None,
            //     mods: Mods::default(),
            // }),
            _ => Err(()),
        }
    }
}

impl TryFrom<TermionMouseEvent> for Event {
    type Error = ();

    fn try_from(tme: TermionMouseEvent) -> Result<Event, Self::Error> {
        match tme {
            TermionMouseEvent::Press(m, x, y) => match m {
                TermionMouseButton::WheelDown => Ok(Event::MouseWheel { dx: 0, dy: 1 }),
                TermionMouseButton::WheelUp => Ok(Event::MouseWheel { dx: 0, dy: -1 }),
                _ => Ok(Event::MouseDown {
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
            TermionMouseEvent::Hold(x, y) => Ok(Event::MouseDrag {
                x: x as i32,
                y: y as i32,
                button: 0,
            }),
            TermionMouseEvent::Release(x, y) => Ok(Event::MouseUp {
                x: x as i32,
                y: y as i32,
                button: 0,
            }),
        }
    }
}

impl TryFrom<TermionEvent> for Event {
    type Error = ();

    fn try_from(te: TermionEvent) -> Result<Event, Self::Error> {
        match te {
            TermionEvent::Key(k) => Event::try_from(k),
            TermionEvent::Mouse(m) => Event::try_from(m),
            _ => Err(()),
        }
    }
}

pub fn run(tx: SyncSender<OutComm>, rx: Receiver<InComm>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut thread_handles = vec![];

    let mut terminal = {
        use termion::raw::IntoRawMode;
        use termion::input::MouseTerminal;
        use termion::screen::AlternateScreen;
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)
    }?;

    type E = Box<dyn Error + Send + Sync>;
    type R = Result<(), E>;
    
    let (state_tx, state_rx) = channel::<State>();

    thread_handles.push(thread::spawn::<_, R>({
        use std::time::Duration;
        use comms::Size;
        type E = Box<dyn Error + Send + Sync>;
        let tx = tx.clone();
        move || -> Result<(), E> {
            let sleep = Duration::from_millis(250);
            let mut terminal_size = (0, 0);
            thread::sleep(sleep); // TODO: wait for SIGWINCH
            let nts = termion::terminal_size();
            if let Ok(nts) = nts {
                if terminal_size != nts {
                    terminal_size = nts;
                    tx.send(Size(terminal_size))?;
                }
            }
            Ok(())
        }
    }));
    
    thread_handles.push(thread::spawn::<_, R>({
        use termion::input::TermRead;
        use comms::Event;
        type E = Box<dyn Error + Send + Sync>;
        type R = Result<(), E>;
        let tx = tx.clone();
        move || -> R {
            let stdin = std::io::stdin();
            let stdin = stdin.lock();
            for e in stdin .events() {
                let e = e?;
                let e = Event::try_from(e);
                if let Ok(e) = e {
                    tx.send(Event(e))?;
                }
            }
            Ok(())
        }
    }));
    
    thread_handles.push(thread::spawn::<_, R>({
        use comms::{Quit, BadComm, State};
        type E = Box<dyn Error + Send + Sync>;
        move || -> Result<(), E> {
            for inc in rx {
                match inc {
                    Quit() => { log::info!("quit"); Ok(()) }, // TODO
                    BadComm(e) => { log::warn!("bad comm {:?}", e); Ok(()) }, // TODO
                    State(s) => state_tx.send(s),
                }?;
            }
            Ok(())
        }
    }));
    
    thread_handles.push(thread::spawn::<_, R>({
        type E = Box<dyn Error + Send + Sync>;
        move || -> Result<(), E> {
            for s in state_rx {
                terminal.draw(|f| f.render_widget(s, f.size()))?;
            }
            Ok(())
        }
    }));
    
    for h in thread_handles {
        h.join().unwrap()?;
    }

    Ok(())

}
