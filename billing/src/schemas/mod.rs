pub mod input;
pub mod output;

use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;
use output::{Billing, BillingItem};

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self, id: Option<ID>) -> Vec<Billing> {
        // TODO: Get the billing from the database

        let billings = vec![
            Billing {
                id: ID::from("1".to_owned()),
                generated_at: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                items: vec![
                    BillingItem {
                        id: ID::from("1".to_owned()),
                        name: "cpu".to_owned(),
                        value: 1.0,
                        price: 1.0,
                        timestamp: chrono::DateTime::parse_from_rfc3339(
                            "2023-06-06T13:23:04+00:00",
                        )
                        .unwrap()
                        .with_timezone(&Utc),
                    },
                    BillingItem {
                        id: ID::from("2".to_owned()),
                        name: "storage".to_owned(),
                        value: 3.4,
                        price: 2.0,
                        timestamp: chrono::DateTime::parse_from_rfc3339(
                            "2023-06-06T13:23:04+00:00",
                        )
                        .unwrap()
                        .with_timezone(&Utc),
                    },
                ],
                total_price: 1.0 + 3.4 * 2.0,
            },
            Billing {
                id: ID::from("2".to_owned()),
                generated_at: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                items: vec![
                    BillingItem {
                        id: ID::from("1".to_owned()),
                        name: "cpu".to_owned(),
                        value: 1.0,
                        price: 1.0,
                        timestamp: chrono::DateTime::parse_from_rfc3339(
                            "2023-06-06T13:23:04+00:00",
                        )
                        .unwrap()
                        .with_timezone(&Utc),
                    },
                    BillingItem {
                        id: ID::from("2".to_owned()),
                        name: "storage".to_owned(),
                        value: 3.9,
                        price: 2.0,
                        timestamp: chrono::DateTime::parse_from_rfc3339(
                            "2023-06-06T13:23:04+00:00",
                        )
                        .unwrap()
                        .with_timezone(&Utc),
                    },
                ],
                total_price: 1.0 + 3.9 * 2.0,
            },
        ];

        match id {
            Some(id) => billings
                .into_iter()
                .filter(|billing| billing.id == id)
                .collect(),
            None => billings,
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
            BillingItem {
                id: "3".into(),
                name: "memory".into(),
                price: 10.99,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                value: 1.0,
            },
            BillingItem {
                id: "4".into(),
                name: "cpu".into(),
                price: 1.99,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                value: 3.5,
            },
            BillingItem {
                id: "5".into(),
                name: "cpu".into(),
                price: 2.30,
                timestamp: chrono::DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
                value: 4.0,
            },
        ];
        let total_price = items.iter().map(|item| item.price * item.value).sum();

        Billing {
            id,
            generated_at,
            items,
            total_price,
        }
    }
}
