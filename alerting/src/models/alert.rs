use chrono::{DateTime, Utc};

#[derive(Debug, Default, Clone)]
pub struct Alert {
    pub id: String,
    pub alerting_rule_id: String,
    pub value: f64,
    pub status: AlertStatus,
    pub notification_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum AlertStatus {
    _Triggered(DateTime<Utc>), // timestamp
    _Resolved(DateTime<Utc>),  // timestamp
}

impl Default for AlertStatus {
    fn default() -> Self {
        Self::_Triggered(Utc::now())
    }
}
