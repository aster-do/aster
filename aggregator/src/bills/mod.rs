use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

pub mod billable_controller;

///
/// Represents a billable originating from the billable kafka topic
/// and stored in the database
#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::FromRow)]
pub struct StoredBillable {
    pub id: Uuid,
    pub name: String,
    pub price: i64,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}
