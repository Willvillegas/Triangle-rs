use triangle_rs::ast::commands::Command::*;
use triangle_rs::ast::commands::*;
use triangle_rs::ast::declarations::Declaration::*;
use triangle_rs::ast::expressions::Expression::*;
use triangle_rs::ast::typedenoters::TypeDenoter::*;
use triangle_rs::ast::vnames::Vname::*;
use triangle_rs::ast::*;
use triangle_rs::parser::*;
use triangle_rs::scanner::*;

#[test]
fn test_emptycommandeot() {
    let source_file = "samples/source/emptycommandeot.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(EmptyCommand(EmptyCommandState));
    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_emptycommandsemicolon() {
    let source_file = "samples/source/emptycommandsemicolon.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(EmptyCommand(EmptyCommandState));
    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_hello() {
    let source_file = "samples/source/hello.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_inc() {
    let source_file = "samples/source/inc.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_echo() {
    let source_file = "samples/source/echo.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_odd() {
    let source_file = "samples/source/odd.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_sum_proc() {
    let source_file = "samples/source/sum_proc.t";
    let mut scanner = Scanner::new(source_file);
}

#[test]
fn test_power() {
    let source_file = "samples/source/power.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_factorial() {
    let source_file = "samples/source/factorial.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_record() {
    let source_file = "samples/source/record.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_leapyear() {
    let source_file = "samples/source/leapyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_date() {
    let source_file = "samples/source/date.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_print_array() {
    let source_file = "samples/source/print_array.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_string() {
    let source_file = "samples/source/string.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_reverse_line() {
    let source_file = "samples/source/reverse_line.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_iteratively() {
    let source_file = "samples/source/iteratively.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_line() {
    let source_file = "samples/source/line.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_dates() {
    let source_file = "samples/source/dates.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_monthsofyear() {
    let source_file = "samples/source/monthsofyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_capitalise() {
    let source_file = "samples/source/capitalise.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_freq() {
    let source_file = "samples/source/freq.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_insertion_sort() {
    let source_file = "samples/source/insertion_sort.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_rationals() {
    let source_file = "samples/source/rationals.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_eqnoteq() {
    let source_file = "samples/source/eqnoteq.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_nestedarrays() {
    let source_file = "samples/source/nestedarrays.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}

#[test]
fn test_nestedrecords() {
    let source_file = "samples/source/nestedrecords.t";
    let mut parser = Parser::new(Scanner::new(source_file));
}
