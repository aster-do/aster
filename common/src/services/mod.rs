use async_trait::async_trait;

use crate::messaging::MessagingFactory;

#[async_trait]
pub trait AsterService {
    async fn init(&mut self, messaging: &mut dyn MessagingFactory);
    async fn run(&mut self);
}
