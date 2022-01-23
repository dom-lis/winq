use std::io::{Read, Write, BufRead, BufReader};
use std::error::Error;

use crate::comms::{InComm, OutComm, BadComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<InComm, E>;

#[derive(Copy, Clone)]
pub enum Mode {
    Text,
    Fg,
    Bg,
    Style,
}

pub fn read_stdout(stdout: Box<dyn Read>) -> Box<dyn Iterator<Item = R>> {
    use crate::state::State;
    let mut mode: Mode = Mode::Text;
    let mut state = State::default();
    let stdout = BufReader::new(stdout);
    let iter = stdout.lines()
        .filter_map(move |line| -> Option<R> {
            let line = line.ok()?;
            if line.chars().next() == Some('\t') {
                match line.as_str() {
                    "\tflush" => {
                        mode = Mode::Text;
                        let my_state = state.clone();
                        state = State::default();
                        Some(crate::comms::State(my_state))
                    },
                    "\ttext" => {
                        mode = Mode::Text;
                        None
                    },
                    "\tfg" => {
                        mode = Mode::Fg;
                        None
                    },
                    "\tbg" => {
                        mode = Mode::Bg;
                        None
                    }
                    "\tstyle" => {
                        mode = Mode::Style;
                        None
                    },
                    _ => Some(BadComm(format!("unknown mode: {:?}", line)))
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
            None
        });
    Box::new(iter)
}

pub fn write_stdin(stdin: &mut dyn Write, comm: OutComm) -> Result<(), E> {
    match comm {
        _ => unimplemented!()
    }
}
