pub mod input;
pub mod output;

use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;
use input::BillingGenerationOptions;
use output::{Billing, BillingItem};

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self, _id: Option<ID>) -> Vec<Billing> {
        // TODO: Get the billing from the database

        let billings = vec![
            Billing {
                id: ID::from("1".to_owned()),
                generated_at: 1627776000,
                items: vec![
                    BillingItem {
                        id: ID::from("1".to_owned()),
                        name: "cpu".to_owned(),
                        value: 1.0,
                        price: 1.0,
                        timestamp: 1627776001,
                    },
                    BillingItem {
                        id: ID::from("2".to_owned()),
                        name: "storage".to_owned(),
                        value: 3.4,
                        price: 2.0,
                        timestamp: 1627776002,
                    },
                ],
                total_price: 1.0 + 3.4 * 2.0,
            },
            Billing {
                id: ID::from("2".to_owned()),
                generated_at: 1627776010,
                items: vec![
                    BillingItem {
                        id: ID::from("1".to_owned()),
                        name: "cpu".to_owned(),
                        value: 1.0,
                        price: 1.0,
                        timestamp: 1627776003,
                    },
                    BillingItem {
                        id: ID::from("2".to_owned()),
                        name: "storage".to_owned(),
                        value: 3.9,
                        price: 2.0,
                        timestamp: 1627776004,
                    },
                ],
                total_price: 1.0 + 3.9 * 2.0,
            },
        ];

        match _id {
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
    async fn generate_billing(&self, _options: Option<BillingGenerationOptions>) -> Billing {
        // TODO: Generate the billing from the database

        let id = ID::from("1".to_string());
        let generated_at = Utc::now().timestamp();
        let items = vec![
            BillingItem {
                id: "3".into(),
                name: "memory".into(),
                price: 10.99,
                timestamp: 1686052774,
                value: 1.0,
            },
            BillingItem {
                id: "4".into(),
                name: "cpu".into(),
                price: 1.99,
                timestamp: 1686052775,
                value: 3.5,
            },
            BillingItem {
                id: "5".into(),
                name: "cpu".into(),
                price: 2.30,
                timestamp: 1686052776,
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
