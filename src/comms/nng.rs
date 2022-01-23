use std::io;
use std::ffi::OsString;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;

use crate::opts::Protocol;
use crate::comms::{CHAN_BOUND, InComm, OutComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;
type Rc = Result<(SyncSender<OutComm>, Receiver<InComm>, Vec<JoinHandle<R>>), E>;

pub fn open_comms(uri: OsString, proto: Protocol) -> Rc {
    unimplemented!();
}