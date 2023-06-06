use crate::models::alerting_rule::{_AlertingRule, _RuleTrigger, _RuleType};
use crate::services::database::DatabaseService;

#[derive(Debug, Default)]
pub struct RuleService {
    database_service: Option<DatabaseService>,
}

impl RuleService {
    pub async fn new() -> Self {
        Self {
            database_service: Some(DatabaseService::new().await),
        }
    }

    pub async fn _create_rule(&self, rule: _AlertingRule) -> anyhow::Result<_AlertingRule> {
        let database_service = self.database_service.as_ref().unwrap();
        println!("{:?}", rule);
        database_service._create_rule(rule).await
    }

    pub async fn _delete_rule(&self, _rule_id: String) {
        let database_service = self.database_service.as_ref().unwrap();
        database_service._delete_rule(_rule_id).await;
    }

    pub async fn _update_rule(
        &self,
        _rule_id: String,
        _rule: _AlertingRule,
    ) -> anyhow::Result<Option<_AlertingRule>> {
        let database_service = self.database_service.as_ref().unwrap();
        database_service._update_rule(_rule_id, _rule).await
    }

    pub async fn _get_rules(&self) -> anyhow::Result<Vec<_AlertingRule>> {
        let database_service = self.database_service.as_ref().unwrap();
        database_service._get_rules().await
    }

    pub async fn _get_rule(&self, _rule_id: String) -> anyhow::Result<Option<_AlertingRule>> {
        let database_service = self.database_service.as_ref().unwrap();
        database_service._get_rule(_rule_id).await
    }
}
