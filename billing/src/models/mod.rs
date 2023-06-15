use async_graphql::{InputObject, SimpleObject, ID};
use chrono::Utc;
use common::models::BillableSQL;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, SimpleObject, InputObject, Serialize, Deserialize)]
pub struct Billable {
    pub id: ID,
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
    pub treated: bool,
}
#[derive(Debug, InputObject, Serialize, Deserialize)]
pub struct BillingInput {
    pub id: i32,
    pub generated_at: Option<chrono::DateTime<Utc>>,
    pub items: Option<Vec<String>>,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Billing {
    pub id: ID,
    pub generated_at: chrono::DateTime<Utc>,
    pub items: Vec<Billable>,
}

impl From<BillableSQL> for Billable {
    fn from(persistence: BillableSQL) -> Self {
        println!("test");
        Billable {
            id: persistence.id.to_string().into(),
            name: persistence.name,
            price: persistence.price,
            timestamp: chrono::DateTime::<Utc>::from_utc(persistence.timestamp.naive_utc(), Utc),
            value: persistence.value,
            treated: persistence.treated,
        }
    }
}

#[derive(Debug)]
pub struct BillingPersistence {
    pub id: i32,
    pub generated_at: chrono::DateTime<Utc>,
    pub items: Option<String>, // json
}
