pub mod billable;
pub mod metric;

pub use billable::{BillableReceiver, BillableSender};
pub use metric::{MetricReceiver, MetricSender};

use std::fmt::Debug;

use async_trait::async_trait;
use thiserror::Error;

use super::{AsyncReceiver, AsyncSender};

#[derive(Error, Debug)]
pub enum AsyncChannelSenderError {
    #[error("Channel disconnected")]
    ChannelDisconnected,
}

#[derive(Error, Debug)]
pub enum AsyncChannelReceiverError {
    #[error("ReceiveError {0}")]
    ReceiveError(async_channel::RecvError),
}

#[derive(Clone)]
pub struct CrossbeamReceiver<T: Send> {
    receiver: async_channel::Receiver<T>,
}

#[derive(Clone)]
pub struct CrossbeamSender<T: Send> {
    sender: async_channel::Sender<T>,
}

impl<T: Send> CrossbeamReceiver<T> {
    pub fn new(receiver: async_channel::Receiver<T>) -> Self {
        Self { receiver }
    }
}

#[async_trait]
impl<T: Send> AsyncReceiver<T, AsyncChannelReceiverError> for CrossbeamReceiver<T> {
    async fn receive(&mut self) -> Result<T, AsyncChannelReceiverError> {
        self.receiver
            .recv()
            .await
            .map_err(AsyncChannelReceiverError::ReceiveError)
    }
}

impl<T: Send> CrossbeamSender<T> {
    pub fn new(sender: async_channel::Sender<T>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl<T: Send> AsyncSender<T, AsyncChannelSenderError> for CrossbeamSender<T> {
    async fn send(&mut self, message: T) -> Result<(), AsyncChannelSenderError> {
        self.sender
            .send(message)
            .await
            .map_err(|_| AsyncChannelSenderError::ChannelDisconnected)
    }
}

pub struct CrossbeamMessagingFactory {
    billable_sender: async_channel::Sender<crate::models::Billable>,
    billable_receiver: async_channel::Receiver<crate::models::Billable>,
    metric_sender: async_channel::Sender<crate::models::Metric>,
    metric_receiver: async_channel::Receiver<crate::models::Metric>,
}

impl CrossbeamMessagingFactory {}

impl Default for CrossbeamMessagingFactory {
    fn default() -> Self {
        let (billable_sender, billable_receiver) = async_channel::unbounded();
        let (metric_sender, metric_receiver) = async_channel::unbounded();

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
