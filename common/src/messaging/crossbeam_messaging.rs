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
