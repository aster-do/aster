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
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
    pub treated: bool,
}
