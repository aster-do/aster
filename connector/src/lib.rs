use std::sync::Arc;

use async_trait::async_trait;
use common::{
    messaging::{
        crossbeam::{BillableReceiver, CrossbeamMessagingFactory},
        MessagingFactory,
    },
    AsterService,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::RwLock;

#[derive(Clone, Default)]
struct ConnectorService {
    state: Option<ConnectorServiceState>,
}

#[derive(Clone)]
struct ConnectorServiceState {
    billable_receiver: BillableReceiver,
    postgres_pool: PgPool,
}

#[async_trait]
impl AsterService for ConnectorService {
    async fn init(&mut self, messaging: Arc<RwLock<dyn MessagingFactory>>) {
        let postgres_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/postgres")
            .await
            .unwrap();

        let lock = messaging.read().await;

        let x = lock.create_billable_receiver();

        self.state = Some(ConnectorServiceState {
            billable_receiver: x,
            postgres_pool,
        });
    }

    async fn run(&mut self) {}
}
