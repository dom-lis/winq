use std::io;
use std::io::{Write, BufRead, BufReader};
use std::ffi::OsString;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;

use crate::comms::{CHAN_BOUND, InComm, OutComm, BadComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;

pub fn open_comms(uri: OsString, json: bool) -> Result<(SyncSender<OutComm>, Receiver<InComm>), Box<dyn Error + Send + Sync>> {
    unimplemented!();
}