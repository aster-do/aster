use crate::models::alerting_rule::_AlertingRule;
use crate::services::database::DatabaseService;

#[derive(Debug, Default)]
pub struct RuleService {
    _database_service: Option<DatabaseService>,
}

impl RuleService {
    pub async fn new() -> Self {
        Self {
            _database_service: Some(DatabaseService::new().await),
        }
    }

    pub async fn _create_rule(&self, rule: _AlertingRule) -> anyhow::Result<_AlertingRule> {
        let database_service = self
            ._database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._create_rule(rule).await
    }

    pub async fn _delete_rule(&self, _rule_id: String) -> anyhow::Result<()> {
        let database_service = self
            ._database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._delete_rule(_rule_id).await
    }

    pub async fn _update_rule(
        &self,
        _rule_id: String,
        _rule: _AlertingRule,
    ) -> anyhow::Result<Option<_AlertingRule>> {
        let database_service = self
            ._database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._update_rule(_rule_id, _rule).await
    }

    pub async fn _get_rules(&self) -> anyhow::Result<Vec<_AlertingRule>> {
        let database_service = self
            ._database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._get_rules().await
    }

    pub async fn _get_rule(&self, _rule_id: String) -> anyhow::Result<Option<_AlertingRule>> {
        let database_service = self
            ._database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service._get_rule(_rule_id).await
    }
}
