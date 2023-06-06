use anyhow::Context;
use async_trait::async_trait;
use axum::Server;
use common::{messaging::tokio_broadcast::CrossbeamMessagingFactory, AsterService};
use log::debug;
use router::get_router;
use sqlx::{postgres::PgPoolOptions, PgPool};
mod dto;
mod router;
pub mod routes;
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
        debug!("DashboardServer initializing");

        // read the database url from the environment
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;

        // create a connection pool
        self.pool = Some(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url.as_str())
                .await?,
        );

        debug!("DashboardServer initialized");
        Ok(())
    }
    async fn run(&mut self) -> Result<(), anyhow::Error> {
        debug!("Starting DashboardServer");

        let router = get_router(self.pool.take().unwrap());

        let listen = format!("0.0.0.0:{}", SERVICE_PORT);

        debug!("DashboardServer running on {}", listen);

        // run the server

        Server::bind(&listen.parse().unwrap())
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}
