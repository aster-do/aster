use std::fmt::Debug;

use async_trait::async_trait;

use super::{AsyncReceiver, AsyncSender};

#[derive(Debug)]
pub enum CrossbeamSenderError {
    ChannelDisconnected,
}

impl std::error::Error for CrossbeamSenderError {}

impl std::fmt::Display for CrossbeamSenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossbeamSenderError::ChannelDisconnected => write!(f, "Channel was disconnected"),
        }
    }
}

#[derive(Debug)]
pub enum CrossbeamReceiverError {
    ReceiveError(crossbeam_channel::RecvError),
}

impl std::error::Error for CrossbeamReceiverError {}

impl std::fmt::Display for CrossbeamReceiverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossbeamReceiverError::ReceiveError(e) => write!(f, "ReceiveError {}", e),
        }
    }
}
pub struct CrossbeamReceiver<T: Send> {
    receiver: crossbeam_channel::Receiver<T>,
}

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
impl<T: Send + Debug> AsyncSender<T, CrossbeamSenderError> for CrossbeamSender<T> {
    async fn send(&mut self, message: T) -> Result<(), CrossbeamSenderError> {
        self.sender
            .send(message)
            .map_err(|_| CrossbeamSenderError::ChannelDisconnected)
    }
}
