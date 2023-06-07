use std::sync::Arc;

use common::models::{
    billable_rules::billable_rule::{BillableOperation, BillableRule},
    Billable, Metric,
};
use tokio::sync::RwLock;

pub async fn transform(metric: &Metric, rules: Arc<RwLock<Vec<BillableRule>>>) -> Billable {
    let mut bill = Billable {
        price: 0,
        name: metric.name.clone(),
        value: metric.value,
        timestamp: metric.timestamp,
    };

    for rule in rules.read().await.iter() {
        if rule.name == metric.name {
            bill.price = match rule.operation {
                BillableOperation::Add => (metric.value + rule.number as f64) as i64,
                BillableOperation::Subtract => (metric.value - rule.number as f64) as i64,
                BillableOperation::Multiply => (metric.value * rule.number as f64) as i64,
                BillableOperation::Divide => (metric.value / rule.number as f64) as i64,
            };
            break;
        }
    }
    bill
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cpu_conversion() {
        let metric = Metric {
            corelation_id: None,
            name: "cpu".to_string(),
            value: 1.0,
            timestamp: chrono::Utc::now(),
        };

        let rules = get_rules();

        let billable = transform(&metric, rules).await;

        assert_eq!(billable.name, "cpu");
        assert_eq!(billable.value, 1.0);
        assert_eq!(billable.price, 200);
    }

    #[tokio::test]
    async fn memory_conversion() {
        let metric = Metric {
            corelation_id: None,
            name: "memory".to_string(),
            value: 1.0,
            timestamp: chrono::Utc::now(),
        };

        let rules = get_rules();

        let billable = transform(&metric, rules).await;

        assert_eq!(billable.name, "memory");
        assert_eq!(billable.value, 1.0);
        assert_eq!(billable.price, 0);
    }

    fn get_rules() -> Arc<RwLock<Vec<BillableRule>>> {
        Arc::new(RwLock::new(vec![
            BillableRule {
                id: 1,
                name: "cpu".to_string(),
                operation: BillableOperation::Multiply,
                number: 200,
                version: 1,
            },
            BillableRule {
                id: 2,
                name: "memory".to_string(),
                operation: BillableOperation::Multiply,
                number: 0,
                version: 1,
            },
        ]))
    }
}
