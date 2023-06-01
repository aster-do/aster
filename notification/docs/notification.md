# This file describes the notification API

## Notification channel configuration

```rust
struct NotificationChannel {
    name: String,
    channel_type: ChannelType,
    recipient: String, // email, phone number, webhook url, etc.
}

enum ChannelType {
    Email,
    Slack,
    Webhook,
    Phone,
}
```

## Notification structure

```rust
struct Notification {
    notification_channel_id: String,
    status: NotificationStatus,
}

enum NotificationStatus {
    Sent(DateTime<Utc>),   // timestamp
    Failed(DateTime<Utc>), // timestamp
}
```
