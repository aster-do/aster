use async_trait::async_trait;
use common::{
    messaging::{
        crossbeam::{BillableReceiver, CrossbeamMessagingFactory},
        MessagingFactory,
    },
    AsterService,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

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
    async fn init(&mut self, messaging: &mut CrossbeamMessagingFactory) {
        // Connect to the database.
        let postgres_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost/postgres")
            .await
            .unwrap();

        // Create the billable receiver.
        self.state = Some(ConnectorServiceState {
            billable_receiver: messaging.create_billable_receiver().await,
            postgres_pool,
        });
    }

    async fn run(&mut self) {}
}
