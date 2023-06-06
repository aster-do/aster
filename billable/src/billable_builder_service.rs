use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::transformer::transform;
use anyhow::anyhow;
use async_trait::async_trait;
use axum::{routing::get, Router, Server};
use common::messaging::tokio_broadcast::{
    BillableSender, CrossbeamMessagingFactory, MetricReceiver,
};
use common::messaging::{AsyncReceiver, AsyncSender, MessagingFactory};
use common::models::billable_rules::billable_rule::{BillableOperation, BillableRule};
use common::monitoring::readiness_handler;
use common::AsterService;

const READINESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3038);
const READINESS_SERVER_ENDPOINT: &str = "/health";

#[derive(Default)]
pub struct BillableBuilderService {
    pub rules: Vec<BillableRule>,
    pub metric_receiver: Option<MetricReceiver>,
    pub billable_sender: Option<BillableSender>,
}

#[async_trait]
impl AsterService for BillableBuilderService {
    async fn init(
        &mut self,
        messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        self.metric_receiver = Some(messaging.create_metric_receiver().await);
        self.billable_sender = Some(messaging.create_billable_sender().await);

        self.rules = vec![
            BillableRule {
                id: Some(1),
                name: "cpu".to_string(),
                operation: BillableOperation::Multiply,
                number: 1,
                version: Some(1),
            },
            BillableRule {
                id: Some(2),
                name: "memory".to_string(),
                operation: BillableOperation::Multiply,
                number: 3,
                version: Some(1),
            },
        ];

        log::info!("BillableBuilderService initialized");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        log::info!("BillableBuilderService running");

        let readiness_app = Router::new().route(READINESS_SERVER_ENDPOINT, get(readiness_handler));
        let readiness_server =
            Server::bind(READINESS_SERVER_ADDRESS).serve(readiness_app.into_make_service());

        let (readiness_result, lifecycle_result) = tokio::join!(readiness_server, self.lifecycle());
        readiness_result.map_err(|e| anyhow!("Readiness server failed").context(e))?;
        lifecycle_result
            .map_err(|e| anyhow!("BillableBuilderService lifecycle failed").context(e))?;

        Ok(())
    }
}

impl BillableBuilderService {
    async fn lifecycle(&self) -> Result<(), anyhow::Error> {
        let metric_receiver = self.metric_receiver.clone().ok_or_else(|| {
            anyhow::anyhow!("Metric receiver not initialized. Did you forget to call init()?")
        })?;

        let billable_sender = self.billable_sender.clone().ok_or_else(|| {
            anyhow::anyhow!("Billable sender not initialized. Did you forget to call init()?")
        })?;

        // In case of an error, we choose to simply log the error and continue.
        loop {
            let received_metric = metric_receiver.clone().receive().await;
            let metric = match received_metric {
                Ok(metric) => metric,
                Err(e) => {
                    log::error!("Error receiving metric: {:?}", e);
                    continue;
                }
            };

            log::debug!("Received metric: {:?}", metric);
            let billable = transform(&metric, self.rules.clone());
            log::debug!("Sending billable: {:?}", billable);

            match billable_sender.clone().send(billable).await {
                Ok(_) => (),
                Err(e) => log::error!("Error sending billable: {:?}", e),
            }

            log::debug!("Billable sent");
        }
    }
}
