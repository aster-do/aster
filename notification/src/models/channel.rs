#[derive(Debug)]
pub struct NotificationChannel {
    pub name: String,
    pub channel_type: ChannelType,
    pub recipient: String, // email, phone number, webhook url, etc.
}

#[derive(Debug)]
pub enum ChannelType {
    Email,
    Slack,
    Webhook,
    Phone,
    Console,
}
