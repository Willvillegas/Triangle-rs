//! The AST module
//!
//! This module contains all the AST (Abstract syntax tree) types for Triangle.
//! This Ast is produced in the form of a hierarchical tree by the Parser, decorated by the
//! Checker, and ultimately used by the Encoder to generate binary code for the TAM (Triangle
//! Abstract Machine).

use crate::scanner::SourcePosition;
use std::default;

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
    fn visit_empty_command(&self, c: &mut Command) -> AstObject;
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
}

impl Ast for Program {}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for Program {}

/// commands

pub enum Command {
    EmptyCommand,
    AssignCommand(Box<AssignCommandState>),
    CallCommand(Box<CallCommandState>),
    BracketedCommand(Box<BracketedCommandState>),
    LetCommand(Box<LetCommandState>),
    IfCommand(Box<IfCommandState>),
    WhileCommand(Box<WhileCommandState>),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Command {}

pub struct AssignCommandState {
    pub vname: Vname,
    pub expr: Expression,
    pub common_state: CommonState,
}

impl AssignCommandState {
    pub fn new(vname: Vname, expr: Expression) -> Self {
        AssignCommandState {
            vname: vname,
            expr: expr,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for AssignCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.expr == other.expr
    }
}

impl Eq for AssignCommandState {}

pub struct CallCommandState {
    pub id: Identifier,
    pub aps: ActualParameterSequence,
    pub common_state: CommonState,
}

impl CallCommandState {
    pub fn new(id: Identifier, aps: ActualParameterSequence) -> Self {
        CallCommandState {
            id: id,
            aps: aps,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for CallCommandState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.aps == other.aps
    }
}

impl Eq for CallCommandState {}

pub struct BracketedCommandState {
    pub cmd: Command,
    common_state: CommonState,
}

impl BracketedCommandState {
    pub fn new(cmd: Command) -> Self {
        BracketedCommandState {
            cmd: cmd,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for BracketedCommandState {
    fn eq(&self, other: &BracketedCommandState) -> bool {
        self.cmd == other.cmd
    }
}

impl Eq for BracketedCommandState {}

pub struct LetCommandState {}

pub struct IfCommandState {}

pub struct WhileCommandState {}

/// declarations

pub enum Declaration {}

/// expressions

pub enum Expression {
    IntegerExpression(Box<IntegerExpressionState>),
    CharacterExpression(Box<CharacterExpressionState>),
    VnameExpression(Box<VnameExpressionState>),
    CallExpression(Box<CallExpressionState>),
    IfExpression(Box<IfExpressionState>),
    LetExpression(Box<LetExpressionState>),
    UnaryExpression(Box<UnaryExpressionState>),
    BinaryExpression(Box<BinaryExpressionState>),
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Expression {}

pub struct IntegerExpressionState {}

pub struct CharacterExpressionState {}

pub struct VnameExpressionState {}

pub struct CallExpressionState {}

pub struct IfExpressionState {}

pub struct LetExpressionState {}

pub struct UnaryExpressionState {}

pub struct BinaryExpressionState {}

/// vnames

pub enum Vname {
    SimpleVname(Box<SimpleVnameState>),
    SubscriptVname(Box<SubscriptVnameState>),
    DotVname(Box<DotVnameState>),
}

impl PartialEq for Vname {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Vname {}

pub struct SimpleVnameState {}

pub struct SubscriptVnameState {}

pub struct DotVnameState {}

/// type denoters

pub enum TypeDenoter {}

/// parameters

pub enum FormalParameterSequence {
    SingleFormalParameterSequence(Box<SingleFormalParameterSequenceState>),
    MultipleFormalParameterSequence(Box<MultipleFormalParameterSequenceState>),
}

impl PartialEq for FormalParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for FormalParameterSequence {}

pub struct SingleFormalParameterSequenceState {
    pub fp: FormalParameter,
    pub common_state: CommonState,
}

impl SingleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter) -> Self {
        SingleFormalParameterSequenceState {
            fp: fp,
            common_state: CommonState::default(),
        }
    }
}

pub struct MultipleFormalParameterSequenceState {
    pub fp: FormalParameter,
    pub fps: FormalParameterSequence,
    pub common_state: CommonState,
}

impl MultipleFormalParameterSequenceState {
    pub fn new(fp: FormalParameter, fps: FormalParameterSequence) -> Self {
        MultipleFormalParameterSequenceState {
            fp: fp,
            fps: fps,
            common_state: CommonState::default(),
        }
    }
}

pub enum FormalParameter {
    VarFormalParameter(Box<VarFormalParameterState>),
    ConstFormalParameter(Box<ConstFormalParameterState>),
    ProcFormalParameter(Box<ProcFormalParameterState>),
    FuncFormalParameter(Box<FuncFormalParameterState>),
}

pub struct VarFormalParameterState {
    pub id: Identifier,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl VarFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        VarFormalParameterState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

pub struct ConstFormalParameterState {
    pub id: Identifier,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl ConstFormalParameterState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        ConstFormalParameterState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

pub struct ProcFormalParameterState {
    pub id: Identifier,
    pub fps: FormalParameterSequence,
    pub common_state: CommonState,
}

impl ProcFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence) -> Self {
        ProcFormalParameterState {
            id: id,
            fps: fps,
            common_state: CommonState::default(),
        }
    }
}

pub struct FuncFormalParameterState {
    pub id: Identifier,
    pub fps: FormalParameterSequence,
    pub td: TypeDenoter,
    pub common_state: CommonState,
}

impl FuncFormalParameterState {
    pub fn new(id: Identifier, fps: FormalParameterSequence, td: TypeDenoter) -> Self {
        FuncFormalParameterState {
            id: id,
            fps: fps,
            td: td,
            common_state: CommonState::default(),
        }
    }
}

pub enum ActualParameterSequence {
    SingleActualParamterSequence(Box<SingleActualParamterSequenceState>),
    MultipleActualParameterSequence(Box<MultipleActualParameterSequenceState>),
}

impl PartialEq for ActualParameterSequence {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for ActualParameterSequence {}

pub struct SingleActualParamterSequenceState {
    pub ap: ActualParameter,
    pub common_state: CommonState,
}

impl SingleActualParamterSequenceState {
    pub fn new(ap: ActualParameter) -> Self {
        SingleActualParamterSequenceState {
            ap: ap,
            common_state: CommonState::default(),
        }
    }
}

pub struct MultipleActualParameterSequenceState {
    pub ap: ActualParameter,
    pub aps: ActualParameterSequence,
    pub common_state: CommonState,
}

impl MultipleActualParameterSequenceState {
    pub fn new(ap: ActualParameter, aps: ActualParameterSequence) -> Self {
        MultipleActualParameterSequenceState {
            ap: ap,
            aps: aps,
            common_state: CommonState::default(),
        }
    }
}

pub enum ActualParameter {
    VarActualParameter(Box<VarActualParameterState>),
    ConstActualParameter(Box<ConstActualParameterState>),
    ProcActualParameter(Box<ProcActualParameterState>),
    FuncActualParameter(Box<FuncActualParameterState>),
}

pub struct VarActualParameterState {
    pub vname: Vname,
    pub common_state: CommonState,
}

impl VarActualParameterState {
    pub fn new(vname: Vname) -> Self {
        VarActualParameterState {
            vname: vname,
            common_state: CommonState::default(),
        }
    }
}

pub struct ConstActualParameterState {
    pub expr: Expression,
    pub common_state: CommonState,
}

impl ConstActualParameterState {
    pub fn new(expr: Expression) -> Self {
        ConstActualParameterState {
            expr: expr,
            common_state: CommonState::default(),
        }
    }
}

pub struct ProcActualParameterState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl ProcActualParameterState {
    pub fn new(id: Identifier) -> Self {
        ProcActualParameterState {
            id: id,
            common_state: CommonState::default(),
        }
    }
}

pub struct FuncActualParameterState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl FuncActualParameterState {
    pub fn new(id: Identifier) -> Self {
        FuncActualParameterState {
            id: id,
            common_state: CommonState::default(),
        }
    }
}

/// arrays

/// records

pub struct IntegerLiteral {}

impl IntegerLiteral {}

pub struct CharacterLiteral {}

impl CharacterLiteral {}

// TODO: factor out the type denoter and decl
pub struct Identifier {
    pub spelling: String,
    pub td: TypeDenoter,
    pub decl: Declaration,
    pub common_state: CommonState,
}

impl Identifier {
    pub fn new(spelling: &str, td: TypeDenoter, decl: Declaration) -> Self {
        Identifier {
            spelling: String::from(spelling),
            td: td,
            decl: decl,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.spelling == other.spelling
    }
}

impl Eq for Identifier {}

pub struct Operator {}

impl Operator {}