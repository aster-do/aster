use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Alert {
    pub id: String,
    pub alerting_rule_id: Option<String>,
    pub value: Option<f64>,
    pub status: Option<AlertStatus>,
    pub notification_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum AlertStatus {
    _Triggered(DateTime<Utc>), // timestamp
    _Resolved(DateTime<Utc>),  // timestamp
}
