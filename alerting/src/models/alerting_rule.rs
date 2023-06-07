use chrono::{DateTime, Utc};

use super::input::alerting_rule::{AlertingRuleInput, RuleTriggerInput, RuleTypeInput};

#[derive(Debug)]
pub struct AlertingRule {
    pub id: String,
    pub name: Option<String>,
    pub rule_type: Option<RuleType>,
    pub metric_name: Option<String>,
    pub threshold: Option<f64>,
    pub trigger: Option<RuleTrigger>,
    pub duration: Option<u64>,
    pub notification_channel_ids: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AlertingRuleInput> for AlertingRule {
    fn from(input: AlertingRuleInput) -> Self {
        Self {
            id: input.id.to_string(),
            name: input.name,
            rule_type: input.rule_type.map(|rule_type| rule_type.into()),
            metric_name: input.metric_name,
            threshold: input.threshold,
            trigger: input.trigger.map(|trigger| trigger.into()),
            duration: input.duration,
            notification_channel_ids: input.notification_channel_ids,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug)]
pub enum RuleType {
    ValueBased,
    PriceBased,
}

impl From<RuleTypeInput> for RuleType {
    fn from(rule_type: RuleTypeInput) -> Self {
        match rule_type {
            RuleTypeInput::ValueBased => Self::ValueBased,
            RuleTypeInput::PriceBased => Self::PriceBased,
        }
    }
}

#[derive(Debug)]
pub enum RuleTrigger {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

impl From<RuleTriggerInput> for RuleTrigger {
    fn from(trigger: RuleTriggerInput) -> Self {
        match trigger {
            RuleTriggerInput::GreaterThan => Self::GreaterThan,
            RuleTriggerInput::LessThan => Self::LessThan,
            RuleTriggerInput::Equal => Self::Equal,
            RuleTriggerInput::NotEqual => Self::NotEqual,
        }
    }
}
