use std::fmt::Display;

use chrono::{DateTime, Utc};

use super::input::alerting_rule::{AlertingRuleInput, RuleTriggerInput, RuleTypeInput};

#[derive(Debug, Default, Clone)]
pub struct AlertingRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub metric_name: String,
    pub threshold: f64,
    pub trigger: RuleTrigger,
    pub grace_period: u64,
    pub notification_channel_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AlertingRuleInput> for AlertingRule {
    fn from(input: AlertingRuleInput) -> Self {
        Self {
            id: input.id.unwrap_or_default().to_string(),
            name: input.name.unwrap_or_default(),
            rule_type: input.rule_type.unwrap_or_default().into(),
            metric_name: input.metric_name.unwrap_or_default(),
            threshold: input.threshold.unwrap_or_default(),
            trigger: input.trigger.unwrap_or_default().into(),
            grace_period: input.grace_period.unwrap_or_default(),
            notification_channel_ids: input.notification_channel_ids.unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum RuleType {
    ValueBased,
    #[default]
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

#[derive(Debug, Default, Clone)]
pub enum RuleTrigger {
    #[default]
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

// implement from_string and to_string

impl RuleType {
    pub fn from_string(rule_type: &str) -> Self {
        match rule_type {
            "value_based" => RuleType::ValueBased,
            "price_based" => RuleType::PriceBased,
            _ => panic!("Invalid rule type"),
        }
    }
}

impl Display for RuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleType::ValueBased => write!(f, "value_based"),
            RuleType::PriceBased => write!(f, "price_based"),
        }
    }
}

impl RuleTrigger {
    pub fn from_string(trigger: &str) -> Self {
        match trigger {
            "greater_than" => RuleTrigger::GreaterThan,
            "less_than" => RuleTrigger::LessThan,
            "equal" => RuleTrigger::Equal,
            "not_equal" => RuleTrigger::NotEqual,
            _ => panic!("Invalid rule trigger"),
        }
    }
}

impl Display for RuleTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleTrigger::GreaterThan => write!(f, "greater_than"),
            RuleTrigger::LessThan => write!(f, "less_than"),
            RuleTrigger::Equal => write!(f, "equal"),
            RuleTrigger::NotEqual => write!(f, "not_equal"),
        }
    }
}
