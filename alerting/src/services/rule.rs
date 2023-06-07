use anyhow::{Ok, Result};

use crate::models::alerting_rule::AlertingRule;

#[derive(Debug)]
pub struct RuleService {
    //Config & stateful info
}

impl RuleService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
        })
    }

    pub async fn mutate_rule(&self, rule: AlertingRule) -> Result<AlertingRule> {
        //TODO: Implement create rule logic
        // If rule exists, update it
        // If rule does not exist, create it
        Ok(rule)
    }

    pub async fn delete_rule(&self, _rule_id: String) -> Result<()> {
        //TODO: Implement delete rule logic
        Ok(())
    }

    pub async fn get_rules(&self) -> Result<Vec<AlertingRule>> {
        //TODO: Implement get rules logic
        Ok(vec![])
    }

    pub async fn get_rule(&self, _rule_id: String) -> Result<Option<AlertingRule>> {
        //TODO: Implement get rule logic
        Ok(None)
    }
}
