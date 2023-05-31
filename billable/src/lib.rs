use common::models::{Billable, Metric};

pub fn transform(metric: &Metric) -> Billable {
    let mut bill = Billable {
        price: 0,
        name: metric.name.clone(),
        value: metric.value,
        timestamp: metric.timestamp,
    };

    bill.price = match metric.name.as_str() {
        // CPU Memory and Disk are just here as examples
        // The next step will be to remove them to allow
        // users to define their own rules and metrics types
        "cpu" => (bill.value * 200.) as i64,
        "memory" => bill.value as i64,
        "disk" => 1000,
        _ => 0,
    };

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

        let billable = transform(&metric);

        assert_eq!(billable.name, "cpu");
        assert_eq!(billable.value, 1.0);
        assert_eq!(billable.price, 200);
    }
}
