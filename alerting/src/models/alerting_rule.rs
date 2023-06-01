#[derive(Debug)]
pub struct _AlertingRule {
    _name: String,
    _rule_type: _RuleType,
    _metric_name: String,
    _threshold: f64,
    _trigger: _RuleTrigger,
    _duration: u64,
    _notification_channel_ids: Vec<String>,
}

#[derive(Debug)]
pub enum _RuleType {
    _ValueBased,
    _PriceBased,
}

#[derive(Debug)]
pub enum _RuleTrigger {
    _GreaterThan,
    _LessThan,
    _Equal,
    _NotEqual,
}
