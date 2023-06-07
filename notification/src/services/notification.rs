use anyhow::anyhow;
use log::info;

use crate::models::{
    channel::{ChannelType, NotificationChannel},
    notification::Notification,
};

use super::channel::ChannelService;

#[derive(Debug)]
pub struct Notificationservice {
    //Config & stateful info
    channel_service: ChannelService,
}

impl Notificationservice {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            //Config & stateful info
            channel_service: ChannelService::new()?,
        })
    }

    pub fn handle_notification(&self, notification: Notification) -> Result<(), anyhow::Error> {
        let channel = self
            .channel_service
            .get_channel(notification.notification_channel_id)?
            .ok_or_else(|| anyhow!("Channel not found when handling notification"))?;

        self.send_notification(channel, notification.message)?;

        Ok(())
    }

    fn send_notification(
        &self,
        channel: NotificationChannel,
        message: String,
    ) -> Result<(), anyhow::Error> {
        match channel.channel_type {
            ChannelType::Email => {
                //TODO Send email
            }
            ChannelType::Slack => {
                //TODO Send slack message
            }
            ChannelType::Webhook => {
                //TODO Send webhook
            }
            ChannelType::Phone => {
                //TODO Send phone message
            }
            ChannelType::Console => {
                info!("Received notification: {:?}", message)
            }
        }
        Ok(())
    }
}
