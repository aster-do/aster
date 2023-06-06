pub mod bills;

use async_trait::async_trait;

use common::models::billable::BillableSQL;

use common::{messaging::tokio_broadcast::CrossbeamMessagingFactory, services::AsterService};
use log::{error, info};
use sqlx::{query, query_as, PgPool};
use tokio::time::{sleep, Duration};

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
            PgPool::connect(&url)
                .await
                .expect("Failed to connect to postgres"),
        );

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting aggregator");

        let mut fail_count = 0;
        const MAX_FAIL_COUNT: u32 = 5;

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

        Ok(())
    }
}

impl BillableAggregatorService {
    async fn get_raw_billings(&self) -> Result<Vec<BillableSQL>, anyhow::Error> {
        let results = query_as::<_, BillableSQL>("SELECT * FROM BILLABLE WHERE TREATED = false")
            .fetch_all(self.connection.as_ref().unwrap())
            .await
            .map_err(Box::new)?;

        Ok(results)
    }

    pub async fn run_aggregation_pipeline(&mut self) -> Result<(), anyhow::Error> {
        let billings = self.get_raw_billings().await?;

        if billings.is_empty() {
            info!("No billings to aggregate");
            return Ok(());
        }

        info!("Got {} billings from database", billings.len());

        let ids = billings.iter().map(|b| b.id).collect::<Vec<i64>>();

        // TODO toggle aggregation and insert what has been aggregated
        // aggregate(billings);

        // Because we are never too careful we start a transaction
        let mut transaction = self.connection.as_ref().unwrap().begin().await?;

        match futures_util::future::try_join(
            query("UPDATE BILLABLE SET TREATED = TRUE WHERE ID = ANY($1)")
                .bind(&ids[..])
                .execute(&mut transaction),
            futures_util::future::ok(()),
        )
        .await
        {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                error!("Error updating billables: {}", e);
                transaction.rollback().await?;
                return Err(e.into());
            }
        };

        info!("Aggregation pipeline ran sucessfully!");

        Ok(())
    }
}
