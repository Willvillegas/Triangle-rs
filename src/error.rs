//! Error-handling module.

use std::error::Error;
use std::io::{stderr, Write};

pub type GenError = Box<dyn Error>;
pub type GenResult<T> = Result<T, GenError>;

macro_rules! impl_errors_with_position {
    ( $( $error_type:ident )* ) => {
        $(
            #[derive(Debug)]
            pub struct $error_type {
                message: String,
                position: $crate::scanner::SourcePosition,
            }

            impl $error_type {
                pub fn new(message: &str, position: $crate::scanner::SourcePosition) -> Self {
                    $error_type {
                        message: String::from(message),
                        position: position,
                    }
                }
            }

            impl std::error::Error for $error_type {}

            impl std::fmt::Display for $error_type {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}",
                        &format!(concat!(stringify!($error_type), " error at line {} and column {}: {}"),
                            self.position.start.line, self.position.start.column, self.message))
                }
            }
          )*
    };
}

macro_rules! impl_errors_with_no_position {
    ( $( $error_type:ident )* ) => {
        $(
            #[derive(Debug)]
            pub struct $error_type {
                message: String,
            }

            impl $error_type {
                pub fn new(message: &str) -> Self {
                    $error_type {
                        message: String::from(message),
                    }
                }
            }

            impl std::error::Error for $error_type {}

            impl std::fmt::Display for $error_type {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}",
                        &format!(concat!(stringify!($error_type), " error: {}"),
                            self.message))
                }
            }
          )*
    };
}

impl_errors_with_position!(ScannerError ParserError);
impl_errors_with_no_position!(CheckerError EncoderError CompilerError);

pub fn report_error_and_exit(error: GenError) -> ! {
    let _ = writeln!(stderr(), "{}", error);
    std::process::exit(1);
}
