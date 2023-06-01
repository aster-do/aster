use async_trait::async_trait;

use crate::bills::{StoredBillable, _BillPerMetric as BillPerMetric};

use common::{messaging::crossbeam::CrossbeamMessagingFactory, services::AsterService};
use log::{debug, error, info};
use sqlx::{query, query_as, types::uuid::Uuid, PgPool};
use tokio::time::{sleep, Duration};

#[derive(Default)]
pub struct BillableController {
    pub connection: Option<PgPool>,
}

#[async_trait]
impl AsterService for BillableController {
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        let url = std::env::var("PG_STRING").expect("PG_STRING must be set");
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
            match self._run_aggregation_pipeline().await {
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

impl BillableController {
    async fn _get_raw_billings(&self) -> Result<Vec<StoredBillable>, anyhow::Error> {
        let results = query_as::<_, StoredBillable>("SELECT * FROM BILLABLE")
            .fetch_all(self.connection.as_ref().unwrap())
            .await
            .map_err(Box::new)?;

        Ok(results)
    }

    /// This will take metrics and aggregate them by name
    fn _aggregate(billings: Vec<StoredBillable>) -> Vec<BillPerMetric> {
        let mut bills_per_metric: Vec<BillPerMetric> = Vec::new();

        billings.into_iter().for_each(|bill| {
            let bill_per_metric = BillPerMetric {
                id: None,
                name: bill.name,
                price: bill.price,
            };

            if bills_per_metric.is_empty() {
                bills_per_metric.push(bill_per_metric);
            } else {
                let mut found = false;
                for bill in bills_per_metric.iter_mut() {
                    if bill.name == bill_per_metric.name {
                        bill.price += bill_per_metric.price;
                        found = true;
                        break;
                    }
                }
                if !found {
                    bills_per_metric.push(bill_per_metric);
                }
            }
        });

        bills_per_metric
    }

    pub async fn _run_aggregation_pipeline(&mut self) -> Result<(), anyhow::Error> {
        let billings = self._get_raw_billings().await?;
        debug!("Got {:?}", billings);

        if billings.is_empty() {
            info!("No billings to aggregate");
            return Ok(());
        }

        info!("Got {} billings from database", billings.len());

        let ids = billings.iter().map(|b| b.id).collect::<Vec<Uuid>>();

        BillableController::_aggregate(billings);

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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_aggregation() {
        let billings = vec![
            StoredBillable {
                id: Uuid::new_v4(),
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
            StoredBillable {
                id: Uuid::new_v4(),
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
            StoredBillable {
                id: Uuid::new_v4(),
                name: "test2".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
        ];

        let aggregated = BillableController::_aggregate(billings);

        assert_eq!(aggregated.len(), 2);
        assert_eq!(aggregated[0].name, "test");
        assert_eq!(aggregated[0].price, 20);
        assert_eq!(aggregated[1].name, "test2");
        assert_eq!(aggregated[1].price, 10);
    }
}
