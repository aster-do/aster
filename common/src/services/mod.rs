use async_trait::async_trait;

use crate::messaging::crossbeam::CrossbeamMessagingFactory;

#[async_trait]
pub trait AsterService {
    async fn init(&mut self, messaging: &mut CrossbeamMessagingFactory);
    async fn run(&mut self);
}
