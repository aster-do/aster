use chrono::NaiveDateTime;

pub struct BillablePersistence {
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub timestamp: NaiveDateTime,
    pub value: f64,
    pub treated: bool,
}

#[derive(Debug)]
pub struct BillingPersistence {
    pub id: i32,
    pub generated_at: NaiveDateTime,
    pub items: Option<String>, // json
}
