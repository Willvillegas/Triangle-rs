use triangle_rs::ast::aggregates::ArrayAggregate::*;
use triangle_rs::ast::aggregates::RecordAggregate::*;
use triangle_rs::ast::aggregates::*;
use triangle_rs::ast::commands::Command::*;
use triangle_rs::ast::commands::*;
use triangle_rs::ast::declarations::Declaration::*;
use triangle_rs::ast::declarations::*;
use triangle_rs::ast::expressions::Expression::*;
use triangle_rs::ast::expressions::*;
use triangle_rs::ast::parameters::ActualParameter::*;
use triangle_rs::ast::parameters::ActualParameterSequence::*;
use triangle_rs::ast::parameters::FormalParameter::*;
use triangle_rs::ast::parameters::FormalParameterSequence::*;
use triangle_rs::ast::parameters::*;
use triangle_rs::ast::primitives::*;
use triangle_rs::ast::typedenoters::FieldTypeDenoter::*;
use triangle_rs::ast::typedenoters::TypeDenoter::*;
use triangle_rs::ast::typedenoters::*;
use triangle_rs::ast::vnames::Vname::*;
use triangle_rs::ast::vnames::*;
use triangle_rs::ast::*;
use triangle_rs::parser::*;
use triangle_rs::scanner::*;

#[test]
fn test_emptycommandeot() {
    let source_file = "samples/source/emptycommandeot.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(EmptyCommand(EmptyCommandState::new()));
    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_emptycommandsemicolon() {
    let source_file = "samples/source/emptycommandsemicolon.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(EmptyCommand(EmptyCommandState::new()));
    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_hello() {
    let source_file = "samples/source/hello.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(CallCommand(CallCommandState::new(
        Identifier::new("putint"),
        SingleActualParameterSequence(SingleActualParameterSequenceState::new(
            ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                IntegerExpressionState::new(IntegerLiteral::new("42")),
            ))),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_inc() {
    let source_file = "samples/source/inc.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("x"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
            )),
            ProcDeclaration(ProcDeclarationState::new(
                Identifier::new("inc"),
                SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                    VarFormalParameter(VarFormalParameterState::new(
                        Identifier::new("n"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                )),
                AssignCommand(AssignCommandState::new(
                    SimpleVname(SimpleVnameState::new(Identifier::new("n"))),
                    BinaryExpression(BinaryExpressionState::new(
                        VnameExpression(VnameExpressionState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("n")),
                        ))),
                        Operator::new("+"),
                        IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))),
                    )),
                )),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            SequentialCommand(SequentialCommandState::new(
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("x")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("inc"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("x")),
                        ))),
                    )),
                )),
            )),
            CallCommand(CallCommandState::new(
                Identifier::new("putint"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                        VnameExpressionState::new(SimpleVname(SimpleVnameState::new(
                            Identifier::new("x"),
                        ))),
                    ))),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_echo() {
    let source_file = "samples/source/echo.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("ch"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Char"))),
            )),
            ProcDeclaration(ProcDeclarationState::new(
                Identifier::new("echo"),
                EmptyFormalParameterSequence(EmptyFormalParameterSequenceState::new()),
                WhileCommand(WhileCommandState::new(
                    UnaryExpression(UnaryExpressionState::new(
                        Operator::new("\\"),
                        CallExpression(CallExpressionState::new(
                            Identifier::new("eol"),
                            EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()),
                        )),
                    )),
                    SequentialCommand(SequentialCommandState::new(
                        CallCommand(CallCommandState::new(
                            Identifier::new("get"),
                            SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                                VarActualParameter(VarActualParameterState::new(SimpleVname(
                                    SimpleVnameState::new(Identifier::new("ch")),
                                ))),
                            )),
                        )),
                        CallCommand(CallCommandState::new(
                            Identifier::new("put"),
                            SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                                ConstActualParameter(ConstActualParameterState::new(
                                    VnameExpression(VnameExpressionState::new(SimpleVname(
                                        SimpleVnameState::new(Identifier::new("ch")),
                                    ))),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )),
        CallCommand(CallCommandState::new(
            Identifier::new("echo"),
            EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_odd() {
    let source_file = "samples/source/odd.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("n"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
            )),
            FuncDeclaration(FuncDeclarationState::new(
                Identifier::new("odd"),
                SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                    ConstFormalParameter(ConstFormalParameterState::new(
                        Identifier::new("n"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                )),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Boolean"))),
                BinaryExpression(BinaryExpressionState::new(
                    BinaryExpression(BinaryExpressionState::new(
                        VnameExpression(VnameExpressionState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("n")),
                        ))),
                        Operator::new("//"),
                        IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("2"))),
                    )),
                    Operator::new("\\="),
                    IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))),
                )),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            CallCommand(CallCommandState::new(
                Identifier::new("getint"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    VarActualParameter(VarActualParameterState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("n")),
                    ))),
                )),
            )),
            IfCommand(IfCommandState::new(
                CallExpression(CallExpressionState::new(
                    Identifier::new("odd"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                            VnameExpressionState::new(SimpleVname(SimpleVnameState::new(
                                Identifier::new("n"),
                            ))),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("1")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("2")),
                        ))),
                    )),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
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
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            SequentialDeclaration(SequentialDeclarationState::new(
                VarDeclaration(VarDeclarationState::new(
                    Identifier::new("m"),
                    SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                )),
                VarDeclaration(VarDeclarationState::new(
                    Identifier::new("n"),
                    SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                )),
            )),
            FuncDeclaration(FuncDeclarationState::new(
                Identifier::new("power"),
                MultipleFormalParameterSequence(MultipleFormalParameterSequenceState::new(
                    ConstFormalParameter(ConstFormalParameterState::new(
                        Identifier::new("a"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                    SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                        ConstFormalParameter(ConstFormalParameterState::new(
                            Identifier::new("b"),
                            SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                "Integer",
                            ))),
                        )),
                    )),
                )),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                IfExpression(IfExpressionState::new(
                    BinaryExpression(BinaryExpressionState::new(
                        VnameExpression(VnameExpressionState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("b")),
                        ))),
                        Operator::new("="),
                        IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))),
                    )),
                    IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))),
                    BinaryExpression(BinaryExpressionState::new(
                        VnameExpression(VnameExpressionState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("a")),
                        ))),
                        Operator::new("*"),
                        CallExpression(CallExpressionState::new(
                            Identifier::new("power"),
                            MultipleActualParameterSequence(
                                MultipleActualParameterSequenceState::new(
                                    ConstActualParameter(ConstActualParameterState::new(
                                        VnameExpression(VnameExpressionState::new(SimpleVname(
                                            SimpleVnameState::new(Identifier::new("a")),
                                        ))),
                                    )),
                                    SingleActualParameterSequence(
                                        SingleActualParameterSequenceState::new(
                                            ConstActualParameter(ConstActualParameterState::new(
                                                BinaryExpression(BinaryExpressionState::new(
                                                    VnameExpression(VnameExpressionState::new(
                                                        SimpleVname(SimpleVnameState::new(
                                                            Identifier::new("b"),
                                                        )),
                                                    )),
                                                    Operator::new("-"),
                                                    IntegerExpression(IntegerExpressionState::new(
                                                        IntegerLiteral::new("1"),
                                                    )),
                                                )),
                                            )),
                                        ),
                                    ),
                                ),
                            ),
                        )),
                    )),
                )),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            SequentialCommand(SequentialCommandState::new(
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("m")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("n")),
                        ))),
                    )),
                )),
            )),
            CallCommand(CallCommandState::new(
                Identifier::new("putint"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    ConstActualParameter(ConstActualParameterState::new(CallExpression(
                        CallExpressionState::new(
                            Identifier::new("power"),
                            MultipleActualParameterSequence(
                                MultipleActualParameterSequenceState::new(
                                    ConstActualParameter(ConstActualParameterState::new(
                                        VnameExpression(VnameExpressionState::new(SimpleVname(
                                            SimpleVnameState::new(Identifier::new("m")),
                                        ))),
                                    )),
                                    SingleActualParameterSequence(
                                        SingleActualParameterSequenceState::new(
                                            ConstActualParameter(ConstActualParameterState::new(
                                                VnameExpression(VnameExpressionState::new(
                                                    SimpleVname(SimpleVnameState::new(
                                                        Identifier::new("n"),
                                                    )),
                                                )),
                                            )),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ))),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_factorial() {
    let source_file = "samples/source/factorial.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            SequentialDeclaration(SequentialDeclarationState::new(
                VarDeclaration(VarDeclarationState::new(
                    Identifier::new("n"),
                    SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                )),
                FuncDeclaration(FuncDeclarationState::new(
                    Identifier::new("factorial"),
                    SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                        ConstFormalParameter(ConstFormalParameterState::new(
                            Identifier::new("n"),
                            SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                "Integer",
                            ))),
                        )),
                    )),
                    SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    IfExpression(IfExpressionState::new(
                        BinaryExpression(BinaryExpressionState::new(
                            VnameExpression(VnameExpressionState::new(SimpleVname(
                                SimpleVnameState::new(Identifier::new("n")),
                            ))),
                            Operator::new("<="),
                            IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(
                                "0",
                            ))),
                        )),
                        IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))),
                        BinaryExpression(BinaryExpressionState::new(
                            VnameExpression(VnameExpressionState::new(SimpleVname(
                                SimpleVnameState::new(Identifier::new("n")),
                            ))),
                            Operator::new("*"),
                            CallExpression(CallExpressionState::new(
                                Identifier::new("factorial"),
                                SingleActualParameterSequence(
                                    SingleActualParameterSequenceState::new(ConstActualParameter(
                                        ConstActualParameterState::new(BinaryExpression(
                                            BinaryExpressionState::new(
                                                VnameExpression(VnameExpressionState::new(
                                                    SimpleVname(SimpleVnameState::new(
                                                        Identifier::new("n"),
                                                    )),
                                                )),
                                                Operator::new("-"),
                                                IntegerExpression(IntegerExpressionState::new(
                                                    IntegerLiteral::new("1"),
                                                )),
                                            ),
                                        )),
                                    )),
                                ),
                            )),
                        )),
                    )),
                )),
            )),
            ProcDeclaration(ProcDeclarationState::new(
                Identifier::new("readnumber"),
                SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                    VarFormalParameter(VarFormalParameterState::new(
                        Identifier::new("n"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("n")),
                        ))),
                    )),
                )),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            SequentialCommand(SequentialCommandState::new(
                SequentialCommand(SequentialCommandState::new(
                    CallCommand(CallCommandState::new(
                        Identifier::new("readnumber"),
                        SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                            VarActualParameter(VarActualParameterState::new(SimpleVname(
                                SimpleVnameState::new(Identifier::new("n")),
                            ))),
                        )),
                    )),
                    CallCommand(CallCommandState::new(
                        Identifier::new("puteol"),
                        EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("puteol"),
                    EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()),
                )),
            )),
            CallCommand(CallCommandState::new(
                Identifier::new("putint"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    ConstActualParameter(ConstActualParameterState::new(CallExpression(
                        CallExpressionState::new(
                            Identifier::new("factorial"),
                            SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                                ConstActualParameter(ConstActualParameterState::new(
                                    VnameExpression(VnameExpressionState::new(SimpleVname(
                                        SimpleVnameState::new(Identifier::new("n")),
                                    ))),
                                )),
                            )),
                        ),
                    ))),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_record() {
    let source_file = "samples/source/record.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            SequentialDeclaration(SequentialDeclarationState::new(
                TypeDeclaration(TypeDeclarationState::new(
                    Identifier::new("Date"),
                    RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(
                        MultipleFieldTypeDenoterState::new(
                            Identifier::new("y"),
                            SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                "Integer",
                            ))),
                            MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(
                                Identifier::new("m"),
                                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                    "Integer",
                                ))),
                                SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(
                                    Identifier::new("d"),
                                    SimpleTypeDenoter(SimpleTypeDenoterState::new(
                                        Identifier::new("Integer"),
                                    )),
                                )),
                            )),
                        ),
                    ))),
                )),
                VarDeclaration(VarDeclarationState::new(
                    Identifier::new("today"),
                    SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))),
                )),
            )),
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("tomorrow"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            SequentialCommand(SequentialCommandState::new(
                SequentialCommand(SequentialCommandState::new(
                    SequentialCommand(SequentialCommandState::new(
                        SequentialCommand(SequentialCommandState::new(
                            SequentialCommand(SequentialCommandState::new(
                                SequentialCommand(SequentialCommandState::new(
                                    AssignCommand(AssignCommandState::new(
                                        SimpleVname(SimpleVnameState::new(Identifier::new(
                                            "today",
                                        ))),
                                        RecordExpression(RecordExpressionState::new(
                                            MultipleRecordAggregate(
                                                MultipleRecordAggregateState::new(
                                                    Identifier::new("y"),
                                                    IntegerExpression(IntegerExpressionState::new(
                                                        IntegerLiteral::new("2021"),
                                                    )),
                                                    MultipleRecordAggregate(
                                                        MultipleRecordAggregateState::new(
                                                            Identifier::new("m"),
                                                            IntegerExpression(
                                                                IntegerExpressionState::new(
                                                                    IntegerLiteral::new("1"),
                                                                ),
                                                            ),
                                                            SingleRecordAggregate(
                                                                SingleRecordAggregateState::new(
                                                                    Identifier::new("d"),
                                                                    IntegerExpression(
                                                                        IntegerExpressionState::new(
                                                                            IntegerLiteral::new(
                                                                                "12",
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        )),
                                    )),
                                    CallCommand(CallCommandState::new(
                                        Identifier::new("putint"),
                                        SingleActualParameterSequence(
                                            SingleActualParameterSequenceState::new(
                                                ConstActualParameter(
                                                    ConstActualParameterState::new(
                                                        VnameExpression(VnameExpressionState::new(
                                                            DotVname(DotVnameState::new(
                                                                SimpleVname(SimpleVnameState::new(
                                                                    Identifier::new("today"),
                                                                )),
                                                                Identifier::new("y"),
                                                            )),
                                                        )),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    )),
                                )),
                                CallCommand(CallCommandState::new(
                                    Identifier::new("putint"),
                                    SingleActualParameterSequence(
                                        SingleActualParameterSequenceState::new(
                                            ConstActualParameter(ConstActualParameterState::new(
                                                VnameExpression(VnameExpressionState::new(
                                                    DotVname(DotVnameState::new(
                                                        SimpleVname(SimpleVnameState::new(
                                                            Identifier::new("today"),
                                                        )),
                                                        Identifier::new("m"),
                                                    )),
                                                )),
                                            )),
                                        ),
                                    ),
                                )),
                            )),
                            CallCommand(CallCommandState::new(
                                Identifier::new("putint"),
                                SingleActualParameterSequence(
                                    SingleActualParameterSequenceState::new(ConstActualParameter(
                                        ConstActualParameterState::new(VnameExpression(
                                            VnameExpressionState::new(DotVname(
                                                DotVnameState::new(
                                                    SimpleVname(SimpleVnameState::new(
                                                        Identifier::new("today"),
                                                    )),
                                                    Identifier::new("d"),
                                                ),
                                            )),
                                        )),
                                    )),
                                ),
                            )),
                        )),
                        AssignCommand(AssignCommandState::new(
                            SimpleVname(SimpleVnameState::new(Identifier::new("tomorrow"))),
                            RecordExpression(RecordExpressionState::new(MultipleRecordAggregate(
                                MultipleRecordAggregateState::new(
                                    Identifier::new("y"),
                                    VnameExpression(VnameExpressionState::new(DotVname(
                                        DotVnameState::new(
                                            SimpleVname(SimpleVnameState::new(Identifier::new(
                                                "today",
                                            ))),
                                            Identifier::new("y"),
                                        ),
                                    ))),
                                    MultipleRecordAggregate(MultipleRecordAggregateState::new(
                                        Identifier::new("m"),
                                        VnameExpression(VnameExpressionState::new(DotVname(
                                            DotVnameState::new(
                                                SimpleVname(SimpleVnameState::new(
                                                    Identifier::new("today"),
                                                )),
                                                Identifier::new("m"),
                                            ),
                                        ))),
                                        SingleRecordAggregate(SingleRecordAggregateState::new(
                                            Identifier::new("d"),
                                            BinaryExpression(BinaryExpressionState::new(
                                                VnameExpression(VnameExpressionState::new(
                                                    DotVname(DotVnameState::new(
                                                        SimpleVname(SimpleVnameState::new(
                                                            Identifier::new("today"),
                                                        )),
                                                        Identifier::new("d"),
                                                    )),
                                                )),
                                                Operator::new("+"),
                                                IntegerExpression(IntegerExpressionState::new(
                                                    IntegerLiteral::new("1"),
                                                )),
                                            )),
                                        )),
                                    )),
                                ),
                            ))),
                        )),
                    )),
                    CallCommand(CallCommandState::new(
                        Identifier::new("putint"),
                        SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                            ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                                VnameExpressionState::new(DotVname(DotVnameState::new(
                                    SimpleVname(SimpleVnameState::new(Identifier::new("tomorrow"))),
                                    Identifier::new("y"),
                                ))),
                            ))),
                        )),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                            VnameExpressionState::new(DotVname(DotVnameState::new(
                                SimpleVname(SimpleVnameState::new(Identifier::new("tomorrow"))),
                                Identifier::new("m"),
                            ))),
                        ))),
                    )),
                )),
            )),
            CallCommand(CallCommandState::new(
                Identifier::new("putint"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                        VnameExpressionState::new(DotVname(DotVnameState::new(
                            SimpleVname(SimpleVnameState::new(Identifier::new("tomorrow"))),
                            Identifier::new("d"),
                        ))),
                    ))),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_leapyear() {
    let source_file = "samples/source/leapyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            SequentialDeclaration(SequentialDeclarationState::new(
                SequentialDeclaration(SequentialDeclarationState::new(
                    TypeDeclaration(TypeDeclarationState::new(
                        Identifier::new("Date"),
                        RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(
                            MultipleFieldTypeDenoterState::new(
                                Identifier::new("y"),
                                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                    "Integer",
                                ))),
                                MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(
                                    Identifier::new("m"),
                                    SimpleTypeDenoter(SimpleTypeDenoterState::new(
                                        Identifier::new("Integer"),
                                    )),
                                    SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(
                                        Identifier::new("d"),
                                        SimpleTypeDenoter(SimpleTypeDenoterState::new(
                                            Identifier::new("Integer"),
                                        )),
                                    )),
                                )),
                            ),
                        ))),
                    )),
                    VarDeclaration(VarDeclarationState::new(
                        Identifier::new("d"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))),
                    )),
                )),
                ProcDeclaration(ProcDeclarationState::new(
                    Identifier::new("readdate"),
                    SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                        VarFormalParameter(VarFormalParameterState::new(
                            Identifier::new("d"),
                            SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))),
                        )),
                    )),
                    SequentialCommand(SequentialCommandState::new(
                        SequentialCommand(SequentialCommandState::new(
                            CallCommand(CallCommandState::new(
                                Identifier::new("getint"),
                                SingleActualParameterSequence(
                                    SingleActualParameterSequenceState::new(VarActualParameter(
                                        VarActualParameterState::new(DotVname(DotVnameState::new(
                                            SimpleVname(SimpleVnameState::new(Identifier::new(
                                                "d",
                                            ))),
                                            Identifier::new("y"),
                                        ))),
                                    )),
                                ),
                            )),
                            CallCommand(CallCommandState::new(
                                Identifier::new("getint"),
                                SingleActualParameterSequence(
                                    SingleActualParameterSequenceState::new(VarActualParameter(
                                        VarActualParameterState::new(DotVname(DotVnameState::new(
                                            SimpleVname(SimpleVnameState::new(Identifier::new(
                                                "d",
                                            ))),
                                            Identifier::new("m"),
                                        ))),
                                    )),
                                ),
                            )),
                        )),
                        CallCommand(CallCommandState::new(
                            Identifier::new("getint"),
                            SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                                VarActualParameter(VarActualParameterState::new(DotVname(
                                    DotVnameState::new(
                                        SimpleVname(SimpleVnameState::new(Identifier::new("d"))),
                                        Identifier::new("d"),
                                    ),
                                ))),
                            )),
                        )),
                    )),
                )),
            )),
            FuncDeclaration(FuncDeclarationState::new(
                Identifier::new("leapyear"),
                SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                    ConstFormalParameter(ConstFormalParameterState::new(
                        Identifier::new("d"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))),
                    )),
                )),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Boolean"))),
                IfExpression(IfExpressionState::new(
                    BinaryExpression(BinaryExpressionState::new(
                        BinaryExpression(BinaryExpressionState::new(
                            BinaryExpression(BinaryExpressionState::new(
                                BinaryExpression(BinaryExpressionState::new(
                                    VnameExpression(VnameExpressionState::new(DotVname(
                                        DotVnameState::new(
                                            SimpleVname(SimpleVnameState::new(Identifier::new(
                                                "d",
                                            ))),
                                            Identifier::new("y"),
                                        ),
                                    ))),
                                    Operator::new("//"),
                                    IntegerExpression(IntegerExpressionState::new(
                                        IntegerLiteral::new("100"),
                                    )),
                                )),
                                Operator::new("="),
                                IntegerExpression(IntegerExpressionState::new(
                                    IntegerLiteral::new("0"),
                                )),
                            )),
                            Operator::new("/\\"),
                            BinaryExpression(BinaryExpressionState::new(
                                BinaryExpression(BinaryExpressionState::new(
                                    VnameExpression(VnameExpressionState::new(DotVname(
                                        DotVnameState::new(
                                            SimpleVname(SimpleVnameState::new(Identifier::new(
                                                "d",
                                            ))),
                                            Identifier::new("y"),
                                        ),
                                    ))),
                                    Operator::new("//"),
                                    IntegerExpression(IntegerExpressionState::new(
                                        IntegerLiteral::new("400"),
                                    )),
                                )),
                                Operator::new("="),
                                IntegerExpression(IntegerExpressionState::new(
                                    IntegerLiteral::new("0"),
                                )),
                            )),
                        )),
                        Operator::new("\\/"),
                        BinaryExpression(BinaryExpressionState::new(
                            BinaryExpression(BinaryExpressionState::new(
                                VnameExpression(VnameExpressionState::new(DotVname(
                                    DotVnameState::new(
                                        SimpleVname(SimpleVnameState::new(Identifier::new("d"))),
                                        Identifier::new("y"),
                                    ),
                                ))),
                                Operator::new("//"),
                                IntegerExpression(IntegerExpressionState::new(
                                    IntegerLiteral::new("4"),
                                )),
                            )),
                            Operator::new("="),
                            IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(
                                "0",
                            ))),
                        )),
                    )),
                    VnameExpression(VnameExpressionState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("true")),
                    ))),
                    VnameExpression(VnameExpressionState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("false")),
                    ))),
                )),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            CallCommand(CallCommandState::new(
                Identifier::new("readdate"),
                SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                    VarActualParameter(VarActualParameterState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("d")),
                    ))),
                )),
            )),
            IfCommand(IfCommandState::new(
                CallExpression(CallExpressionState::new(
                    Identifier::new("leapyear"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                            VnameExpressionState::new(SimpleVname(SimpleVnameState::new(
                                Identifier::new("d"),
                            ))),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("1")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("0")),
                        ))),
                    )),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_date() {
    let source_file = "samples/source/date.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    // rustfmt does not work here, apparently due to the length of the value.
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("Date"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("y"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("m"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("displaydate"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("y"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("getdate"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("y"))))))))), CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))))))))), CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))))))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getdate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))))))))), CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), RecordExpression(RecordExpressionState::new(MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("y"), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("y"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))), MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("m"), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))), SingleRecordAggregate(SingleRecordAggregateState::new(Identifier::new("d"), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))), CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))))))))))))), EmptyCommand(EmptyCommandState::new()))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_print_array() {
    let source_file = "samples/source/print_array.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            SequentialDeclaration(SequentialDeclarationState::new(
                VarDeclaration(VarDeclarationState::new(
                    Identifier::new("arr"),
                    ArrayTypeDenoter(ArrayTypeDenoterState::new(
                        IntegerLiteral::new("5"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                )),
                ProcDeclaration(ProcDeclarationState::new(
                    Identifier::new("println"),
                    SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                        ConstFormalParameter(ConstFormalParameterState::new(
                            Identifier::new("x"),
                            SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                "Integer",
                            ))),
                        )),
                    )),
                    SequentialCommand(SequentialCommandState::new(
                        CallCommand(CallCommandState::new(
                            Identifier::new("putint"),
                            SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                                ConstActualParameter(ConstActualParameterState::new(
                                    VnameExpression(VnameExpressionState::new(SimpleVname(
                                        SimpleVnameState::new(Identifier::new("x")),
                                    ))),
                                )),
                            )),
                        )),
                        CallCommand(CallCommandState::new(
                            Identifier::new("puteol"),
                            EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()),
                        )),
                    )),
                )),
            )),
            ProcDeclaration(ProcDeclarationState::new(
                Identifier::new("iterate"),
                MultipleFormalParameterSequence(MultipleFormalParameterSequenceState::new(
                    ProcFormalParameter(ProcFormalParameterState::new(
                        Identifier::new("f"),
                        SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                            ConstFormalParameter(ConstFormalParameterState::new(
                                Identifier::new("n"),
                                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                    "Integer",
                                ))),
                            )),
                        )),
                    )),
                    SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(
                        ConstFormalParameter(ConstFormalParameterState::new(
                            Identifier::new("arr"),
                            ArrayTypeDenoter(ArrayTypeDenoterState::new(
                                IntegerLiteral::new("5"),
                                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new(
                                    "Integer",
                                ))),
                            )),
                        )),
                    )),
                )),
                LetCommand(LetCommandState::new(
                    VarDeclaration(VarDeclarationState::new(
                        Identifier::new("i"),
                        SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
                    )),
                    SequentialCommand(SequentialCommandState::new(
                        AssignCommand(AssignCommandState::new(
                            SimpleVname(SimpleVnameState::new(Identifier::new("i"))),
                            IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(
                                "0",
                            ))),
                        )),
                        WhileCommand(WhileCommandState::new(
                            BinaryExpression(BinaryExpressionState::new(
                                VnameExpression(VnameExpressionState::new(SimpleVname(
                                    SimpleVnameState::new(Identifier::new("i")),
                                ))),
                                Operator::new("<"),
                                IntegerExpression(IntegerExpressionState::new(
                                    IntegerLiteral::new("5"),
                                )),
                            )),
                            SequentialCommand(SequentialCommandState::new(
                                CallCommand(CallCommandState::new(
                                    Identifier::new("f"),
                                    SingleActualParameterSequence(
                                        SingleActualParameterSequenceState::new(
                                            ConstActualParameter(ConstActualParameterState::new(
                                                VnameExpression(VnameExpressionState::new(
                                                    SubscriptVname(SubscriptVnameState::new(
                                                        SimpleVname(SimpleVnameState::new(
                                                            Identifier::new("arr"),
                                                        )),
                                                        VnameExpression(VnameExpressionState::new(
                                                            SimpleVname(SimpleVnameState::new(
                                                                Identifier::new("i"),
                                                            )),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                        ),
                                    ),
                                )),
                                AssignCommand(AssignCommandState::new(
                                    SimpleVname(SimpleVnameState::new(Identifier::new("i"))),
                                    BinaryExpression(BinaryExpressionState::new(
                                        VnameExpression(VnameExpressionState::new(SimpleVname(
                                            SimpleVnameState::new(Identifier::new("i")),
                                        ))),
                                        Operator::new("+"),
                                        IntegerExpression(IntegerExpressionState::new(
                                            IntegerLiteral::new("1"),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )),
        LetCommand(LetCommandState::new(
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("i"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
            )),
            SequentialCommand(SequentialCommandState::new(
                SequentialCommand(SequentialCommandState::new(
                    AssignCommand(AssignCommandState::new(
                        SimpleVname(SimpleVnameState::new(Identifier::new("i"))),
                        IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))),
                    )),
                    WhileCommand(WhileCommandState::new(
                        BinaryExpression(BinaryExpressionState::new(
                            VnameExpression(VnameExpressionState::new(SimpleVname(
                                SimpleVnameState::new(Identifier::new("i")),
                            ))),
                            Operator::new("<"),
                            IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new(
                                "5",
                            ))),
                        )),
                        SequentialCommand(SequentialCommandState::new(
                            AssignCommand(AssignCommandState::new(
                                SubscriptVname(SubscriptVnameState::new(
                                    SimpleVname(SimpleVnameState::new(Identifier::new("arr"))),
                                    VnameExpression(VnameExpressionState::new(SimpleVname(
                                        SimpleVnameState::new(Identifier::new("i")),
                                    ))),
                                )),
                                BinaryExpression(BinaryExpressionState::new(
                                    BinaryExpression(BinaryExpressionState::new(
                                        VnameExpression(VnameExpressionState::new(SimpleVname(
                                            SimpleVnameState::new(Identifier::new("i")),
                                        ))),
                                        Operator::new("+"),
                                        IntegerExpression(IntegerExpressionState::new(
                                            IntegerLiteral::new("1"),
                                        )),
                                    )),
                                    Operator::new("*"),
                                    IntegerExpression(IntegerExpressionState::new(
                                        IntegerLiteral::new("100"),
                                    )),
                                )),
                            )),
                            AssignCommand(AssignCommandState::new(
                                SimpleVname(SimpleVnameState::new(Identifier::new("i"))),
                                BinaryExpression(BinaryExpressionState::new(
                                    VnameExpression(VnameExpressionState::new(SimpleVname(
                                        SimpleVnameState::new(Identifier::new("i")),
                                    ))),
                                    Operator::new("+"),
                                    IntegerExpression(IntegerExpressionState::new(
                                        IntegerLiteral::new("1"),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("iterate"),
                    MultipleActualParameterSequence(MultipleActualParameterSequenceState::new(
                        ProcActualParameter(ProcActualParameterState::new(Identifier::new(
                            "println",
                        ))),
                        SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                            ConstActualParameter(ConstActualParameterState::new(VnameExpression(
                                VnameExpressionState::new(SimpleVname(SimpleVnameState::new(
                                    Identifier::new("arr"),
                                ))),
                            ))),
                        )),
                    )),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_string() {
    let source_file = "samples/source/string.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("String"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("buf"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("100"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Char"))))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("idx"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("displaystring"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("s"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("String"))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("idx"))))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("buf"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))), EmptyCommand(EmptyCommandState::new()))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("readstring"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("s"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("String"))))))), WhileCommand(WhileCommandState::new(UnaryExpression(UnaryExpressionState::new(Operator::new("\\"), CallExpression(CallExpressionState::new(Identifier::new("eol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("get"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SubscriptVname(SubscriptVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("buf"))), VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("idx"))))))))))))), AssignCommand(AssignCommandState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("idx"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))), Identifier::new("idx"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))), EmptyCommand(EmptyCommandState::new()))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("s"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("String"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("readstring"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))))))))), CallCommand(CallCommandState::new(Identifier::new("displaystring"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("s"))))))))))))))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
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
    let expected_program = 
        Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(ProcDeclaration(ProcDeclarationState::new(Identifier::new("iteratively"), MultipleFormalParameterSequence(MultipleFormalParameterSequenceState::new(ProcFormalParameter(ProcFormalParameterState::new(Identifier::new("p"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("n"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("arr"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("10"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("10"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("p"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("arr"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("readnums"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("a"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("10"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("10"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("a"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("putintln"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("n"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("n"))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("a"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("10"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("readnums"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("a"))))))))), CallCommand(CallCommandState::new(Identifier::new("iteratively"), MultipleActualParameterSequence(MultipleActualParameterSequenceState::new(ProcActualParameter(ProcActualParameterState::new(Identifier::new("putint"))), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("a"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), CallCommand(CallCommandState::new(Identifier::new("iteratively"), MultipleActualParameterSequence(MultipleActualParameterSequenceState::new(ProcActualParameter(ProcActualParameterState::new(Identifier::new("putintln"))), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("a"))))))))))))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_line() {
    let source_file = "samples/source/line.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("Line"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("length"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("content"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("80"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Char"))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("getline"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("l"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Line"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(UnaryExpression(UnaryExpressionState::new(Operator::new("\\"), CallExpression(CallExpressionState::new(Identifier::new("eol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("get"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SubscriptVname(SubscriptVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("content"))), VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))))))))))))), AssignCommand(AssignCommandState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))), EmptyCommand(EmptyCommandState::new()))))))), EmptyCommand(EmptyCommandState::new()))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("putline"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("l"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Line"))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("content"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))), EmptyCommand(EmptyCommandState::new()))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("putreversedline"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("l"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Line"))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("length"))))), Operator::new("-"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new(">="), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("l"))), Identifier::new("content"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("-"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))), EmptyCommand(EmptyCommandState::new()))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("currentline"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Line"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getline"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("currentline"))))))))), CallCommand(CallCommandState::new(Identifier::new("putline"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("currentline"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("putreversedline"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("currentline"))))))))))))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_dates() {
    let source_file = "samples/source/dates.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("Date"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("m"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))), ConstDeclaration(ConstDeclarationState::new(Identifier::new("xmas"), RecordExpression(RecordExpressionState::new(MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("m"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("12"))), SingleRecordAggregate(SingleRecordAggregateState::new(Identifier::new("d"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("25"))))))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("easter"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("holidays"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("3"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("displaydate"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new("-"))))))))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("readdate"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))))))), CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("readdates"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("a"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("3"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("3"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("readdate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("a"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("readdate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("easter"))))))))), CallCommand(CallCommandState::new(Identifier::new("readdates"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("holidays"))))))))))), CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("xmas"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("easter"))))))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("3"))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("holidays"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))), CallCommand(CallCommandState::new(Identifier::new("displaydate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("holidays"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))))))))))))), EmptyCommand(EmptyCommandState::new()))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}

#[test]
fn test_monthsofyear() {
    let source_file = "samples/source/monthsofyear.t";
    let mut parser = Parser::new(Scanner::new(source_file));
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("Date"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("y"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("m"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("months"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("12"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("printmonths"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("ms"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("12"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("i"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), WhileCommand(WhileCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("<"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("12"))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SubscriptVname(SubscriptVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("ms"))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))), BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("i"))))), Operator::new("+"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1"))))))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("readdate"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(VarFormalParameter(VarFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("y"))))))))), CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("m"))))))))))), CallCommand(CallCommandState::new(Identifier::new("getint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("d"))))))))))))))), FuncDeclaration(FuncDeclarationState::new(Identifier::new("leap"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("y"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Boolean"))), IfExpression(IfExpressionState::new(BinaryExpression(BinaryExpressionState::new(BinaryExpression(BinaryExpressionState::new(BinaryExpression(BinaryExpressionState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("y"))))), Operator::new("//"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("100"))))), Operator::new("="), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))), Operator::new("/\\"), BinaryExpression(BinaryExpressionState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("y"))))), Operator::new("//"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("400"))))), Operator::new("="), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))))), Operator::new("\\/"), BinaryExpression(BinaryExpressionState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("y"))))), Operator::new("//"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("4"))))), Operator::new("="), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("0"))))))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("true"))))), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("false"))))))))))), FuncDeclaration(FuncDeclarationState::new(Identifier::new("getmonths"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("12"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))), ArrayExpression(ArrayExpressionState::new(MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IfExpression(IfExpressionState::new(CallExpression(CallExpressionState::new(Identifier::new("leap"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))), Identifier::new("y"))))))))))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("29"))), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("28"))))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("30"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("30"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("30"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))), MultipleArrayAggregate(MultipleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("30"))), SingleArrayAggregate(SingleArrayAggregateState::new(IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("31"))))))))))))))))))))))))))))))))), SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("readdate"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(VarActualParameter(VarActualParameterState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))))))))), LetCommand(LetCommandState::new(VarDeclaration(VarDeclarationState::new(Identifier::new("ms"), ArrayTypeDenoter(ArrayTypeDenoterState::new(IntegerLiteral::new("12"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("ms"))), CallExpression(CallExpressionState::new(Identifier::new("getmonths"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("d"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("printmonths"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("ms"))))))))))))))))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
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
    let expected_program = Program::new(LetCommand(LetCommandState::new(
        SequentialDeclaration(SequentialDeclarationState::new(
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("x"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
            )),
            VarDeclaration(VarDeclarationState::new(
                Identifier::new("y"),
                SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))),
            )),
        )),
        SequentialCommand(SequentialCommandState::new(
            SequentialCommand(SequentialCommandState::new(
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("x")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("getint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        VarActualParameter(VarActualParameterState::new(SimpleVname(
                            SimpleVnameState::new(Identifier::new("y")),
                        ))),
                    )),
                )),
            )),
            IfCommand(IfCommandState::new(
                BinaryExpression(BinaryExpressionState::new(
                    VnameExpression(VnameExpressionState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("x")),
                    ))),
                    Operator::new("="),
                    VnameExpression(VnameExpressionState::new(SimpleVname(
                        SimpleVnameState::new(Identifier::new("y")),
                    ))),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("1")),
                        ))),
                    )),
                )),
                CallCommand(CallCommandState::new(
                    Identifier::new("putint"),
                    SingleActualParameterSequence(SingleActualParameterSequenceState::new(
                        ConstActualParameter(ConstActualParameterState::new(IntegerExpression(
                            IntegerExpressionState::new(IntegerLiteral::new("2")),
                        ))),
                    )),
                )),
            )),
        )),
    )));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
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
    let expected_program = 
Program::new(LetCommand(LetCommandState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(SequentialDeclaration(SequentialDeclarationState::new(TypeDeclaration(TypeDeclarationState::new(Identifier::new("Date"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("y"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("m"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("d"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Integer"))))))))))))), TypeDeclaration(TypeDeclarationState::new(Identifier::new("Person"), RecordTypeDenoter(RecordTypeDenoterState::new(MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("initials"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Char"))), MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new(Identifier::new("married"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Boolean"))), SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new(Identifier::new("dob"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Date"))))))))))))))), ProcDeclaration(ProcDeclarationState::new(Identifier::new("displayperson"), SingleFormalParameterSequence(SingleFormalParameterSequenceState::new(ConstFormalParameter(ConstFormalParameterState::new(Identifier::new("p"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Person"))))))), SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(SequentialCommand(SequentialCommandState::new(CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("p"))), Identifier::new("initials"))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), IfCommand(IfCommandState::new(BinaryExpression(BinaryExpressionState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("p"))), Identifier::new("married"))))), Operator::new("="), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("true"))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new("y"))))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new("n"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("puteol"), EmptyActualParameterSequence(EmptyActualParameterSequenceState::new()))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("p"))), Identifier::new("dob"))), Identifier::new("y"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new(" "))))))))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("p"))), Identifier::new("dob"))), Identifier::new("m"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new(" "))))))))))), CallCommand(CallCommandState::new(Identifier::new("putint"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(DotVname(DotVnameState::new(DotVname(DotVnameState::new(SimpleVname(SimpleVnameState::new(Identifier::new("p"))), Identifier::new("dob"))), Identifier::new("d"))))))))))))), CallCommand(CallCommandState::new(Identifier::new("put"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new(" "))))))))))))))), VarDeclaration(VarDeclarationState::new(Identifier::new("bob"), SimpleTypeDenoter(SimpleTypeDenoterState::new(Identifier::new("Person"))))))), SequentialCommand(SequentialCommandState::new(AssignCommand(AssignCommandState::new(SimpleVname(SimpleVnameState::new(Identifier::new("bob"))), RecordExpression(RecordExpressionState::new(MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("initials"), CharacterExpression(CharacterExpressionState::new(CharacterLiteral::new("B"))), MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("married"), VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("false"))))), SingleRecordAggregate(SingleRecordAggregateState::new(Identifier::new("dob"), RecordExpression(RecordExpressionState::new(MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("y"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("1970"))), MultipleRecordAggregate(MultipleRecordAggregateState::new(Identifier::new("m"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("12"))), SingleRecordAggregate(SingleRecordAggregateState::new(Identifier::new("d"), IntegerExpression(IntegerExpressionState::new(IntegerLiteral::new("22"))))))))))))))))))))), CallCommand(CallCommandState::new(Identifier::new("displayperson"), SingleActualParameterSequence(SingleActualParameterSequenceState::new(ConstActualParameter(ConstActualParameterState::new(VnameExpression(VnameExpressionState::new(SimpleVname(SimpleVnameState::new(Identifier::new("bob"))))))))))))))));

    let actual_program = parser.parse_program();
    assert_eq!(expected_program, actual_program);
}
