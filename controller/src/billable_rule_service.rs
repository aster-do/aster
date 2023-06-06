use common::models::billable_rules::{
    billable_rule::BillableRule, billable_rule_persistent::BillableRulePersistent,
};
use sqlx::{postgres::PgConnectOptions, PgPool, Pool, Postgres};
use std::str::FromStr;

pub struct BillableRuleService {
    pub pool: Pool<Postgres>,
}

impl BillableRuleService {
    pub async fn new(db_url: &str) -> Self {
        let pool = PgPool::connect_with(
            PgConnectOptions::from_str(db_url)
                .unwrap()
                .options([("search_path", "controller")]),
        )
        .await
        .expect("failed to connect to Postgres");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("failed to run migrations");

        Self { pool }
    }

    pub async fn create(
        &self,
        rule: &BillableRule,
    ) -> Result<BillableRulePersistent, anyhow::Error> {
        let rule_persistent = BillableRulePersistent::from(rule)?;

        let rule = sqlx::query_as!(
            BillableRulePersistent,
            "INSERT INTO controller.billable_rule (name, operation, number, version) VALUES ($1, $2, $3, $4) RETURNING id, name, operation as \"operation: _\", number, version",
            rule_persistent.name,
            rule_persistent.operation as _,
            rule_persistent.number as i32,
            rule_persistent.version
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rule)
    }

    pub async fn get_all(&self) -> Result<Vec<BillableRulePersistent>, anyhow::Error> {
        let rules = sqlx::query_as!(
            BillableRulePersistent,
            "SELECT id, name, operation as \"operation: _\", number, version FROM controller.billable_rule"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rules)
    }

    pub async fn get_by_id(
        &self,
        id: i32,
    ) -> Result<Option<BillableRulePersistent>, anyhow::Error> {
        let rule = sqlx::query_as!(
            BillableRulePersistent,
            "SELECT id, name, operation as \"operation: _\", number, version FROM controller.billable_rule WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rule)
    }

    pub async fn update(
        &self,
        rule: &BillableRulePersistent,
    ) -> Result<BillableRulePersistent, anyhow::Error> {
        let rule = sqlx::query_as!(
            BillableRulePersistent,
            "UPDATE controller.billable_rule SET name = $1, operation = $2, number = $3, version = $4 WHERE id = $5 RETURNING id, name, operation as \"operation: _\", number, version",
            rule.name,
            rule.operation as _,
            rule.number as i32,
            rule.version,
            rule.id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rule)
    }

    pub async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        sqlx::query!("DELETE FROM controller.billable_rule WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
