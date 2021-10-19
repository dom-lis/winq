mod aux;
mod opts;
mod error;
mod input;
mod stdio;

use tui::style::Style;
use std::fs::File;
use std::io::{stdin, stdout};
use std::error::Error;
use std::thread;
use aux::parse_color;
use tui::Terminal;
use tui::backend::TermionBackend;
use clap::Parser;
use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use simplelog::{CombinedLogger, WriteLogger, LevelFilter};

use crate::opts::Opts;
use crate::error::ChildError;

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
    let mut labels: Vec<(u16, u16, String, Style)> = Vec::new();
    let mut flushed_labels: Vec<(u16, u16, String, Style)>;
    let mut flush: bool = false;

    loop {
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
                    let tokens = line.split("\t").collect::<Vec<_>>();
                    match tokens.get(0) {
                        Some(&"flush") => {
                            flush = true;
                        }
                        Some(&"print") => {
                            let x = tokens.get(1).map(|s| s.parse::<u16>());
                            let y = tokens.get(2).map(|s| s.parse::<u16>());
                            let text = tokens.get(3);
                            let color = tokens.get(4).map(|s| parse_color(s));
                            if let (Some(Ok(x)), Some(Ok(y)), Some(text), Some(Ok(color))) = (x, y, text, color) {
                                labels.push((x, y, text.to_string(), Style::default().fg(color)));
                            }
                        }
                        Some(_) => {}
                        None => {}
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }

        if flush {
            flush = false;
            flushed_labels = labels;
            labels = Vec::new();

            let b = terminal.current_buffer_mut();

            for (x, y, string, style) in &flushed_labels {
                b.set_string(*x, *y, string, *style);
            }

            // ugly hack in order to avoid dealing with french lifetimes
            terminal.draw(|_f| {})?;
        }
    }

}