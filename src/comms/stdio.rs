use std::io;
use std::ffi::OsString;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;

use crate::opts::Protocol;
use crate::opts::Protocol::*;
use crate::comms::{CHAN_BOUND, InComm, OutComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;
type Rc = Result<(SyncSender<OutComm>, Receiver<InComm>, Vec<JoinHandle<R>>), E>;

pub fn open_comms(cmd: OsString, args: &[OsString], proto: Protocol) -> Rc {
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut handles = vec![];

    let (in_tx, in_rx) = sync_channel(CHAN_BOUND);
    handles.push(thread::spawn({
        let stdout_err = io::Error::new(io::ErrorKind::Other, "can't take child stdout");
        let stdout = child.stdout.take().ok_or(stdout_err)?;
        let stdout = Box::new(stdout);
        move || -> R {
            let seq = match proto {
                Simple => crate::comms::simple::read_stdout(stdout),
                Json => crate::comms::json::read_stdout(stdout),
            };

            for comm in seq {
                let comm = comm?;
                in_tx.send(comm)?;
            }
            // TODO: handle child status etc
            Ok(())
        }
    }));
    
    let (out_tx, out_rx) = sync_channel(CHAN_BOUND);
    handles.push(thread::spawn({
        let stdin_err = io::Error::new(io::ErrorKind::Other, "can't take child stdin");
        let stdin = child.stdin.take().ok_or(stdin_err)?;
        let mut stdin = Box::new(stdin);
        let write = match proto {
            Simple => crate::comms::simple::write_stdin,
            Json => crate::comms::json::write_stdin,
        };
        move || -> R {
            for outc in out_rx {
                write(&mut stdin, outc)?;
            }
            Ok(())
        }
    }));
    
    Ok((out_tx, in_rx, handles))
}
