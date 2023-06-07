use chrono::Utc;

#[derive(Debug)]
pub struct BillingPersistence {
    pub id: i32,
    pub generated_at: chrono::DateTime<Utc>,
    pub items: Option<String>, // json
}
