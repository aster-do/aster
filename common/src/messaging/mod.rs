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

#[async_trait]
pub trait MessagingFactory {
    async fn create_billable_sender(&self) -> BillableSender;
    async fn create_billable_receiver(&self) -> BillableReceiver;
    async fn create_metric_sender(&self) -> MetricSender;
    async fn create_metric_receiver(&self) -> MetricReceiver;
}
