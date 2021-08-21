use std::io;
use std::thread;
use std::io::{Stdin, Write};
use std::io::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::process::{ChildStdin, ChildStdout, ChildStderr};
use termion::event::Event;
use termion::input::TermRead;

pub fn child_read_stdout(stdout: ChildStdout) -> Receiver<Result<String, io::Error>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        use std::io::{BufReader};

        for line in BufReader::new(stdout).lines() {
            tx.send(line).unwrap();
        }
    });
    rx
}

pub fn child_read_stderr(stderr: ChildStderr) -> Receiver<io::Result<String>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        use std::io::{BufReader};

        for line in BufReader::new(stderr).lines() {
            tx.send(line).unwrap();
        }
    });
    rx
}

pub fn child_write_stdin(stdin: ChildStdin) -> Sender<String> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut stream = stdin;
        for s in rx {
            writeln!(stream, "{}", s).unwrap();
            stream.flush().unwrap();
        }
    });
    tx
}

pub fn host_read_stdin(stdin: Stdin) -> Receiver<io::Result<Event>> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        for i in stdin.events() {
            tx.send(i).unwrap();
        }
    });
    rx
}
