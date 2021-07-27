mod comms;
mod err;
mod input;

use std::error::Error;
use std::io;
use std::io::{Stdin, Write};
use std::ffi::OsString;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::process::{ChildStdin, ChildStdout, ChildStderr};
use clap::Clap;
use termion::raw::IntoRawMode;
use termion::input::{TermRead, MouseTerminal};
use termion::event::Event as TermionEvent;
use termion::screen::AlternateScreen;
use crate::comms::{InComms, OutComms};
use crate::err::ChildError;


#[derive(Clap)]
struct Opts {
    cmd: OsString,
    cmd_args: Vec<OsString>
}

fn child_read_stdout(stdout: ChildStdout) -> Receiver<serde_json::Result<InComms>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let stream = stdout;
        let br = BufReader::new(stream);
        for line in br.lines() {
            if let Ok(line1) = line {
                tx.send(serde_json::Result::Ok(InComms::Draw(line1))).unwrap();
            }
        }
    });
    rx
}

fn child_read_stderr(stderr: ChildStderr) -> Receiver<io::Result<String>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let stream = stderr;
        let br = BufReader::new(stream);
        for line in br.lines() {
            tx.send(line).unwrap();
        }
    });
    rx
}

fn child_write_stdin(stdin: ChildStdin) -> Sender<OutComms> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut stream = stdin;
        for c in rx {
            if let Ok(s) = serde_json::to_string(&c) {
                writeln!(stream, "{}", s).unwrap();
            }
        }
    });
    tx
}

fn host_read_stdin(stdin: Stdin) -> Receiver<io::Result<TermionEvent>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        for i in stdin.events() {
            tx.send(i).unwrap();
        }
    });
    rx
}

fn main() -> Result<(), Box<dyn Error>> {

    let opts = Opts::parse();

    let _stderr = std::io::stderr();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let mut stdout = AlternateScreen::from(stdout);

    let mut child = std::process::Command::new(opts.cmd)
        .args(opts.cmd_args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
        
    let host_stdin = host_read_stdin(stdin);
    let child_stdout = child_read_stdout(child.stdout.take().unwrap());
    let child_stderr = child_read_stderr(child.stderr.take().unwrap());
    let child_stdin = child_write_stdin(child.stdin.take().unwrap());

    writeln!(stdout, "{}", termion::clear::All)?;
    writeln!(stdout, "{}", termion::cursor::Hide)?;

    let mut terminal_size = termion::terminal_size()?;

    loop {
        thread::sleep(std::time::Duration::from_millis(20));

        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            } else {
                return Err(Box::new(ChildError::Exit(status.code())));
            }
        }

        let new_terminal_size = termion::terminal_size()?;
        if terminal_size != new_terminal_size {
            terminal_size = new_terminal_size;
            child_stdin.send(OutComms::TerminalSize(terminal_size))?;
        }

        // collect terminal input (host_stdin)
        for hi in host_stdin.try_iter() {
            // relay terminal input to child (child_stdin)
            match hi.map(|te| input::Event::from_termion_event(te)) {
                Ok(maybe_ev) => {
                    if let Some(ev) = maybe_ev {
                        child_stdin.send(OutComms::InputEvent(ev))?;
                    }
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
        for co in child_stdout.try_iter() {
            match co {
                Err(serde_err) => {
                    child_stdin.send(OutComms::JsonError(format!("{}", serde_err)))?;
                },
                Ok(comm) => match comm {
                    InComms::Draw(s) => {
                        writeln!(stdout, "{}", s)?;
                        writeln!(stdout, "{}", termion::cursor::Goto::default())?;
                    }
                }
            }
        }
    }

}