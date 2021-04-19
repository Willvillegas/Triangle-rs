#![allow(unused_imports)]

//! The Parser module.
//!
//! This module consumes the stream of tokens produced by the Scanner, and constructs an AST for
//! Triangle, which is then used by all subsequent phases of the compiler.

use crate::ast::aggregates::ArrayAggregate::*;
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

    ///```
    /// Command ::= single-Command | Command ; single-Command
    /// ```
    fn parse_command(&mut self) -> Command {
        let mut cmd_pos = SourcePosition::default();
        self.start(&mut cmd_pos);
        let mut cmd = self.parse_single_command();
        self.finish(&mut cmd_pos);

        while self.current_token.spelling == ";" {
            self.accept_it();
            let cmd1 = self.parse_single_command();
            self.finish(&mut cmd_pos);
            cmd = Command::SequentialCommand(SequentialCommandState::new_with_position(
                cmd, cmd1, cmd_pos,
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
        let mut cmd_pos = SourcePosition::default();
        self.start(&mut cmd_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();

                match self.current_token.kind {
                    TokenType::Becomes => {
                        self.accept_it();
                        let vname = self.parse_vname(Some(id));
                        let expr = self.parse_expression();
                        self.finish(&mut cmd_pos);
                        cmd = AssignCommand(AssignCommandState::new_with_position(
                            vname, expr, cmd_pos,
                        ));
                    }

                    TokenType::LeftParen => {
                        self.accept_it();
                        let aps = self.parse_actual_parameter_sequence();
                        self.accept(TokenType::RightParen);
                        self.finish(&mut cmd_pos);
                        cmd = CallCommand(CallCommandState::new_with_position(id, aps, cmd_pos));
                    }

                    _ => error::report_error_and_exit(GenError::from(ParserError::new(
                        &format!(
                            "expected either `:=` or `(`, but found {:?}",
                            self.current_token.spelling
                        ),
                        self.current_token.position,
                    ))),
                }
            }

            TokenType::Let => {
                self.accept_it();
                let decl = self.parse_declaration();
                self.accept(TokenType::In);
                let cmd1 = self.parse_single_command();
                self.finish(&mut cmd_pos);
                cmd = LetCommand(LetCommandState::new_with_position(decl, cmd1, cmd_pos));
            }

            TokenType::If => todo!(),
            TokenType::While => todo!(),
            TokenType::Begin => {
                self.accept_it();
                cmd = self.parse_command();
                self.accept(TokenType::End);
            }

            TokenType::Semicolon | TokenType::End | TokenType::Eot => {
                self.accept_it();
                self.finish(&mut cmd_pos);
                cmd = EmptyCommand(EmptyCommandState::new_with_position(cmd_pos));
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

        cmd
    }

    ///```
    /// Declaration ::= single-Declaration
    ///             | Declaration ; single-Declaration
    ///```
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

    ///```
    /// single-Declaration ::= ConstDeclaration
    ///                 | VarDeclaration
    ///                 | ProcDeclaration
    ///                 | FuncDeclaration
    ///                 | TypeDeclaration
    ///```
    fn parse_single_declaration(&mut self) -> Declaration {
        let decl;
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
                decl = VarDeclaration(VarDeclarationState::new_with_position(id, td, decl_pos));
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
                decl = ProcDeclaration(ProcDeclarationState::new_with_position(
                    id, fps, cmd, decl_pos,
                ));
            }
            TokenType::Function => todo!(),
            TokenType::Type => todo!(),
            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!("{:?} cannot start a declaration", self.current_token.kind),
                self.current_token.position,
            ))),
        }

        decl
    }

    ///```
    /// TypeDenoter ::= SimpleTypeDenoter
    ///               | ArrayTypeDenoter
    ///               | RecordTypeDenoter
    ///```
    fn parse_type_denoter(&mut self) -> TypeDenoter {
        let td;
        let mut td_pos = SourcePosition::default();
        self.start(&mut td_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();
                self.finish(&mut td_pos);
                td = SimpleTypeDenoter(SimpleTypeDenoterState::new_with_position(id, td_pos));
            }
            TokenType::Array => todo!(),
            TokenType::Record => todo!(),
            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!("{:?} cannot start a type denoter", self.current_token.kind),
                self.current_token.position,
            ))),
        }

        td
    }

    ///```
    /// FormalParameterSequence ::= EmptyFormalParameterSequence
    ///                         | SingleFormalParameterSequence
    ///                         | MultipleFormalParameterSequence
    ///```
    fn parse_formal_parameter_sequence(&mut self) -> FormalParameterSequence {
        let mut fps_pos = SourcePosition::default();
        self.start(&mut fps_pos);

        if self.current_token.kind == TokenType::RightParen {
            self.finish(&mut fps_pos);
            return EmptyFormalParameterSequence(
                EmptyFormalParameterSequenceState::new_with_position(fps_pos),
            );
        } else {
            let fp = self.parse_formal_parameter();
            if self.current_token.kind == TokenType::Comma {
                self.accept_it();
                let fps = self.parse_formal_parameter_sequence();
                self.finish(&mut fps_pos);
                return MultipleFormalParameterSequence(
                    MultipleFormalParameterSequenceState::new_with_position(fp, fps, fps_pos),
                );
            } else {
                self.finish(&mut fps_pos);
                return SingleFormalParameterSequence(
                    SingleFormalParameterSequenceState::new_with_position(fp, fps_pos),
                );
            }
        }
    }

    ///```
    /// FormalParamater ::= ConstFormalParameter
    ///                  | VarFormalParameter
    ///                  | ProcFormalParameter
    ///                  | FuncFormalParameter
    ///```
    fn parse_formal_parameter(&mut self) -> FormalParameter {
        let fp;
        let mut fp_pos = SourcePosition::default();
        self.start(&mut fp_pos);

        match self.current_token.kind {
            TokenType::Identifier => {
                let id = self.parse_identifier();
                let td = self.parse_type_denoter();
                self.finish(&mut fp_pos);
                fp = ConstFormalParameter(ConstFormalParameterState::new_with_position(
                    id, td, fp_pos,
                ));
            }

            TokenType::Var => {
                self.accept_it();
                let id = self.parse_identifier();
                self.accept(TokenType::Colon);
                let td = self.parse_type_denoter();
                self.finish(&mut fp_pos);
                fp = VarFormalParameter(VarFormalParameterState::new_with_position(id, td, fp_pos));
            }
            TokenType::Procedure => todo!(),
            TokenType::Function => todo!(),
            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!(
                    "{:?} cannot start a formal parameter",
                    self.current_token.kind
                ),
                self.current_token.position,
            ))),
        }

        fp
    }

    ///```
    /// ActualParameterSequence ::= EmptyActualParameterSequence
    ///                          | SingleActualParameterSequenceState
    ///                          | MultipleActualParameterSequence
    ///```
    fn parse_actual_parameter_sequence(&mut self) -> ActualParameterSequence {
        let mut aps_pos = SourcePosition::default();

        self.start(&mut aps_pos);
        if self.current_token.kind == TokenType::RightParen {
            self.finish(&mut aps_pos);
            return EmptyActualParameterSequence(
                EmptyActualParameterSequenceState::new_with_position(aps_pos),
            );
        } else {
            let ap = self.parse_actual_parameter();
            if self.current_token.kind == TokenType::Comma {
                self.accept_it();
                let aps = self.parse_actual_parameter_sequence();
                self.finish(&mut aps_pos);
                return MultipleActualParameterSequence(
                    MultipleActualParameterSequenceState::new_with_position(ap, aps, aps_pos),
                );
            } else {
                self.finish(&mut aps_pos);
                return SingleActualParameterSequence(
                    SingleActualParameterSequenceState::new_with_position(ap, aps_pos),
                );
            }
        }
    }

    ///```
    /// ActualParameter ::= ConstActualParameter
    ///                 | VarActualParameter
    ///                 | ProcActualParameter
    ///                 | FuncActualParameter
    ///```
    fn parse_actual_parameter(&mut self) -> ActualParameter {
        let ap;
        let mut ap_pos = SourcePosition::default();
        self.start(&mut ap_pos);

        match self.current_token.kind {
            TokenType::Var => {
                self.accept_it();
                let id = self.parse_identifier();
                let vname = self.parse_vname(Some(id));
                self.finish(&mut ap_pos);
                ap = VarActualParameter(VarActualParameterState::new_with_position(vname, ap_pos));
            }
            TokenType::Procedure => todo!(),
            TokenType::Function => todo!(),
            _ => {
                let expr = self.parse_expression();
                self.finish(&mut ap_pos);
                ap = ConstActualParameter(ConstActualParameterState::new_with_position(
                    expr, ap_pos,
                ));
            }
        }

        ap
    }

    fn parse_var_actual_parameter(&mut self) -> ActualParameter {
        todo!()
    }

    fn parse_proc_actual_parameter(&mut self) -> ActualParameter {
        todo!()
    }

    fn parse_func_actual_parameter(&mut self) -> ActualParameter {
        todo!()
    }

    ///```
    /// Expression :: = secondary-Expression
    ///             | LetExpression
    ///             | IfExpression
    ///```
    fn parse_expression(&mut self) -> Expression {
        match self.current_token.kind {
            TokenType::If => todo!(),
            TokenType::Let => todo!(),
            _ => self.parse_secondary_expression(),
        }
    }

    ///```
    /// secondary-Expression ::= primary-Expression
    ///                     | primary-Expression Operator secondary-Expression
    ///```
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

    ///```
    /// primary-Expression ::= IntegerExpression
    ///                     | CharacterExpression
    ///                     | VnameExpression
    ///                     | CallExpression
    ///                     | UnaryExpression
    ///                     | ParenthesisedExpression
    ///                     | ArrayExpression
    ///                     | RecordExpression
    ///
    ///```
    fn parse_primary_expression(&mut self) -> Expression {
        let expr;
        let mut expr_pos = SourcePosition::default();
        self.start(&mut expr_pos);

        match self.current_token.kind {
            TokenType::IntegerLiteral => {
                let il = self.parse_integer_literal();
                self.finish(&mut expr_pos);
                expr = IntegerExpression(IntegerExpressionState::new_with_position(il, expr_pos));
            }

            TokenType::CharacterLiteral => {
                let cl = self.parse_character_literal();
                self.finish(&mut expr_pos);
                expr =
                    CharacterExpression(CharacterExpressionState::new_with_position(cl, expr_pos));
            }
            TokenType::Identifier => {
                let id = self.parse_identifier();
                let vname = self.parse_vname(Some(id));
                self.finish(&mut expr_pos);
                expr = VnameExpression(VnameExpressionState::new_with_position(vname, expr_pos));
            }
            TokenType::Operator => todo!(),
            TokenType::LeftParen => {
                self.accept_it();
                expr = self.parse_expression();
                self.accept(TokenType::RightParen);
            }
            TokenType::LeftSquareBracket => todo!(),
            TokenType::LeftCurlyBracket => todo!(),
            _ => error::report_error_and_exit(GenError::from(ParserError::new(
                &format!(
                    "{:?} cannot start a primary expression",
                    self.current_token.kind
                ),
                self.current_token.position,
            ))),
        }

        expr
    }

    ///```
    /// Vname ::= SimpleVname
    ///         | DotVname
    ///         | SubscriptVname
    ///```
    fn parse_vname(&mut self, id: Option<Identifier>) -> Vname {
        let vname;
        let mut vname_pos = SourcePosition::default();
        self.start(&mut vname_pos);

        if let Some(id_val) = id {
            self.finish(&mut vname_pos);
            vname = SimpleVname(SimpleVnameState::new_with_position(id_val, vname_pos));
        } else if self.current_token.kind == TokenType::Dot {
            todo!()
        } else if self.current_token.kind == TokenType::LeftSquareBracket {
            todo!()
        } else {
            error::report_error_and_exit(GenError::from(ParserError::new(
                &format!("{:?} cannot start a vname", self.current_token.kind),
                self.current_token.position,
            )));
        }

        vname
    }

    ///```
    /// Program ::= Command <Eot>
    /// ```
    pub fn parse_program(&mut self) -> Program {
        let mut pos = SourcePosition::default();
        self.start(&mut pos);
        let cmd = self.parse_command();
        self.finish(&mut pos);
        self.accept(TokenType::Eot);

        Program::new_with_position(cmd, pos)
    }
}