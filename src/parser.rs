//! The Parser module.
//!
//! This module consumes the stream of tokens produced by the Scanner, and constructs an AST for
//! Triangle, which is then used by all subsequent phases of the compiler.

use crate::ast::aggregates::RecordAggregate::*;
use crate::ast::aggregates::*;
use crate::ast::commands::Command::*;
use crate::ast::commands::*;
use crate::ast::declarations::Declaration::*;
use crate::ast::declarations::*;
use crate::ast::expressions::Expression::*;
use crate::ast::expressions::*;
use crate::ast::parameters::ActualParameter::*;
use crate::ast::parameters::ActualParameterSequence::*;
use crate::ast::parameters::FormalParameter::*;
use crate::ast::parameters::FormalParameterSequence::*;
use crate::ast::parameters::*;
use crate::ast::primitives::*;
use crate::ast::typedenoters::FieldTypeDenoter::*;
use crate::ast::typedenoters::TypeDenoter::*;
use crate::ast::typedenoters::*;
use crate::ast::vnames::Vname::*;
use crate::ast::vnames::*;
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

    fn parse_identifier(&mut self) -> Identifier {
        let id = Identifier::new_with_position(
            &self.current_token.spelling,
            self.current_token.position,
        );
        self.accept_it();
        id
    }

    fn parse_integer_literal(&mut self) -> IntegerLiteral {
        let il = IntegerLiteral::new_with_position(
            &self.current_token.spelling,
            self.current_token.position,
        );
        self.accept_it();
        il
    }

    fn parse_character_literal(&mut self) -> CharacterLiteral {
        let cl = CharacterLiteral::new_with_position(
            &self.current_token.spelling,
            self.current_token.position,
        );
        self.accept_it();
        cl
    }

    fn parse_operator(&mut self) -> Operator {
        let op =
            Operator::new_with_position(&self.current_token.spelling, self.current_token.position);
        self.accept_it();
        op
    }

    ///
    /// Command ::= single-Command | Command ; single-Command
    ///
    fn parse_command(&mut self) -> Command {
        let mut cmd_pos = SourcePosition::default();
        self.start(&mut cmd_pos);
        let mut cmd = self.parse_single_command();
        self.finish(&mut cmd_pos);

        while self.current_token.kind == TokenType::Semicolon {
            self.accept_it();
            let cmd1 = self.parse_single_command();
            self.finish(&mut cmd_pos);
            cmd = Command::SequentialCommand(SequentialCommandState::new_with_position(
                cmd, cmd1, cmd_pos,
            ));
        }

        cmd
    }

    ///
    /// single-Command:: EmptyCommand
    ///                  | AssignCommand
    ///                  | CallCommand
    ///                  | BracketedCommand
    ///                  | IfCommand
    ///                  | LetCommand
    ///                  | WhileCommand
    ///
    fn parse_single_command(&mut self) -> Command {
        let mut cmd_pos = SourcePosition::default();
        self.start(&mut cmd_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();

                if self.current_token.kind == TokenType::LeftParen {
                    self.accept_it();
                    let aps = self.parse_actual_parameter_sequence();
                    self.accept(TokenType::RightParen);
                    self.finish(&mut cmd_pos);
                    CallCommand(CallCommandState::new_with_position(id, aps, cmd_pos))
                } else {
                    let vname = self.parse_vname(id);

                    if self.current_token.kind == TokenType::Becomes {
                        self.accept_it();
                        let expr = self.parse_expression();
                        self.finish(&mut cmd_pos);
                        AssignCommand(AssignCommandState::new_with_position(vname, expr, cmd_pos))
                    } else {
                        error::report_error_and_exit(GenError::from(ParserError::new(
                            &format!(
                                "expected to find `:=`, found `{:?}` instead",
                                self.current_token.spelling
                            ),
                            self.current_token.position,
                        )));
                    }
                }
            }

            TokenType::Let => {
                self.accept_it();
                let decl = self.parse_declaration();
                self.accept(TokenType::In);
                let cmd1 = self.parse_single_command();
                self.finish(&mut cmd_pos);
                LetCommand(LetCommandState::new_with_position(decl, cmd1, cmd_pos))
            }

            TokenType::If => {
                self.accept_it();
                let expr = self.parse_expression();
                self.accept(TokenType::Then);
                let cmd1 = self.parse_single_command();
                self.accept(TokenType::Else);
                let cmd2 = self.parse_single_command();
                self.finish(&mut cmd_pos);
                IfCommand(IfCommandState::new_with_position(expr, cmd1, cmd2, cmd_pos))
            }

            TokenType::While => {
                self.accept_it();
                let expr = self.parse_expression();
                self.accept(TokenType::Do);
                let cmd1 = self.parse_single_command();
                self.finish(&mut cmd_pos);
                WhileCommand(WhileCommandState::new_with_position(expr, cmd1, cmd_pos))
            }

            TokenType::Begin => {
                self.accept_it();
                let cmd = self.parse_command();
                self.accept(TokenType::End);
                cmd
            }

            TokenType::End => {
                self.finish(&mut cmd_pos);
                EmptyCommand(EmptyCommandState::new_with_position(cmd_pos))
            }

            TokenType::Semicolon | TokenType::Eot => {
                self.accept_it();
                self.finish(&mut cmd_pos);
                EmptyCommand(EmptyCommandState::new_with_position(cmd_pos))
            }
            _ => {
                println!(
                    "curr token = {:?}, not implemented or error",
                    self.current_token
                );
                error::report_error_and_exit(GenError::from(ParserError::new(
                    &format!(
                        "a command cannot begin with a {:?}",
                        self.current_token.kind
                    ),
                    self.current_token.position,
                )));
            }
        }
    }

    ///
    /// Declaration ::= single-Declaration
    ///             | Declaration ; single-Declaration
    ///
    fn parse_declaration(&mut self) -> Declaration {
        let mut decl_pos = SourcePosition::default();
        self.start(&mut decl_pos);

        let mut decl = self.parse_single_declaration();
        while self.current_token.kind == TokenType::Semicolon {
            self.accept_it();
            self.finish(&mut decl_pos);
            let decl1 = self.parse_single_declaration();
            decl = SequentialDeclaration(SequentialDeclarationState::new_with_position(
                decl, decl1, decl_pos,
            ));
        }

        decl
    }

    ///
    /// single-Declaration ::= ConstDeclaration
    ///                 | VarDeclaration
    ///                 | ProcDeclaration
    ///                 | FuncDeclaration
    ///                 | TypeDeclaration
    ///
    fn parse_single_declaration(&mut self) -> Declaration {
        let mut decl_pos = SourcePosition::default();
        self.start(&mut decl_pos);

        match self.current_token.kind {
            TokenType::Const => todo!(),
            TokenType::Var => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.finish(&mut decl_pos);
                VarDeclaration(VarDeclarationState::new_with_position(id, td, decl_pos))
            }

            TokenType::Procedure => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::LeftParen);
                let fps = self.parse_formal_parameter_sequence();
                self.accept(TokenType::RightParen);
                self.accept(TokenType::Is);
                let cmd = self.parse_single_command();
                self.finish(&mut decl_pos);
                ProcDeclaration(ProcDeclarationState::new_with_position(
                    id, fps, cmd, decl_pos,
                ))
            }

            TokenType::Function => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::LeftParen);
                let fps = self.parse_formal_parameter_sequence();
                self.accept(TokenType::RightParen);
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.accept(TokenType::Is);
                let expr = self.parse_expression();
                self.finish(&mut decl_pos);
                FuncDeclaration(FuncDeclarationState::new_with_position(
                    id, fps, td, expr, decl_pos,
                ))
            }

            TokenType::Type => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::Is);
                let td = self.parse_type_denoter();
                self.finish(&mut decl_pos);
                TypeDeclaration(TypeDeclarationState::new_with_position(id, td, decl_pos))
            }

            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!("{:?} cannot start a declaration", self.current_token.kind),
                self.current_token.position,
            ))),
        }
    }

    ///
    /// TypeDenoter ::= SimpleTypeDenoter
    ///               | ArrayTypeDenoter
    ///               | RecordTypeDenoter
    ///
    fn parse_type_denoter(&mut self) -> TypeDenoter {
        let mut td_pos = SourcePosition::default();
        self.start(&mut td_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();
                self.finish(&mut td_pos);
                SimpleTypeDenoter(SimpleTypeDenoterState::new_with_position(id, td_pos))
            }

            TokenType::Array => {
                self.accept_it();
                let il = self.parse_integer_literal();
                self.accept(TokenType::Of);
                let td1 = self.parse_type_denoter();
                self.finish(&mut td_pos);
                ArrayTypeDenoter(ArrayTypeDenoterState::new_with_position(il, td1, td_pos))
            }

            TokenType::Record => {
                self.accept(TokenType::Record);
                let ftd = self.parse_field_type_denoter();
                self.accept(TokenType::End);
                self.finish(&mut td_pos);
                RecordTypeDenoter(RecordTypeDenoterState::new_with_position(ftd, td_pos))
            }

            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!("{:?} cannot start a type denoter", self.current_token.kind),
                self.current_token.position,
            ))),
        }
    }

    ///
    /// FieldTypeDenoter ::= SingleFieldTypeDenoter
    ///                    | MultipleFieldTypeDenoter
    ///
    /// SingleFieldTypeDenoter ::= Identifier : Type-Denoter
    /// MultipleFieldTypeDenoter ::= Identifier : Type-Denoter , FieldTypeDenoter
    ///
    fn parse_field_type_denoter(&mut self) -> FieldTypeDenoter {
        let mut ftd_pos = SourcePosition::default();
        self.start(&mut ftd_pos);

        let id = self.parse_identifier();
        self.accept(TokenType::Colon);
        let td = self.parse_type_denoter();

        if self.current_token.kind == TokenType::Comma {
            self.accept_it();
            let ftd = self.parse_field_type_denoter();
            self.finish(&mut ftd_pos);
            MultipleFieldTypeDenoter(MultipleFieldTypeDenoterState::new_with_position(
                id, td, ftd, ftd_pos,
            ))
        } else {
            self.finish(&mut ftd_pos);
            SingleFieldTypeDenoter(SingleFieldTypeDenoterState::new_with_position(
                id, td, ftd_pos,
            ))
        }
    }

    ///
    /// FormalParameterSequence ::= EmptyFormalParameterSequence
    ///                         | SingleFormalParameterSequence
    ///                         | MultipleFormalParameterSequence
    ///
    fn parse_formal_parameter_sequence(&mut self) -> FormalParameterSequence {
        let mut fps_pos = SourcePosition::default();
        self.start(&mut fps_pos);

        if self.current_token.kind == TokenType::RightParen {
            self.finish(&mut fps_pos);
            EmptyFormalParameterSequence(EmptyFormalParameterSequenceState::new_with_position(
                fps_pos,
            ))
        } else {
            let fp = self.parse_formal_parameter();
            if self.current_token.kind == TokenType::Comma {
                self.accept_it();
                let fps = self.parse_formal_parameter_sequence();
                self.finish(&mut fps_pos);
                MultipleFormalParameterSequence(
                    MultipleFormalParameterSequenceState::new_with_position(fp, fps, fps_pos),
                )
            } else {
                self.finish(&mut fps_pos);
                SingleFormalParameterSequence(
                    SingleFormalParameterSequenceState::new_with_position(fp, fps_pos),
                )
            }
        }
    }

    ///
    /// FormalParamater ::= ConstFormalParameter
    ///                  | VarFormalParameter
    ///                  | ProcFormalParameter
    ///                  | FuncFormalParameter
    ///
    fn parse_formal_parameter(&mut self) -> FormalParameter {
        let mut fp_pos = SourcePosition::default();
        self.start(&mut fp_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.finish(&mut fp_pos);
                ConstFormalParameter(ConstFormalParameterState::new_with_position(id, td, fp_pos))
            }

            TokenType::Var => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.finish(&mut fp_pos);
                VarFormalParameter(VarFormalParameterState::new_with_position(id, td, fp_pos))
            }

            TokenType::Procedure => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::LeftParen);
                let fps = self.parse_formal_parameter_sequence();
                self.accept(TokenType::RightParen);
                self.finish(&mut fp_pos);
                ProcFormalParameter(ProcFormalParameterState::new_with_position(id, fps, fp_pos))
            }

            TokenType::Function => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::LeftParen);
                let fps = self.parse_formal_parameter_sequence();
                self.accept(TokenType::RightParen);
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.finish(&mut fp_pos);
                FuncFormalParameter(FuncFormalParameterState::new_with_position(
                    id, fps, td, fp_pos,
                ))
            }

            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!(
                    "{:?} cannot start a formal parameter",
                    self.current_token.kind
                ),
                self.current_token.position,
            ))),
        }
    }

    ///
    /// ActualParameterSequence ::= EmptyActualParameterSequence
    ///                          | SingleActualParameterSequenceState
    ///                          | MultipleActualParameterSequence
    ///
    fn parse_actual_parameter_sequence(&mut self) -> ActualParameterSequence {
        let mut aps_pos = SourcePosition::default();

        self.start(&mut aps_pos);
        if self.current_token.kind == TokenType::RightParen {
            self.finish(&mut aps_pos);
            EmptyActualParameterSequence(EmptyActualParameterSequenceState::new_with_position(
                aps_pos,
            ))
        } else {
            let ap = self.parse_actual_parameter();
            if self.current_token.kind == TokenType::Comma {
                self.accept_it();
                let aps = self.parse_actual_parameter_sequence();
                self.finish(&mut aps_pos);
                MultipleActualParameterSequence(
                    MultipleActualParameterSequenceState::new_with_position(ap, aps, aps_pos),
                )
            } else {
                self.finish(&mut aps_pos);
                SingleActualParameterSequence(
                    SingleActualParameterSequenceState::new_with_position(ap, aps_pos),
                )
            }
        }
    }

    ///
    /// ActualParameter ::= ConstActualParameter
    ///                 | VarActualParameter
    ///                 | ProcActualParameter
    ///                 | FuncActualParameter
    ///
    fn parse_actual_parameter(&mut self) -> ActualParameter {
        let mut ap_pos = SourcePosition::default();
        self.start(&mut ap_pos);

        match self.current_token.kind {
            TokenType::Var => {
                self.accept_it();
                let id = self.parse_identifier();
                let vname = self.parse_vname(id);
                self.finish(&mut ap_pos);
                VarActualParameter(VarActualParameterState::new_with_position(vname, ap_pos))
            }

            TokenType::Procedure => {
                self.accept_it();
                let id = self.parse_identifier();
                self.finish(&mut ap_pos);
                ProcActualParameter(ProcActualParameterState::new_with_position(id, ap_pos))
            }

            TokenType::Function => {
                self.accept_it();
                let id = self.parse_identifier();
                self.finish(&mut ap_pos);
                FuncActualParameter(FuncActualParameterState::new_with_position(id, ap_pos))
            }

            _ => {
                let expr = self.parse_expression();
                self.finish(&mut ap_pos);
                ConstActualParameter(ConstActualParameterState::new_with_position(expr, ap_pos))
            }
        }
    }

    ///
    /// Expression :: = secondary-Expression
    ///             | LetExpression
    ///             | IfExpression
    ///
    fn parse_expression(&mut self) -> Expression {
        let mut expr_pos = SourcePosition::default();

        match self.current_token.kind {
            TokenType::If => {
                self.start(&mut expr_pos);
                self.accept_it();
                let expr1 = self.parse_expression();
                self.accept(TokenType::Then);
                let expr2 = self.parse_expression();
                self.accept(TokenType::Else);
                let expr3 = self.parse_expression();
                IfExpression(IfExpressionState::new_with_position(
                    expr1, expr2, expr3, expr_pos,
                ))
            }

            TokenType::Let => {
                todo!()
            }

            _ => self.parse_secondary_expression(),
        }
    }

    ///
    /// secondary-Expression ::= primary-Expression
    ///                     | primary-Expression Operator secondary-Expression
    ///
    fn parse_secondary_expression(&mut self) -> Expression {
        let mut expr_pos = SourcePosition::default();
        self.start(&mut expr_pos);

        let mut expr = self.parse_primary_expression();
        while self.current_token.kind == TokenType::Operator {
            let op = self.parse_operator();
            self.finish(&mut expr_pos);
            let expr1 = self.parse_primary_expression();
            expr = BinaryExpression(BinaryExpressionState::new_with_position(
                expr, op, expr1, expr_pos,
            ));
        }

        expr
    }

    ///
    /// primary-Expression ::= IntegerExpression
    ///                     | CharacterExpression
    ///                     | VnameExpression
    ///                     | CallExpression
    ///                     | UnaryExpression
    ///                     | ParenthesisedExpression
    ///                     | ArrayExpression
    ///                     | RecordExpression
    ///
    ///
    fn parse_primary_expression(&mut self) -> Expression {
        let mut expr_pos = SourcePosition::default();
        self.start(&mut expr_pos);

        match self.current_token.kind {
            TokenType::IntegerLiteral => {
                let il = self.parse_integer_literal();
                self.finish(&mut expr_pos);
                IntegerExpression(IntegerExpressionState::new_with_position(il, expr_pos))
            }

            TokenType::CharacterLiteral => {
                let cl = self.parse_character_literal();
                self.finish(&mut expr_pos);
                CharacterExpression(CharacterExpressionState::new_with_position(cl, expr_pos))
            }

            TokenType::Identifier => {
                let id = self.parse_identifier();
                if self.current_token.kind == TokenType::LeftParen {
                    self.accept_it();
                    let aps = self.parse_actual_parameter_sequence();
                    self.accept(TokenType::RightParen);
                    self.finish(&mut expr_pos);
                    CallExpression(CallExpressionState::new_with_position(id, aps, expr_pos))
                } else {
                    let vname = self.parse_vname(id);
                    self.finish(&mut expr_pos);
                    VnameExpression(VnameExpressionState::new_with_position(vname, expr_pos))
                }
            }

            TokenType::Operator => {
                let op = self.parse_operator();
                let expr1 = self.parse_secondary_expression();
                self.finish(&mut expr_pos);
                UnaryExpression(UnaryExpressionState::new_with_position(op, expr1, expr_pos))
            }

            TokenType::LeftParen => {
                self.accept_it();
                let expr = self.parse_expression();
                self.accept(TokenType::RightParen);
                expr
            }

            TokenType::LeftSquareBracket => {
                todo!()
            }

            TokenType::LeftCurlyBracket => {
                self.accept_it();
                let ra = self.parse_record_aggregate();
                self.accept(TokenType::RightCurlyBracket);
                self.finish(&mut expr_pos);
                RecordExpression(RecordExpressionState::new_with_position(ra, expr_pos))
            }

            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!(
                    "{:?} cannot start a primary expression",
                    self.current_token.kind
                ),
                self.current_token.position,
            ))),
        }
    }

    ///
    /// RecordAggregate ::= SingleRecordAggregate
    ///                   | MultipleRecordAggregate
    ///
    /// SingleRecordAggregate ::= Identifier ~ Expression
    /// MultipleRecordAggregate ::= Identifier ~ Expression , RecordAggregate
    ///
    fn parse_record_aggregate(&mut self) -> RecordAggregate {
        let mut ra_pos = SourcePosition::default();
        self.start(&mut ra_pos);

        let id = self.parse_identifier();
        self.accept(TokenType::Is);
        let expr = self.parse_expression();

        if self.current_token.kind == TokenType::Comma {
            self.accept_it();
            let ra = self.parse_record_aggregate();
            self.finish(&mut ra_pos);
            MultipleRecordAggregate(MultipleRecordAggregateState::new_with_position(
                id, expr, ra, ra_pos,
            ))
        } else {
            self.finish(&mut ra_pos);
            SingleRecordAggregate(SingleRecordAggregateState::new_with_position(
                id, expr, ra_pos,
            ))
        }
    }

    ///
    /// Vname ::= SimpleVname
    ///         | DotVname
    ///         | SubscriptVname
    ///
    /// SimpleVname ::= Identifier
    /// DotVname ::= Vname . Identifier
    /// SubscriptVname ::= Vname [ Expression ]
    ///
    fn parse_vname(&mut self, id: Identifier) -> Vname {
        let mut vname_pos = SourcePosition::default();
        self.start(&mut vname_pos);

        self.finish(&mut vname_pos);
        let mut vname = SimpleVname(SimpleVnameState::new_with_position(id, vname_pos));

        while self.current_token.kind == TokenType::LeftSquareBracket
            || self.current_token.kind == TokenType::Dot
        {
            match self.current_token.kind {
                TokenType::LeftSquareBracket => {
                    self.accept_it();
                    let expr = self.parse_expression();
                    self.accept(TokenType::RightSquareBracket);
                    self.finish(&mut vname_pos);
                    vname = SubscriptVname(SubscriptVnameState::new_with_position(
                        vname, expr, vname_pos,
                    ));
                }

                TokenType::Dot => {
                    self.accept_it();
                    let id1 = self.parse_identifier();
                    vname = DotVname(DotVnameState::new_with_position(vname, id1, vname_pos));
                }

                _ => error::report_error_and_exit(GenError::from(ParserError::new(
                    &format!(
                        "{:?} cannot start or continue a vname",
                        self.current_token.kind
                    ),
                    self.current_token.position,
                ))),
            };
        }

        vname
    }

    ///
    /// Program ::= Command <Eot>
    ///
    pub fn parse_program(&mut self) -> Program {
        let mut pos = SourcePosition::default();
        self.start(&mut pos);
        let cmd = self.parse_command();
        self.finish(&mut pos);
        self.accept(TokenType::Eot);

        Program::new_with_position(cmd, pos)
    }
}