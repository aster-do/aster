use crate::transformer::transform;
use async_trait::async_trait;
use common::messaging::tokio_broadcast::{
    BillableSender, CrossbeamMessagingFactory, MetricReceiver,
};
use common::messaging::{AsyncReceiver, AsyncSender, MessagingFactory};
use common::models::billable_rules::billable_rule::{BillableOperation, BillableRule};
use common::AsterService;

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
                name: "cpu".to_string(),
                operation: BillableOperation::Multiply,
                number: 1,
            },
            BillableRule {
                name: "memory".to_string(),
                operation: BillableOperation::Multiply,
                number: 3,
            },
        ];

        log::info!("BillableBuilderService initialized");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        log::info!("BillableBuilderService running");
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
