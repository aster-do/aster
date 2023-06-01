#[derive(Debug)]
pub struct NotificationChannel {
    _name: String,
    _channel_type: ChannelType,
    _recipient: String, // email, phone number, webhook url, etc.
}

#[derive(Debug)]
pub enum ChannelType {
    Email,
    Slack,
    Webhook,
    Phone,
}
