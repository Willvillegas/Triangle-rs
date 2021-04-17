//! declaration asts

use super::commands::Command;
use super::expressions::Expression;
use super::parameters::FormalParameterSequence;
use super::primitives::{Identifier, Operator};
use super::typedenoters::TypeDenoter;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum Declaration {
    BinaryOperatorDeclaration(BinaryOperatorDeclarationState),
    ConstDeclaration(ConstDeclarationState),
    FuncDeclaration(FuncDeclarationState),
    ProcDeclaration(ProcDeclarationState),
    SequentialDeclaration(SequentialDeclarationState),
    TypeDeclaration(TypeDeclarationState),
    UnaryOperatorDeclaration(UnaryOperatorDeclarationState),
    VarDeclaration(VarDeclarationState),
}

impl Ast for Declaration {
    fn accept(&mut self, visitor: &dyn AstVisitor) -> AstObject {
        visitor.visit_declaration(self)
    }
}

impl PartialEq for Declaration {
    fn eq(&self, other: &Self) -> bool {
        use Declaration::*;
        match (self, other) {
            (
                BinaryOperatorDeclaration(ref binopdecl1),
                BinaryOperatorDeclaration(ref binopdecl2),
            ) => binopdecl1 == binopdecl2,
            (ConstDeclaration(ref constdecl1), ConstDeclaration(constdecl2)) => {
                constdecl1 == constdecl2
            }
            (FuncDeclaration(ref funcdecl1), FuncDeclaration(funcdecl2)) => funcdecl1 == funcdecl2,
            (ProcDeclaration(ref procdecl1), ProcDeclaration(ref procdecl2)) => {
                procdecl1 == procdecl1
            }
            (SequentialDeclaration(ref seqdecl1), SequentialDeclaration(ref seqdecl2)) => {
                seqdecl1 == seqdecl2
            }
            (TypeDeclaration(ref typedecl1), TypeDeclaration(ref typedecl2)) => {
                typedecl1 == typedecl2
            }
            (UnaryOperatorDeclaration(ref unopdecl1), UnaryOperatorDeclaration(ref unopdecl2)) => {
                unopdecl1 == unopdecl2
            }
            (VarDeclaration(ref vardecl1), VarDeclaration(ref vardecl2)) => vardecl1 == vardecl2,
            (_, _) => false,
        }
    }
}

impl Eq for Declaration {}

#[derive(Debug)]
pub struct BinaryOperatorDeclarationState {
    arg1type: Box<TypeDenoter>,
    op: Operator,
    arg2type: Box<TypeDenoter>,
    restype: Box<TypeDenoter>,
    common_state: CommonState,
}

impl BinaryOperatorDeclarationState {
    pub fn new(
        arg1type: TypeDenoter,
        op: Operator,
        arg2type: TypeDenoter,
        restype: TypeDenoter,
    ) -> Self {
        BinaryOperatorDeclarationState {
            arg1type: Box::new(arg1type),
            op: op,
            arg2type: Box::new(arg2type),
            restype: Box::new(restype),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for BinaryOperatorDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.arg1type == other.arg1type
            && self.op == other.op
            && self.arg2type == other.arg2type
            && self.restype == other.restype
    }
}

impl Eq for BinaryOperatorDeclarationState {}

#[derive(Debug)]
pub struct UnaryOperatorDeclarationState {
    op: Operator,
    argtype: Box<TypeDenoter>,
    restype: Box<TypeDenoter>,
    common_state: CommonState,
}

impl UnaryOperatorDeclarationState {
    pub fn new(op: Operator, argtype: TypeDenoter, restype: TypeDenoter) -> Self {
        UnaryOperatorDeclarationState {
            op: op,
            argtype: Box::new(argtype),
            restype: Box::new(restype),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for UnaryOperatorDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.argtype == other.argtype && self.restype == other.restype
    }
}

impl Eq for UnaryOperatorDeclarationState {}

#[derive(Debug)]
pub struct ConstDeclarationState {
    id: Identifier,
    expr: Box<Expression>,
    common_state: CommonState,
}

impl ConstDeclarationState {
    pub fn new(id: Identifier, expr: Expression) -> Self {
        ConstDeclarationState {
            id: id,
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for ConstDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Eq for ConstDeclarationState {}

#[derive(Debug)]
pub struct VarDeclarationState {
    id: Identifier,
    td: Box<TypeDenoter>,
    common_state: CommonState,
}

impl VarDeclarationState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        VarDeclarationState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for VarDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

#[derive(Debug)]
pub struct ProcDeclarationState {
    id: Identifier,
    fps: Box<FormalParameterSequence>,
    cmd: Box<Command>,
    common_state: CommonState,
}

impl ProcDeclarationState {
    pub fn new(id: Identifier, fps: FormalParameterSequence, cmd: Command) -> Self {
        ProcDeclarationState {
            id: id,
            fps: Box::new(fps),
            cmd: Box::new(cmd),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for ProcDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.fps == other.fps && self.cmd == other.cmd
    }
}

impl Eq for ProcDeclarationState {}

#[derive(Debug)]
pub struct FuncDeclarationState {
    id: Identifier,
    fps: Box<FormalParameterSequence>,
    td: Box<TypeDenoter>,
    expr: Box<Expression>,
    common_state: CommonState,
}

impl FuncDeclarationState {
    pub fn new(
        id: Identifier,
        fps: FormalParameterSequence,
        td: TypeDenoter,
        expr: Expression,
    ) -> Self {
        FuncDeclarationState {
            id: id,
            fps: Box::new(fps),
            td: Box::new(td),
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for FuncDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.fps == other.fps
            && self.td == other.td
            && self.expr == other.expr
    }
}

impl Eq for FuncDeclarationState {}

#[derive(Debug)]
pub struct TypeDeclarationState {
    id: Identifier,
    td: Box<TypeDenoter>,
    common_state: CommonState,
}

impl TypeDeclarationState {
    pub fn new(id: Identifier, td: TypeDenoter) -> Self {
        TypeDeclarationState {
            id: id,
            td: Box::new(td),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for TypeDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.td == other.td
    }
}

impl Eq for TypeDeclarationState {}

#[derive(Debug)]
pub struct SequentialDeclarationState {
    decl1: Box<Declaration>,
    decl2: Box<Declaration>,
    common_state: CommonState,
}

impl SequentialDeclarationState {
    pub fn new(decl1: Declaration, decl2: Declaration) -> Self {
        SequentialDeclarationState {
            decl1: Box::new(decl1),
            decl2: Box::new(decl2),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SequentialDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.decl1 == other.decl1 && self.decl2 == other.decl2
    }
}
