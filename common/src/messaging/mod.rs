use async_trait::async_trait;

pub mod crossbeam;

pub use crossbeam::{CrossbeamReceiver, CrossbeamSender};

#[async_trait]
pub trait AsyncReceiver<T, E: std::error::Error> {
    async fn receive(&mut self) -> Result<T, E>;
}

#[async_trait]
pub trait AsyncSender<T: Send, E: std::error::Error> {
    async fn send(&mut self, message: T) -> Result<(), E>;
}
