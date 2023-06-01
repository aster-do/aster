use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

pub mod aggregators;

pub struct BillPerMetricAndHour {
    pub id: Option<Uuid>,
    /// name of the metric
    pub name: String,
    /// Hour at which the bill was issued (rounded down)
    pub hour: DateTime<Utc>,
    /// total price of bill
    pub price: i64,
}

pub struct AverageMetricPerHour {
    /// name of the metric
    pub name: String,
    /// Hour at which the bill was issued (rounded down)
    pub hour: DateTime<Utc>,
    /// total price of bill
    pub average: f64,
}
