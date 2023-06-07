use crate::models::alerting_rule::AlertingRule;
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

    pub async fn _create_rule(&self, rule: AlertingRule) -> anyhow::Result<AlertingRule> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.create_rule(rule).await
    }

    pub async fn _delete_rule(&self, _rule_id: String) -> anyhow::Result<()> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.delete_rule(_rule_id).await
    }

    pub async fn _update_rule(
        &self,
        rule_id: String,
        rule: AlertingRule,
    ) -> anyhow::Result<Option<AlertingRule>> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._update_rule(rule_id, rule).await
    }

    pub async fn get_rules(&self) -> anyhow::Result<Vec<AlertingRule>> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.get_rules().await
    }

    pub async fn _get_rule(&self, _rule_id: String) -> anyhow::Result<Option<AlertingRule>> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.get_rule(_rule_id).await
    }
}
