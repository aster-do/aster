use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Notification {
    _notification_channel_id: String,
    _status: NotificationStatus,
}

#[derive(Debug)]
pub enum NotificationStatus {
    Sent(DateTime<Utc>),   // timestamp
    Failed(DateTime<Utc>), // timestamp
}
