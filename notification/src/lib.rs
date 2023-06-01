use std::net::{IpAddr, SocketAddr};

use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use common::AsterService;
use controllers::channel::ChannelController;
use log::debug;

mod controllers;
pub mod models;
pub mod services;

#[derive(Default)]
pub struct NotificationInterface {
    //Config & stateful info
    channel_controller: Option<ChannelController>,
}

const PORT: u16 = 3032;
const ADDRESS: IpAddr = IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));

#[async_trait]
impl AsterService for NotificationInterface {
    async fn init(
        &mut self,
        _: &mut common::messaging::crossbeam::CrossbeamMessagingFactory,
    ) -> Result<()> {
        debug!("Initializing channel controller");
        self.channel_controller = Some(controllers::channel::ChannelController::new(
            SocketAddr::new(ADDRESS, PORT),
        )?);

        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        debug!("Starting channel controller");
        self.channel_controller
            .as_mut()
            .ok_or(anyhow!("Channel controller not initialized"))?
            .start()
            .await?;

        Ok(())
    }
}
