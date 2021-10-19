use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
pub struct Opts {
    pub cmd: OsString,
    pub cmd_args: Vec<OsString>
}
