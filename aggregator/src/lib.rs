pub mod bills;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::anyhow;
use bills::AggregatesToBeWritten;
use futures_util::TryFutureExt;
use std::str::FromStr;

use async_trait::async_trait;

use axum::{routing::get, Router, Server};
use common::models::billable::{BillableAggregate, BillableSQL};

use common::monitoring::readiness_handler;
use common::{messaging::tokio_broadcast::CrossbeamMessagingFactory, services::AsterService};
use log::{debug, error, info};
use sqlx::{postgres::PgConnectOptions, query, query_as, PgPool};
use tokio::time::{sleep, Duration};

const MAX_FAIL_COUNT: u32 = 5;
const READINESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3037);
const READINESS_SERVER_ENDPOINT: &str = "/health";
use crate::bills::aggregators::aggregate;

#[derive(Default)]
pub struct BillableAggregatorService {
    pub connection: Option<PgPool>,
}

#[async_trait]
impl AsterService for BillableAggregatorService {
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());
        self.connection = Some(
            PgPool::connect_with(
                PgConnectOptions::from_str(&url)
                    .map_err(|e| anyhow!("Failed to parse database url").context(e))?,
            )
            .await
            .map_err(|e| anyhow!("Failed to connect to database").context(e))?,
        );

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting aggregator");

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

impl BillableAggregatorService {
    async fn get_raw_billings(&self) -> Result<Vec<BillableSQL>, anyhow::Error> {
        let results = query_as!(BillableSQL, "SELECT * FROM BILLABLE WHERE TREATED = false")
            .fetch_all(self.connection.as_ref().unwrap())
            .await
            .map_err(Box::new)?;

        Ok(results)
    }

    async fn get_all_aggregates(&self) -> Result<Vec<BillableAggregate>, anyhow::Error> {
        let results = query_as!(BillableAggregate, "SELECT * FROM BILLABLE_AGGREGATE")
            .fetch_all(self.connection.as_ref().unwrap())
            .await
            .map_err(Box::new)?;

        Ok(results)
    }

    async fn upsert_aggregates(
        &self,
        aggregates: AggregatesToBeWritten,
    ) -> Result<(), anyhow::Error> {
        debug!("Aggregates to insert: {:?}", aggregates.inserts);
        debug!("Aggregates to update: {:?}", aggregates.updates);

        let mut transaction = match self.connection.as_ref() {
            None => return Err(anyhow!("No connection to database")),
            Some(connection) => connection.begin().await.map_err(Box::new)?,
        };

        for aggregate in aggregates.inserts {
            query!(
                "INSERT INTO billable_aggregate(name, \"timestamp\", min, max, avg, count, sum) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                aggregate.name,
                aggregate.timestamp,
                aggregate.min,
                aggregate.max,
                aggregate.avg,
                aggregate.count,
                aggregate.sum
            )
            .execute(&mut transaction)
            .await?;
        }

        for aggregate in aggregates.updates {
            query!(
                "UPDATE BILLABLE_AGGREGATE SET MIN = $1, MAX = $2, AVG = $3, COUNT = $4, SUM = $5  WHERE \"timestamp\" = $6 AND NAME = $7",
                aggregate.min,
                aggregate.max,
                aggregate.avg,
                aggregate.count,
                aggregate.sum,
                aggregate.timestamp,
                aggregate.name
            )
            .execute(&mut transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn run_aggregation_pipeline(&mut self) -> Result<(), anyhow::Error> {
        let billings = self.get_raw_billings().await?;

        let aggregates = self.get_all_aggregates().await?;

        if billings.is_empty() {
            info!("No billings to aggregate");
            return Ok(());
        }

        info!("Got {} billings from database", billings.len());

        let ids = billings.iter().map(|b| b.id).collect::<Vec<i32>>();

        // TODO toggle aggregation and insert what has been aggregated
        let aggregates = aggregate(billings, aggregates);

        // Because we are never too careful we start a transaction
        let mut transaction = self.connection.as_ref().unwrap().begin().await?;

        match futures_util::future::try_join(
            query!(
                "UPDATE BILLABLE SET TREATED = TRUE WHERE ID = ANY($1)",
                &ids[..]
            )
            .execute(&mut transaction)
            .map_err(|e| anyhow!("{}", e)),
            self.upsert_aggregates(aggregates)
                .map_err(|e| anyhow!("{}", e)),
        )
        .await
        {
            Ok(_) => {
                transaction.commit().await?;
            }
            Err(e) => {
                error!("Error updating billables: {}", e);
                transaction.rollback().await?;
                return Err(e);
            }
        };

        info!("Aggregation pipeline ran sucessfully!");

        Ok(())
    }

    async fn lifecycle(&mut self) -> Result<(), anyhow::Error> {
        let mut fail_count = 0;

        while fail_count < MAX_FAIL_COUNT {
            match self.run_aggregation_pipeline().await {
                Ok(_) => {
                    info!("Aggregation pipeline completed successfully");
                    fail_count = 0;
                    sleep(Duration::from_secs(60)).await;
                }
                Err(e) => {
                    error!(
                        "Aggregation pipeline failed: {}, retrying in 5 seconds...",
                        e
                    );
                    fail_count += 1;
                    sleep(Duration::from_secs(5)).await;
                }
            };
        }

        error!(
            "Aggregation pipeline failed {} times in a row. Shutting down",
            MAX_FAIL_COUNT
        );

        Err(anyhow!(
            "Aggregation pipeline failed {} times in a row. Shutting down",
            MAX_FAIL_COUNT
        ))
    }
}
