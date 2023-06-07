use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use common::{messaging::MessagingFactory, AsterService};
use controllers::{billable::BillableController, rule::RuleController};
use log::debug;
use tokio::join;

mod controllers;
mod models;
mod services;

#[derive(Default)]
pub struct AlertingInterface {
    //Config & stateful info
    billable_controller: Option<BillableController>,
    rule_controller: Option<RuleController>,
}

const PORT: u16 = 3031;
const ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const READINESS_SERVER_ENDPOINT: &str = "/health";

#[async_trait]
impl AsterService for AlertingInterface {
    async fn init(
        &mut self,
        messaging: &mut common::messaging::tokio_broadcast::CrossbeamMessagingFactory,
    ) -> Result<()> {
        debug!("Initializing billable controller");
        self.billable_controller =
            Some(BillableController::new(messaging.create_billable_receiver().await).await?);

        debug!("Initializing rule controller");
        self.rule_controller = Some(
            RuleController::new(
                SocketAddr::new(ADDRESS, PORT),
                READINESS_SERVER_ENDPOINT.to_string(),
            )
            .await?,
        );

        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        debug!("Starting rule controller");
        let rule = self
            .rule_controller
            .as_mut()
            .ok_or(anyhow!("Rule controller not initialized"))?
            .start();

        debug!("Starting billable controller");
        let billable = self
            .billable_controller
            .as_mut()
            .ok_or(anyhow!("Billable controller not initialized"))?
            .start();

        match join!(billable, rule) {
            (Ok(_), Ok(_)) => anyhow::Ok(()),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}
