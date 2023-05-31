pub mod billable;
pub mod metric;

pub use billable::{BillableReceiver, BillableSender};
pub use metric::{MetricReceiver, MetricSender};

use std::fmt::Debug;

use async_trait::async_trait;
use thiserror::Error;

use super::{AsyncReceiver, AsyncSender};

#[derive(Error, Debug)]
pub enum CrossbeamSenderError {
    #[error("Channel disconnected")]
    ChannelDisconnected,
}

#[derive(Error, Debug)]
pub enum CrossbeamReceiverError {
    #[error("ReceiveError {0}")]
    ReceiveError(crossbeam_channel::RecvError),
}

#[derive(Clone)]
pub struct CrossbeamReceiver<T: Send> {
    receiver: crossbeam_channel::Receiver<T>,
}

#[derive(Clone)]
pub struct CrossbeamSender<T: Send> {
    sender: crossbeam_channel::Sender<T>,
}

impl<T: Send> CrossbeamReceiver<T> {
    pub fn new(receiver: crossbeam_channel::Receiver<T>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl<T: Send> AsyncReceiver<T, CrossbeamReceiverError> for CrossbeamReceiver<T> {
    async fn receive(&mut self) -> Result<T, CrossbeamReceiverError> {
        self.receiver
            .recv()
            .map_err(CrossbeamReceiverError::ReceiveError)
    }
}

impl<T: Send> CrossbeamSender<T> {
    pub fn new(sender: crossbeam_channel::Sender<T>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl<T: Send> AsyncSender<T, CrossbeamSenderError> for CrossbeamSender<T> {
    async fn send(&mut self, message: T) -> Result<(), CrossbeamSenderError> {
        self.sender
            .send(message)
            .map_err(|_| CrossbeamSenderError::ChannelDisconnected)
    }
}

pub struct CrossbeamMessagingFactory {
    billable_sender: crossbeam_channel::Sender<crate::models::Billable>,
    billable_receiver: crossbeam_channel::Receiver<crate::models::Billable>,
    metric_sender: crossbeam_channel::Sender<crate::models::Metric>,
    metric_receiver: crossbeam_channel::Receiver<crate::models::Metric>,
}

impl CrossbeamMessagingFactory {}

impl Default for CrossbeamMessagingFactory {
    fn default() -> Self {
        let (billable_sender, billable_receiver) = crossbeam_channel::unbounded();
        let (metric_sender, metric_receiver) = crossbeam_channel::unbounded();

        Self {
            billable_sender,
            billable_receiver,
            metric_sender,
            metric_receiver,
        }
    }
}

#[async_trait]
impl super::MessagingFactory for CrossbeamMessagingFactory {
    async fn create_billable_sender(&self) -> BillableSender {
        BillableSender::new(self.billable_sender.clone())
    }

    async fn create_billable_receiver(&self) -> BillableReceiver {
        BillableReceiver::new(self.billable_receiver.clone())
    }

    async fn create_metric_sender(&self) -> MetricSender {
        MetricSender::new(self.metric_sender.clone())
    }

    async fn create_metric_receiver(&self) -> MetricReceiver {
        MetricReceiver::new(self.metric_receiver.clone())
    }
}
