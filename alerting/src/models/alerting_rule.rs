use chrono::{DateTime, Utc};

use super::input::alerting_rule::{AlertingRuleInput, RuleTriggerInput, RuleTypeInput};

#[derive(Debug, Default, Clone)]
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

    pub fn to_string(&self) -> String {
        match self {
            RuleType::ValueBased => "value_based".to_string(),
            RuleType::PriceBased => "price_based".to_string(),
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

    pub fn to_string(&self) -> String {
        match self {
            RuleTrigger::GreaterThan => "greater_than".to_string(),
            RuleTrigger::LessThan => "less_than".to_string(),
            RuleTrigger::Equal => "equal".to_string(),
            RuleTrigger::NotEqual => "not_equal".to_string(),
        }
    }
}
