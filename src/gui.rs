use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::error::Error;
use std::convert::TryFrom;
use fltk::prelude::*;
use fltk::{app, draw};
use fltk::app::App;
use fltk::window::Window;
use fltk::enums::{Event as FltkEvent};
use unicode_segmentation::UnicodeSegmentation;

use crate::opts::Opts;
use crate::event::Event;
use crate::msg::{GuiMsg, ClientMsg};
use crate::state::State;
use crate::config::Config;

macro_rules! join {
    ($a:expr, $b:expr) => {
        match ($a, $b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        }
    }
}

pub fn run(opts: &Opts, tx: Sender<ClientMsg>, rx: Receiver<GuiMsg>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let app = App::default()
        .load_system_fonts();
    
    let config: Arc<Mutex<Config>> = Arc::new(Mutex::from(Config::load().unwrap_or_default()));

    let state: Arc<Mutex<State>> = Arc::new(Mutex::default());
    
    let font_size = config.lock().unwrap().font_size;
    draw::set_font(config.lock().unwrap().font_styles.regular, font_size);
    let col_wf = draw::width("@");
    let col_wi = col_wf as i32;
    let row_hi = fltk::draw::height();
    let row_hf = row_hi as f64;

    let mut win = {
        let mut w = Window::default();

        let default_label = "winq".to_string();
        let label = &opts.title.as_ref().unwrap_or(&default_label);
        w = w.with_label(&label);

        let wh = join!(opts.width, opts.height);
        let cr = join!(opts.cols, opts.rows);
        w = match (wh, cr) {
            (Some((w1, h)), None) => w.with_size(w1, h),
            (None, Some((c, r))) => w.with_size((c as f64 * col_wf) as i32, (r as f64 * row_hf) as i32),
            _ => w,
        };

        w = match opts.center {
            true => w.center_screen(),
            false => match (opts.x, opts.y) {
                (Some(x), Some(y)) => w.with_pos(x, y),
                _ => w
            }
        };

        // w.set_frame(FrameType::NoBox);
        w
    };

    win.draw({
        let tx = tx.clone();
        let state = state.clone();
        let config = config.clone();
        let mut cols: i32 = 0;
        let mut rows: i32 = 0;
        move |w| {
            log::trace!("draw");
            let win_wi = w.w();
            let win_hi = w.h();
            let win_wf = win_wi as f64;
            let win_hf = win_hi as f64;
            let new_size_w = (win_wf / col_wf) as i32;
            let new_size_h = (win_hf / row_hf) as i32;
            if cols != new_size_w || rows != new_size_h {
                cols = new_size_w;
                rows = new_size_h;
                tx.send(ClientMsg::Size((cols, rows))).unwrap();
            }
            let state = state.lock().unwrap();
            let config = config.lock().unwrap();
            let line_shift = config.line_shift * (config.font_size as f64);
            let color_scheme = &config.color_scheme;
            let font_styles = &config.font_styles;
            draw::set_draw_color(color_scheme.background);
            draw::draw_rectf(0, 0, win_wi, win_hi);
            for i in 0..(rows as usize + 1) {
                if let Some(bg) = state.bg.get(i) {
                    let y = ((i as f64) * row_hf - line_shift) as i32;
                    let chars = bg.chars().take(cols as usize + 1);
                    for (j, bg) in chars.enumerate() {
                        let x = ((j as f64) * col_wf) as i32;
                        let digit = bg.to_digit(16);
                        let color = digit.and_then(|i| color_scheme.by_index(i as usize));
                        if let Some(color) = color {
                            draw::set_draw_color(*color);
                            draw::draw_rectf(x, y, col_wi, row_hi);
                        }
                    }
                }
            }
            for i in 0..(rows as usize + 1) {
                if let Some(line) = state.text.get(i) {
                    let style = state.style.get(i).map(|s| s.chars().collect::<Vec<_>>()).unwrap_or_default();
                    let fg = state.fg.get(i).map(|s| s.chars().collect::<Vec<_>>()).unwrap_or_default();
                    let chunks = {
                        let mut curr_style = &font_styles.regular;
                        let mut curr_fg = &color_scheme.foreground;
                        UnicodeSegmentation::graphemes(line.as_str(), true)
                            .enumerate()
                            .fold(Vec::default(), |mut chunks, (j, g)| {
                                let new_style = style.get(j)
                                    .and_then(|c| c.to_digit(16))
                                    .and_then(|i| font_styles.by_index(i as usize))
                                    .unwrap_or(&font_styles.regular);

                                let new_fg = fg.get(j)
                                    .and_then(|c| c.to_digit(16)
                                    .and_then(|i| color_scheme.by_index(i as usize)))
                                    .unwrap_or(&color_scheme.foreground);
                                
                                if chunks.is_empty() || curr_style != new_style || curr_fg != new_fg {
                                    curr_style = new_style;
                                    curr_fg = new_fg;
                                    chunks.push((j, curr_style, curr_fg, String::from(g)));
                                } else if let Some(last) = chunks.last_mut() {
                                    last.3 += g;
                                }

                                chunks
                            })
                    };
                    let y = (((i + 1) as f64) * row_hf - line_shift) as i32;
                    for (j, style, fg, text) in chunks {
                        let x = ((j as f64) * col_wf) as i32;
                        draw::set_draw_color(*fg);
                        draw::set_font(*style, font_size);
                        draw::draw_text(&text, x, y);
                    }
                }
            }
        }
    });

    win.handle({
        let tx = tx.clone();

        let mut last_mx = -1;
        let mut last_my = -1;

        move |_, ev| {
            let x = app::event_x() / col_wi;
            let y = app::event_y() / row_hi;
            let button = app::event_button();
            
            let new_xy = if last_mx != x || last_my != y {
                last_mx = x;
                last_my = y;
                Some((x, y))
            } else {
                None
            };
            
            let ev = match ev {
                FltkEvent::Move => new_xy.map(|(x, y)| Ok(Event::MouseMove { x, y })),
                FltkEvent::Push => Some(Ok(Event::MouseDown { x, y, button })),
                FltkEvent::Released => Some(Ok(Event::MouseUp { x, y, button })),
                FltkEvent::Drag => Some(Ok(Event::MouseDrag { x, y, button })),
                _ => Some(Event::try_from(ev)),
            };
            
            match ev {
                Some(ev) => match ev {
                    Ok(ev) => tx.send(ClientMsg::Event(ev)).is_ok(),
                    Err(_) => false,
                },
                None => true
            }
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
                        GuiMsg::Quit => app::quit(),
                        GuiMsg::BadComm(e) => log::warn!("bad comm: {:?}", e),
                        GuiMsg::State(new_state) => {
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

