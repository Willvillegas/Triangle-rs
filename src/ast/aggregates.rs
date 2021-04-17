//! aggregate asts

use super::expressions::Expression;
use super::typedenoters::TypeDenoter;
use super::CommonState;

#[derive(Debug)]
pub enum ArrayAggregate {
    SingleArrayAggregate(SingleArrayAggregateState),
    MultipleArrayAggregate(MultipleArrayAggregateState),
}

impl PartialEq for ArrayAggregate {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct SingleArrayAggregateState {
    expr: Box<Expression>,
    td: Option<Box<TypeDenoter>>,
    elem_count: usize,
    common_state: CommonState,
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

#[derive(Debug)]
pub struct MultipleArrayAggregateState {
    expr: Box<Expression>,
    aa: Box<ArrayAggregate>,
    td: Option<Box<TypeDenoter>>,
    elem_count: usize,
    common_state: CommonState,
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

#[derive(Debug)]
pub enum RecordAggregate {
    SingleRecordAggregate(SingleRecordAggregateState),
    MultipleRecordAggregate(MultipleRecordAggregateState),
}

impl PartialEq for RecordAggregate {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for RecordAggregate {}

#[derive(Debug)]
pub struct SingleRecordAggregateState {}

#[derive(Debug)]
pub struct MultipleRecordAggregateState {}
