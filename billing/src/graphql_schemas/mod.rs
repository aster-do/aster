pub mod structures;

use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;
use structures::{Billable, Billing};

use crate::services::database::DatabaseService;

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self, id: Option<ID>) -> Vec<Billing> {
        let database_service = DatabaseService::new().await;
        match id {
            Some(id) => {
                let id = id.to_string().parse::<i32>().unwrap();
                database_service
                    ._get_billing(id)
                    .await
                    .unwrap()
                    .into_iter()
                    .collect()
            }
            None => database_service
                ._get_billings()
                .await
                .unwrap()
                .into_iter()
                .collect(),
        }
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn generate_billing(&self, _date: Option<chrono::DateTime<Utc>>) -> Billing {
        // TODO: Generate the billing from the database

        let id = ID::from("1".to_string());
        let generated_at = Utc::now();
        let items = vec![
            Billable {
                id: "3".into(),
                name: "memory".into(),
                price: 199,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                treated: false,
                value: 1.0,
            },
            Billable {
                id: "4".into(),
                name: "cpu".into(),
                price: 199,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                treated: false,
                value: 3.5,
            },
            Billable {
                id: "5".into(),
                name: "cpu".into(),
                price: 230,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                treated: false,
                value: 4.0,
            },
        ];

        Billing {
            id,
            generated_at,
            items,
        }
    }
}
