use anyhow::{Ok, Result};

use crate::services::channel::ChannelService;

pub struct ChannelController {
    //Config & stateful info
    _channel_service: ChannelService,
}

impl ChannelController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _channel_service: ChannelService::new()?,
        })
    }

    pub fn _start(&self) -> Result<()> {
        //TODO Start billable receiver
        Ok(())
    }
}

//TODO Implement graphql trait
