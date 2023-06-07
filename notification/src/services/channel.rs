use crate::models::channel::NotificationChannel;

#[derive(Debug)]
pub struct ChannelService {
    //Config & stateful info
}

impl ChannelService {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            //Config & stateful info
        })
    }

    pub fn _create_channel(
        &self,
        channel: NotificationChannel,
    ) -> Result<NotificationChannel, anyhow::Error> {
        //TODO Create channel
        Ok(channel)
    }

    pub fn _update_channel(
        &self,
        _channel: NotificationChannel,
    ) -> Result<Option<NotificationChannel>, anyhow::Error> {
        //TODO Update channel
        Ok(None)
    }

    pub fn _delete_channel(&self, _channel_id: String) -> Result<(), anyhow::Error> {
        //TODO Delete channel
        Ok(())
    }

    pub fn get_channel(
        &self,
        _channel_id: String,
    ) -> Result<Option<NotificationChannel>, anyhow::Error> {
        //TODO Get channel
        Ok(None)
    }

    pub fn _get_channels(&self) -> Result<Vec<NotificationChannel>, anyhow::Error> {
        //TODO Get channels
        Ok(vec![])
    }
}
