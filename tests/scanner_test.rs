use triangle_rs::scanner::*;

#[test]
fn test_sourcefile() {
    let mut source_file = SourceFile::new("samples/source/odd_degenerate.t");
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
        NULL_STR,
        SourcePosition::dummy_source_position(),
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
        NULL_STR,
        SourcePosition::dummy_source_position(),
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
            TokenType::Semicolon,
            ";",
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
fn test_emptycommandsemicolon_degenerate() {
    let source_file = "samples/source/emptycommandsemicolon_degenerate.t";
    let mut scanner = Scanner::new(source_file);
    let test_cases = vec![
        Token::new(
            TokenType::Semicolon,
            ";",
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
fn test_hello() {
    let source_file = "samples/source/hello.t";
    let mut scanner = Scanner::new(source_file);
    let test_cases = vec![
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
            TokenType::IntegerLiteral,
            "42",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
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
fn test_hello_degenerate() {
    let source_file = "samples/source/hello_degenerate.t";
    let mut scanner = Scanner::new(source_file);
    let test_cases = vec![
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
            TokenType::IntegerLiteral,
            "42",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
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
fn test_eqnoteq() {}

#[test]
fn test_eqnoteq_degenerate() {}

#[test]
fn test_inc() {
    let source_file = "samples/source/inc.t";
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
            TokenType::Procedure,
            "proc",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "inc",
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
            "n",
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
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Becomes,
            ":=",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "+",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "1",
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
            "inc",
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
            "x",
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
fn test_inc_degenerate() {
    let source_file = "samples/source/inc_degenerate.t";
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
            TokenType::Procedure,
            "proc",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "inc",
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
            "n",
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
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Becomes,
            ":=",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "+",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "1",
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
            "inc",
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
            "x",
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
fn test_echo() {
    let source_file = "samples/source/echo.t";
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
            "ch",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Char",
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
            "echo",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Is, "~", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Begin,
            "begin",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::While,
            "while",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "\\",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "eol",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Do, "do", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Begin,
            "begin",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "get",
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
            "ch",
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
            "put",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "ch",
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
            TokenType::End,
            "end",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::In, "in", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Identifier,
            "echo",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
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
fn test_echo_degenerate() {
    let source_file = "samples/source/echo_degenerate.t";
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
            "ch",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Char",
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
            "echo",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Is, "~", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Begin,
            "begin",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::While,
            "while",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "\\",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "eol",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Do, "do", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Begin,
            "begin",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "get",
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
            "ch",
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
            "put",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "ch",
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
            TokenType::End,
            "end",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::In, "in", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Identifier,
            "echo",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
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
fn test_odd() {
    let source_file = "samples/source/odd.t";
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
            "n",
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
            TokenType::Function,
            "func",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "odd",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
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
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Boolean",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Is, "~", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "//",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "2",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "\\=",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "0",
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
            "n",
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
        Token::new(TokenType::If, "if", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Identifier,
            "odd",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Then,
            "then",
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
            TokenType::IntegerLiteral,
            "1",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Else,
            "else",
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
            TokenType::IntegerLiteral,
            "2",
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
fn test_odd_degenerate() {
    let source_file = "samples/source/odd_degenerate.t";
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
            "n",
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
            TokenType::Function,
            "func",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "odd",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
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
        Token::new(
            TokenType::Colon,
            ":",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "Boolean",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(TokenType::Is, "~", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "//",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "2",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Operator,
            "\\=",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::IntegerLiteral,
            "0",
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
            "n",
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
        Token::new(TokenType::If, "if", SourcePosition::dummy_source_position()),
        Token::new(
            TokenType::Identifier,
            "odd",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Identifier,
            "n",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Then,
            "then",
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
            TokenType::IntegerLiteral,
            "1",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            SourcePosition::dummy_source_position(),
        ),
        Token::new(
            TokenType::Else,
            "else",
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
            TokenType::IntegerLiteral,
            "2",
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
