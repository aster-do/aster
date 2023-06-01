use anyhow::{Ok, Result};

use crate::models::channel::NotificationChannel;

#[derive(Debug)]
pub struct ChannelService {
    //Config & stateful info
}

impl ChannelService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
        })
    }

    pub fn _create_channel(&self, channel: NotificationChannel) -> Result<NotificationChannel> {
        //TODO Create channel
        Ok(channel)
    }

    pub fn _update_channel(
        &self,
        _channel: NotificationChannel,
    ) -> Result<Option<NotificationChannel>> {
        //TODO Update channel
        Ok(None)
    }

    pub fn _delete_channel(&self, _channel: NotificationChannel) -> Result<()> {
        //TODO Delete channel
        Ok(())
    }

    pub fn _get_channel(
        &self,
        _channel: NotificationChannel,
    ) -> Result<Option<NotificationChannel>> {
        //TODO Get channel
        Ok(None)
    }

    pub fn _get_channels(&self) -> Result<Vec<NotificationChannel>> {
        //TODO Get channels
        Ok(vec![])
    }
}
