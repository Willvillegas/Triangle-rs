//! aggregate asts
//!
//! These represent aggregate expressions like array expressions
//! and record expressions.

use super::expressions::Expression;
use super::primitives::Identifier;
use super::typedenoters::TypeDenoter;
use super::{Ast, AstObject, AstVisitor, CommonState};
use crate::scanner::SourcePosition;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ArrayAggregate {
    SingleArrayAggregate(SingleArrayAggregateState),
    MultipleArrayAggregate(MultipleArrayAggregateState),
}

impl ArrayAggregate {
    pub fn get_single_array_aggregate(&self) -> Option<&SingleArrayAggregateState> {
        match *self {
            ArrayAggregate::SingleArrayAggregate(ref single) => Some(&single),
            _ => None,
        }
    }

    pub fn get_multiple_array_aggregate(&self) -> Option<&MultipleArrayAggregateState> {
        match *self {
            ArrayAggregate::MultipleArrayAggregate(ref multiple) => Some(&multiple),
            _ => None,
        }
    }
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

impl fmt::Display for ArrayAggregate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ArrayAggregate::*;

        match *self {
            SingleArrayAggregate(ref aa) => write!(f, "SingleArrayAggregate({})", aa),
            MultipleArrayAggregate(ref aa) => write!(f, "MultipleArrayAggregate({})", aa),
        }
    }
}

impl Ast for ArrayAggregate {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        use ArrayAggregate::*;

        match *self {
            SingleArrayAggregate(ref mut saa) => saa.accept(visitor, arg),
            MultipleArrayAggregate(ref mut raa) => raa.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(expr: Expression, position: SourcePosition) -> Self {
        let mut saa = SingleArrayAggregateState::new(expr);
        saa.common_state.position = position;
        saa
    }
}

impl PartialEq for SingleArrayAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

impl Eq for SingleArrayAggregateState {}

impl fmt::Display for SingleArrayAggregateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SingleArrayAggregateState::new({})", self.expr)
    }
}

impl Ast for SingleArrayAggregateState {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_array_aggregate(self, arg)
    }
}

#[derive(Debug, Clone)]
pub struct MultipleArrayAggregateState {
    pub expr: Box<Expression>,
    pub aa: Box<ArrayAggregate>,
    pub td: Option<TypeDenoter>,
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

    pub fn new_with_position(
        expr: Expression,
        aa: ArrayAggregate,
        position: SourcePosition,
    ) -> Self {
        let mut maa = MultipleArrayAggregateState::new(expr, aa);
        maa.common_state.position = position;
        maa
    }
}

impl PartialEq for MultipleArrayAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.aa == other.aa
    }
}

impl Eq for MultipleArrayAggregateState {}

impl fmt::Display for MultipleArrayAggregateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultipleArrayAggregateState::new({}, {})",
            self.expr, self.aa
        )
    }
}

impl Ast for MultipleArrayAggregateState {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_array_aggregate(self, arg)
    }
}

#[derive(Debug, Clone)]
pub enum RecordAggregate {
    SingleRecordAggregate(SingleRecordAggregateState),
    MultipleRecordAggregate(MultipleRecordAggregateState),
}

impl RecordAggregate {
    pub fn get_single_record_aggregate(&self) -> Option<&SingleRecordAggregateState> {
        match *self {
            RecordAggregate::SingleRecordAggregate(ref single) => Some(&single),
            _ => None,
        }
    }

    pub fn get_multiple_record_aggregate(&self) -> Option<&MultipleRecordAggregateState> {
        match *self {
            RecordAggregate::MultipleRecordAggregate(ref multiple) => Some(&multiple),
            _ => None,
        }
    }
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

impl fmt::Display for RecordAggregate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RecordAggregate::*;

        match *self {
            SingleRecordAggregate(ref ra) => write!(f, "SingleRecordAggregate({})", ra),
            MultipleRecordAggregate(ref ra) => write!(f, "MultipleRecordAggregate({})", ra),
        }
    }
}

impl Ast for RecordAggregate {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        use RecordAggregate::*;

        match *self {
            SingleRecordAggregate(ref mut sra) => sra.accept(visitor, arg),
            MultipleRecordAggregate(ref mut mra) => mra.accept(visitor, arg),
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(id: Identifier, expr: Expression, position: SourcePosition) -> Self {
        let mut sra = SingleRecordAggregateState::new(id, expr);
        sra.common_state.position = position;
        sra
    }
}

impl PartialEq for SingleRecordAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr
    }
}

impl Eq for SingleRecordAggregateState {}

impl fmt::Display for SingleRecordAggregateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SingleRecordAggregateState::new({}, {})",
            self.id, self.expr
        )
    }
}

impl Ast for SingleRecordAggregateState {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_single_record_aggregate(self, arg)
    }
}

#[derive(Debug, Clone)]
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

    pub fn new_with_position(
        id: Identifier,
        expr: Expression,
        ra: RecordAggregate,
        position: SourcePosition,
    ) -> Self {
        let mut mra = MultipleRecordAggregateState::new(id, expr, ra);
        mra.common_state.position = position;
        mra
    }
}

impl PartialEq for MultipleRecordAggregateState {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.expr == other.expr && self.ra == other.ra
    }
}

impl Eq for MultipleRecordAggregateState {}

impl fmt::Display for MultipleRecordAggregateState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultipleRecordAggregateState::new({}, {}, {})",
            self.id, self.expr, self.ra
        )
    }
}

impl Ast for MultipleRecordAggregateState {
    fn accept(&mut self, visitor: &mut dyn AstVisitor, arg: AstObject) -> AstObject {
        visitor.visit_multiple_record_aggregate(self, arg)
    }
}
