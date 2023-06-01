use crate::bills::{StoredBillable, _BillPerMetric as BillPerMetric};

use log::{debug, error, info};
use sqlx::{query, query_as, types::uuid::Uuid, PgPool};
use std::error::Error;

pub struct _BillableController {
    pub connection: PgPool,
}

impl _BillableController {
    pub async fn _new(connection_string: &str) -> Result<Self, Box<dyn Error>> {
        let connection = PgPool::connect(connection_string).await?;

        Ok(Self { connection })
    }

    async fn _get_raw_billings(&self) -> Result<Vec<StoredBillable>, Box<dyn Error>> {
        let results = query_as::<_, StoredBillable>("SELECT * FROM BILLABLE")
            .fetch_all(&self.connection)
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

    pub async fn _run_aggregation_pipeline(&mut self) -> Result<(), Box<dyn Error>> {
        let billings = self._get_raw_billings().await?;
        debug!("Got {:?}", billings);

        if billings.is_empty() {
            info!("No billings to aggregate");
            return Ok(());
        }

        info!("Got {} billings from database", billings.len());

        let ids = billings.iter().map(|b| b.id).collect::<Vec<Uuid>>();

        _BillableController::_aggregate(billings);

        // Because we are never too careful we start a transaction
        let mut transaction = self.connection.begin().await?;

        match futures_util::future::try_join(
            query("UPDATE BILLABLE SET TREATED = FALSE WHERE ID = ANY($1)")
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

        let aggregated = _BillableController::_aggregate(billings);

        assert_eq!(aggregated.len(), 2);
        assert_eq!(aggregated[0].name, "test");
        assert_eq!(aggregated[0].price, 20);
        assert_eq!(aggregated[1].name, "test2");
        assert_eq!(aggregated[1].price, 10);
    }
}
