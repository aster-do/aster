#[derive(Debug)]
pub struct _AlertingRule {
    pub _id: String,
    pub _name: String,
    pub _rule_type: _RuleType,
    pub _metric_name: String,
    pub _threshold: f64,
    pub _trigger: _RuleTrigger,
    pub _duration: u64,
    pub _notification_channel_ids: Vec<String>,
}

#[derive(Debug)]
pub enum _RuleType {
    _ValueBased,
    _PriceBased,
}

#[derive(Debug)]
// implement from_string and to_string

pub enum _RuleTrigger {
    _GreaterThan,
    _LessThan,
    _Equal,
    _NotEqual,
}

// implement from_string and to_string

impl _RuleType {
    pub fn from_string(rule_type: &str) -> Self {
        match rule_type {
            "value_based" => _RuleType::_ValueBased,
            "price_based" => _RuleType::_PriceBased,
            _ => panic!("Invalid rule type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            _RuleType::_ValueBased => "value_based".to_string(),
            _RuleType::_PriceBased => "price_based".to_string(),
        }
    }
}

impl _RuleTrigger {
    pub fn from_string(trigger: &str) -> Self {
        match trigger {
            "greater_than" => _RuleTrigger::_GreaterThan,
            "less_than" => _RuleTrigger::_LessThan,
            "equal" => _RuleTrigger::_Equal,
            "not_equal" => _RuleTrigger::_NotEqual,
            _ => panic!("Invalid rule trigger"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            _RuleTrigger::_GreaterThan => "greater_than".to_string(),
            _RuleTrigger::_LessThan => "less_than".to_string(),
            _RuleTrigger::_Equal => "equal".to_string(),
            _RuleTrigger::_NotEqual => "not_equal".to_string(),
        }
    }
}
