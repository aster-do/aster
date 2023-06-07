use std::str::FromStr;

use crate::models::{
    alerting_rule::{AlertingRule, RuleTrigger, RuleType},
    database::{fetch_alerting_rule::AlertingRuleFetch, insert_alerting_rule::AlertingRuleInsert},
};
use chrono::Utc;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    types::BigDecimal,
};

#[derive(Debug, Default)]
pub struct DatabaseService {
    pool: Option<PgPool>,
}

impl DatabaseService {
    pub async fn new() -> Self {
        let pool = match PgPoolOptions::new()
            .max_connections(100)
            .connect(
                &std::env::var("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string()),
            )
            .await
        {
            Ok(_pool) => Some(_pool),
            Err(_) => None,
        };

        Self { pool }
    }

    pub async fn create_rule(&self, rule: AlertingRule) -> anyhow::Result<AlertingRule> {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            .pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Alerting : Failed to acquire connection: {}", e))?;

        let clone = rule.clone();

        let rule_to_insert = AlertingRuleInsert {
            id: rule.id.clone(),
            name: rule.name.clone().unwrap_or_default(),
            rule_type: rule.rule_type.unwrap_or_default().to_string(),
            metric_name: rule.metric_name.unwrap_or_default(),
            threshold: BigDecimal::from_str(&rule.threshold.unwrap_or_default().to_string())
                .unwrap(),
            trigger: rule.trigger.unwrap_or_default().to_string(),
            duration: rule.grace_period.unwrap_or_default() as i32,
            notification_channel_ids: Some(format!("{:?}", rule.notification_channel_ids)),
        };

        sqlx::query!(
            "INSERT INTO alertingrule (id, name, rule_type, metric_name, threshold, trigger, duration, notification_channel_ids) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            rule_to_insert.id,
            rule_to_insert.name,
            rule_to_insert.rule_type,
            rule_to_insert.metric_name,
            rule_to_insert.threshold,
            rule_to_insert.trigger,
            rule_to_insert.duration,
            rule_to_insert.notification_channel_ids
        ).execute(
            &mut conn
        ).await.map_err(|e| anyhow::anyhow!("Alerting : Failed to insert rule: {}", e))?;

        Ok(clone)
    }

    pub async fn delete_rule(&self, rule_id: String) -> anyhow::Result<()> {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            .pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Alerting : Failed to acquire connection: {}", e))?;

        sqlx::query!("DELETE FROM alertingrule WHERE id = $1", rule_id)
            .execute(&mut conn)
            .await
            .map_err(|e| anyhow::anyhow!("Alerting : Failed to delete rule: {}", e))?;

        Ok(())
    }

    pub async fn update_rule(
        &self,
        rule_id: String,
        rule: AlertingRule,
    ) -> anyhow::Result<Option<AlertingRule>> {
        // check if rule exists
        let rule_to_update = &self.get_rule(rule_id).await?;

        if rule_to_update.is_none() {
            return Ok(None);
        }

        let clone = rule.clone();

        let rule_to_update = AlertingRuleInsert {
            id: rule.id.clone(),
            name: rule.name.unwrap_or_default(),
            rule_type: rule.rule_type.unwrap_or_default().to_string(),
            metric_name: rule.metric_name.unwrap_or_default(),
            threshold: BigDecimal::from_str(&rule.threshold.unwrap_or_default().to_string())
                .unwrap(),
            trigger: rule.trigger.unwrap_or_default().to_string(),
            duration: rule.grace_period.unwrap_or_default() as i32,
            notification_channel_ids: Some(format!("{:?}", rule.notification_channel_ids)),
        };

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self
            .pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Alerting : No pool found"))?
            .acquire()
            .await
            .map_err(|e| anyhow::anyhow!("Alerting : Failed to acquire connection: {}", e))?;

        sqlx::query!(
            "UPDATE alertingrule SET name = $1, rule_type = $2, metric_name = $3, threshold = $4, trigger = $5, duration = $6, notification_channel_ids = $7 WHERE id = $8",
            rule_to_update.name,
            rule_to_update.rule_type,
            rule_to_update.metric_name,
            rule_to_update.threshold,
            rule_to_update.trigger,
            rule_to_update.duration,
            rule_to_update.notification_channel_ids,
            rule_to_update.id
        ).execute(
            &mut conn
        ).await.map_err(|e| anyhow::anyhow!("Alerting : Failed to update rule: {}", e))?;

        Ok(Some(clone))
    }

    pub async fn get_rule(&self, rule_id: String) -> anyhow::Result<Option<AlertingRule>> {
        let rule: Option<AlertingRuleFetch> = sqlx::query_as!(
            AlertingRuleFetch,
            "SELECT * FROM alertingrule WHERE id = $1",
            rule_id
        )
        .fetch_optional(self.pool.as_ref().unwrap())
        .await
        .map_err(|e| anyhow::anyhow!("Alerting : Failed to fetch rule: {}", e))?;

        Ok(rule.map(|rule| AlertingRule {
            id: rule.id,
            name: Some(rule.name),
            rule_type: Some(RuleType::from_string(&rule.rule_type)),
            metric_name: Some(rule.metric_name),
            threshold: Some(rule.threshold.to_string().parse().unwrap()),
            trigger: Some(RuleTrigger::from_string(&rule.trigger)),
            grace_period: Some(rule.duration as u64),
            notification_channel_ids: rule
                .notification_channel_ids
                .map(|ids| {
                    Some(
                        ids[1..ids.len() - 1]
                            .split(", ")
                            .map(|id| id.to_string())
                            .collect(),
                    )
                })
                .unwrap_or(Some(vec![])),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }

    pub async fn get_rules(&self) -> anyhow::Result<Vec<AlertingRule>> {
        let rules: Vec<AlertingRuleFetch> =
            sqlx::query_as!(AlertingRuleFetch, "SELECT * FROM alertingrule")
                .fetch_all(self.pool.as_ref().unwrap())
                .await
                .map_err(|e| anyhow::anyhow!("Alerting : Failed to fetch rules: {}", e))?;

        // Map AlertingRuleFetch to AlertingRule
        let alerting_rules: Vec<AlertingRule> = rules
            .into_iter()
            .map(|rule| AlertingRule {
                id: rule.id,
                name: Some(rule.name),
                rule_type: Some(RuleType::from_string(&rule.rule_type)),
                metric_name: Some(rule.metric_name),
                threshold: Some(rule.threshold.to_string().parse().unwrap()),
                trigger: Some(RuleTrigger::from_string(&rule.trigger)),
                grace_period: Some(rule.duration as u64),
                notification_channel_ids: rule
                    .notification_channel_ids
                    .map(|ids| {
                        Some(
                            ids[1..ids.len() - 1]
                                .split(", ")
                                .map(|id| id.to_string())
                                .collect(),
                        )
                    })
                    .unwrap_or(Some(vec![])),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .collect();

        Ok(alerting_rules)
    }
}
