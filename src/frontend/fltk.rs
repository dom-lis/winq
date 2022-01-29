use std::sync::{Arc, Mutex};
use std::sync::mpsc::{SyncSender, Receiver};
use std::error::Error;
use std::convert::TryFrom;
use fltk::prelude::*;
use fltk::{app, draw};
use fltk::app::App;
use fltk::window::Window;
use fltk::enums::{Font, FrameType, Color};

use crate::utils::parse_key;
use crate::event::{Event, Key, Mods};
use crate::comms::{InComm, OutComm};
use crate::state::State;

use fltk::enums::{Event as FltkEvent, Key as FltkKey, Shortcut as FltkMods};

impl From<FltkMods> for Mods {
    fn from(state: FltkMods) -> Mods {
        let mask = state.bits();
        Mods {
            shift: 0 != mask & FltkMods::Shift.bits(),
            caps_lock: 0 != mask & FltkMods::CapsLock.bits(),
            ctrl: 0 != mask & FltkMods::Ctrl.bits(),
            alt: 0 != mask & FltkMods::Alt.bits(),
            meta: 0 != mask & FltkMods::Meta.bits(),
            command: 0 != mask & FltkMods::Command.bits(),
            control: 0 != mask & FltkMods::Control.bits(),
        }
    }
}

impl TryFrom<FltkKey> for Key {
    type Error = ();
    
    fn try_from(fk: FltkKey) -> Result<Key, Self::Error> {
        match fk {
            FltkKey::BackSpace => Ok(Key::Backspace),
            FltkKey::Tab => Ok(Key::Tab),
            FltkKey::IsoKey => Ok(Key::IsoKey),
            FltkKey::Enter => Ok(Key::Enter),
            FltkKey::Pause => Ok(Key::Pause),
            FltkKey::ScrollLock => Ok(Key::ScrollLock),
            FltkKey::Escape => Ok(Key::Esc),
            FltkKey::Kana => Ok(Key::Kana),
            FltkKey::Eisu => Ok(Key::Eisu),
            FltkKey::Yen => Ok(Key::Yen),
            FltkKey::JISUnderscore => Ok(Key::JISUnderscore),
            FltkKey::Home => Ok(Key::Home),
            FltkKey::Left => Ok(Key::Left),
            FltkKey::Up => Ok(Key::Up),
            FltkKey::Right => Ok(Key::Right),
            FltkKey::Down => Ok(Key::Down),
            FltkKey::PageUp => Ok(Key::PageUp),
            FltkKey::PageDown => Ok(Key::PageDown),
            FltkKey::End => Ok(Key::End),
            FltkKey::Print => Ok(Key::Print),
            FltkKey::Insert => Ok(Key::Insert),
            FltkKey::Menu => Ok(Key::Menu),
            FltkKey::Help => Ok(Key::Help),
            FltkKey::NumLock => Ok(Key::NumLock),
            FltkKey::KP => Ok(Key::KP),
            FltkKey::KPEnter => Ok(Key::KPEnter),
            FltkKey::KPLast => Ok(Key::KPLast),
            FltkKey::F1 => Ok(Key::F1),
            FltkKey::F2 => Ok(Key::F2),
            FltkKey::F3 => Ok(Key::F3),
            FltkKey::F4 => Ok(Key::F4),
            FltkKey::F5 => Ok(Key::F5),
            FltkKey::F6 => Ok(Key::F6),
            FltkKey::F7 => Ok(Key::F7),
            FltkKey::F8 => Ok(Key::F8),
            FltkKey::F9 => Ok(Key::F9),
            FltkKey::F10 => Ok(Key::F10),
            FltkKey::F11 => Ok(Key::F11),
            FltkKey::F12 => Ok(Key::F12),
            FltkKey::FLast => Ok(Key::FLast),
            FltkKey::ShiftL => Ok(Key::Shift),
            FltkKey::ShiftR => Ok(Key::RShift),
            FltkKey::ControlL => Ok(Key::Ctrl),
            FltkKey::ControlR => Ok(Key::RCtrl),
            FltkKey::CapsLock => Ok(Key::CapsLock),
            FltkKey::MetaL => Ok(Key::Meta),
            FltkKey::MetaR => Ok(Key::RMeta),
            FltkKey::AltL => Ok(Key::Alt),
            FltkKey::AltR => Ok(Key::RAlt),
            FltkKey::Delete => Ok(Key::Delete),
            _ => Ok(Key::Button(parse_key(fk.bits() as u8))),
        }
    }
}

impl TryFrom<FltkEvent> for Event {
    type Error = ();

    fn try_from(fe: FltkEvent) -> Result<Event, Self::Error> {
        match fe {
            FltkEvent::Move => Ok({
                let x = app::event_x();
                let y = app::event_y();

                Event::MouseMove { x, y }
            }),

            FltkEvent::Push => Ok({
                let x = app::event_x();
                let y = app::event_y();
                let button = app::event_button();

                Event::MouseDown { x, y, button }
            }),

            FltkEvent::Released => Ok({
                let x = app::event_x();
                let y = app::event_y();
                let button = app::event_button();

                Event::MouseUp { x, y, button }
            }),

            FltkEvent::Drag => Ok({
                let x = app::event_x();
                let y = app::event_y();
                let button = app::event_button();

                Event::MouseDrag { x, y, button }
            }),

            FltkEvent::MouseWheel => Ok({
                let dx = match app::event_dx() {
                    app::MouseWheel::Right => 1,
                    app::MouseWheel::Left => -1,
                    _ => 0
                };
                let dy = match app::event_dy() {
                    app::MouseWheel::Up => 1,
                    app::MouseWheel::Down => -1,
                    _ => 0
                };

                Event::MouseWheel { dx, dy }
            }),

            FltkEvent::KeyDown => Ok({
                let fl_key = app::event_key();
                let key = Key::try_from(fl_key).ok();
                let code = Some(fl_key.bits());
                let text = Some(app::event_text());
                let mods = Mods::from(app::event_state());
                
                Event::KeyDown { key, code, text, mods }
            }),

            FltkEvent::KeyUp => Ok({
                let fl_key = app::event_key();
                let key = Key::try_from(fl_key).ok();
                let code = Some(fl_key.bits());
                let text = Some(app::event_text());
                let mods = Mods::from(app::event_state());
                
                Event::KeyUp { key, code, text, mods }
            }),

            FltkEvent::Close => Ok(Event::Close),
            FltkEvent::Deactivate => Ok(Event::Deactivate),
            FltkEvent::Activate => Ok(Event::Activate),
            FltkEvent::Hide => Ok(Event::Hide),
            FltkEvent::Show => Ok(Event::Show),
            FltkEvent::Paste => Ok(Event::Paste),
            FltkEvent::SelectionClear => Ok(Event::SelectionClear),
            FltkEvent::DndEnter => Ok(Event::DndEnter),
            FltkEvent::DndDrag => Ok(Event::DndDrag),
            FltkEvent::DndLeave => Ok(Event::DndLeave),
            FltkEvent::DndRelease => Ok(Event::DndRelease),
            FltkEvent::ScreenConfigChanged => Ok(Event::ScreenConfigChanged),
            FltkEvent::Fullscreen => Ok(Event::Fullscreen),
            FltkEvent::ZoomGesture => Ok(Event::ZoomGesture),
            FltkEvent::ZoomEvent => Ok(Event::ZoomEvent),
            FltkEvent::Enter => Ok(Event::Enter),
            FltkEvent::Leave => Ok(Event::Leave),
            FltkEvent::Focus => Ok(Event::Focus),
            FltkEvent::Unfocus => Ok(Event::Unfocus),

            _ => Err(())
        }
    }
}

pub fn run(tx: SyncSender<OutComm>, rx: Receiver<InComm>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let state: Arc<Mutex<State>> = Arc::new(Mutex::default());

    let app = App::default()
        .load_system_fonts();
    
    let font = Font::by_name("Fira Code Regular");
    let _font_bold = Font::by_name("Fira Code Bold");
    let _font_italic = Font::by_name("Fira Code Italic");
    let _font_bold_italic = Font::by_name("Fira Code Bold Italic");

    let mut win = Window::default();
    win.set_frame(FrameType::NoBox);
    win.draw({
        let tx = tx.clone();
        let state = state.clone();
        let mut size = (0, 0);
        move |w| {
            log::info!("draw");
            let new_size = (w.w(), w.h());
            if size != new_size {
                size = new_size;
                let (w, h) = size;
                tx.send(OutComm::Size((w as u16, h as u16))).unwrap();
            }
            draw::set_draw_color(Color::White);
            draw::draw_rectf(0, 0, w.width(), w.height());
            draw::set_font(font, 32);
            draw::set_draw_color(Color::Black);
            let y = w.y();
            let s = state.lock().unwrap();
            for (i, line) in s.text.iter().enumerate() {
                draw::draw_text(&line, 0, y + 32 * i as i32);
            }
        }
    });
    win.handle({
        let tx = tx.clone();
        move |_, ev| {
            Event::try_from(ev)
                .map(|ev| tx.send(OutComm::Event(ev)))
                .is_ok()
        }
    });
    win.end();
    win.show();
    
    app::add_idle3({
        let state = state.clone();
        move |_| {
            rx
                .try_iter()
                .try_for_each(|recv| -> Result<(), Box<dyn Error>> {
                    match recv {
                        InComm::Quit() => app::quit(),
                        InComm::BadComm(e) => log::warn!("bad comm: {:?}", e),
                        InComm::State(new_state) => {
                            let mut s = state.lock().unwrap();
                            *s = new_state;
                            app::redraw();
                        },
                    }
                    Ok(())
                })
                .unwrap()
        }
    });
    
    app.run()?;
    
    Ok(())
}
