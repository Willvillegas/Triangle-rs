//! The AST module
//!
//! This module contains all the AST (Abstract syntax tree) types for Triangle.
//! This Ast is produced in the form of a hierarchical tree by the Parser, decorated by the
//! Checker, and ultimately used by the Encoder to generate binary code for the TAM (Triangle
//! Abstract Machine).

use crate::scanner::SourcePosition;
use std::default;

pub mod arrays;
pub mod commands;
pub mod declarations;
pub mod expressions;
pub mod parameters;
pub mod primitives;
pub mod records;
pub mod typedenoters;
pub mod vnames;

use commands::Command;
use declarations::Declaration;
use expressions::Expression;
use typedenoters::TypeDenoter;
use vnames::Vname;

pub trait Ast {
    fn accept(&mut self, visitor: &dyn AstVisitor) -> AstObject;
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

// todo - fill this out with all the possible concrete types of asts
pub trait AstVisitor {
    fn visit_program(&self, prog: &mut Program) -> AstObject;
    fn visit_command(&self, cmd: &mut Command) -> AstObject;
    fn visit_expression(&self, expr: &mut Expression) -> AstObject;
    fn visit_declaration(&self, decl: &mut Declaration) -> AstObject;
    fn visit_vname(&self, vname: &mut Vname) -> AstObject;
    fn visit_type_denoter(&self, td: &mut TypeDenoter) -> AstObject;
}

/// A frame represents the runtime state of execution of a function
#[derive(Debug)]
pub struct Frame {
    level: usize,
    size: usize,
}

/// runtime entities - these entities are used by the encoder to
/// generate the correct bytecode for the TAM (Triangle Abstract Machine).

#[derive(Debug)]
pub enum RuntimeEntity {
    None,
    KnownAddress(KnownAddressState),
    UnknownAddress(UnknownAddressState),
    KnownValue(KnownValueState),
    UnknownValue(UnknownValueState),
    PrimitiveRoutine(PrimitiveRoutineState),
    EqualityRoutine(EqualityRoutineState),
    UnknownRoutine(UnknownRoutineState),
    Field(FieldState),
    TypeDeclaration(TypeDeclarationState),
}

/// represents the declared state of an entity
#[derive(Debug)]
pub struct EntityAddress {
    level: usize,
    displacement: usize,
}

#[derive(Debug)]
pub struct KnownAddressState {}

impl KnownAddressState {}

#[derive(Debug)]
pub struct UnknownAddressState {}

impl UnknownAddressState {}

#[derive(Debug)]
pub struct KnownValueState {}

impl KnownValueState {}

#[derive(Debug)]
pub struct UnknownValueState {}

impl UnknownValueState {}

#[derive(Debug)]
pub struct KnownRoutineState {}

impl KnownRoutineState {}

#[derive(Debug)]
pub struct UnknownRoutineState {}

impl UnknownRoutineState {}

#[derive(Debug)]
pub struct PrimitiveRoutineState {}

impl PrimitiveRoutineState {}

#[derive(Debug)]
pub struct EqualityRoutineState {}

impl EqualityRoutineState {}

#[derive(Debug)]
pub struct FieldState {}

impl FieldState {}

#[derive(Debug)]
pub struct TypeDeclarationState {}

impl TypeDeclarationState {}

/// The core AST types representing the entities
/// of the Triangle language.

/// represents the common state that every Ast
/// must have.
#[derive(Debug)]
pub struct CommonState {
    pub position: SourcePosition,
    pub entity: RuntimeEntity,
}

impl CommonState {
    pub fn new(position: SourcePosition, entity: RuntimeEntity) -> Self {
        CommonState { position, entity }
    }
}

impl default::Default for CommonState {
    fn default() -> Self {
        CommonState {
            position: SourcePosition::dummy_source_position(),
            entity: RuntimeEntity::None,
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
}

impl Ast for Program {
    fn accept(&mut self, visitor: &dyn AstVisitor) -> AstObject {
        visitor.visit_program(self)
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for Program {}
