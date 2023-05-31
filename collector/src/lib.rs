pub mod metric;

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{routing::post, Router};
use log::info;
use std::net::SocketAddr;

use common::{
    messaging::{
        tokio_broadcast::{CrossbeamMessagingFactory, MetricSender},
        MessagingFactory,
    },
    AsterService,
};

use crate::metric::{metric_controller::create_metric, model::AxumAppState};

#[derive(Clone, Default)]
pub struct ConnectorService {
    state: Option<ConnectorServiceState>,
}

const SERVICE_PORT: u16 = 3035;

#[derive(Clone)]
struct ConnectorServiceState {
    metric_sender: MetricSender,
    port: u16,
}

#[async_trait]
impl AsterService for ConnectorService {
    async fn init(
        &mut self,
        messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        info!("Initializing Collector service");
        let sender = messaging.create_metric_sender().await;

        self.state = Some(ConnectorServiceState {
            metric_sender: sender,
            port: SERVICE_PORT,
        });

        info!("Initialized Collector service");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Running Collector service");
        let sender = self
            .state
            .as_mut()
            .ok_or(anyhow!("Failed to create collector sender"))?
            .metric_sender
            .clone();

        let shared_state = AxumAppState::new(sender);

        let router = Router::new()
            .route("/metrics", post(create_metric))
            .with_state(shared_state);

        let port = self.state.as_mut().expect("Port not initialized").port;

        // Start the server
        info!("Collector server started on port {}", port);
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to bind collector's server with port : {}, err: {}",
                    port,
                    e
                )
            })?;

        Ok(())
    }
}
