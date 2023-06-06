use async_trait::async_trait;
use common::{
    messaging::tokio_broadcast::{BillableReceiver, CrossbeamMessagingFactory},
    messaging::{AsyncReceiver, MessagingFactory},
    AsterService,
};
use log::{debug, info, trace};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;

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
                "INSERT INTO billable (name, price, timestamp, value, treated) VALUES ($1, $2, $3, $4, false)",
                billable.name,
                billable.price,
                billable.timestamp.naive_utc(),
                billable.value
            )
            .execute(&mut conn)
            .await?;

            debug!("Inserted billable into the database");
        }
    }
}
