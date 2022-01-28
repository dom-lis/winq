use std::ffi::OsString;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;
use std::os::unix::net::UnixStream;

use crate::opts::Protocol;
use crate::opts::Protocol::*;
use crate::comms::{CHAN_BOUND, InComm, OutComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;
type Rc = Result<(SyncSender<OutComm>, Receiver<InComm>), E>;

pub fn open_comms(uri: OsString, proto: Protocol) -> Rc {
    let socket = UnixStream::connect(uri)?;
    
    let (in_tx, in_rx) = sync_channel(CHAN_BOUND);
    thread::spawn({
        let socket = socket.try_clone()?;
        let socket = Box::new(socket);
        move || -> R {
            let seq = match proto {
                Simple => crate::proto::simple::read(socket),
                Json => crate::proto::simple::read(socket),
            };

            for comm in seq {
                let comm = comm?;
                in_tx.send(comm)?;
            }

            Ok(())
        }
    });
    
    let (out_tx, out_rx) = sync_channel(CHAN_BOUND);
    thread::spawn({
        let socket = socket.try_clone()?;
        let mut socket = Box::new(socket);
        let write = match proto {
            Simple => crate::proto::simple::write,
            Json => crate::proto::json::write,
        };
        move || -> R {
            for outc in out_rx {
                write(&mut socket, outc)?;
            }
            Ok(())
        }
    });
    
    Ok((out_tx, in_rx))
}