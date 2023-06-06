use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::anyhow;
use axum::{routing::get, Router, Server};
use billable_rule_service::BillableRuleService;
use common::{
    messaging::tokio_broadcast::CrossbeamMessagingFactory, monitoring::readiness_handler,
    AsterService,
};
use log::info;

pub mod billable_rule_service;

const READINESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3032);
const READINESS_SERVER_ENDPOINT: &str = "/health";

pub struct ControllerService {
    pub billable_rules_service: Option<BillableRuleService>,
}

impl ControllerService {
    pub fn new() -> Self {
        Self {
            billable_rules_service: None,
        }
    }
}

impl Default for ControllerService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AsterService for ControllerService {
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        info!("ControllerService is initializing");

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

        self.billable_rules_service = Some(BillableRuleService::new(&database_url).await);

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("ControllerService is running");

        let rules = self.billable_rules_service.as_ref().unwrap().get_all();

        let readiness_app = Router::new().route(READINESS_SERVER_ENDPOINT, get(readiness_handler));
        let readiness_server =
            Server::bind(READINESS_SERVER_ADDRESS).serve(readiness_app.into_make_service());

        let (readiness_result, lifecycle_result) = tokio::join!(readiness_server, rules);
        readiness_result.map_err(|e| anyhow!("Readiness server failed").context(e))?;
        lifecycle_result
            .map_err(|e| anyhow!("BillableBuilderService lifecycle failed").context(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
