use anyhow::{Ok, Result};
use async_trait::async_trait;
use common::AsterService;
use log::debug;

mod controllers;
pub mod models;
pub mod services;

pub struct NotificationInterface {
    //Config & stateful info
    channel_controller: controllers::channel::ChannelController,
}

//TODO Implement AsterService trait
#[async_trait]
impl AsterService for NotificationInterface {
    const SERVICE_PORT: u16 = 0;

    async fn init(
        &mut self,
        _: &mut common::messaging::crossbeam::CrossbeamMessagingFactory,
    ) -> Result<()> {
        debug!("Initializing channel controller");
        self.channel_controller = controllers::channel::ChannelController::new()?;

        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        debug!("Starting servers");
        //TODO Start servers
        Ok(())
    }
}
