use std::io;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;

use crate::opts::Protocol;
use crate::opts::Protocol::*;
use crate::comms::{CHAN_BOUND, InComm, OutComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;
type Rc = Result<(SyncSender<OutComm>, Receiver<InComm>), E>;

pub fn open_comms(proto: Protocol) -> Rc {
    
    let (in_tx, in_rx) = sync_channel(CHAN_BOUND);
    thread::spawn({
        move || -> R {
            let stdin = Box::new(io::stdin());

            let seq = match proto {
                Simple => crate::proto::simple::read(stdin),
                Json => crate::proto::json::read(stdin),
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
        move || -> R {
            let write = match proto {
                Simple => crate::proto::simple::write,
                Json => crate::proto::json::write,
            };

            for outc in out_rx {
                write(&mut io::stdout().lock(), outc)?;
            }
            Ok(())
        }
    });

    Ok((out_tx, in_rx))
}
