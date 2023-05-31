use chrono::Utc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Billable {
    name: String,
    price: i64,
    timestamp: chrono::DateTime<Utc>,
    value: f64,
}
