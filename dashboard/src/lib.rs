use anyhow::Context;
use async_trait::async_trait;
use axum::{Router, Server};
use common::{messaging::tokio_broadcast::CrossbeamMessagingFactory, AsterService};
use sqlx::{postgres::PgPoolOptions, PgPool};
const SERVICE_PORT: u16 = 3036;

#[derive(Debug, Default)]
pub struct DashboardServer {
    pool: Option<PgPool>,
}

#[async_trait]
impl AsterService for DashboardServer {
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        // read the database url from the environment
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

        // create a connection pool
        self.pool = Some(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url.as_str())
                .await?,
        );

        Ok(())
    }
    async fn run(&mut self) -> Result<(), anyhow::Error> {
        let router = Router::new();

        // run the server

        Server::bind(&format!("0.0.0.0:{}", SERVICE_PORT).parse().unwrap())
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}
