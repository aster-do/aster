use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::messaging::{crossbeam::CrossbeamMessagingFactory, MessagingFactory};

#[async_trait]
pub trait AsterService {
    async fn init(&mut self, messaging: Arc<RwLock<dyn MessagingFactory>>);
    async fn run(&mut self);
}
