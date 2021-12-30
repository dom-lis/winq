mod aux;
mod opts;
mod error;
mod input;
mod stdio;
mod mode;
mod state;

use std::fs::File;
use std::io::{stdin, stdout};
use std::error::Error;
use std::thread;
use tui::Terminal;
use tui::backend::TermionBackend;
use clap::Parser;
use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use simplelog::{CombinedLogger, WriteLogger, LevelFilter};

use crate::mode::Mode;
use crate::opts::Opts;
use crate::error::ChildError;
use crate::state::State;

fn main() -> Result<(), Box<dyn Error>> {

    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, simplelog::Config::default(), File::create("log").unwrap()),
        ]
    ).unwrap();

    let opts = Opts::parse();

    let mut terminal = {
        let stdout = stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)?
    };

    let mut child = std::process::Command::new(opts.cmd)
        .args(opts.cmd_args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
        
    let host_stdin = stdio::host_read_stdin(stdin());
    let child_stdout = stdio::child_read_stdout(child.stdout.take().unwrap());
    let child_stderr = stdio::child_read_stderr(child.stderr.take().unwrap());
    let child_stdin = stdio::child_write_stdin(child.stdin.take().unwrap());

    let mut terminal_size = (0, 0);

    let mut mode: Mode = Mode::Text;
    let mut state: State = State::default();
    let mut flushed_state: Option<State> = None;

    loop {
        // throttling it so it won't busy-wait 100% of cpu
        thread::sleep(std::time::Duration::from_millis(20));

        let new_terminal_size = termion::terminal_size()?;
        if terminal_size != new_terminal_size {
            terminal_size = new_terminal_size;
            let (w, h) = terminal_size;
            child_stdin.send(format!("size:{},{}", w, h))?;
        }

        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            } else {
                return Err(Box::new(ChildError::Exit(status.code())));
            }
        }

        // collect terminal input (host_stdin)
        for hi in host_stdin.try_iter() {
            // relay terminal input to child (child_stdin)
            match hi {
                Ok(e) => if let Some(s) = input::repr_event(&e) {
                    child_stdin.send(s)?;
                },
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }

        // collect child stderr
        let child_errs = child_stderr.try_iter().collect::<Result<Vec<_>, std::io::Error>>()?;
        if !child_errs.is_empty() {
            // gracefully fail if there were lines in child stderr
            return Err(Box::new(ChildError::Stderr(child_errs)));
        }
        
        // collect child comms from child stdout
        for s in child_stdout.try_iter() {
            match s {
                Ok(line) => {
                    if line.chars().next() == Some('\t') {
                        match line.as_str() {
                            "\tflush" => {
                                mode = Mode::Text;
                                flushed_state = Some(state);
                                state = State::default();
                            },
                            "\ttext" => {
                                mode = Mode::Text;
                            },
                            "\tfg" => {
                                mode = Mode::Fg;
                            },
                            "\tbg" => {
                                mode = Mode::Bg;
                            }
                            "\tstyle" => {
                                mode = Mode::Style;
                            },
                            _ => {
                                return Err(Box::new(error::InternalError::BadMode(line)))
                            }
                        }
                    } else {
                        state.push(mode, line);
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }

        if let Some(draw_state) = flushed_state.take() {
            terminal.draw(|f| f.render_widget(draw_state, f.size()))?;
        }
    }

}
