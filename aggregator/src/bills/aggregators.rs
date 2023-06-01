use common::models::billable::BillableSQL;

use crate::bills::{AverageMetricPerHour, BillPerMetricAndHour};

pub fn aggregate(
    billings: Vec<BillableSQL>,
) -> (Vec<BillPerMetricAndHour>, Vec<AverageMetricPerHour>) {
    (
        aggregate_by_hour_and_metric(billings),
        vec![], //TODO
    )
}

/// This will take metrics and aggregate them by name
fn aggregate_by_hour_and_metric(billings: Vec<BillableSQL>) -> Vec<BillPerMetricAndHour> {
    let mut bills_per_metric: Vec<BillPerMetricAndHour> = Vec::new();

    billings.into_iter().for_each(|bill| {
        let bill_per_metric = BillPerMetricAndHour {
            id: None,
            name: bill.name,
            hour: bill.timestamp,
            price: bill.price,
        };

        if bills_per_metric.is_empty() {
            bills_per_metric.push(bill_per_metric);
        } else {
            let mut found = false;
            for bill in bills_per_metric.iter_mut() {
                if bill.name == bill_per_metric.name {
                    bill.price += bill_per_metric.price;
                    found = true;
                    break;
                }
            }
            if !found {
                bills_per_metric.push(bill_per_metric);
            }
        }
    });

    bills_per_metric
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregation() {
        let billings = vec![
            BillableSQL {
                id: 0,
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
            BillableSQL {
                id: 1,
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
            BillableSQL {
                id: 2,
                name: "test2".to_string(),
                price: 10,
                timestamp: chrono::Utc::now(),
                value: 1.0,
                treated: false,
            },
        ];

        let aggregated = aggregate(billings);

        assert_eq!(aggregated.0.len(), 2);
        assert_eq!(aggregated.0[0].name, "test");
        assert_eq!(aggregated.0[0].price, 20);
        assert_eq!(aggregated.0[1].name, "test2");
        assert_eq!(aggregated.0[1].price, 10);
    }
}