use anyhow::{Ok, Result};

use crate::models::alerting_rule::_AlertingRule;

#[derive(Debug)]
pub struct RuleService {
    //Config & stateful info
}

impl RuleService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
        })
    }

    pub fn _create_rule(&self, rule: _AlertingRule) -> Result<_AlertingRule> {
        //TODO: Implement create rule logic
        Ok(rule)
    }

    pub fn _delete_rule(&self, _rule_id: String) -> Result<()> {
        //TODO: Implement delete rule logic
        Ok(())
    }

    pub fn _update_rule(
        &self,
        _rule_id: String,
        _rule: _AlertingRule,
    ) -> Result<Option<_AlertingRule>> {
        //TODO: Implement update rule logic
        Ok(None)
    }

    pub fn _get_rules(&self) -> Result<Vec<_AlertingRule>> {
        //TODO: Implement get rules logic
        Ok(vec![])
    }

    pub fn _get_rule(&self, _rule_id: String) -> Result<Option<_AlertingRule>> {
        //TODO: Implement get rule logic
        Ok(None)
    }
}
