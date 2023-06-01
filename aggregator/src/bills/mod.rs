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
    /// name of the metric
    pub name: String,
    /// Total price of bill
    pub price: i64,
    /// Timestamp at which the bill was issued
    pub timestamp: DateTime<Utc>,
    /// Value per unit of the metric
    pub value: f64,
    /// Whether the bill has been treated or not
    pub treated: bool,
}

pub struct _BillPerMetric {
    pub id: Option<Uuid>,
    /// name of the metric
    pub name: String,
    /// total price of bill
    pub price: i64,
}
