use common::models::{
    billable_rule::{BillableOperation, BillableRule},
    Billable, Metric,
};

pub fn transform(metric: &Metric, rules: Vec<BillableRule>) -> Billable {
    let mut bill = Billable {
        price: 0,
        name: metric.name.clone(),
        value: metric.value,
        timestamp: metric.timestamp,
    };

    for rule in rules {
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

    #[test]
    fn cpu_conversion() {
        let metric = Metric {
            corelation_id: None,
            name: "cpu".to_string(),
            value: 1.0,
            timestamp: chrono::Utc::now(),
        };

        let rules = get_rules();

        let billable = transform(&metric, rules);

        assert_eq!(billable.name, "cpu");
        assert_eq!(billable.value, 1.0);
        assert_eq!(billable.price, 200);
    }

    #[test]
    fn memory_conversion() {
        let metric = Metric {
            corelation_id: None,
            name: "memory".to_string(),
            value: 1.0,
            timestamp: chrono::Utc::now(),
        };

        let rules = get_rules();

        let billable = transform(&metric, rules);

        assert_eq!(billable.name, "memory");
        assert_eq!(billable.value, 1.0);
        assert_eq!(billable.price, 0);
    }

    fn get_rules() -> Vec<BillableRule> {
        vec![
            BillableRule {
                name: "cpu".to_string(),
                operation: BillableOperation::Multiply,
                number: 200,
            },
            BillableRule {
                name: "memory".to_string(),
                operation: BillableOperation::Multiply,
                number: 0,
            },
        ]
    }
}
