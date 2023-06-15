pub struct AlertingRuleInsert {
    pub id: String,
    pub name: String,
    pub rule_type: String,
    pub metric_name: String,
    pub threshold: sqlx::types::BigDecimal,
    pub trigger: String,
    pub grace_period: i32,
    pub notification_channel_ids: Option<String>,
}
