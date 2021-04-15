//! The scanner module

use crate::error::{GenError, GenResult, ScannerError};
use std::fmt;
use std::path::Path;

pub struct Scanner<'a> {
    source_file: &'a Path,
}

impl<'a> Scanner<'a> {
    pub fn new(source_file: &'a str) -> Self {
        Scanner {
            source_file: Path::new(source_file),
        }
    }

    pub fn scan_token(&mut self) -> GenResult<Token> {
        Err(GenError::from(ScannerError::new("todo")))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    spelling: String,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String) -> Self {
        Token { kind, spelling }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
    Func,
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
    Proc,
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
            TokenKind::Func => "function",
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
            TokenKind::Proc => "proc",
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
        let scanner = Scanner::new(source_file);
        let test_cases = vec![Token::new(TokenKind::Eof, "-1")];

        for tt in test_cases {
            let token = scanner.scan_token();
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
