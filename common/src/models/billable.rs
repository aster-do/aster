use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Billable {
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
}

#[derive(sqlx::FromRow)]
pub struct BillableSQL {
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
    pub treated: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::FromRow)]
pub struct BillableAggregate {
    pub name: String,                     // composite
    pub timestamp: chrono::DateTime<Utc>, // key
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: f64,
    pub sum: f64,
}

/// OAT = Of All Times: this is the aggregation of all the billables since launch
#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::FromRow)]
pub struct BillableAggregateOAT {
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: f64,
    pub sum: f64,
}
