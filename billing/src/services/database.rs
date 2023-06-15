use crate::models::{Billable, Billing, BillingPersistence};
use anyhow::anyhow;
use async_graphql::ID;
use sqlx::postgres::{PgPool, PgPoolOptions};

use common::models::billable::BillableSQL;

#[derive(Debug, Default)]
pub struct DatabaseService {
    _pool: Option<PgPool>,
}

impl DatabaseService {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = match PgPoolOptions::new()
            .max_connections(100)
            .connect(
                &std::env::var("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string()),
            )
            .await
        {
            Ok(_pool) => Some(_pool),
            Err(e) => return Err(anyhow!("Billing : Failed to connect to database: {}", e)),
        };

        Ok(Self { _pool: pool })
    }

    pub async fn _create_billing(&self, billing: Billing) -> anyhow::Result<Billing> {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            ._pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Billing : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Billing : Failed to acquire connection: {}", e))?;

        // get all id of billable in a array of i32
        let mut billable_ids: Vec<String> = Vec::new();
        for billable in billing.items.iter() {
            billable_ids.push(billable.id.to_string());
        }

        let billing_to_insert = BillingPersistence {
            id: billing.id.parse::<i32>()?,
            generated_at: billing.generated_at,
            items: Some(billable_ids.join(",")),
        };

        sqlx::query!(
            "INSERT INTO billing (id, generated_at, items) VALUES ($1, $2, $3)",
            billing_to_insert.id,
            billing_to_insert.generated_at,
            billing_to_insert.items
        )
        .execute(&mut conn)
        .await
        .map_err(|e| anyhow::anyhow!("Billing : Failed to insert rule: {}", e))?;

        Ok(billing)
    }

    pub async fn _delete_billing(&self, billing_id: i32) -> anyhow::Result<()> {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            ._pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Billing : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Billing : Failed to acquire connection: {}", e))?;

        sqlx::query!("DELETE FROM billing WHERE id = $1", billing_id)
            .execute(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Billing : Failed to delete rule: {}", e))?;

        Ok(())
    }

    pub async fn _update_billing(
        &self,
        billing_id: i32,
        billing: Billing,
    ) -> anyhow::Result<Option<Billing>> {
        // check if rule exists
        let billing_to_update = &self._get_billing(billing_id).await?;

        if billing_to_update.is_none() {
            return Ok(None);
        }

        // get all billable id
        let mut billable_ids: Vec<String> = Vec::new();
        for billable in billing.items.iter() {
            billable_ids.push(billable.id.to_string());
        }

        let billing_persistence = BillingPersistence {
            id: billing.id.parse::<i32>()?,
            generated_at: billing.generated_at,
            items: Some(billable_ids.join(",")),
        };

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            ._pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Billing : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Billing : Failed to acquire connection: {}", e))?;

        sqlx::query!(
            "UPDATE billing SET id = $1, generated_at = $2, items = $3 WHERE id = $4",
            billing_persistence.id,
            billing_persistence.generated_at,
            billing_persistence.items,
            billing_id
        )
        .execute(&mut conn)
        .await
        .map_err(|e| anyhow::anyhow!("Billing : Failed to update rule: {}", e))?;

        Ok(Some(billing))
    }

    pub async fn _get_billings(&self) -> anyhow::Result<Vec<Billing>> {
        let billings_persistence_list: Vec<BillingPersistence> =
            sqlx::query_as!(BillingPersistence, "SELECT * FROM billing")
                .fetch_all(
                    self._pool
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Billing : No pool found"))?,
                )
                .await
                .map_err(|e| anyhow::anyhow!("Billing : Failed to fetch rule: {}", e))?;

        let mut billings: Vec<Billing> = Vec::new();

        for billing_persistence in billings_persistence_list {
            // get all billable ids
            let mut billable_ids: Vec<i32> = Vec::new();
            if let Some(items) = billing_persistence.items {
                for item in items.split(',') {
                    billable_ids.push(item.parse::<i32>()?);
                }
            }

            // get all billable from billable ids
            let mut billables: Vec<Billable> = Vec::new();
            for billable_id in billable_ids {
                let billable = self._get_billable(billable_id).await?;
                match billable {
                    None => continue,
                    Some(billable) => billables.push(billable),
                }
            }

            billings.push(Billing {
                id: ID::from(billing_persistence.id),
                generated_at: billing_persistence.generated_at,
                items: billables,
            });
        }

        Ok(billings)
    }

    pub async fn _get_billing(&self, billing_id: i32) -> anyhow::Result<Option<Billing>> {
        let billing_persistence: Option<BillingPersistence> = sqlx::query_as!(
            BillingPersistence,
            "SELECT * FROM billing WHERE id = $1",
            billing_id
        )
        .fetch_optional(
            self._pool
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Billing : No pool found"))?,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Billing : Failed to fetch rule: {}", e))?;

        match billing_persistence {
            None => Ok(None),
            Some(billing_persistence) => {
                // get all billable ids
                let mut billable_ids: Vec<i32> = Vec::new();
                if let Some(items) = billing_persistence.items {
                    for item in items.split(',') {
                        billable_ids.push(item.parse::<i32>()?);
                    }
                }

                // get all billable from billable ids
                let mut billables: Vec<Billable> = Vec::new();
                for billable_id in billable_ids {
                    let billable = self._get_billable(billable_id).await?;
                    if let Some(billable) = billable {
                        billables.push(billable);
                    }
                }

                Ok(Some(Billing {
                    id: ID::from(billing_persistence.id),
                    generated_at: billing_persistence.generated_at,
                    items: billables,
                }))
            }
        }
    }

    pub async fn _get_billable(&self, billable_id: i32) -> anyhow::Result<Option<Billable>> {
        let billable_persistence: Option<BillableSQL> = sqlx::query_as!(
            BillableSQL,
            "SELECT * FROM billable WHERE id = $1",
            billable_id
        )
        .fetch_optional(self._pool.as_ref().unwrap())
        .await
        .map_err(|e| anyhow::anyhow!("Billing : Failed to fetch billable: {}", e))?;

        match billable_persistence {
            None => Ok(None),
            Some(billable_persistence) => Ok(Some(Billable::from(billable_persistence))),
        }
    }
}
