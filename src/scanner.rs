//! The scanner module

use crate::error::{self, GenError, GenResult, ScannerError};
use phf::phf_map;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;
use std::path::Path;

pub const NULL: char = '\x00';

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "array" => TokenKind::Array,
    "begin" => TokenKind::Begin,
    "const" => TokenKind::Const,
    "do" => TokenKind::Do,
    "else" => TokenKind::Else,
    "end" => TokenKind::End,
    "func" => TokenKind::Function,
    "if" => TokenKind::If,
    "in" => TokenKind::In,
    "let" => TokenKind::Let,
    "proc" => TokenKind::Procedure,
    "record" => TokenKind::Record,
    "then" => TokenKind::Then,
    "type" => TokenKind::Type,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};

pub struct Scanner {
    source_file: SourceFile,
}

impl Scanner {
    pub fn new(source_file: &'static str) -> Self {
        Scanner {
            source_file: SourceFile::new(source_file),
        }
    }

    pub fn scan_token(&mut self) -> GenResult<Token> {
        unimplemented!()
    }
}

// Represents the start and end of a token in the source code
#[derive(Debug, Copy, Clone)]
pub struct SourcePosition {
    start: Position,
    finish: Position,
}

struct Position {
    line: isize,
    column: isize,
}

#[derive(Debug, Copy, Clone)]
struct Char {
    c: char,
    line: isize,
    column: isize,
}

impl Char {
    pub fn new(c: char, line: isize, column: isize) -> Self {
        Char { c, line, column }
    }
}

/// Abstraction of a source file - an iterator that returns the next available character
/// from the file stream.
struct SourceFile {
    curr_idx: usize,
    characters: Vec<Char>,
}

impl SourceFile {
    pub fn new(source_file: &str) -> Self {
        let characters = match SourceFile::load_source_file(source_file) {
            Err(e) => error::report_error_and_exit(GenError::from(e)),
            Ok(val) => val,
        };

        SourceFile {
            curr_idx: 0,
            characters: characters,
        }
    }

    fn load_source_file(source_file: &str) -> GenResult<Vec<Char>> {
        let mut reader = BufReader::new(File::open(source_file)?);
        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        let mut line = 1;
        let mut column = 1;
        let mut contents = Vec::new();

        for c in text.chars() {
            if c == '\n' {
                line += 1;
                column = 1;
            }
            contents.push(Char::new(c, line, column));
        }
        contents.push(Char::new(NULL, -1, -1));

        Ok(contents)
    }
}

impl Iterator for SourceFile {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_idx >= self.characters.len() {
            return None;
        }
        let val = self.characters[self.curr_idx];
        self.curr_idx += 1;
        Some(val)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    spelling: &'static str,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: &'static str) -> Self {
        if Token::is_keyword(spelling) {
            Token {
                kind: Token::get_token_kind_for_keyword(spelling),
                spelling: spelling,
            }
        } else {
            Token { kind, spelling }
        }
    }

    pub fn is_keyword(spelling: &'static str) -> bool {
        KEYWORDS.contains_key(spelling)
    }

    pub fn get_token_kind_for_keyword(spelling: &'static str) -> TokenKind {
        *KEYWORDS.get(spelling).unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Array,
    Becomes,
    Begin,
    Colon,
    Comma,
    Const,
    Do,
    Dot,
    Else,
    End,
    Eof,
    Eot,
    Function,
    Identifier,
    If,
    In,
    Is,
    LeftBracket,
    LeftParen,
    LeftSquareBracket,
    Let,
    Number,
    Operator,
    Procedure,
    Record,
    RightBracket,
    RightParen,
    RightSquareBracket,
    SemiColon,
    Then,
    Type,
    Var,
    While,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TokenKind::Array => "array",
            TokenKind::Becomes => ":=",
            TokenKind::Begin => "begin",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Const => "const",
            TokenKind::Do => "do",
            TokenKind::Dot => ".",
            TokenKind::Else => "else",
            TokenKind::End => "end",
            TokenKind::Eof => "<eof>",
            TokenKind::Eot => "<eot>",
            TokenKind::Function => "function",
            TokenKind::Identifier => "identifier",
            TokenKind::If => "if",
            TokenKind::In => "in",
            TokenKind::Is => "~",
            TokenKind::LeftBracket => "{",
            TokenKind::LeftParen => "(",
            TokenKind::LeftSquareBracket => "[",
            TokenKind::Let => "let",
            TokenKind::Number => "number",
            TokenKind::Operator => "operator",
            TokenKind::Procedure => "proc",
            TokenKind::Record => "record",
            TokenKind::RightBracket => "}",
            TokenKind::RightParen => ")",
            TokenKind::RightSquareBracket => "]",
            TokenKind::SemiColon => ";",
            TokenKind::Then => "then",
            TokenKind::Type => "type",
            TokenKind::Var => "var",
            TokenKind::While => "while",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emptycommandeot() {
        let source_file = "samples/source/emptycommandeot.t";
        let mut scanner = Scanner::new(source_file);
        let test_cases = vec![Token::new(TokenKind::Eof, "-1")];

        for tt in test_cases {
            let token = scanner.scan_token().unwrap();
            assert_eq!(tt, token);
        }
    }

    #[test]
    fn test_emptycommandeot_degenerate() {}

    #[test]
    fn test_empty_commandsemicolon() {}

    #[test]
    fn test_empty_commandsemicolon_degenerate() {}

    #[test]
    fn test_hello() {}

    #[test]
    fn test_hello_degenerate() {}

    #[test]
    fn test_eqnoteq() {}

    #[test]
    fn test_eqnoteq_degenerate() {}

    #[test]
    fn test_inc() {}

    #[test]
    fn test_inc_degenerate() {}

    #[test]
    fn test_echo() {}

    #[test]
    fn test_echo_degenerate() {}

    #[test]
    fn test_odd() {}

    #[test]
    fn test_odd_degenerate() {}

    #[test]
    fn test_sum_proc() {}

    #[test]
    fn test_sum_proc_degenerate() {}

    #[test]
    fn test_power() {}

    #[test]
    fn test_power_degenerate() {}

    #[test]
    fn test_factorial() {}

    #[test]
    fn test_factorial_degenerate() {}

    #[test]
    fn test_record() {}

    #[test]
    fn test_record_degenerate() {}

    #[test]
    fn test_leapyear() {}

    #[test]
    fn test_leapyear_degenerate() {}

    #[test]
    fn test_date() {}

    #[test]
    fn test_date_degenerate() {}

    #[test]
    fn test_print_array() {}

    #[test]
    fn test_print_array_degnerate() {}

    #[test]
    fn test_string() {}

    #[test]
    fn test_string_degenerate() {}

    #[test]
    fn test_reverse_line() {}

    #[test]
    fn test_reverse_line_degenerate() {}

    #[test]
    fn test_nestedrecords() {}

    #[test]
    fn test_nestedrecords_degenerate() {}

    #[test]
    fn test_iteratively() {}

    #[test]
    fn test_iteratively_degenerate() {}

    #[test]
    fn test_nestedarrays() {}

    #[test]
    fn test_nestedarrays_degenerate() {}

    #[test]
    fn test_line() {}

    #[test]
    fn test_line_degenerate() {}

    #[test]
    fn test_dates() {}

    #[test]
    fn test_dates_degenerate() {}

    #[test]
    fn test_monthsofyear() {}

    #[test]
    fn test_monthsofyear_degenerate() {}

    #[test]
    fn test_capitalise() {}

    #[test]
    fn test_capitalise_degenerate() {}

    #[test]
    fn test_freq() {}

    #[test]
    fn test_freq_degenerate() {}

    #[test]
    fn test_insertion_sort() {}

    #[test]
    fn test_insertion_sort_degenerate() {}

    #[test]
    fn test_rationals() {}

    #[test]
    fn test_rationals_degenerate() {}
}
