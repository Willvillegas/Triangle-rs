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

pub trait Ast {}

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

pub trait AstVisitor {
    fn visit_program(&self, p: &mut Program) -> AstObject;
    fn visit_command(&self, c: &mut commands::Command) -> AstObject;
}

/// A frame represents the runtime state of execution of a function
#[derive(Debug)]
pub struct Frame {
    level: usize,
    size: usize,
}

/// runtime entities - these entities are used by the encoder to
/// generate the correct bytecode for the TAM (Triangle Abstract Machine).

pub enum RuntimeEntity {
    None,
    KnownAddress(Box<KnownAddressState>),
    UnknownAddress(Box<UnknownAddressState>),
    KnownValue(Box<KnownValueState>),
    UnknownValue(Box<UnknownValueState>),
    PrimitiveRoutine(Box<PrimitiveRoutineState>),
    EqualityRoutine(Box<EqualityRoutineState>),
    UnknownRoutine(Box<UnknownRoutineState>),
    Field(Box<FieldState>),
    TypeDeclaration(Box<TypeDeclarationState>),
}

/// represents the declared state of an entity
pub struct EntityAddress {
    level: usize,
    displacement: usize,
}

pub struct KnownAddressState {}

impl KnownAddressState {}

pub struct UnknownAddressState {}

impl UnknownAddressState {}

pub struct KnownValueState {}

impl KnownValueState {}

pub struct UnknownValueState {}

impl UnknownValueState {}

pub struct KnownRoutineState {}

impl KnownRoutineState {}

pub struct UnknownRoutineState {}

impl UnknownRoutineState {}

pub struct PrimitiveRoutineState {}

impl PrimitiveRoutineState {}

pub struct EqualityRoutineState {}

impl EqualityRoutineState {}

pub struct FieldState {}

impl FieldState {}

pub struct TypeDeclarationState {}

impl TypeDeclarationState {}

/// The core AST types representing the entities
/// of the Triangle language.

/// represents the common state that every Ast
/// must have.
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

impl Ast for Program {}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for Program {}
