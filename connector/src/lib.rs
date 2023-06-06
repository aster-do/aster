use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{routing::get, Router, Server};
use common::{
    messaging::tokio_broadcast::{BillableReceiver, CrossbeamMessagingFactory},
    messaging::{AsyncReceiver, MessagingFactory},
    monitoring::readiness_handler,
    AsterService,
};
use log::{debug, info, trace};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;

const READINESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3039);
const READINESS_SERVER_ENDPOINT: &str = "/health";

#[derive(Clone, Default)]
pub struct ConnectorService {
    state: Option<ConnectorServiceState>,
}

#[derive(Clone)]
struct ConnectorServiceState {
    billable_receiver: BillableReceiver,
    pool: PgPool,
}

#[derive(Error, Debug)]
enum ConnectorServiceError {
    #[error("Failed to connect to database: {0}")]
    ConnectorServiceDatabaseConnectionFailed(sqlx::Error),
}

#[async_trait]
impl AsterService for ConnectorService {
    async fn init(
        &mut self,
        messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        info!("Initializing connector service ...");

        debug!("Connecting to database ...");

        // Get the database URL from the environment
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

        // Connect to the database
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(ConnectorServiceError::ConnectorServiceDatabaseConnectionFailed)?;

        debug!("Connected to database");

        // Create the billable receiver
        self.state = Some(ConnectorServiceState {
            billable_receiver: messaging.create_billable_receiver().await,
            pool,
        });

        info!("Connector service initialized");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Running connector service ...");

        let readiness_app = Router::new().route(READINESS_SERVER_ENDPOINT, get(readiness_handler));
        let readiness_server =
            Server::bind(READINESS_SERVER_ADDRESS).serve(readiness_app.into_make_service());

        let (readiness_result, lifecycle_result) = tokio::join!(readiness_server, self.lifecycle());
        readiness_result.map_err(|e| anyhow!("Readiness server failed").context(e))?;
        lifecycle_result
            .map_err(|e| anyhow!("BillableBuilderService lifecycle failed").context(e))?;

        Ok(())
    }
}

impl ConnectorService {
    async fn lifecycle(&mut self) -> Result<(), anyhow::Error> {
        let mut receiver = self
            .state
            .as_mut()
            .expect("Connector service not initialized")
            .billable_receiver
            .clone();

        loop {
            // Receive a billable from the receiver
            let billable = receiver.receive().await?;
            debug!("Received a billable from the receiver");
            trace!("Billable: {:?}", billable);

            // Get a connection from the pool
            let mut conn = self
                .state
                .as_mut()
                .expect("Connector service not initialized")
                .pool
                .acquire()
                .await?;

            // Insert the billable into the database
            sqlx::query!(
                "INSERT INTO billables.billable (name, price, timestamp, value, treated) VALUES ($1, $2, $3, $4, false)",
                billable.name,
                billable.price,
                billable.timestamp,
                billable.value
            )
            .execute(&mut conn)
            .await?;

            debug!("Inserted billable into the database");
        }
    }
}
