use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::transformer::transform;
use anyhow::anyhow;
use async_trait::async_trait;
use axum::{routing::get, Router, Server};
use common::messaging::tokio_broadcast::{
    BillableRuleReceiver, BillableSender, CrossbeamMessagingFactory, MetricReceiver,
};
use common::messaging::{AsyncReceiver, AsyncSender, MessagingFactory};
use common::models::billable_rules::billable_rule::{BillableRule, BillableRuleId};
use common::monitoring::readiness_handler;
use common::AsterService;
use tokio::sync::RwLock;

const READINESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3038);
const READINESS_SERVER_ENDPOINT: &str = "/health";

#[derive(Default)]
pub struct BillableBuilderService {
    pub rules: Arc<RwLock<HashMap<BillableRuleId, BillableRule>>>,
    pub metric_receiver: Option<MetricReceiver>,
    pub billable_sender: Option<BillableSender>,
    pub billable_rule_receiver: Option<BillableRuleReceiver>,
}

#[async_trait]
impl AsterService for BillableBuilderService {
    async fn init(
        &mut self,
        messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        self.metric_receiver = Some(messaging.create_metric_receiver().await);
        self.billable_sender = Some(messaging.create_billable_sender().await);
        self.billable_rule_receiver = Some(messaging.create_billable_rule_receiver().await);

        // init billable rules state
        self.rules = Arc::new(RwLock::new(HashMap::new()));

        log::info!("BillableBuilderService initialized");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        log::info!("BillableBuilderService running");

        let readiness_app = Router::new().route(READINESS_SERVER_ENDPOINT, get(readiness_handler));
        let readiness_server =
            Server::bind(READINESS_SERVER_ADDRESS).serve(readiness_app.into_make_service());

        let (
            readiness_result,
            listen_metrics_result,
            listen_billable_rules_result,
            fetch_billable_rules_result,
        ) = tokio::join!(
            readiness_server,
            self.listen_metrics(),
            self.listen_billable_rules(),
            self.fetch_billable_rules()
        );

        readiness_result.map_err(|e| anyhow!("Readiness server failed").context(e))?;
        listen_billable_rules_result.map_err(|e| {
            anyhow!("BillableBuilderService listen_billable_rules failed").context(e)
        })?;
        listen_metrics_result
            .map_err(|e| anyhow!("BillableBuilderService listen_metrics failed").context(e))?;
        fetch_billable_rules_result.map_err(|e| {
            anyhow!("BillableBuilderService fetch_billable_rules failed").context(e)
        })?;

        Ok(())
    }
}

impl BillableBuilderService {
    async fn listen_metrics(&self) -> Result<(), anyhow::Error> {
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
            let billable = transform(&metric, self.rules.clone()).await;
            log::debug!("Sending billable: {:?}", billable);

            match billable_sender.clone().send(billable).await {
                Ok(_) => (),
                Err(e) => log::error!("Error sending billable: {:?}", e),
            }

            log::debug!("Billable sent");
        }
    }

    async fn listen_billable_rules(&self) -> Result<(), anyhow::Error> {
        let billable_rule_receiver = self.billable_rule_receiver.clone().ok_or_else(|| {
            anyhow::anyhow!(
                "Billable rule receiver not initialized. Did you forget to call init()?"
            )
        })?;

        // In case of an error, we choose to simply log the error and continue.
        loop {
            let received_billable_rule = billable_rule_receiver.clone().receive().await;
            let billable_rule = match received_billable_rule {
                Ok(billable_rule) => billable_rule,
                Err(e) => {
                    log::error!("Error receiving billable rule: {:?}", e);
                    continue;
                }
            };

            log::debug!("Received billable rule: {:?}", billable_rule);

            self.add_billing_rule(billable_rule).await;
        }
    }

    async fn fetch_billable_rules(&self) -> Result<(), anyhow::Error> {
        let controller_address =
            std::env::var("CONTROLLER_ADDRESS").unwrap_or("http://localhost:3032".to_string());
        let get_rules_address = format!("{}/rules", controller_address);

        let rules = reqwest::get(get_rules_address)
            .await?
            .json::<Vec<BillableRule>>()
            .await?;

        for rule in rules {
            self.add_billing_rule(rule).await;
        }

        log::info!("Billable rules fetched");

        Ok(())
    }

    async fn add_billing_rule(&self, rule: BillableRule) {
        let mut rules = self.rules.write().await;

        // check if rule already exists
        rules
            .get_mut(&rule.id)
            .map(|r| {
                if r.version < rule.version {
                    *r = rule.clone();
                }
            })
            .unwrap_or_else(|| {
                rules.insert(rule.id, rule);
            });
    }
}
