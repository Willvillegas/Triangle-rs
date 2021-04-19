//! The Parser module.
//!
//! This module consumes the stream of tokens produced by the Scanner, and constructs an AST for
//! Triangle, which is then used by all subsequent phases of the compiler.

use crate::ast;
use crate::ast::commands::{Command, EmptyCommandState, SequentialCommandState};
use crate::ast::Program;
use crate::error::{self, GenError, ParserError};
use crate::scanner::{Scanner, SourcePosition, Token, TokenType};

pub struct Parser {
    scanner: Scanner,
    current_token: Token,
}

impl Parser {
    pub fn new(mut scanner: Scanner) -> Self {
        let initial_token = scanner.scan_token();

        Parser {
            scanner: scanner,
            current_token: initial_token,
        }
    }

    fn start(&mut self, position: &mut SourcePosition) {
        position.start.line = self.current_token.position.start.line;
        position.start.column = self.current_token.position.start.column;
    }

    fn finish(&mut self, position: &mut SourcePosition) {
        position.finish.line = self.current_token.position.finish.line;
        position.finish.column = self.current_token.position.finish.column;
    }

    fn accept(&mut self, tt: TokenType) {
        if self.current_token.kind == tt {
            self.current_token = self.scanner.scan_token();
        } else {
            error::report_error_and_exit(GenError::from(ParserError::new(
                &format!(
                    "expected to accept token of kind {:?}, but got kind {:}",
                    tt, self.current_token.kind
                ),
                self.current_token.position,
            )));
        }
    }

    fn accept_it(&mut self) {
        self.current_token = self.scanner.scan_token();
    }

    ///```
    /// Command ::= single-Command | Command ; single-Command
    /// ```
    fn parse_command(&mut self) -> Command {
        let mut cmdpos = SourcePosition::default();
        self.start(&mut cmdpos);
        let mut cmd = self.parse_single_command();
        self.finish(&mut cmdpos);

        while self.current_token.spelling == ";" {
            self.accept_it();
            let cmd1 = self.parse_single_command();
            self.finish(&mut cmdpos);
            cmd = Command::SequentialCommand(SequentialCommandState::new_with_position(
                cmd, cmd1, cmdpos,
            ));
        }

        cmd
    }

    /// ```
    /// single-Command:: EmptyCommand
    ///                  | AssignCommand
    ///                  | CallCommand
    ///                  | BracketedCommand
    ///                  | IfCommand
    ///                  | LetCommand
    ///                  | WhileCommand
    /// ```
    fn parse_single_command(&mut self) -> Command {
        let cmd;
        let mut cmdpos = SourcePosition::default();
        self.start(&mut cmdpos);

        match self.current_token.kind {
            TokenType::Semicolon | TokenType::End | TokenType::Eot => {
                self.accept_it();
                cmd = Command::EmptyCommand(EmptyCommandState)
            }
            _ => {
                println!(
                    "curr token = {:?}, not implemented or error",
                    self.current_token
                );
                error::report_error_and_exit(GenError::from(ParserError::new(
                    &format!("unexpected token of kind {:?}", self.current_token.kind),
                    self.current_token.position,
                )));
            }
        }

        cmd
    }

    ///```
    /// Program ::= Command <Eot>
    /// ```
    pub fn parse_program(&mut self) -> ast::Program {
        let mut pos = SourcePosition::default();
        self.start(&mut pos);
        let cmd = self.parse_command();
        self.finish(&mut pos);
        self.accept(TokenType::Eot);

        Program::new_with_position(cmd, pos)
    }
}