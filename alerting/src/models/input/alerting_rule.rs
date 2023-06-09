use async_graphql::{Enum, InputObject, ID};
use serde::{Deserialize, Serialize};

#[derive(Debug, InputObject, Serialize, Deserialize)]
pub struct AlertingRuleInput {
    pub id: Option<ID>,
    pub name: Option<String>,
    pub rule_type: Option<RuleTypeInput>,
    pub metric_name: Option<String>,
    pub threshold: Option<f64>,
    pub trigger: Option<RuleTriggerInput>,
    pub duration: Option<u64>,
    pub notification_channel_ids: Option<Vec<String>>,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RuleTypeInput {
    ValueBased,
    PriceBased,
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RuleTriggerInput {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
