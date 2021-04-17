//! The Parser module.
//!
//! This module consumes the stream of tokens produced by the Scanner, and constructs an AST for
//! Triangle, which is then used by all subsequent phases of the compiler.

use crate::ast::commands;
use crate::ast::Program;
use crate::error::report_error_and_exit;
use crate::scanner::Scanner;

pub struct Parser {}

impl Parser {
    pub fn new(scanner: Scanner) -> Self {
        Parser {}
    }

    pub fn parse_program(&mut self) -> Program {
        Program::new(commands::Command::EmptyCommand(commands::EmptyCommandState))
    }
}