use common::models::billable::BillableAggregate;

pub mod aggregators;

pub struct AggregatesToBeWritten {
    pub inserts: Vec<BillableAggregate>,
    pub updates: Vec<BillableAggregate>,
}
