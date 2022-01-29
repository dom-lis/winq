use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub fn open<A: ToSocketAddrs>(addr: A) -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    let s = TcpStream::connect(addr)?;
    Ok((
        Box::new(s.try_clone()?),
        Box::new(s.try_clone()?),
    ))
}
