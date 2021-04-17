//! array asts

use super::expressions::Expression;
use super::typedenoters::TypeDenoter;
use super::CommonState;

pub enum ArrayAggregate {
    SingleArrayAggregate(Box<SingleArrayAggregateState>),
    MultipleArrayAggregate(Box<MultipleArrayAggregateState>),
}

impl PartialEq for ArrayAggregate {
    fn eq(&self, other: &Self) -> bool {
        true // todo
    }
}

pub struct SingleArrayAggregateState {
    expr: Expression,
    td: Option<TypeDenoter>,
    elem_count: usize,
    common_state: CommonState,
}

impl SingleArrayAggregateState {
    pub fn new(expr: Expression) -> Self {
        SingleArrayAggregateState {
            expr: expr,
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

pub struct MultipleArrayAggregateState {
    expr: Expression,
    aa: ArrayAggregate,
    td: Option<TypeDenoter>,
    elem_count: usize,
    common_state: CommonState,
}

impl MultipleArrayAggregateState {
    pub fn new(expr: Expression, aa: ArrayAggregate) -> Self {
        MultipleArrayAggregateState {
            expr: expr,
            aa: aa,
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
