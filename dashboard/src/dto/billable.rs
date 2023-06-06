use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Billable {
    pub name: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub value: f64,
}
