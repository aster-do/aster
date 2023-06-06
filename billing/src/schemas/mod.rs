pub mod structures;

use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;
use structures::{Billing, BillingItem, BillingItemInput};

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self) -> Vec<Billing> {
        // TODO: Get the billing from the database

        vec![
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
        ]
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    /// Generate a billing for the month
    async fn generate_billing(&self) -> Billing {
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

    /// Generate a billing from the given items
    async fn generate_billing_from_given_items(
        &self,
        billing_items_input: Vec<BillingItemInput>,
    ) -> Billing {
        // TODO: Add the billing to the database

        let id = ID::from("1".to_string());
        let generated_at = Utc::now().timestamp();
        let mut items = vec![];

        let mut total_price = 0.0;
        for billing_item in billing_items_input {
            let billing_item = BillingItem::from(billing_item);

            total_price += billing_item.price * billing_item.value;

            items.push(billing_item);
        }

        Billing {
            id,
            generated_at,
            items,
            total_price,
        }
    }
}
