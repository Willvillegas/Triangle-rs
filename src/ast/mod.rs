//! The AST module
//!
//! This module contains all the AST (Abstract syntax tree) types for Triangle.
//! This Ast is produced in the form of a hierarchical tree by the Parser, decorated by the
//! Checker, and ultimately used by the Encoder to generate binary code for the TAM (Triangle
//! Abstract Machine).

use crate::scanner;
use std::default;

pub mod aggregates;
pub mod commands;
pub mod declarations;
pub mod expressions;
pub mod parameters;
pub mod primitives;
pub mod runtime_entities;
pub mod typedenoters;
pub mod vnames;

/// Any entity that wants to be traversable needs to implement this trait
pub trait Ast {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject;
}

/// The AstVisitor visitor will be used by the parser, checker, and encoder for different
/// processing of the Ast, returning different values at each phase, or even within the
/// same phase.
/// This object enumerates all the possible variants that can be returned by a visitor method.
#[derive(Debug)]
pub enum AstObject {
    Null,
    Frame(Frame),
    Size(usize),
}

/// Visitor for the Triangle Ast - both the checker and the encoder (code generator)
/// make use of this visitor to traverse the parsed and checked asts respectively.
pub trait AstVisitor {
    fn visit_program(&self, program: &mut Program, arg: AstObject) -> AstObject;
    fn visit_empty_command(
        &self,
        cmd: &mut commands::EmptyCommandState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_assign_command(
        &self,
        cmd: &mut commands::AssignCommandState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_call_command(&self, cmd: &mut commands::CallCommandState, arg: AstObject)
        -> AstObject;
    fn visit_let_command(&self, cmd: &mut commands::LetCommandState, arg: AstObject) -> AstObject;
    fn visit_if_command(&self, cmd: &mut commands::IfCommandState, arg: AstObject) -> AstObject;
    fn visit_while_command(
        &self,
        cmd: &mut commands::WhileCommandState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_sequential_command(
        &self,
        cmd: &mut commands::SequentialCommandState,
        arg: AstObject,
    ) -> AstObject;

    fn visit_empty_expression(
        &self,
        expr: &mut expressions::EmptyExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_integer_expression(
        &self,
        expr: &mut expressions::IntegerExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_character_expression(
        &self,
        expr: &mut expressions::CharacterExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_vname_expression(
        &self,
        expr: &mut expressions::VnameExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_let_expression(
        &self,
        expr: &mut expressions::LetExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_call_expression(
        &self,
        expr: &mut expressions::CallExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_if_expression(
        &self,
        expr: &mut expressions::IfExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_unary_expression(
        &self,
        expr: &mut expressions::UnaryExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_binary_expression(
        &self,
        expr: &mut expressions::BinaryExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_array_expression(
        &self,
        expr: &mut expressions::ArrayExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_record_expression(
        &self,
        expr: &mut expressions::RecordExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_array_aggregate(
        &self,
        agg: &mut aggregates::SingleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_array_aggregate(
        &self,
        agg: &mut aggregates::MultipleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_record_aggregate(
        &self,
        agg: &mut aggregates::SingleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_record_aggregate(
        &self,
        agg: &mut aggregates::MultipleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_declaration(
        &self,
        decl: &mut declarations::ConstDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_declaration(
        &self,
        decl: &mut declarations::VarDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_declaration(
        &self,
        decl: &mut declarations::ProcDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_declaration(
        &self,
        decl: &mut declarations::FuncDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_type_declaration(
        &self,
        decl: &mut declarations::TypeDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_unary_operator_declaration(
        &self,
        decl: &mut declarations::UnaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_binary_operator_declaration(
        &self,
        decl: &mut declarations::BinaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_sequential_declaration(
        &self,
        decl: &mut declarations::SequentialDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_any_type_denoter(
        &self,
        td: &mut typedenoters::AnyTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_error_type_denoter(
        &self,
        td: &mut typedenoters::ErrorTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_bool_type_denoter(
        &self,
        td: &mut typedenoters::BoolTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_char_type_denoter(
        &self,
        td: &mut typedenoters::CharTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_int_type_denoter(
        &self,
        td: &mut typedenoters::IntTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_array_type_denoter(
        &self,
        td: &mut typedenoters::ArrayTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_simple_type_denoter(
        &self,
        td: &mut typedenoters::SimpleTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_field_type_denoter(
        &self,
        td: &mut typedenoters::SingleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_field_type_denoter(
        &self,
        td: &mut typedenoters::MultipleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_record_type_denoter(
        &self,
        td: &mut typedenoters::RecordTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_formal_parameter_sequence(
        &self,
        fps: &mut parameters::EmptyFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_formal_parameter_sequence(
        &self,
        fps: &mut parameters::SingleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_formal_parameter_sequence(
        &self,
        fps: &mut parameters::MultipleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_formal_parameter(
        &self,
        fp: &mut parameters::ConstFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_formal_parameter(
        &self,
        fp: &mut parameters::VarFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_formal_parameter(
        &self,
        fp: &mut parameters::ProcFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_formal_parameter(
        &self,
        fp: &mut parameters::FuncFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_actual_parameter_sequence(
        &self,
        aps: &mut parameters::EmptyActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_actual_parameter_sequence(
        &self,
        aps: &mut parameters::SingleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_actual_parameter_sequence(
        &self,
        aps: &mut parameters::MultipleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_actual_parameter(
        &self,
        ap: &mut parameters::ConstActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_actual_parameter(
        &self,
        ap: &mut parameters::VarActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_actual_parameter(
        &self,
        ap: &mut parameters::ProcActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_actual_parameter(
        &self,
        ap: &mut parameters::FuncActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_simple_vname(&self, vname: &mut vnames::SimpleVnameState, arg: AstObject)
        -> AstObject;
    fn visit_dot_vname(&self, vname: &mut vnames::DotVnameState, arg: AstObject) -> AstObject;
    fn visit_subscript_vname(
        &self,
        vname: &mut vnames::SubscriptVnameState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_identifier(&self, id: primitives::Identifier, arg: AstObject) -> AstObject;
    fn visit_integer_literal(&self, il: primitives::IntegerLiteral, arg: AstObject) -> AstObject;
    fn visit_character_literal(
        &self,
        cl: primitives::CharacterLiteral,
        arg: AstObject,
    ) -> AstObject;
    fn visit_operator(&self, op: primitives::Operator, arg: AstObject) -> AstObject;
}

/// A frame represents the runtime state of execution of a function
#[derive(Debug)]
pub struct Frame {
    pub level: usize,
    pub size: usize,
}

/// The core AST types representing the entities
/// of the Triangle language.

/// Common state that is shared by every Ast
#[derive(Debug)]
pub struct CommonState {
    pub position: scanner::SourcePosition,
    pub entity: runtime_entities::RuntimeEntity,
}

impl CommonState {
    pub fn new(position: scanner::SourcePosition, entity: runtime_entities::RuntimeEntity) -> Self {
        CommonState { position, entity }
    }
}

impl default::Default for CommonState {
    fn default() -> Self {
        CommonState {
            position: scanner::SourcePosition::default(),
            entity: runtime_entities::RuntimeEntity::None,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub cmd: commands::Command,
    pub common_state: CommonState,
}

impl Program {
    pub fn new(cmd: commands::Command) -> Self {
        Program {
            cmd: cmd,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(cmd: commands::Command, position: scanner::SourcePosition) -> Self {
        let mut program = Program::new(cmd);
        program.common_state.position = position;
        program
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for Program {}

impl Ast for Program {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_program(self, arg)
    }
}
