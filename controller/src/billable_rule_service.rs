use common::models::billable_rules::billable_rule_persistent::BillableRulePersistent;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct BillableRuleService {
    pub pool: Pool<Postgres>,
}

impl BillableRuleService {
    pub async fn new(db_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(db_url)
            .await
            .expect("failed to connect to Postgres");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("failed to run migrations");

        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<BillableRulePersistent>, anyhow::Error> {
        let rules = sqlx::query_as!(
            BillableRulePersistent,
            "SELECT id, name, operation as \"operation: _\", number, version FROM billable_rule"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rules)
    }
}
