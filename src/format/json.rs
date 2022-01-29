use std::io::{Read, Write, BufRead, BufReader};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::error::Error;
use std::thread;
use serde_json::error::Category;
use crate::transport::{CHAN_BOUND, InComm, OutComm, BadComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<(), E>;

pub fn open(r: Box<dyn Read + Send>, w: Box<dyn Write + Send>) -> (SyncSender<OutComm>, Receiver<InComm>) {
    let (in_tx, in_rx) = sync_channel(CHAN_BOUND);
    thread::spawn({
        let r = r;
        move || -> R {
            let reader = BufReader::new(r);
            for line in reader.lines() {
                let line = line?;
                let comm = serde_json::from_str(&line);
                let comm = match comm {
                    Ok(comm) => Ok(comm),
                    Err(e) => match e.classify() {
                        Category::Io | Category::Eof => Err(Box::new(e)),
                        _ => Ok(BadComm(format!("{:?}", e)))
                    }
                }?;
                in_tx.send(comm)?;
            }
            Ok(())
        }
    });
    
    let (out_tx, out_rx) = sync_channel(CHAN_BOUND);
    thread::spawn({
        let mut w = w;
        move || -> R {
            for c in out_rx {
                let line = serde_json::to_string(&c)?;
                writeln!(&mut w, "{}", line)?;
            }
            Ok(())
        }
    });

    (out_tx, in_rx)
}
