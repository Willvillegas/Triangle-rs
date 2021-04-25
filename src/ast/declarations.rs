//! declaration asts

use super::commands::Command;
use super::expressions::Expression;
use super::parameters::FormalParameterSequence;
use super::primitives::{Identifier, Operator};
use super::typedenoters::TypeDenoter;
use super::{Ast, AstObject, AstVisitor, CommonState};
use crate::scanner::SourcePosition;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
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
                procdecl1 == procdecl2
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

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Declaration::*;

        match *self {
            BinaryOperatorDeclaration(ref decl) => write!(f, "BinaryOperatorDeclaration({})", decl),
            ConstDeclaration(ref decl) => write!(f, "ConstDeclaration({})", decl),
            FuncDeclaration(ref decl) => write!(f, "FuncDeclaration({})", decl),
            ProcDeclaration(ref decl) => write!(f, "ProcDeclaration({})", decl),
            SequentialDeclaration(ref decl) => write!(f, "SequentialDeclaration({})", decl),
            TypeDeclaration(ref decl) => write!(f, "TypeDeclaration({})", decl),
            UnaryOperatorDeclaration(ref decl) => write!(f, "UnaryOperatorDeclaration({})", decl),
            VarDeclaration(ref decl) => write!(f, "VarDeclaration({})", decl),
        }
    }
}

impl Ast for Declaration {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use Declaration::*;

        match *self {
            BinaryOperatorDeclaration(ref mut binopdecl) => binopdecl.accept(visitor, arg),
            ConstDeclaration(ref mut constdecl) => constdecl.accept(visitor, arg),
            FuncDeclaration(ref mut funcdecl) => funcdecl.accept(visitor, arg),
            ProcDeclaration(ref mut procdecl) => procdecl.accept(visitor, arg),
            SequentialDeclaration(ref mut seqdecl) => seqdecl.accept(visitor, arg),
            TypeDeclaration(ref mut typedecl) => typedecl.accept(visitor, arg),
            UnaryOperatorDeclaration(ref mut unopdecl) => unopdecl.accept(visitor, arg),
            VarDeclaration(ref mut vardecl) => vardecl.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOperatorDeclarationState {
    pub arg1_type: Arc<Mutex<TypeDenoter>>,
    pub op: Operator,
    pub arg2_type: Arc<Mutex<TypeDenoter>>,
    pub res_type: Arc<Mutex<TypeDenoter>>,
    pub common_state: CommonState,
}

impl BinaryOperatorDeclarationState {
    pub fn new(
        arg1_type: Arc<Mutex<TypeDenoter>>,
        op: Operator,
        arg2_type: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
    ) -> Self {
        BinaryOperatorDeclarationState {
            arg1_type: arg1_type,
            op: op,
            arg2_type: arg2_type,
            res_type: res_type,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        arg1_type: Arc<Mutex<TypeDenoter>>,
        op: Operator,
        arg2_type: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
        position: SourcePosition,
    ) -> Self {
        let mut binopdecl = BinaryOperatorDeclarationState::new(arg1_type, op, arg2_type, res_type);
        binopdecl.common_state.position = position;
        binopdecl
    }
}

impl PartialEq for BinaryOperatorDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        *self.arg1_type.lock().unwrap() == *other.arg1_type.lock().unwrap()
            && self.op == other.op
            && *self.arg2_type.lock().unwrap() == *other.arg2_type.lock().unwrap()
            && *self.res_type.lock().unwrap() == *other.res_type.lock().unwrap()
    }
}

impl Eq for BinaryOperatorDeclarationState {}

impl fmt::Display for BinaryOperatorDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BinaryOperatorDeclarationState::new({}, {}, {}, {})",
            *self.arg1_type.lock().unwrap(),
            self.op,
            *self.arg2_type.lock().unwrap(),
            *self.res_type.lock().unwrap()
        )
    }
}

impl Ast for BinaryOperatorDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_binary_operator_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOperatorDeclarationState {
    pub op: Operator,
    pub argtype: Arc<Mutex<TypeDenoter>>,
    pub res_type: Arc<Mutex<TypeDenoter>>,
    pub common_state: CommonState,
}

impl UnaryOperatorDeclarationState {
    pub fn new(
        op: Operator,
        argtype: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
    ) -> Self {
        UnaryOperatorDeclarationState {
            op: op,
            argtype: argtype,
            res_type: res_type,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        op: Operator,
        argtype: Arc<Mutex<TypeDenoter>>,
        res_type: Arc<Mutex<TypeDenoter>>,
        position: SourcePosition,
    ) -> Self {
        let mut unopdecl = UnaryOperatorDeclarationState::new(op, argtype, res_type);
        unopdecl.common_state.position = position;
        unopdecl
    }
}

impl PartialEq for UnaryOperatorDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
            && *self.argtype.lock().unwrap() == *other.argtype.lock().unwrap()
            && *self.res_type.lock().unwrap() == *other.res_type.lock().unwrap()
    }
}

impl Eq for UnaryOperatorDeclarationState {}

impl fmt::Display for UnaryOperatorDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UnaryOperatorDeclarationState::new({}, {}, {})",
            self.op,
            *self.argtype.lock().unwrap(),
            *self.res_type.lock().unwrap()
        )
    }
}

impl Ast for UnaryOperatorDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_unary_operator_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ConstDeclarationState {
    pub id: Identifier,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl ConstDeclarationState {
    pub fn new(id: Identifier, expr: Expression) -> Self {
        ConstDeclarationState {
            id: id,
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, expr: Expression, position: SourcePosition) -> Self {
        let mut constdecl = ConstDeclarationState::new(id, expr);
        constdecl.common_state.position = position;
        constdecl
    }
}

impl PartialEq for ConstDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Eq for ConstDeclarationState {}

impl fmt::Display for ConstDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstDeclarationState::new({}, {})", self.id, self.expr)
    }
}

impl Ast for ConstDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_const_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclarationState {
    pub id: Identifier,
    pub td: Arc<Mutex<TypeDenoter>>,
    pub common_state: CommonState,
}

impl VarDeclarationState {
    pub fn new(id: Identifier, td: Arc<Mutex<TypeDenoter>>) -> Self {
        VarDeclarationState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        id: Identifier,
        td: Arc<Mutex<TypeDenoter>>,
        position: SourcePosition,
    ) -> Self {
        let mut vardecl = VarDeclarationState::new(id, td);
        vardecl.common_state.position = position;
        vardecl
    }
}

impl PartialEq for VarDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && *self.td.lock().unwrap() == *other.td.lock().unwrap()
    }
}

impl Eq for VarDeclarationState {}

impl fmt::Display for VarDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "VarDeclarationState::new({}, {})",
            self.id,
            *self.td.lock().unwrap()
        )
    }
}

impl Ast for VarDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_var_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct ProcDeclarationState {
    pub id: Identifier,
    pub fps: Box<FormalParameterSequence>,
    pub cmd: Box<Command>,
    pub common_state: CommonState,
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

    pub fn new_with_position(
        id: Identifier,
        fps: FormalParameterSequence,
        cmd: Command,
        position: SourcePosition,
    ) -> Self {
        let mut procdecl = ProcDeclarationState::new(id, fps, cmd);
        procdecl.common_state.position = position;
        procdecl
    }
}

impl PartialEq for ProcDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.fps == other.fps && self.cmd == other.cmd
    }
}

impl Eq for ProcDeclarationState {}

impl fmt::Display for ProcDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProcDeclarationState::new({}, {}, {})",
            self.id, self.fps, self.cmd
        )
    }
}

impl Ast for ProcDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_proc_declaration(self, arg)
    }
}
#[derive(Debug, Clone)]
pub struct FuncDeclarationState {
    pub id: Identifier,
    pub fps: Box<FormalParameterSequence>,
    pub td: Arc<Mutex<TypeDenoter>>,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl FuncDeclarationState {
    pub fn new(
        id: Identifier,
        fps: FormalParameterSequence,
        td: Arc<Mutex<TypeDenoter>>,
        expr: Expression,
    ) -> Self {
        FuncDeclarationState {
            id: id,
            fps: Box::new(fps),
            td: td,
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        id: Identifier,
        fps: FormalParameterSequence,
        td: Arc<Mutex<TypeDenoter>>,
        expr: Expression,
        position: SourcePosition,
    ) -> Self {
        let mut funcdecl = FuncDeclarationState::new(id, fps, td, expr);
        funcdecl.common_state.position = position;
        funcdecl
    }
}

impl PartialEq for FuncDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.fps == other.fps
            && *self.td.lock().unwrap() == *other.td.lock().unwrap()
            && self.expr == other.expr
    }
}

impl Eq for FuncDeclarationState {}

impl fmt::Display for FuncDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FuncDeclarationState::new({}, {}, {}, {})",
            self.id,
            self.fps,
            *self.td.lock().unwrap(),
            self.expr
        )
    }
}

impl Ast for FuncDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_func_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct TypeDeclarationState {
    pub id: Identifier,
    pub td: Arc<Mutex<TypeDenoter>>,
    pub common_state: CommonState,
}

impl TypeDeclarationState {
    pub fn new(id: Identifier, td: Arc<Mutex<TypeDenoter>>) -> Self {
        TypeDeclarationState {
            id: id,
            td: td,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        id: Identifier,
        td: Arc<Mutex<TypeDenoter>>,
        position: SourcePosition,
    ) -> Self {
        let mut typedecl = TypeDeclarationState::new(id, td);
        typedecl.common_state.position = position;
        typedecl
    }
}

impl PartialEq for TypeDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && *self.td.lock().unwrap() == *other.td.lock().unwrap()
    }
}

impl Eq for TypeDeclarationState {}

impl fmt::Display for TypeDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TypeDeclarationState::new({}, {})",
            self.id,
            *self.td.lock().unwrap()
        )
    }
}

impl Ast for TypeDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_type_declaration(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct SequentialDeclarationState {
    pub decl1: Box<Declaration>,
    pub decl2: Box<Declaration>,
    pub common_state: CommonState,
}

impl SequentialDeclarationState {
    pub fn new(decl1: Declaration, decl2: Declaration) -> Self {
        SequentialDeclarationState {
            decl1: Box::new(decl1),
            decl2: Box::new(decl2),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(
        decl1: Declaration,
        decl2: Declaration,
        position: SourcePosition,
    ) -> Self {
        let mut seqdecl = SequentialDeclarationState::new(decl1, decl2);
        seqdecl.common_state.position = position;
        seqdecl
    }
}

impl PartialEq for SequentialDeclarationState {
    fn eq(&self, other: &Self) -> bool {
        self.decl1 == other.decl1 && self.decl2 == other.decl2
    }
}

impl Eq for SequentialDeclarationState {}

impl fmt::Display for SequentialDeclarationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SequentialDeclarationState::new({}, {})",
            self.decl1, self.decl2
        )
    }
}

impl Ast for SequentialDeclarationState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_sequential_declaration(self, arg)
    }
}
