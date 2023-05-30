use std::error::Error;

pub mod billable_controller;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

///
/// Represents a billable originating from the billable kafka topic
/// TODO
pub struct Billable {}

///
/// This represents the total of all billables from a certain time period
/// TODO
pub struct AggregedBillable {}