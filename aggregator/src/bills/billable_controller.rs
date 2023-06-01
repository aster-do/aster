use crate::bills::StoredBillable;

use log::{debug, info};
use sqlx::{query_as, PgPool};
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

    pub async fn _run_aggregation_pipeline(&mut self) -> Result<(), Box<dyn Error>> {
        let billings = self._get_raw_billings().await?;
        debug!("Got {:?}", billings);

        if billings.is_empty() {
            info!("No billings to aggregate");
            return Ok(());
        }

        info!("Got {} billings from database", billings.len());

        // let aggreged: AggregedBillable = billings.into_iter().fold(
        //     AggregedBillable {},
        //     |acc, bill| {
        //         acc
        //     }
        // );

        // // Because we are never too careful
        // let mut transaction = self.client.transaction().await?;

        // futures_util::future::try_join(
        //      transaction.query("INSERT INTO BILLABLE ...", &[]),
        //      billings.into_iter().map(|bill| {
        //          transaction.query("DELETE FROM BILLABLE WHERE ...", &[])
        //      })
        // );

        info!("Aggregation pipeline ran sucessfully!");

        Ok(())
    }
}
