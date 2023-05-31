use chrono::Utc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metric {
    corelation_id: Option<String>,
    name: String,
    timestamp: chrono::DateTime<Utc>,
    value: f64,
}
