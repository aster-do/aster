use common::models::billable::BillableAggregate;

pub mod aggregators;

pub struct AggregatesToBeWritten {
    _inserts: Vec<BillableAggregate>,
    _updates: Vec<BillableAggregate>,
}
