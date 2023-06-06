use async_graphql::{SimpleObject, ID};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::input::InputBillingItem;

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct BillingItem {
    pub id: ID,
    pub name: String,
    pub price: f64,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
}

impl From<InputBillingItem> for BillingItem {
    fn from(billing_item_input: InputBillingItem) -> Self {
        let serialised = serde_json::to_string(&billing_item_input).unwrap();
        serde_json::from_str(&serialised).unwrap()
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Billing {
    pub id: ID,
    pub generated_at: chrono::DateTime<Utc>,
    pub items: Vec<BillingItem>,
    pub total_price: f64,
}
