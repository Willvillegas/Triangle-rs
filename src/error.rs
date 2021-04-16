//! Error-handling module.

use crate::scanner::SourcePosition;
use std::error::Error;
use std::fmt;
use std::io::{stderr, Write};

pub(crate) type GenError = Box<dyn Error>;
pub(crate) type GenResult<T> = Result<T, GenError>;

pub fn report_error_and_exit(error: GenError) -> ! {
    let _ = writeln!(stderr(), "{}", error);
    std::process::exit(1);
}

#[derive(Debug)]
pub(crate) struct ScannerError {
    message: String,
    position: SourcePosition,
}

impl ScannerError {
    pub fn new(message: &str, position: SourcePosition) -> Self {
        ScannerError {
            message: String::from(message),
            position: position,
        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Scanner error at line {}, column {}: {}",
            self.position.start.line, self.position.start.column, &self.message
        )
    }
}

impl Error for ScannerError {}

#[derive(Debug)]
pub(crate) struct ParserError {}

#[derive(Debug)]
pub(crate) struct CheckerError {}

#[derive(Debug)]
pub(crate) struct EncoderError {}

#[derive(Debug)]
pub(crate) struct CompilerError {}