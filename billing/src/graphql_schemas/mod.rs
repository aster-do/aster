use std::sync::Arc;

use crate::models::{Billing, BillingInput};
use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;

use crate::services::database::DatabaseService;

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot {
    pub database_service: Arc<DatabaseService>,
}

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self, id: Option<ID>) -> anyhow::Result<Vec<Billing>> {
        match id {
            Some(id) => {
                let id = id.to_string().parse::<i32>().unwrap();
                Ok(self
                    .database_service
                    ._get_billing(id)
                    .await
                    .map_err(|e| anyhow::anyhow!("Billing : Failed to fetch billing: {}", e))?
                    .into_iter()
                    .collect())
            }
            None => Ok(self
                .database_service
                ._get_billings()
                .await
                .map_err(|e| anyhow::anyhow!("Billing : Failed to fetch all billings: {}", e))?
                .into_iter()
                .collect()),
        }
    }
}

pub struct MutationRoot {
    pub database_service: Arc<DatabaseService>,
}

#[async_graphql::Object]
impl MutationRoot {
    async fn billing(&self, billing: BillingInput) -> anyhow::Result<Billing> {
        let database_service = self.database_service.as_ref();

        if let Some(_billing) = database_service._get_billing(billing.id).await? {
            let updated_billing = Billing {
                id: billing.id.into(),
                generated_at: match billing.generated_at {
                    Some(generated_at) => generated_at,
                    None => _billing.generated_at,
                },
                items: match billing.items {
                    Some(items) => {
                        let mut billables = Vec::new();
                        for item in items {
                            let billable = database_service
                                ._get_billable(item.parse::<i32>()?)
                                .await?
                                .ok_or_else(|| anyhow::anyhow!("Billing : Billable not found"))?;
                            billables.push(billable);
                        }
                        billables
                    }
                    None => _billing.items,
                },
            };
            database_service
                ._update_billing(billing.id, updated_billing.clone())
                .await?;
            Ok(updated_billing)
        } else {
            database_service
                ._create_billing(Billing {
                    id: billing.id.into(),
                    generated_at: match billing.generated_at {
                        Some(generated_at) => generated_at,
                        None => Utc::now(),
                    },
                    items: match billing.items {
                        Some(items) => {
                            let mut billables = Vec::new();
                            for item in items {
                                let billable = database_service
                                    ._get_billable(item.parse::<i32>()?)
                                    .await?
                                    .ok_or_else(|| {
                                        anyhow::anyhow!("Billing : Billable not found")
                                    })?;
                                billables.push(billable);
                            }
                            billables
                        }
                        None => Vec::new(),
                    },
                })
                .await
        }
    }
}
