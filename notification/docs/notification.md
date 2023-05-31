# This file describes the notification API

## Notification channel configuration

```rust
struct NotificationChannel {
    name: String,
    channelType: ChannelType,
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
    notificationChannelId: String,
    status: Status,
}

enum Status {
    Sent(u64), // timestamp
    Failed(u64), // timestamp
}
```
