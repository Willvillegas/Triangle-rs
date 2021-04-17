//! record asts

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
