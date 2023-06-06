use chrono::{DateTime, Utc};
use serde::Deserialize;

use self::{frequency::Frequency, operator::Operator};

pub mod frequency;
pub mod operator;

#[derive(Debug, Clone, Deserialize)]
pub struct BillableFilter {
    pub name: Option<String>,
    pub operator: Operator,
    pub frequency: Frequency,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
