//! vname asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum Vname {
    DotVname(DotVnameState),
    SimpleVname(SimpleVnameState),
    SubscriptVname(SubscriptVnameState),
}

impl Ast for Vname {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use Vname::*;

        match *self {
            DotVname(ref mut dotvname) => dotvname.accept(visitor, arg),
            SimpleVname(ref mut simplevname) => simplevname.accept(visitor, arg),
            SubscriptVname(ref mut subscriptvname) => subscriptvname.accept(visitor, arg),
        }
    }
}

impl PartialEq for Vname {
    fn eq(&self, other: &Self) -> bool {
        use Vname::*;
        match (self, other) {
            (DotVname(ref dotvn1), DotVname(ref dotvn2)) => dotvn1 == dotvn2,
            (SimpleVname(ref simplevn1), SimpleVname(ref simplevn2)) => simplevn1 == simplevn2,
            (SubscriptVname(ref subsvn1), SubscriptVname(ref subsvn2)) => subsvn1 == subsvn2,
            (_, _) => false,
        }
    }
}

impl Eq for Vname {}

#[derive(Debug)]
pub struct SimpleVnameState {
    id: Identifier,
    common_state: CommonState,
}

impl SimpleVnameState {
    pub fn new(id: Identifier) -> Self {
        SimpleVnameState {
            id: id,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SimpleVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SimpleVnameState {}

impl Ast for SimpleVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_simple_vname(self, arg)
    }
}
#[derive(Debug)]
pub struct SubscriptVnameState {
    vname: Box<Vname>,
    expr: Box<Expression>,
    common_state: CommonState,
}

impl SubscriptVnameState {
    pub fn new(vname: Vname, expr: Expression) -> Self {
        SubscriptVnameState {
            vname: Box::new(vname),
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SubscriptVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.expr == other.expr
    }
}

impl Eq for SubscriptVnameState {}

impl Ast for SubscriptVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_subscript_vname(self, arg)
    }
}
#[derive(Debug)]
pub struct DotVnameState {
    vname: Box<Vname>,
    id: Identifier,
    common_state: CommonState,
}

impl DotVnameState {
    pub fn new(vname: Vname, id: Identifier) -> Self {
        DotVnameState {
            vname: Box::new(vname),
            id: id,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for DotVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.id == other.id
    }
}

impl Eq for DotVnameState {}

impl Ast for DotVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_dot_vname(self, arg)
    }
}
