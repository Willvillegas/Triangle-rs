//! The AST module
//!
//! This module contains all the AST (Abstract syntax tree) types for Triangle.
//! This Ast is produced in the form of a hierarchical tree by the Parser, decorated by the
//! Checker, and ultimately used by the Encoder to generate binary code for the TAM (Triangle
//! Abstract Machine).

pub trait Ast {}
pub trait Declaration: Ast {}
pub trait Command: Ast {}
pub trait Expression: Ast {}
pub trait Vname: Ast {}
pub trait TypeDenoter: Ast {}

pub struct Program {}

impl Program {
    pub fn new() -> Self {
        Program {}
    }
}

impl Ast for Program {}

// declarations

// commands

// expressions

// vnames

// type-denoters

// primitive asts

pub struct IntegerLiteral {}

impl IntegerLiteral {}

pub struct CharacterLiteral {}

impl CharacterLiteral {}

pub struct Identifier {}

impl Identifier {}

pub struct Operator {}

impl Operator {}

pub struct Frame {
    level: usize,
    size: usize,
}

// runtime entities

pub trait RuntimeEntity {}

pub struct EntityAddress {
    level: usize,
    displacement: usize,
}

pub struct KnownAddress {}

impl KnownAddress {}

impl RuntimeEntity for KnownAddress {}

pub struct UnknownAddress {}

impl UnknownAddress {}

impl RuntimeEntity for UnknownAddress {}

pub struct KnownValue {}

impl KnownValue {}

impl RuntimeEntity for KnownValue {}

pub struct UnknownValue {}

impl UnknownValue {}

impl RuntimeEntity for UnknownValue {}

pub struct KnownRoutine {}

impl KnownRoutine {}

impl RuntimeEntity for KnownRoutine {}

pub struct UnknownRoutine {}

impl UnknownRoutine {}

impl RuntimeEntity for UnknownRoutine {}

pub struct PrimitiveRoutine {}

impl PrimitiveRoutine {}

impl RuntimeEntity for PrimitiveRoutine {}

pub struct EqualityRoutine {}

impl EqualityRoutine {}

impl RuntimeEntity for EqualityRoutine {}

pub struct Field {}

impl Field {}

impl RuntimeEntity for Field {}

pub struct TypeDeclaration {}

impl TypeDeclaration {}

impl RuntimeEntity for TypeDeclaration {}