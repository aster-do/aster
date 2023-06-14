mod controller;
mod graphql_schemas;
mod models;
pub mod services;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_trait::async_trait;
use common::{messaging::tokio_broadcast::CrossbeamMessagingFactory, AsterService};
use controller::billing::BillingController;
use log::info;

const SERVICE_PORT: u16 = 3033;
const READINESS_SERVER_ENDPOINT: &str = "/health";
const ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Default)]
pub struct BillingRuntime {
    pub billing_controller: Option<BillingController>,
}

#[async_trait]
impl AsterService for BillingRuntime {
    async fn init(&mut self, _: &mut CrossbeamMessagingFactory) -> Result<(), anyhow::Error> {
        info!("Initializing the billing service...");
        self.billing_controller = Some(
            BillingController::new(
                SocketAddr::new(ADDRESS, SERVICE_PORT),
                READINESS_SERVER_ENDPOINT.to_string(),
            )
            .await?,
        );
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting the billing service...");
        let controller = self
            .billing_controller
            .as_ref()
            .expect("Billing controller not initialized");

        controller.start().await?;
        Ok(())
    }
}
