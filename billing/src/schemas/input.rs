use async_graphql::{InputObject, ID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, InputObject, Serialize, Deserialize)]
pub struct InputBillingItem {
    id: ID,
    name: String,
    price: f64,
    timestamp: i64,
    value: f64,
}

#[derive(Debug, Clone, InputObject)]
pub struct BillingGenerationOptions {
    specific_date: Option<SpecificDate>,
    items: Option<Vec<InputBillingItem>>,
}

#[derive(Debug, Clone, InputObject)]
pub struct SpecificDate {
    month: i32,
    year: i32,
}
