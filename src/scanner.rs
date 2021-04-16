//! The scanner module
//!
//! Scan the source file into a stream of tokens that are consumed by the parser, one token at a
//! time.

use crate::error::{self, GenError, GenResult, ScannerError};
use phf::phf_map;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;

pub const NULL: char = '\x00';

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "array" => TokenType::Array,
    "begin" => TokenType::Begin,
    "const" => TokenType::Const,
    "do" => TokenType::Do,
    "else" => TokenType::Else,
    "end" => TokenType::End,
    "func" => TokenType::Function,
    "if" => TokenType::If,
    "in" => TokenType::In,
    "let" => TokenType::Let,
    "of" => TokenType::Of,
    "proc" => TokenType::Procedure,
    "record" => TokenType::Record,
    "then" => TokenType::Then,
    "type" => TokenType::Type,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

/// Scanner - read a character at a time from the underlying source file,
/// and generate a token on demand.
pub struct Scanner {
    source_file: SourceFile,
    current_char: Char,
    current_spelling: String,
    current_position: SourcePosition,
}

impl Scanner {
    pub fn new(source_file: &str) -> Self {
        let mut scanner = Scanner {
            source_file: SourceFile::new(source_file),
            current_char: Char::null_char(),
            current_position: SourcePosition::null_source_position(),
            current_spelling: String::new(),
        };

        scanner.skip_it();
        scanner
    }

    fn start(&mut self) {
        self.current_position.start.line = self.current_char.line;
        self.current_position.start.column = self.current_char.column;
    }

    fn finish(&mut self) {
        self.current_position.finish.line = self.current_char.line;
        self.current_position.finish.column = self.current_char.column;
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.c.is_whitespace() {
            self.skip_it();
        }
    }

    fn skip_it(&mut self) {
        if let Some(next_char) = self.source_file.next() {
            self.current_char = next_char;
        } else {
            error::report_error_and_exit(GenError::from(ScannerError::new(
                "tried to skip character, found no more characters",
                self.current_position,
            )));
        }
    }

    fn eat_it(&mut self) {
        if self.current_char.c == NULL {
            self.current_spelling.push(self.current_char.c);
            return;
        }

        if let Some(next_char) = self.source_file.next() {
            self.current_spelling.push(self.current_char.c);
            self.current_char = next_char;
        } else {
            error::report_error_and_exit(GenError::from(ScannerError::new(
                "expected to eat character, found no more characters",
                self.current_position,
            )));
        }
    }

    pub fn scan_token(&mut self) -> Token {
        while self.current_char.c.is_whitespace() || self.current_char.c == '!' {
            self.skip_whitespace();
        }

        self.current_spelling = String::new();
        let current_kind = self.scan();
        Token::new(current_kind, &self.current_spelling, self.current_position)
    }

    fn scan(&mut self) -> TokenType {
        let kind;
        self.current_position = SourcePosition::null_source_position();
        self.start();

        match self.current_char.c {
            '!' => {
                self.skip_whitespace();
                return self.scan();
            }

            '~' => {
                self.finish();
                self.eat_it();
                kind = TokenType::Is;
            }

            '(' => {
                self.finish();
                self.eat_it();
                kind = TokenType::LeftParen;
            }

            '{' => {
                self.finish();
                self.eat_it();
                kind = TokenType::LeftBracket;
            }

            '[' => {
                self.finish();
                self.eat_it();
                kind = TokenType::LeftSquareBracket;
            }

            ')' => {
                self.finish();
                self.eat_it();
                kind = TokenType::RightParen;
            }

            '}' => {
                self.finish();
                self.eat_it();
                kind = TokenType::RightBracket;
            }

            ']' => {
                self.finish();
                self.eat_it();
                kind = TokenType::RightSquareBracket;
            }

            '+' | '-' | '*' | '=' => {
                self.finish();
                self.eat_it();
                kind = TokenType::Operator;
            }

            '/' => {
                self.finish();
                self.eat_it();

                if self.current_char.c == '\\' || self.current_char.c == '/' {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Operator;
            }

            '\\' => {
                self.finish();
                self.eat_it();

                if self.current_char.c == '=' {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Operator;
            }

            '<' => {
                self.finish();
                self.eat_it();

                if self.current_char.c == '=' {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Operator;
            }

            '>' => {
                self.finish();
                self.eat_it();

                if self.current_char.c == '=' {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Operator;
            }

            ',' => {
                self.finish();
                self.eat_it();
                kind = TokenType::Comma;
            }

            '.' => {
                self.finish();
                self.eat_it();
                kind = TokenType::Dot;
            }

            ';' => {
                self.finish();
                self.eat_it();
                kind = TokenType::SemiColon;
            }

            ':' => {
                self.finish();
                self.eat_it();

                if self.current_char.c == '=' {
                    self.finish();
                    self.eat_it();
                    kind = TokenType::Becomes;
                } else {
                    kind = TokenType::Colon;
                }
            }

            'a'..='z' | 'A'..='Z' => {
                self.finish();
                self.eat_it();

                while self.current_char.c.is_alphanumeric() {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Identifier;
            }

            '0'..='9' => {
                self.finish();
                self.eat_it();

                while self.current_char.c.is_numeric() {
                    self.finish();
                    self.eat_it();
                }
                kind = TokenType::Number;
            }

            '\x00' => {
                self.finish();
                self.eat_it();
                kind = TokenType::Eot;
            }

            _ => {
                error::report_error_and_exit(GenError::from(ScannerError::new(
                    &format!("unexpected character {}", self.current_char.c),
                    self.current_position,
                )));
            }
        }

        kind
    }
}

/// Represents the start and end of a token in the source code.
/// Both the beginning and the end of the token are recorded for
/// error reporting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourcePosition {
    pub start: Position,
    pub finish: Position,
}

impl SourcePosition {
    pub fn new(start: Position, finish: Position) -> Self {
        SourcePosition { start, finish }
    }

    pub fn null_source_position() -> Self {
        SourcePosition {
            start: Position::null_position(),
            finish: Position::null_position(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub line: isize,
    pub column: isize,
}

impl Position {
    pub fn new(line: isize, column: isize) -> Self {
        Position { line, column }
    }

    pub fn null_position() -> Self {
        Position {
            line: -1,
            column: -1,
        }
    }
}

/// Simple wrapper around a character in the source file
#[derive(Debug, Copy, Clone)]
struct Char {
    pub c: char,
    pub line: isize,
    pub column: isize,
}

impl Char {
    pub fn new(c: char, line: isize, column: isize) -> Self {
        Char { c, line, column }
    }

    pub fn null_char() -> Self {
        Char {
            c: NULL,
            line: -1,
            column: -1,
        }
    }
}

impl PartialEq for Char {
    fn eq(&self, other: &Char) -> bool {
        return self.c == other.c;
    }
}

impl Eq for Char {}

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

/// Token - the basic lexeme in the source language
#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenType,
    spelling: String,
    position: SourcePosition,
}

impl Token {
    pub fn new(kind: TokenType, spelling: &str, position: SourcePosition) -> Self {
        let mut token = Token {
            kind: kind,
            spelling: String::from(spelling),
            position: position,
        };

        if Token::is_keyword(&token.spelling) {
            token.kind = Token::get_token_kind_for_keyword(&token.spelling);
        }

        token
    }

    pub fn is_keyword(spelling: &str) -> bool {
        KEYWORDS.contains_key(spelling)
    }

    pub fn get_token_kind_for_keyword(spelling: &str) -> TokenType {
        *KEYWORDS.get(spelling).unwrap()
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind && self.spelling == other.spelling
    }
}

impl Eq for Token {}

/// All the possible kinds of tokens
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
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
    Of,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TokenType::Array => "array",
            TokenType::Becomes => ":=",
            TokenType::Begin => "begin",
            TokenType::Colon => ":",
            TokenType::Comma => ",",
            TokenType::Const => "const",
            TokenType::Do => "do",
            TokenType::Dot => ".",
            TokenType::Else => "else",
            TokenType::End => "end",
            TokenType::Eot => "<eot>",
            TokenType::Function => "function",
            TokenType::Identifier => "identifier",
            TokenType::If => "if",
            TokenType::In => "in",
            TokenType::Is => "~",
            TokenType::LeftBracket => "{",
            TokenType::LeftParen => "(",
            TokenType::LeftSquareBracket => "[",
            TokenType::Let => "let",
            TokenType::Number => "number",
            TokenType::Of => "of",
            TokenType::Operator => "operator",
            TokenType::Procedure => "proc",
            TokenType::Record => "record",
            TokenType::RightBracket => "}",
            TokenType::RightParen => ")",
            TokenType::RightSquareBracket => "]",
            TokenType::SemiColon => ";",
            TokenType::Then => "then",
            TokenType::Type => "type",
            TokenType::Var => "var",
            TokenType::While => "while",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sourcefile() {
        let mut source_file = SourceFile::new("samples/source/rationals.t");
        while let Some(c) = source_file.next() {
            println!("{:?}", c);
        }
    }

    #[test]
    fn test_emptycommandeot() {
        let source_file = "samples/source/emptycommandeot.t";
        let mut scanner = Scanner::new(source_file);
        let test_cases = vec![Token::new(
            TokenType::Eot,
            "\x00",
            SourcePosition::null_source_position(),
        )];

        for tt in test_cases {
            let token = scanner.scan_token();
            assert_eq!(tt, token);
        }
    }

    #[test]
    fn test_emptycommandeot_degenerate() {
        let source_file = "samples/source/emptycommandeot_degenerate.t";
        let mut scanner = Scanner::new(source_file);
        let test_cases = vec![Token::new(
            TokenType::Eot,
            "\x00",
            SourcePosition::null_source_position(),
        )];

        for tt in test_cases {
            let token = scanner.scan_token();
            assert_eq!(tt, token);
        }
    }

    #[test]
    fn test_emptycommandsemicolon() {
        let source_file = "samples/source/emptycommandsemicolon.t";
        let mut scanner = Scanner::new(source_file);
        let test_cases = vec![
            Token::new(
                TokenType::SemiColon,
                ";",
                SourcePosition::null_source_position(),
            ),
            Token::new(
                TokenType::Eot,
                "\x00",
                SourcePosition::null_source_position(),
            ),
        ];

        for tt in test_cases {
            let token = scanner.scan_token();
            assert_eq!(tt, token);
        }
    }

    #[test]
    fn test_emptycommandsemicolon_degenerate() {
        let source_file = "samples/source/emptycommandsemicolon_degenerate.t";
        let mut scanner = Scanner::new(source_file);
        let test_cases = vec![
            Token::new(
                TokenType::SemiColon,
                ";",
                SourcePosition::null_source_position(),
            ),
            Token::new(
                TokenType::Eot,
                "\x00",
                SourcePosition::null_source_position(),
            ),
        ];

        for tt in test_cases {
            let token = scanner.scan_token();
            assert_eq!(tt, token);
        }
    }

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
