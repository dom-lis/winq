use std::io;
use std::io::{Read, Write};

pub fn open() -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    Ok((
        Box::new(io::stdin()),
        Box::new(io::stdout())
    ))
}
