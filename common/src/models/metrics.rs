use chrono::Utc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metric {
    pub corelation_id: Option<String>,
    pub name: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
}
