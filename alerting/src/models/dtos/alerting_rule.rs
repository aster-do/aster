use async_graphql::{Enum, SimpleObject, ID};
use serde::Serialize;

use crate::models::alerting_rule::{AlertingRule, RuleTrigger, RuleType};

#[derive(Debug, SimpleObject, Serialize)]
pub struct AlertingRuleDTO {
    pub id: ID,
    pub name: Option<String>,
    pub rule_type: Option<RuleTypeDTO>,
    pub metric_name: Option<String>,
    pub threshold: Option<f64>,
    pub trigger: Option<RuleTriggerDTO>,
    pub duration: Option<u64>,
    pub notification_channel_ids: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AlertingRule> for AlertingRuleDTO {
    fn from(value: AlertingRule) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            rule_type: value.rule_type.map(|rt| rt.into()),
            metric_name: value.metric_name,
            threshold: value.threshold,
            trigger: value.trigger.map(|t| t.into()),
            duration: value.duration,
            notification_channel_ids: value.notification_channel_ids,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum RuleTypeDTO {
    ValueBased,
    PriceBased,
}

impl From<RuleType> for RuleTypeDTO {
    fn from(value: RuleType) -> Self {
        match value {
            RuleType::ValueBased => Self::ValueBased,
            RuleType::PriceBased => Self::PriceBased,
        }
    }
}

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum RuleTriggerDTO {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

impl From<RuleTrigger> for RuleTriggerDTO {
    fn from(value: RuleTrigger) -> Self {
        match value {
            RuleTrigger::GreaterThan => Self::GreaterThan,
            RuleTrigger::LessThan => Self::LessThan,
            RuleTrigger::Equal => Self::Equal,
            RuleTrigger::NotEqual => Self::NotEqual,
        }
    }
}
