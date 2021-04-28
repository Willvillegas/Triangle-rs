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
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject;
}

/// The AstVisitor visitor will be used by the parser, checker, and encoder for different
/// processing of the Ast, returning different values at each phase, or even within the
/// same phase.
/// This object enumerates all the possible variants that can be returned by a visitor method.
#[derive(Debug, Clone)]
pub enum AstObject {
    Null,
    Frame(Frame),
    Size(usize),
    Declaration(Box<Declaration>),
    FormalParameterSequence(Box<FormalParameterSequence>),
    FormalParameter(Box<FormalParameter>),
    TypeDenoter(Box<TypeDenoter>),
}

impl AstObject {
    pub fn get_declaration(&self) -> Option<&Box<Declaration>> {
        match *self {
            AstObject::Declaration(ref decl) => Some(&decl),
            _ => None,
        }
    }

    pub fn get_formal_parameter_sequence(&self) -> Option<&Box<FormalParameterSequence>> {
        match *self {
            AstObject::FormalParameterSequence(ref fps) => Some(&fps),
            _ => None,
        }
    }

    pub fn get_formal_parameter(&self) -> Option<&Box<FormalParameter>> {
        match *self {
            AstObject::FormalParameter(ref fp) => Some(&fp),
            _ => None,
        }
    }

    pub fn get_type_denoter(&self) -> Option<&Box<TypeDenoter>> {
        match *self {
            AstObject::TypeDenoter(ref td) => Some(&td),
            _ => None,
        }
    }
}

/// Visitor for the Triangle Ast - both the checker and the encoder (code generator)
/// make use of this visitor to traverse the parsed and checked asts respectively.
pub trait AstVisitor {
    fn visit_program(&mut self, program: &mut Program, arg: AstObject) -> AstObject;
    fn visit_empty_command(&mut self, cmd: &mut EmptyCommandState, arg: AstObject) -> AstObject;
    fn visit_assign_command(&mut self, cmd: &mut AssignCommandState, arg: AstObject) -> AstObject;
    fn visit_call_command(&mut self, cmd: &mut CallCommandState, arg: AstObject) -> AstObject;
    fn visit_let_command(&mut self, cmd: &mut LetCommandState, arg: AstObject) -> AstObject;
    fn visit_if_command(&mut self, cmd: &mut IfCommandState, arg: AstObject) -> AstObject;
    fn visit_while_command(&mut self, cmd: &mut WhileCommandState, arg: AstObject) -> AstObject;
    fn visit_sequential_command(
        &mut self,
        cmd: &mut SequentialCommandState,
        arg: AstObject,
    ) -> AstObject;

    fn visit_empty_expression(
        &mut self,
        expr: &mut EmptyExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_integer_expression(
        &mut self,
        expr: &mut IntegerExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_character_expression(
        &mut self,
        expr: &mut CharacterExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_vname_expression(
        &mut self,
        expr: &mut VnameExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_let_expression(&mut self, expr: &mut LetExpressionState, arg: AstObject) -> AstObject;
    fn visit_call_expression(
        &mut self,
        expr: &mut CallExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_if_expression(&mut self, expr: &mut IfExpressionState, arg: AstObject) -> AstObject;
    fn visit_unary_expression(
        &mut self,
        expr: &mut UnaryExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_binary_expression(
        &mut self,
        expr: &mut BinaryExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_array_expression(
        &mut self,
        expr: &mut ArrayExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_record_expression(
        &mut self,
        expr: &mut RecordExpressionState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_array_aggregate(
        &mut self,
        agg: &mut SingleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_array_aggregate(
        &mut self,
        agg: &mut MultipleArrayAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_record_aggregate(
        &mut self,
        agg: &mut SingleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_record_aggregate(
        &mut self,
        agg: &mut MultipleRecordAggregateState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_declaration(
        &mut self,
        decl: &mut ConstDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_declaration(
        &mut self,
        decl: &mut VarDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_declaration(
        &mut self,
        decl: &mut ProcDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_declaration(
        &mut self,
        decl: &mut FuncDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_type_declaration(
        &mut self,
        decl: &mut TypeDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_unary_operator_declaration(
        &mut self,
        decl: &mut UnaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_binary_operator_declaration(
        &mut self,
        decl: &mut BinaryOperatorDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_sequential_declaration(
        &mut self,
        decl: &mut SequentialDeclarationState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_any_type_denoter(&mut self, td: &mut AnyTypeDenoterState, arg: AstObject)
        -> AstObject;
    fn visit_error_type_denoter(
        &mut self,
        td: &mut ErrorTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_bool_type_denoter(
        &mut self,
        td: &mut BoolTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_char_type_denoter(
        &mut self,
        td: &mut CharTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_int_type_denoter(&mut self, td: &mut IntTypeDenoterState, arg: AstObject)
        -> AstObject;
    fn visit_array_type_denoter(
        &mut self,
        td: &mut ArrayTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_simple_type_denoter(
        &mut self,
        td: &mut SimpleTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_field_type_denoter(
        &mut self,
        td: &mut SingleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_field_type_denoter(
        &mut self,
        td: &mut MultipleFieldTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_record_type_denoter(
        &mut self,
        td: &mut RecordTypeDenoterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_formal_parameter_sequence(
        &mut self,
        fps: &mut EmptyFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_formal_parameter_sequence(
        &mut self,
        fps: &mut SingleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_formal_parameter_sequence(
        &mut self,
        fps: &mut MultipleFormalParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_formal_parameter(
        &mut self,
        fp: &mut ConstFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_formal_parameter(
        &mut self,
        fp: &mut VarFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_formal_parameter(
        &mut self,
        fp: &mut ProcFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_formal_parameter(
        &mut self,
        fp: &mut FuncFormalParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_empty_actual_parameter_sequence(
        &mut self,
        aps: &mut EmptyActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_single_actual_parameter_sequence(
        &mut self,
        aps: &mut SingleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_multiple_actual_parameter_sequence(
        &mut self,
        aps: &mut MultipleActualParameterSequenceState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_const_actual_parameter(
        &mut self,
        ap: &mut ConstActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_var_actual_parameter(
        &mut self,
        ap: &mut VarActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_proc_actual_parameter(
        &mut self,
        ap: &mut ProcActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_func_actual_parameter(
        &mut self,
        ap: &mut FuncActualParameterState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_simple_vname(&mut self, vname: &mut SimpleVnameState, arg: AstObject) -> AstObject;
    fn visit_dot_vname(&mut self, vname: &mut DotVnameState, arg: AstObject) -> AstObject;
    fn visit_subscript_vname(
        &mut self,
        vname: &mut SubscriptVnameState,
        arg: AstObject,
    ) -> AstObject;
    fn visit_identifier(&mut self, id: &mut Identifier, arg: AstObject) -> AstObject;
    fn visit_integer_literal(&mut self, il: &mut IntegerLiteral, arg: AstObject) -> AstObject;
    fn visit_character_literal(&mut self, cl: &mut CharacterLiteral, arg: AstObject) -> AstObject;
    fn visit_operator(&mut self, op: &mut Operator, arg: AstObject) -> AstObject;
}

/// A frame represents the runtime state of execution of a function
#[derive(Debug, Clone)]
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
    pub cmd: Box<Command>,
    pub common_state: CommonState,
}

impl Program {
    pub fn new(cmd: Command) -> Self {
        Program {
            cmd: Box::new(cmd),
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
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_program(self, arg)
    }
}
