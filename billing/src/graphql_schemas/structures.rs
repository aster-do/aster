use async_graphql::{SimpleObject, ID};
use chrono::Utc;

use crate::db_models::BillablePersistence;

#[derive(Debug, Clone, SimpleObject)]
pub struct Billable {
    pub id: ID,
    pub name: String,
    pub price: i64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
    pub treated: bool,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Billing {
    pub id: ID,
    pub generated_at: chrono::DateTime<Utc>,
    pub items: Vec<Billable>,
}

impl From<BillablePersistence> for Billable {
    fn from(persistence: BillablePersistence) -> Self {
        Billable {
            id: persistence.id.to_string().into(),
            name: persistence.name,
            price: persistence.price,
            timestamp: chrono::DateTime::<Utc>::from_utc(persistence.timestamp, Utc),
            value: persistence.value,
            treated: persistence.treated,
        }
    }
}
