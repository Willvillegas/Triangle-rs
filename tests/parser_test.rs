use triangle_rs::parser::*;
use triangle_rs::scanner::*;

#[test]
fn test_emptycommandeot() {
    let source_file = "samples/source/emptycommandeot.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_emptycommandeot_degenerate() {
    let source_file = "samples/source/emptycommandeot_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_emptycommandsemicolon() {
    let source_file = "samples/source/emptycommandsemicolon.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_emptycommandsemicolon_degenerate() {
    let source_file = "samples/source/emptycommandsemicolon_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_hello() {
    let source_file = "samples/source/hello.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_hello_degenerate() {
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_inc() {
    let source_file = "samples/source/inc.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_inc_degenerate() {
    let source_file = "samples/source/inc_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_echo() {
    let source_file = "samples/source/echo.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_echo_degenerate() {
    let source_file = "samples/source/echo_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_odd() {
    let source_file = "samples/source/odd.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_odd_degenerate() {
    let source_file = "samples/source/odd_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_sum_proc() {
    let source_file = "samples/source/sum_proc.t";
    let mut scanner = Scanner::new(source_file);
    let test_cases = vec![
        Token::new(
            TokenType::Let,
            "let",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "x",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "y",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "s",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Procedure,
            "proc",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "add",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "a",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Comma,
            ",",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "b",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Comma,
            ",",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "r",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Integer",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Is, "~", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Identifier,
            "r",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Becomes,
            ":=",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "a",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "+",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "b",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::In, "in", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Begin,
            "begin",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "getint",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "x",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "getint",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "y",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "add",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "x",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Comma,
            ",",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "y",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Comma,
            ",",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Var,
            "var",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "s",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Semicolon,
            ";",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "putint",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "s",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::End,
            "end",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Eot,
            NULL_STR,
            SourcePosition::dummy_source_position(),
        ),
    ];

    for tt in test_cases {
        let token = scanner.scan_token();
        assert_eq!(tt, token);
    }
}

#[test]
fn test_sum_proc_degenerate() {
    let source_file = "samples/source/sum_proc_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_power() {
    let source_file = "samples/source/power.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_power_degenerate() {
    let source_file = "samples/source/power_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_factorial() {
    let source_file = "samples/source/factorial.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_factorial_degenerate() {
    let source_file = "samples/source/factorial_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_record() {
    let source_file = "samples/source/record.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_record_degenerate() {
    let source_file = "samples/source/record_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_leapyear() {
    let source_file = "samples/source/leapyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_leapyear_degenerate() {
    let source_file = "samples/source/leapyear_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_date() {
    let source_file = "samples/source/date.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_date_degenerate() {
    let source_file = "samples/source/date_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_print_array() {
    let source_file = "samples/source/print_array.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_print_array_degnerate() {
    let source_file = "samples/source/print_array_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_string() {
    let source_file = "samples/source/string.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_string_degenerate() {
    let source_file = "samples/source/string_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_reverse_line() {
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_reverse_line_degenerate() {
    let source_file = "samples/source/reverse_line_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_iteratively() {
    let source_file = "samples/source/iteratively.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_iteratively_degenerate() {
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_line() {
    let source_file = "samples/source/line.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_line_degenerate() {
    let source_file = "samples/source/line_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_dates() {
    let source_file = "samples/source/dates.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_dates_degenerate() {
    let source_file = "samples/source/dates_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_monthsofyear() {
    let source_file = "samples/source/monthsofyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_monthsofyear_degenerate() {
    let source_file = "samples/source/monthsofyear_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_capitalise() {
    let source_file = "samples/source/capitalise.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_capitalise_degenerate() {
    let source_file = "samples/source/capitalise_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_freq() {
    let source_file = "samples/source/freq.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_freq_degenerate() {
    let source_file = "samples/source/freq_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_insertion_sort() {
    let source_file = "samples/source/insertion_sort.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_insertion_sort_degenerate() {
    let source_file = "samples/source/insertion_sort_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_rationals() {
    let source_file = "samples/source/rationals.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_rationals_degenerate() {
    let source_file = "samples/source/rationals_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_eqnoteq() {
    let source_file = "samples/source/eqnoteq.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_eqnoteq_degenerate() {
    let source_file = "samples/source/eqnoteq_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_nestedarrays() {
    let source_file = "samples/source/nestedarrays.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_nestedarrays_degenerate() {
    let source_file = "samples/source/nestedarrays_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_nestedrecords() {
    let source_file = "samples/source/nestedrecords.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_nestedrecords_degenerate() {
    let source_file = "samples/source/nestedrecords_degenerate.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new();
    let actual_program = parser.parseProgram();
    assert_eq!(expected_program, actual_program);
}
