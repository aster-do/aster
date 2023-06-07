use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Notification {
    pub notification_channel_id: String,
    pub status: NotificationStatus,
    pub message: String,
}

impl Notification {
    pub fn new(notification_channel_id: String, message: String) -> Self {
        Self {
            notification_channel_id,
            status: NotificationStatus::Sent(Utc::now()),
            message,
        }
    }
}

#[derive(Debug)]
pub enum NotificationStatus {
    Sent(DateTime<Utc>),   // timestamp
    Failed(DateTime<Utc>), // timestamp
}
