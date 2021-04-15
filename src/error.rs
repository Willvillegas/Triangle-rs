//! Error-handling module.

use std::error;
use std::fmt;

pub(crate) type GenError = Box<dyn error::Error>;
pub(crate) type GenResult<T> = Result<T, GenError>;

#[derive(Debug)]
pub(crate) struct ScannerError {
    message: String,
}

impl ScannerError {
    pub fn new(message: &str) -> Self {
        ScannerError {
            message: String::from(message),
        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for ScannerError {}

#[derive(Debug)]
pub(crate) struct ParserError {}

#[derive(Debug)]
pub(crate) struct CheckerError {}

#[derive(Debug)]
pub(crate) struct EncoderError {}

#[derive(Debug)]
pub(crate) struct CompilerError {}