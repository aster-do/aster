use chrono::Utc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Billable {
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
}
