pub mod billable;
pub mod billable_rule;
pub mod metric;

pub use billable::{BillableReceiver, BillableSender};
pub use billable_rule::{BillableRuleReceiver, BillableRuleSender};
pub use metric::{MetricReceiver, MetricSender};

use std::fmt::Debug;

use async_trait::async_trait;
use thiserror::Error;

use super::{AsyncReceiver, AsyncSender};

const CHANNEL_SIZE: usize = 1024;

#[derive(Error, Debug)]
pub enum TokioBroadcastSenderError {
    #[error("Channel disconnected")]
    ChannelDisconnected,
}

#[derive(Error, Debug)]
pub enum TokioBroadcastReceiverError {
    #[error("ReceiveError {0}")]
    ReceiveError(tokio::sync::broadcast::error::RecvError),
}

pub struct TokioBroadcastReceiver<T: Clone + Send> {
    receiver: tokio::sync::broadcast::Receiver<T>,
}

#[derive(Clone)]
pub struct TokioBroadcastSender<T: Clone + Send> {
    sender: tokio::sync::broadcast::Sender<T>,
}

impl<T: Clone + Send> TokioBroadcastReceiver<T> {
    pub fn new(receiver: tokio::sync::broadcast::Receiver<T>) -> Self {
        Self { receiver }
    }
}

impl<T: Clone + Send> Clone for TokioBroadcastReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.resubscribe(),
        }
    }
}

#[async_trait]
impl<T: Clone + Send> AsyncReceiver<T, TokioBroadcastReceiverError> for TokioBroadcastReceiver<T> {
    async fn receive(&mut self) -> Result<T, TokioBroadcastReceiverError> {
        self.receiver
            .recv()
            .await
            .map_err(TokioBroadcastReceiverError::ReceiveError)
    }
}

impl<T: Clone + Send> TokioBroadcastSender<T> {
    pub fn new(sender: tokio::sync::broadcast::Sender<T>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl<T: Clone + Send> AsyncSender<T, TokioBroadcastSenderError> for TokioBroadcastSender<T> {
    async fn send(&mut self, message: T) -> Result<(), TokioBroadcastSenderError> {
        self.sender.send(message).map_or_else(
            |_| Err(TokioBroadcastSenderError::ChannelDisconnected),
            |_| Ok(()),
        )
    }
}

pub struct CrossbeamMessagingFactory {
    billable_sender: tokio::sync::broadcast::Sender<crate::models::Billable>,
    billable_receiver: tokio::sync::broadcast::Receiver<crate::models::Billable>,
    billable_rule_sender:
        tokio::sync::broadcast::Sender<crate::models::billable_rules::billable_rule::BillableRule>,
    billable_rule_receiver: tokio::sync::broadcast::Receiver<
        crate::models::billable_rules::billable_rule::BillableRule,
    >,
    metric_sender: tokio::sync::broadcast::Sender<crate::models::Metric>,
    metric_receiver: tokio::sync::broadcast::Receiver<crate::models::Metric>,
}

impl CrossbeamMessagingFactory {}

impl Default for CrossbeamMessagingFactory {
    fn default() -> Self {
        let (billable_sender, billable_receiver) = tokio::sync::broadcast::channel(CHANNEL_SIZE);
        let (billable_rule_sender, billable_rule_receiver) =
            tokio::sync::broadcast::channel(CHANNEL_SIZE);
        let (metric_sender, metric_receiver) = tokio::sync::broadcast::channel(CHANNEL_SIZE);

        Self {
            billable_sender,
            billable_receiver,
            billable_rule_sender,
            billable_rule_receiver,
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
        BillableReceiver::new(self.billable_receiver.resubscribe())
    }

    async fn create_billable_rule_sender(&self) -> BillableRuleSender {
        BillableRuleSender::new(self.billable_rule_sender.clone())
    }

    async fn create_billable_rule_receiver(&self) -> BillableRuleReceiver {
        BillableRuleReceiver::new(self.billable_rule_receiver.resubscribe())
    }

    async fn create_metric_sender(&self) -> MetricSender {
        MetricSender::new(self.metric_sender.clone())
    }

    async fn create_metric_receiver(&self) -> MetricReceiver {
        MetricReceiver::new(self.metric_receiver.resubscribe())
    }
}
