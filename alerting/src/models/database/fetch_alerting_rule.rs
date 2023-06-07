pub struct _AlertingRuleFetch {
    pub id: String,
    pub name: String,
    pub rule_type: String,
    pub metric_name: String,
    pub threshold: sqlx::types::BigDecimal,
    pub trigger: String,
    pub duration: i32,
    pub notification_channel_ids: Option<String>,
}
