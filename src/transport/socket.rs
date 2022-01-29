use std::io;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

pub fn open<P: AsRef<Path>>(path: P) -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    let s = UnixStream::connect(path)?;
    Ok((
        Box::new(s.try_clone()?),
        Box::new(s.try_clone()?),
    ))
}
