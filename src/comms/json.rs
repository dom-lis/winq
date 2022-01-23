use std::io::{Read, Write, BufRead, BufReader};
use std::error::Error;
use serde_json::error::Category;

use crate::comms::{InComm, OutComm, BadComm};

type E = Box<dyn Error + Send + Sync>;
type R = Result<InComm, E>;

pub fn read_stdout(stdout: Box<dyn Read>) -> Box<dyn Iterator<Item=R>> {
    let stdout = BufReader::new(stdout);
    let iter = stdout
        .lines()
        .map(|line| -> R {
            let line = line?;
            let comm = serde_json::from_str(&line);
            match comm {
                Ok(comm) => Ok(comm),
                Err(e) => match e.classify() {
                    Category::Io | Category::Eof => Err(Box::new(e)),
                    _ => Ok(BadComm(format!("{:?}", e)))
                }
            }
        });
    Box::new(iter)
}

pub fn write_stdin(stdin: &mut dyn Write, comm: OutComm) -> Result<(), E> {
    let line = serde_json::to_string(&comm)?;
    writeln!(stdin, "{}", line)?;
    Ok(())
}
