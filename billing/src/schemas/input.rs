use async_graphql::{InputObject, ID};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct InputBillingItem {
    id: ID,
    name: String,
    price: f64,
    timestamp: chrono::DateTime<Utc>,
    value: f64,
}

#[derive(Debug, Clone, InputObject)]
pub struct BillingGenerationOptions {
    specific_date: Option<SpecificDate>,
}

#[derive(Debug, Clone, InputObject)]
pub struct SpecificDate {
    date: chrono::DateTime<Utc>,
}
