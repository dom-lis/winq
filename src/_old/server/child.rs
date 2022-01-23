use std::io::prelude::*;
use std::ffi::OsString;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::process::Child;

use crate::mode::Mode;
use crate::state::State;
use crate::server::{Comms, Server};

pub struct ChildServer {
    mode: Mode,
    state: State,
    child: Child,
    stdin: Sender<String>,
    stdout: Receiver<Result<String, std::io::Error>>,
    stderr: Receiver<Result<String, std::io::Error>>,
}

impl ChildServer {
    pub fn new(cmd: OsString, args: Vec<OsString>) -> Result<Self, std::io::Error> {
        let mut child = std::process::Command::new(cmd)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let child_stdout = {
            // todo: handle error
            let stdout = child.stdout.take().unwrap();
            let (tx, rx) = channel();
            thread::spawn(move || {
                use std::io::{BufReader};

                for line in BufReader::new(stdout).lines() {
                    tx.send(line).unwrap();
                }
            });
            rx
        };
        
        let child_stderr = {
            // todo: handle error
            let stderr = child.stderr.take().unwrap();
            let (tx, rx) = channel();
            thread::spawn(move || {
                use std::io::{BufReader};

                for line in BufReader::new(stderr).lines() {
                    tx.send(line).unwrap();
                }
            });
            rx
        };
        
        let child_stdin = {
            // todo: handle error
            let stdin = child.stdin.take().unwrap();
            let (tx, rx) = channel::<String>();
            thread::spawn(move || {
                let mut stream = stdin;
                for s in rx {
                    writeln!(stream, "{}", s).unwrap();
                    stream.flush().unwrap();
                }
            });
            tx
        };

        Ok(ChildServer {
            mode: Mode::Text,
            state: State::default(),
            child: child,
            stdin: child_stdin,
            stdout: child_stdout,
            stderr: child_stderr,
        })
    }
}

impl Server for ChildServer {
    fn poll_comms(&mut self) -> Option<Comms> {
        if let Some(status) = self.child.try_wait().unwrap() {
            return Some(Comms::Exit);
        }
        
        let errors = self.stderr.try_iter().collect::<Result<Vec<_>, std::io::Error>>().unwrap();
        if !errors.is_empty() {
            return Some(Comms::Exit);
        }
        
        let mut mode = &mut self.mode;
        let mut state = &mut self.state;
        let mut flushed_state = None;
        
        for s in self.stdout.try_iter() {
            match s {
                Ok(line) => {
                    if line.chars().next() == Some('\t') {
                        match line.as_str() {
                            "\tflush" => {
                                *mode = Mode::Text;
                                flushed_state = Some(state);
                                *state = State::default();
                            },
                            "\ttext" => {
                                *mode = Mode::Text;
                            },
                            "\tfg" => {
                                *mode = Mode::Fg;
                            },
                            "\tbg" => {
                                *mode = Mode::Bg;
                            }
                            "\tstyle" => {
                                *mode = Mode::Style;
                            },
                            _ => {
                                return Err(Box::new(error::InternalError::BadMode(line)))
                            }
                        }
                    } else {
                        state.push(*mode, line);
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }

        return None;
    }
}
