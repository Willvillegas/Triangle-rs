//! record asts

pub enum RecordAggregate {
    SingleRecordAggregate(Box<SingleRecordAggregateState>),
    MultipleRecordAggregate(Box<MultipleRecordAggregate>),
}

impl PartialEq for RecordAggregate {
    fn eq(&self, other: &Self) -> bool {
        true // todo
    }
}

impl Eq for RecordAggregate {}

pub struct SingleRecordAggregateState {}

pub struct MultipleRecordAggregate {}
