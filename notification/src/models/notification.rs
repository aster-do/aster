use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Notification {
    _notification_channel_id: String,
    _status: NotificationStatus,
}

impl Notification {
    pub fn new(_notification_channel_id: String) -> Self {
        Self {
            _notification_channel_id,
            _status: NotificationStatus::Sent(Utc::now()),
        }
    }
}

#[derive(Debug)]
pub enum NotificationStatus {
    Sent(DateTime<Utc>),   // timestamp
    Failed(DateTime<Utc>), // timestamp
}
