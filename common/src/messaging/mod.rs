use async_trait::async_trait;

pub mod crossbeam;

pub use crossbeam::{CrossbeamReceiver, CrossbeamSender};

use self::crossbeam::{BillableReceiver, BillableSender, MetricReceiver, MetricSender};

#[async_trait]
pub trait AsyncReceiver<T, E: std::error::Error> {
    async fn receive(&mut self) -> Result<T, E>;
}

#[async_trait]
pub trait AsyncSender<T: Send, E: std::error::Error> {
    async fn send(&mut self, message: T) -> Result<(), E>;
}

pub trait MessagingFactory: Send + Sync {
    fn create_billable_sender(&self) -> BillableSender;
    fn create_billable_receiver(&self) -> BillableReceiver;
    fn create_metric_sender(&self) -> MetricSender;
    fn create_metric_receiver(&self) -> MetricReceiver;
}
