use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

#[derive(Debug)]
pub enum ChildError {
    Exit(Option<i32>),
    Stderr(Vec<String>)
}

impl Display for ChildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

impl Error for ChildError {}

#[derive(Debug)]
pub enum InternalError {
    BadMode(String)
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

impl Error for InternalError {}
