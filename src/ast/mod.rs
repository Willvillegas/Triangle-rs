//! The AST module
//!
//! This module contains all the AST (Abstract syntax tree) types for Triangle.
//! This Ast is produced in the form of a hierarchical tree by the Parser, decorated by the
//! Checker, and ultimately used by the Encoder to generate binary code for the TAM (Triangle
//! Abstract Machine).

use crate::scanner::SourcePosition;

use std::default;
use std::fmt;

pub mod aggregates;
pub mod commands;
pub mod declarations;
pub mod expressions;
pub mod parameters;
pub mod primitives;
pub mod runtime_entities;
pub mod typedenoters;
pub mod vnames;

use aggregates::*;
use commands::*;
use declarations::*;
use expressions::*;
use parameters::*;
use primitives::*;
use runtime_entities::RuntimeEntity;
use typedenoters::*;
use vnames::*;

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
    fn visit_empty_command(&self, cmd: &mut EmptyCommandState, arg: AstObject) -> AstObject;
    fn visit_assign_command(&self, cmd: &mut AssignCommandState, arg: AstObject) -> AstObject;
    fn visit_call_command(&self, cmd: &mut CallCommandState, arg: AstObject) -> AstObject;
    fn visit_let_command(&self, cmd: &mut LetCommandState, arg: AstObject) -> AstObject;
    fn visit_if_command(&self, cmd: &mut IfCommandState, arg: AstObject) -> AstObject;
    fn visit_while_command(&self, cmd: &mut WhileCommandState, arg: AstObject) -> AstObject;
    fn visit_sequential_command(
        &self,
        cmd: &mut SequentialCommandState,
        arg: AstObject,
    ) -> AstObject;

    fn visit_empty_expression(&self, expr: &mut EmptyExpressionState, arg: AstObject) -> AstObject;
    fn visit_integer_expression(
        &self,
        expr: &mut IntegerExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_character_expression(
        &self,
        expr: &mut CharacterExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_vname_expression(&self, expr: &mut VnameExpressionState, arg: AstObject) -> AstObject;
    fn visit_let_expression(&self, expr: &mut LetExpressionState, arg: AstObject) -> AstObject;
    fn visit_call_expression(&self, expr: &mut CallExpressionState, arg: AstObject) -> AstObject;
    fn visit_if_expression(&self, expr: &mut IfExpressionState, arg: AstObject) -> AstObject;
    fn visit_unary_expression(&self, expr: &mut UnaryExpressionState, arg: AstObject) -> AstObject;
    fn visit_binary_expression(
        &self,
        expr: &mut BinaryExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_array_expression(&self, expr: &mut ArrayExpressionState, arg: AstObject) -> AstObject;
    fn visit_record_expression(
        &self,
        expr: &mut RecordExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_array_aggregate(
        &self,
        agg: &mut SingleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_array_aggregate(
        &self,
        agg: &mut MultipleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_record_aggregate(
        &self,
        agg: &mut SingleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_record_aggregate(
        &self,
        agg: &mut MultipleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_declaration(
        &self,
        decl: &mut ConstDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_declaration(&self, decl: &mut VarDeclarationState, arg: AstObject) -> AstObject;
    fn visit_proc_declaration(&self, decl: &mut ProcDeclarationState, arg: AstObject) -> AstObject;
    fn visit_func_declaration(&self, decl: &mut FuncDeclarationState, arg: AstObject) -> AstObject;
    fn visit_type_declaration(&self, decl: &mut TypeDeclarationState, arg: AstObject) -> AstObject;
    fn visit_unary_operator_declaration(
        &self,
        decl: &mut UnaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_binary_operator_declaration(
        &self,
        decl: &mut BinaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_sequential_declaration(
        &self,
        decl: &mut SequentialDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_any_type_denoter(&self, td: &mut AnyTypeDenoterState, arg: AstObject) -> AstObject;
    fn visit_error_type_denoter(&self, td: &mut ErrorTypeDenoterState, arg: AstObject)
        -> AstObject;
    fn visit_bool_type_denoter(&self, td: &mut BoolTypeDenoterState, arg: AstObject) -> AstObject;
    fn visit_char_type_denoter(&self, td: &mut CharTypeDenoterState, arg: AstObject) -> AstObject;
    fn visit_int_type_denoter(&self, td: &mut IntTypeDenoterState, arg: AstObject) -> AstObject;
    fn visit_array_type_denoter(&self, td: &mut ArrayTypeDenoterState, arg: AstObject)
        -> AstObject;
    fn visit_simple_type_denoter(
        &self,
        td: &mut SimpleTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_field_type_denoter(
        &self,
        td: &mut SingleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_field_type_denoter(
        &self,
        td: &mut MultipleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_record_type_denoter(
        &self,
        td: &mut RecordTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_formal_parameter_sequence(
        &self,
        fps: &mut EmptyFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_formal_parameter_sequence(
        &self,
        fps: &mut SingleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_formal_parameter_sequence(
        &self,
        fps: &mut MultipleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_formal_parameter(
        &self,
        fp: &mut ConstFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_formal_parameter(
        &self,
        fp: &mut VarFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_formal_parameter(
        &self,
        fp: &mut ProcFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_formal_parameter(
        &self,
        fp: &mut FuncFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_actual_parameter_sequence(
        &self,
        aps: &mut EmptyActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_actual_parameter_sequence(
        &self,
        aps: &mut SingleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_actual_parameter_sequence(
        &self,
        aps: &mut MultipleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_actual_parameter(
        &self,
        ap: &mut ConstActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_actual_parameter(
        &self,
        ap: &mut VarActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_actual_parameter(
        &self,
        ap: &mut ProcActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_actual_parameter(
        &self,
        ap: &mut FuncActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_simple_vname(&self, vname: &mut SimpleVnameState, arg: AstObject) -> AstObject;
    fn visit_dot_vname(&self, vname: &mut DotVnameState, arg: AstObject) -> AstObject;
    fn visit_subscript_vname(&self, vname: &mut SubscriptVnameState, arg: AstObject) -> AstObject;
    fn visit_identifier(&self, id: Identifier, arg: AstObject) -> AstObject;
    fn visit_integer_literal(&self, il: IntegerLiteral, arg: AstObject) -> AstObject;
    fn visit_character_literal(&self, cl: CharacterLiteral, arg: AstObject) -> AstObject;
    fn visit_operator(&self, op: Operator, arg: AstObject) -> AstObject;
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
#[derive(Debug, Clone)]
pub struct CommonState {
    pub position: SourcePosition,
    pub entity: RuntimeEntity,
}

impl CommonState {
    pub fn new(position: SourcePosition, entity: runtime_entities::RuntimeEntity) -> Self {
        CommonState { position, entity }
    }
}

impl default::Default for CommonState {
    fn default() -> Self {
        CommonState {
            position: SourcePosition::default(),
            entity: runtime_entities::RuntimeEntity::None,
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub cmd: Command,
    pub common_state: CommonState,
}

impl Program {
    pub fn new(cmd: Command) -> Self {
        Program {
            cmd: cmd,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(cmd: Command, position: SourcePosition) -> Self {
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

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Program::new({})", self.cmd)
    }
}

impl Ast for Program {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_program(self, arg)
    }
}
