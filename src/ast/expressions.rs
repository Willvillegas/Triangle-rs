//! expression asts

use super::aggregates::{ArrayAggregate, RecordAggregate};
use super::declarations::Declaration;
use super::parameters::ActualParameterSequence;
use super::primitives::{CharacterLiteral, Identifier, IntegerLiteral, Operator};
use super::vnames::Vname;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum Expression {
    ArrayExpression(ArrayExpressionState),
    BinaryExpression(BinaryExpressionState),
    CallExpression(CallExpressionState),
    CharacterExpression(CharacterExpressionState),
    EmptyExpression(EmptyExpressionState),
    IfExpression(IfExpressionState),
    IntegerExpression(IntegerExpressionState),
    LetExpression(LetExpressionState),
    RecordExpression(RecordExpressionState),
    UnaryExpression(UnaryExpressionState),
    VnameExpression(VnameExpressionState),
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        use Expression::*;

        match (self, other) {
            (ArrayExpression(ref arr1), ArrayExpression(ref arr2)) => arr1 == arr2,
            (BinaryExpression(ref bexpr1), BinaryExpression(ref bexpr2)) => bexpr1 == bexpr2,
            (CallExpression(ref cexpr1), CallExpression(ref cexpr2)) => cexpr1 == cexpr2,
            (CharacterExpression(ref cexpr1), CharacterExpression(ref cexpr2)) => cexpr1 == cexpr2,
            (EmptyExpression(_), EmptyExpression(_)) => true,
            (IfExpression(ref ifexpr1), IfExpression(ref ifexpr2)) => ifexpr1 == ifexpr2,
            (IntegerExpression(ref iexpr1), IntegerExpression(ref iexpr2)) => iexpr1 == iexpr2,
            (LetExpression(ref lexpr1), LetExpression(ref lexpr2)) => lexpr1 == lexpr2,
            (RecordExpression(ref rec1), RecordExpression(ref rec2)) => rec1 == rec2,
            (UnaryExpression(ref unexpr1), UnaryExpression(ref unexpr2)) => unexpr1 == unexpr2,
            (VnameExpression(ref vexpr1), VnameExpression(ref vexpr2)) => vexpr1 == vexpr2,
            (_, __) => false,
        }
    }
}

impl Eq for Expression {}

impl Ast for Expression {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use Expression::*;

        match *self {
            ArrayExpression(ref mut arrayexpr) => arrayexpr.accept(visitor, arg),
            BinaryExpression(ref mut binexpr) => binexpr.accept(visitor, arg),
            CallExpression(ref mut callexpr) => callexpr.accept(visitor, arg),
            CharacterExpression(ref mut charexpr) => charexpr.accept(visitor, arg),
            EmptyExpression(ref mut emptyexpr) => emptyexpr.accept(visitor, arg),
            IfExpression(ref mut ifexpr) => ifexpr.accept(visitor, arg),
            IntegerExpression(ref mut intexpr) => intexpr.accept(visitor, arg),
            LetExpression(ref mut letexpr) => letexpr.accept(visitor, arg),
            RecordExpression(ref mut recexpr) => recexpr.accept(visitor, arg),
            UnaryExpression(ref mut unexpr) => unexpr.accept(visitor, arg),
            VnameExpression(ref mut vexpr) => vexpr.accept(visitor, arg),
        }
    }
}

#[derive(Debug)]
pub struct IntegerExpressionState {
    pub il: IntegerLiteral,
    pub common_state: CommonState,
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

impl Ast for IntegerExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_integer_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct CharacterExpressionState {
    pub cl: CharacterLiteral,
    pub common_state: CommonState,
}

impl CharacterExpressionState {
    pub fn new(cl: CharacterLiteral) -> Self {
        CharacterExpressionState {
            cl: cl,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for CharacterExpressionState {
    fn eq(&self, other: &Self) -> bool {
        self.cl == other.cl
    }
}

impl Eq for CharacterExpressionState {}

impl Ast for CharacterExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_character_expression(self, arg)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EmptyExpressionState;

impl Ast for EmptyExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_empty_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct VnameExpressionState {
    pub vname: Vname,
    pub common_state: CommonState,
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

impl Ast for VnameExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_vname_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct CallExpressionState {
    pub id: Identifier,
    pub aps: ActualParameterSequence,
    pub common_state: CommonState,
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

impl Ast for CallExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_call_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct IfExpressionState {
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
    pub expr3: Box<Expression>,
    pub common_state: CommonState,
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

impl Ast for IfExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_if_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct LetExpressionState {
    pub decl: Box<Declaration>,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
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

impl Ast for LetExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_let_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct UnaryExpressionState {
    pub op: Operator,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
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

impl Ast for UnaryExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_unary_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct BinaryExpressionState {
    pub expr1: Box<Expression>,
    pub op: Operator,
    pub expr2: Box<Expression>,
    pub common_state: CommonState,
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

impl Ast for BinaryExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_binary_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct ArrayExpressionState {
    pub aa: Box<ArrayAggregate>,
    pub elem_count: usize,
    pub common_state: CommonState,
}

impl ArrayExpressionState {
    pub fn new(aa: ArrayAggregate, elem_count: usize) -> Self {
        ArrayExpressionState {
            aa: Box::new(aa),
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

impl Ast for ArrayExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_array_expression(self, arg)
    }
}

#[derive(Debug)]
pub struct RecordExpressionState {
    pub ra: Box<RecordAggregate>,
    pub common_state: CommonState,
}

impl RecordExpressionState {
    pub fn new(ra: RecordAggregate) -> Self {
        RecordExpressionState {
            ra: Box::new(ra),
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

impl Ast for RecordExpressionState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_record_expression(self, arg)
    }
}
