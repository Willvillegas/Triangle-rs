//! aggregate asts

use super::expressions::Expression;
use super::primitives::Identifier;
use super::typedenoters::TypeDenoter;
use super::{Ast, AstObject, AstVisitor, CommonState};

#[derive(Debug)]
pub enum ArrayAggregate {
    SingleArrayAggregate(SingleArrayAggregateState),
    MultipleArrayAggregate(MultipleArrayAggregateState),
}

impl PartialEq for ArrayAggregate {
    fn eq(&self, other: &Self) -> bool {
        use ArrayAggregate::*;

        match (self, other) {
            (SingleArrayAggregate(ref sagg1), SingleArrayAggregate(sagg2)) => sagg1 == sagg2,
            (MultipleArrayAggregate(ref magg1), MultipleArrayAggregate(ref magg2)) => {
                magg1 == magg2
            }
            (_, __) => false,
        }
    }
}

impl Eq for ArrayAggregate {}

impl Ast for ArrayAggregate {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use ArrayAggregate::*;

        match *self {
            SingleArrayAggregate(ref mut saa) => saa.accept(visitor, arg),
            MultipleArrayAggregate(ref mut raa) => raa.accept(visitor, arg),
        }
    }
}

#[derive(Debug)]
pub struct SingleArrayAggregateState {
    pub expr: Box<Expression>,
    pub td: Option<Box<TypeDenoter>>,
    pub elem_count: usize,
    pub common_state: CommonState,
}

impl SingleArrayAggregateState {
    pub fn new(expr: Expression) -> Self {
        SingleArrayAggregateState {
            expr: Box::new(expr),
            td: None,
            elem_count: 0,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SingleArrayAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

impl Eq for SingleArrayAggregateState {}

impl Ast for SingleArrayAggregateState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_array_aggregate(self, arg)
    }
}

#[derive(Debug)]
pub struct MultipleArrayAggregateState {
    pub expr: Box<Expression>,
    pub aa: Box<ArrayAggregate>,
    pub td: Option<Box<TypeDenoter>>,
    pub elem_count: usize,
    pub common_state: CommonState,
}

impl MultipleArrayAggregateState {
    pub fn new(expr: Expression, aa: ArrayAggregate) -> Self {
        MultipleArrayAggregateState {
            expr: Box::new(expr),
            aa: Box::new(aa),
            td: None,
            elem_count: 0,
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for MultipleArrayAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.aa == other.aa
    }
}

impl Eq for MultipleArrayAggregateState {}

impl Ast for MultipleArrayAggregateState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_array_aggregate(self, arg)
    }
}

#[derive(Debug)]
pub enum RecordAggregate {
    SingleRecordAggregate(SingleRecordAggregateState),
    MultipleRecordAggregate(MultipleRecordAggregateState),
}

impl PartialEq for RecordAggregate {
    fn eq(&self, other: &Self) -> bool {
        use RecordAggregate::*;

        match (self, other) {
            (SingleRecordAggregate(ref sagg1), SingleRecordAggregate(ref sagg2)) => sagg1 == sagg2,
            (MultipleRecordAggregate(ref magg1), MultipleRecordAggregate(ref magg2)) => {
                magg1 == magg2
            }
            (_, __) => false,
        }
    }
}

impl Eq for RecordAggregate {}

impl Ast for RecordAggregate {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        use RecordAggregate::*;

        match *self {
            SingleRecordAggregate(ref mut sra) => sra.accept(visitor, arg),
            MultipleRecordAggregate(ref mut mra) => mra.accept(visitor, arg),
        }
    }
}

#[derive(Debug)]
pub struct SingleRecordAggregateState {
    pub id: Identifier,
    pub expr: Box<Expression>,
    pub common_state: CommonState,
}

impl SingleRecordAggregateState {
    pub fn new(id: Identifier, expr: Expression) -> Self {
        SingleRecordAggregateState {
            id: id,
            expr: Box::new(expr),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for SingleRecordAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Eq for SingleRecordAggregateState {}

impl Ast for SingleRecordAggregateState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_record_aggregate(self, arg)
    }
}

#[derive(Debug)]
pub struct MultipleRecordAggregateState {
    pub id: Identifier,
    pub expr: Box<Expression>,
    pub ra: Box<RecordAggregate>,
    pub common_state: CommonState,
}

impl MultipleRecordAggregateState {
    pub fn new(id: Identifier, expr: Expression, ra: RecordAggregate) -> Self {
        MultipleRecordAggregateState {
            id: id,
            expr: Box::new(expr),
            ra: Box::new(ra),
            common_state: CommonState::default(),
        }
    }
}

impl PartialEq for MultipleRecordAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr && self.ra == other.ra
    }
}

impl Eq for MultipleRecordAggregateState {}

impl Ast for MultipleRecordAggregateState {
    fn accept(&mut self, visitor: &dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_record_aggregate(self, arg)
    }
}
