mod structures;

use async_graphql::{EmptySubscription, Schema, ID};
use chrono::Utc;
use structures::{Billing, BillingItem, BillingItemInput};

pub type BillingSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn billing(&self) -> Vec<Billing> {
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
    async fn add_billing(&self, billing_items_input: Vec<BillingItemInput>) -> Billing {
        let mut items = vec![];

        let mut total_price = 0.0;
        for billing_item in billing_items_input {
            let billing_item = BillingItem::from(billing_item);

            total_price += billing_item.price * billing_item.value;

            items.push(billing_item);
        }

        let id = ID::from("1".to_string());
        let generated_at = Utc::now().timestamp();

        Billing {
            id,
            generated_at,
            items,
            total_price,
        }
    }
}
