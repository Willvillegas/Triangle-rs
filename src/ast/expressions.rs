//! expression asts

use super::arrays::ArrayAggregate;
use super::declarations::Declaration;
use super::parameters::ActualParameterSequence;
use super::primitives::{CharacterLiteral, Identifier, IntegerLiteral, Operator};
use super::records::RecordAggregate;
use super::vnames::Vname;
use super::{Ast, AstVisitor, CommonState};

pub enum Expression {
    EmptyExpression,
    IntegerExpression(IntegerExpressionState),
    CharacterExpression(CharacterExpressionState),
    VnameExpression(VnameExpressionState),
    CallExpression(CallExpressionState),
    IfExpression(IfExpressionState),
    LetExpression(LetExpressionState),
    UnaryExpression(UnaryExpressionState),
    BinaryExpression(BinaryExpressionState),
    ArrayExpression(ArrayExpressionState),
    RecordExpression(RecordExpressionState),
}

impl Ast for Expression {
    fn accept(&mut self, visitor: &dyn AstVisitor) {
        visitor.visit_expression(self);
    }
}

// todo - fill this out
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        use Expression::*;
        //match (self, other) {
        //    (EmptyExpression, EmptyExpression) => true,
        //    (IntegerExpression(ref ies1), IntegerExpression(ref ies2)) => ines1 == ies2,
        //    (CharacterExpression(ref ches1), CharacterExpression(ref ches2)) => *ches1 == *ches2,
        //    (VnameExpression(ref ves1), VnameExpression(ref ves2)) => ves1 == ves2,
        //    (CallExpression(ref ces1), CallExpression(ref ces2)) => ces1 == ces2,
        //    (IfExpression(ref ies1), IfExpression(ref ies2)) => ies1 == ies2,
        //    (CharacterExpression(ref ces1), CharacterExpression(ref ces2)) => ces1 == ces2,
        //    (CharacterExpression(ref ces1), CharacterExpression(ref ces2)) => ces1 == ces2,
        //    (CharacterExpression(ref ces1), CharacterExpression(ref ces2)) => ces1 == ces2,
        //    (ArrayExpression(ref aes1), ArrayExpression(ref aes2)) => aes1 == aes2,
        //    (RecordExpression(ref res1), RecordExpression(ref res2)) => res1 == res2
        //    (_, _) => false,
        //}
        true
    }
}

impl Eq for Expression {}

pub struct IntegerExpressionState {
    il: IntegerLiteral,
    common_state: CommonState,
}

impl IntegerExpressionState {
    pub fn new(il: IntegerLiteral) -> Self {
        IntegerExpressionState {
            il: il,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for IntegerExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.il == other.il
    }
}

impl Eq for IntegerExpressionState {}

pub struct CharacterExpressionState {
    cl: CharacterLiteral,
    common_state: CommonState,
}

impl CharacterExpressionState {
    pub fn new(cl: CharacterLiteral) -> Self {
        CharacterExpressionState {
            cl: cl,
            common_state: CommonState::default(),
        }
    }
}

pub struct VnameExpressionState {
    vname: Vname,
    common_state: CommonState,
}

impl VnameExpressionState {
    pub fn new(vname: Vname) -> Self {
        VnameExpressionState {
            vname: vname,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for VnameExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.vname == other.vname
    }
}

impl Eq for VnameExpressionState {}

pub struct CallExpressionState {
    id: Identifier,
    aps: ActualParameterSequence,
    common_state: CommonState,
}

impl CallExpressionState {
    pub fn new(id: Identifier, aps: ActualParameterSequence) -> Self {
        CallExpressionState {
            id: id,
            aps: aps,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for CallExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.aps == other.aps
    }
}

impl Eq for CallExpressionState {}

pub struct IfExpressionState {
    expr1: Box<Expression>,
    expr2: Box<Expression>,
    expr3: Box<Expression>,
    common_state: CommonState,
}

impl IfExpressionState {
    pub fn new(expr1: Expression, expr2: Expression, expr3: Expression) -> Self {
        IfExpressionState {
            expr1: Box::new(expr1),
            expr2: Box::new(expr2),
            expr3: Box::new(expr3),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for IfExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.expr1 == other.expr1 && self.expr2 == other.expr2 && self.expr3 == other.expr3
    }
}

impl Eq for IfExpressionState {}

pub struct LetExpressionState {
    decl: Box<Declaration>,
    expr: Box<Expression>,
    common_state: CommonState,
}

impl LetExpressionState {
    pub fn new(decl: Declaration, expr: Expression) -> Self {
        LetExpressionState {
            decl: Box::new(decl),
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for LetExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.decl == other.decl && self.expr == other.expr
    }
}

impl Eq for LetExpressionState {}

pub struct UnaryExpressionState {
    op: Operator,
    expr: Box<Expression>,
    common_state: CommonState,
}

impl UnaryExpressionState {
    pub fn new(op: Operator, expr: Expression) -> Self {
        UnaryExpressionState {
            op: op,
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for UnaryExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.expr == other.expr
    }
}

impl Eq for UnaryExpressionState {}

pub struct BinaryExpressionState {
    expr1: Box<Expression>,
    op: Operator,
    expr2: Box<Expression>,
    common_state: CommonState,
}

impl BinaryExpressionState {
    pub fn new(expr1: Expression, op: Operator, expr2: Expression) -> Self {
        BinaryExpressionState {
            expr1: Box::new(expr1),
            op: op,
            expr2: Box::new(expr2),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for BinaryExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.expr1 == other.expr1 && self.op == other.op && self.expr2 == other.expr2
    }
}

impl Eq for BinaryExpressionState {}

pub struct ArrayExpressionState {
    aa: ArrayAggregate,
    elem_count: usize,
    common_state: CommonState,
}

impl ArrayExpressionState {
    pub fn new(aa: ArrayAggregate, elem_count: usize) -> Self {
        ArrayExpressionState {
            aa: aa,
            elem_count: elem_count,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for ArrayExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.aa == other.aa && self.elem_count == other.elem_count
    }
}

impl Eq for ArrayExpressionState {}

pub struct RecordExpressionState {
    ra: RecordAggregate,
    common_state: CommonState,
}

impl RecordExpressionState {
    pub fn new(ra: RecordAggregate) -> Self {
        RecordExpressionState {
            ra: ra,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for RecordExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.ra == other.ra
    }
}

impl Eq for RecordExpressionState {}
