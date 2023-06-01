use async_graphql::{EmptyMutation, EmptySubscription, Schema, SimpleObject, ID};

pub type BillingSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone, SimpleObject)]
struct BillingItem {
    id: ID,
    name: String,
    price: f64,
    timestamp: i64,
    value: f64,
}

#[derive(Clone, SimpleObject)]
struct Billing {
    id: ID,
    generated_at: i64,
    items: Vec<BillingItem>,
    total_price: f64,
}

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

    async fn test(&self) -> String {
        "test".to_owned()
    }
}
