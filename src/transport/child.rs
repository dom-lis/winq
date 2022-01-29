use std::io;
use std::io::{Read, Write};
use std::ffi::OsString;

pub fn open(cmd: OsString, args: &[OsString]) -> io::Result<(Box<dyn Read + Send>, Box<dyn Write + Send>)> {
    let stdin_err = io::Error::new(io::ErrorKind::Other, "can't take child stdin");
    let stdout_err = io::Error::new(io::ErrorKind::Other, "can't take child stdout");

    let mut child = std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    let r = child.stdout.take().ok_or(stdout_err)?;
    let w = child.stdin.take().ok_or(stdin_err)?;
    Ok((Box::new(r), Box::new(w)))
}
