use std::io::{Read, Write, BufRead, BufReader};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::error::Error;
use std::thread;
use crate::msg;
use crate::msg::{GuiMsg, ClientMsg, BadComm};
use crate::state::State;

mod repr;

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;

#[derive(Copy, Clone)]
pub enum Mode {
    Text,
    Fg,
    Bg,
    Style,
}

pub fn open(r: Box<dyn Read + Send>, w: Box<dyn Write + Send>) -> (Sender<ClientMsg>, Receiver<GuiMsg>) {
    let (in_tx, in_rx) = channel();
    thread::spawn({
        let r = r;
        move || -> R {
            let reader = BufReader::new(r);
            let mut mode: Mode = Mode::Text;
            let mut state = State::default();
            for line in reader.lines() {
                let line = line?;
                if line.chars().next() == Some('\t') {
                    match line.as_str() {
                        "\tflush" => {
                            mode = Mode::Text;
                            let my_state = state.clone();
                            state = State::default();
                            in_tx.send(msg::State(my_state))?;
                        }
                        "\ttext" => {
                            mode = Mode::Text;
                        }
                        "\tfg" => {
                            mode = Mode::Fg;
                        }
                        "\tbg" => {
                            mode = Mode::Bg;
                        }
                        "\tstyle" => {
                            mode = Mode::Style;
                        }
                        _ => {
                            in_tx.send(BadComm(format!("unknown mode: {:?}", line)))?;
                        }
                    };
                } else {
                    let buff = match mode {
                        Mode::Text => &mut state.text,
                        Mode::Fg => &mut state.fg,
                        Mode::Bg => &mut state.bg,
                        Mode::Style => &mut state.style,
                    };
                    buff.push(line);
                }
            }
            Ok(())
        }
    });

    let (out_tx, out_rx) = channel();
    thread::spawn({
        let mut w = w;
        move || -> R {
            for c in out_rx {
                if let Some(line) = repr::repr_comm(&c) {
                    writeln!(&mut w, "{}", line)?;
                }
            }
            Ok(())
        }
    });

    (out_tx, in_rx)
}

