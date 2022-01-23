use std::io::{stdin, stdout};
use std::error::Error;
use std::thread;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;

use crate::stdio;
use crate::mode::Mode;
use crate::error::ChildError;
use crate::state::State;
use crate::server::Server;

pub fn client(server: &dyn Server) -> Result<(), Box<dyn Error>> {
    let host_stdin = stdio::host_read_stdin(stdin());

    let mut terminal = {
        let stdout = stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)?
    };

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