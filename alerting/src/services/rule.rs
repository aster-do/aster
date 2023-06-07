use crate::models::alerting_rule::AlertingRule;
use crate::services::database::DatabaseService;

#[derive(Debug, Default)]
pub struct RuleService {
    database_service: Option<DatabaseService>,
}

impl RuleService {
    pub async fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            database_service: Some(DatabaseService::new().await),
        })
    }

    pub async fn mutate_rule(&self, rule: AlertingRule) -> anyhow::Result<AlertingRule> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;

        if let Some(_rule) = database_service.get_rule(rule.id.clone()).await? {
            database_service
                .update_rule(rule.clone().id, rule.clone())
                .await?
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Alerting : Failed to update rule with id {}",
                        rule.clone().id
                    )
                })
        } else {
            database_service.create_rule(rule).await
        }
    }

    pub async fn delete_rule(&self, rule_id: String) -> anyhow::Result<()> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.delete_rule(rule_id).await
    }

    pub async fn get_rules(&self) -> anyhow::Result<Vec<AlertingRule>> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.get_rules().await
    }

    pub async fn get_rule(&self, rule_id: String) -> anyhow::Result<Option<AlertingRule>> {
        let database_service = self
            .database_service
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : Service not found"))?;
        database_service.get_rule(rule_id).await
    }
}
