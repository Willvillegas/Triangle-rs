//! vname asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::{Ast, AstObject, AstVisitor, CommonState};
use crate::scanner::SourcePosition;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Vname {
    DotVname(DotVnameState),
    SimpleVname(SimpleVnameState),
    SubscriptVname(SubscriptVnameState),
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

impl fmt::Display for Vname {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Vname::*;

        match *self {
            DotVname(ref vname) => write!(f, "DotVname({})", vname),
            SimpleVname(ref vname) => write!(f, "SimpleVname({})", vname),
            SubscriptVname(ref vname) => write!(f, "SubscriptVname({})", vname),
        }
    }
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

#[derive(Debug, Clone)]
pub struct SimpleVnameState {
    pub id: Identifier,
    pub common_state: CommonState,
}

impl SimpleVnameState {
    pub fn new(id: Identifier) -> Self {
        SimpleVnameState {
            id: id,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(id: Identifier, position: SourcePosition) -> Self {
        let mut svn = SimpleVnameState::new(id);
        svn.common_state.position = position;
        svn
    }
}

impl PartialEq for SimpleVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SimpleVnameState {}

impl fmt::Display for SimpleVnameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleVnameState::new({})", self.id)
    }
}

impl Ast for SimpleVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_simple_vname(self, arg)
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptVnameState {
    pub vname: Box<Vname>,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl SubscriptVnameState {
    pub fn new(vname: Vname, expr: Expression) -> Self {
        SubscriptVnameState {
            vname: Box::new(vname),
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(vname: Vname, expr: Expression, position: SourcePosition) -> Self {
        let mut svn = SubscriptVnameState::new(vname, expr);
        svn.common_state.position = position;
        svn
    }
}

impl PartialEq for SubscriptVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.expr == other.expr
    }
}

impl Eq for SubscriptVnameState {}

impl fmt::Display for SubscriptVnameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubscriptVnameState::new({}, {})", self.vname, self.expr)
    }
}

impl Ast for SubscriptVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_subscript_vname(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct DotVnameState {
    pub vname: Box<Vname>,
    pub id: Identifier,
    pub common_state: CommonState,
}

impl DotVnameState {
    pub fn new(vname: Vname, id: Identifier) -> Self {
        DotVnameState {
            vname: Box::new(vname),
            id: id,
            common_state: CommonState::default(),
        }
    }

    pub fn new_with_position(vname: Vname, id: Identifier, position: SourcePosition) -> Self {
        let mut dvn = DotVnameState::new(vname, id);
        dvn.common_state.position = position;
        dvn
    }
}

impl PartialEq for DotVnameState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname && self.id == other.id
    }
}

impl Eq for DotVnameState {}

impl fmt::Display for DotVnameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DotVnameState::new({}, {})", self.vname, self.id)
    }
}

impl Ast for DotVnameState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_dot_vname(self, arg)
    }
}
