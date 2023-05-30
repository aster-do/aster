use async_trait::async_trait;

mod crossbeam_messaging;

pub use crossbeam_messaging::{CrossbeamReceiver, CrossbeamSender};

#[async_trait]
pub trait AsyncReceiver<T, E: std::error::Error> {
    async fn receive(&mut self) -> Result<T, E>;
}

#[async_trait]
pub trait AsyncSender<T: Send, E: std::error::Error> {
    async fn send(&mut self, message: T) -> Result<(), E>;
}
