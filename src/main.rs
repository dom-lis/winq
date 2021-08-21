mod error;
mod input;
mod stdio;

use error::ChildError;

use std::error::Error;
use std::io;
use std::io::Write;
use std::ffi::OsString;
use std::thread;
use std::collections::HashMap;
use clap::Clap;
use regex::Regex;
use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use termion::screen::AlternateScreen;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clap)]
struct Opts {
    cmd: OsString,
    cmd_args: Vec<OsString>
}

fn main() -> Result<(), Box<dyn Error>> {

    let put_cmd_re = Regex::new(r"put:(\d+):(.*)").unwrap();
    let erase_cmd_re = Regex::new(r"erase:(\d+)").unwrap();

    let opts = Opts::parse();

    let mut buffers = [
        HashMap::<u16, String>::new(),
        HashMap::<u16, String>::new(),
    ];
    let mut buffers_current = 0;

    let mut stdout = {
        let s = std::io::stdout().into_raw_mode()?;
        let s = MouseTerminal::from(s);
        AlternateScreen::from(s)
    };

    let mut child = std::process::Command::new(opts.cmd)
        .args(opts.cmd_args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
        
    let host_stdin = stdio::host_read_stdin(std::io::stdin());
    let child_stdout = stdio::child_read_stdout(child.stdout.take().unwrap());
    let child_stderr = stdio::child_read_stderr(child.stderr.take().unwrap());
    let child_stdin = stdio::child_write_stdin(child.stdin.take().unwrap());

    writeln!(stdout, "{}", termion::clear::All)?;
    writeln!(stdout, "{}", termion::cursor::Hide)?;

    let mut terminal_size = (0, 0);

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
        let child_errs = child_stderr.try_iter().collect::<Result<Vec<_>, io::Error>>()?;
        if !child_errs.is_empty() {
            // gracefully fail if there were lines in child stderr
            return Err(Box::new(ChildError::Stderr(child_errs)));
        }
        
        // collect child comms from child stdout
        for s in child_stdout.try_iter() {
            match s {
                Ok(line) => {
                    match line.as_str() {
                        "reset" => {
                            buffers[buffers_current].clear();
                        },
                        "flish" => {
                            // todo: diff+flush
                            // writeln!(stdout, "{}", termion::cursor::Goto::default())?;
                            // stdout.flush()?;
                        },
                        _ => {
                            if let Some(cap) = put_cmd_re.captures_iter(&line).next() {
                                if let Some(line) = &cap[1].parse::<u16>().ok() {
                                    buffers[buffers_current].insert(*line, cap[2].to_owned());
                                }
                            } else if let Some(cap) = erase_cmd_re.captures_iter(&line).next() {
                                if let Some(line) = &cap[1].parse::<u16>().ok() {
                                    buffers[buffers_current].remove(line);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            }
        }
    }

}