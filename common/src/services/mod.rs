use async_trait::async_trait;
use thiserror::Error;

use crate::messaging::crossbeam::CrossbeamMessagingFactory;

#[derive(Error, Debug)]
pub enum AsterServiceError {
    #[error("Failed to initialize Aster service: {0}")]
    AsterServiceInitFailed(anyhow::Error),
    #[error("Failed to run Aster service: {0}")]
    AsterServiceRunFailed(anyhow::Error),
}

#[async_trait]
pub trait AsterService {
    const SERVICE_PORT: u16;

    async fn init(
        &mut self,
        messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error>;
    async fn run(&mut self) -> Result<(), anyhow::Error>;
}
