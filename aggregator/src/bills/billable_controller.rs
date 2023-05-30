use std::future::Future;

use super::{Result, Billable, AggregedBillable};

use tokio_postgres::{Client, NoTls};


pub struct BillableController {
    pub client: Client,
}

impl BillableController {
    pub async fn new(connection_string: &str) -> Result<Self> {
        let (client, _) = tokio_postgres::connect(
            connection_string,
            NoTls,
        )
        .await?;

        Ok(Self { client })
    }
}


impl BillableController {
    async fn get_raw_billings(&self) -> Result<Vec<Billable>> {
        return self.client.query(
            "SELECT * FROM billables", &[]
        ).await?.into_iter().map(|row| {
            Ok(Billable {})
        }).collect::<Result<Vec<Billable>>>();
    }

    pub async fn run_aggregation_pipeline(&mut self) -> Result<()> {
        let billings = self.get_raw_billings().await?;

        let aggreged: AggregedBillable = billings.into_iter().fold(
            AggregedBillable {},
            |acc, bill| {
                acc
            }
        );


        // Because we are never too careful
        let mut transaction = self.client.transaction().await?;

        // futures_util::future::try_join(
        //      transaction.query("INSERT INTO BILLABLE ...", &[]),
        //      billings.into_iter().map(|bill| {
        //          transaction.query("DELETE FROM BILLABLE WHERE ...", &[])
        //      })
        // );

        Ok(())
    }
}