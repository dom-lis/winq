use clap::Clap;
use std::ffi::OsString;

#[derive(Clap)]
pub struct Opts {
    pub cmd: OsString,
    pub cmd_args: Vec<OsString>
}
