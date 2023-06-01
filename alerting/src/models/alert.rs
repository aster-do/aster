use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct _Alert {
    _alerting_rule_id: String,
    _value: f64,
    _status: _AlertStatus,
    _notification_id: String,
}

#[derive(Debug)]
pub enum _AlertStatus {
    _Triggered(DateTime<Utc>), // timestamp
    _Resolved(DateTime<Utc>),  // timestamp
}
