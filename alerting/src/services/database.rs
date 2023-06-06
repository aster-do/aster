use std::str::FromStr;

use crate::models::{
    alerting_rule::{_AlertingRule, _RuleTrigger, _RuleType},
    database::{fetch_alerting_rule::AlertingRuleFetch, insert_alerting_rule::AlertingRuleInsert},
};
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
        Self {
            pool: Some(
                PgPoolOptions::new()
                    .max_connections(100)
                    .connect("postgres://postgres:postgres@localhost:5432/postgres")
                    .await
                    .expect("Unable to connect to Postgres"),
            ),
        }
    }

    pub async fn _create_rule(&self, rule: _AlertingRule) -> _AlertingRule {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
            self.pool.as_ref().unwrap().acquire().await.unwrap();

        let rule_to_insert = AlertingRuleInsert {
            id: rule._id.clone(),
            name: rule._name.clone(),
            rule_type: rule._rule_type.to_string(),
            metric_name: rule._metric_name.clone(),
            // TODO: Convert to bigDecimal
            threshold: BigDecimal::from_str(&rule._threshold.to_string()).unwrap(),
            trigger: rule._trigger.to_string(),
            duration: rule._duration as i32,
            notification_channel_ids: Some(format!("{:?}", rule._notification_channel_ids)),
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
        ).await.unwrap();

        rule
    }

    pub async fn _delete_rule(&self, _rule_id: String) {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
            self.pool.as_ref().unwrap().acquire().await.unwrap();

        sqlx::query!("DELETE FROM alertingrule WHERE id = $1", _rule_id)
            .execute(&mut conn)
            .await
            .unwrap();
    }

    pub async fn _update_rule(
        &self,
        _rule_id: String,
        rule: _AlertingRule,
    ) -> Option<_AlertingRule> {
        // check if rule exists
        let rule_to_update = &self._get_rule(_rule_id).await;

        if rule_to_update.is_none() {
            return None;
        }

        let rule_to_update = AlertingRuleInsert {
            id: rule._id.clone(),
            name: rule._name.clone(),
            rule_type: rule._rule_type.to_string(),
            metric_name: rule._metric_name.clone(),
            // TODO: Convert to bigDecimal
            threshold: BigDecimal::from_str(&rule._threshold.to_string()).unwrap(),
            trigger: rule._trigger.to_string(),
            duration: rule._duration as i32,
            notification_channel_ids: Some(format!("{:?}", rule._notification_channel_ids)),
        };

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
            self.pool.as_ref().unwrap().acquire().await.unwrap();

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
        ).await.unwrap();

        Some(rule)
    }

    pub async fn _get_rule(&self, _rule_id: String) -> Option<_AlertingRule> {
        let rule: Option<AlertingRuleFetch> = sqlx::query_as!(
            AlertingRuleFetch,
            "SELECT * FROM alertingrule WHERE id = $1",
            _rule_id
        )
        .fetch_optional(self.pool.as_ref().unwrap())
        .await
        .expect("Unable to query alertingrule table");

        rule.map(|rule| _AlertingRule {
            _id: rule.id,
            _name: rule.name,
            _rule_type: _RuleType::from_string(&rule.rule_type),
            _metric_name: rule.metric_name,
            _threshold: rule.threshold.to_string().parse().unwrap(),
            _trigger: _RuleTrigger::from_string(&rule.trigger),
            _duration: rule.duration as u64,
            _notification_channel_ids: rule
                .notification_channel_ids
                .map(|ids| {
                    ids[1..ids.len() - 1]
                        .split(", ")
                        .map(|id| id.to_string())
                        .collect()
                })
                .unwrap_or(vec![]),
        })
    }

    pub async fn _get_rules(&self) -> Vec<_AlertingRule> {
        let rules: Vec<AlertingRuleFetch> =
            sqlx::query_as!(AlertingRuleFetch, "SELECT * FROM alertingrule")
                .fetch_all(self.pool.as_ref().unwrap())
                .await
                .expect("Unable to query alertingarule table");

        // Map AlertingRuleFetch to AlertingRule
        let alerting_rules: Vec<_AlertingRule> = rules
            .into_iter()
            .map(|rule| _AlertingRule {
                _id: rule.id,
                _name: rule.name,
                _rule_type: _RuleType::from_string(&rule.rule_type),
                _metric_name: rule.metric_name,
                _threshold: rule.threshold.to_string().parse().unwrap(),
                _trigger: _RuleTrigger::from_string(&rule.trigger),
                _duration: rule.duration as u64,
                _notification_channel_ids: rule
                    .notification_channel_ids
                    .map(|ids| {
                        ids[1..ids.len() - 1]
                            .split(", ")
                            .map(|id| id.to_string())
                            .collect()
                    })
                    .unwrap_or(vec![]),
            })
            .collect();
        alerting_rules
    }
}
