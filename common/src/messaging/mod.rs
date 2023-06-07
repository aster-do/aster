use async_trait::async_trait;

pub mod tokio_broadcast;

pub use tokio_broadcast::{TokioBroadcastReceiver, TokioBroadcastSender};

use self::tokio_broadcast::{
    BillableReceiver, BillableRuleReceiver, BillableRuleSender, BillableSender, MetricReceiver,
    MetricSender,
};

#[async_trait]
pub trait AsyncReceiver<T, E: std::error::Error> {
    async fn receive(&mut self) -> Result<T, E>;
}

#[async_trait]
pub trait AsyncSender<T: Send, E: std::error::Error> {
    async fn send(&mut self, message: T) -> Result<(), E>;
}

#[async_trait]
pub trait MessagingFactory {
    async fn create_billable_sender(&self) -> BillableSender;
    async fn create_billable_receiver(&self) -> BillableReceiver;
    async fn create_billable_rule_sender(&self) -> BillableRuleSender;
    async fn create_billable_rule_receiver(&self) -> BillableRuleReceiver;
    async fn create_metric_sender(&self) -> MetricSender;
    async fn create_metric_receiver(&self) -> MetricReceiver;
}
