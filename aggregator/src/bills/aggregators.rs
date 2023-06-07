use common::models::billable::{BillableAggregate, BillableSQL};
use log::info;
use sqlx::types::chrono::{DateTime, Utc};

use super::AggregatesToBeWritten;

pub fn aggregate(
    billings: Vec<BillableSQL>,
    aggregations: Vec<BillableAggregate>,
) -> AggregatesToBeWritten {
    aggregate_by_hour_and_metric(billings, aggregations)
}

/// This will take metrics and aggregate them by name
fn aggregate_by_hour_and_metric(
    billings: Vec<BillableSQL>,
    aggregations: Vec<BillableAggregate>,
) -> AggregatesToBeWritten {
    let mut inserts: Vec<BillableAggregate> = Vec::new();
    let mut updates: Vec<BillableAggregate> = aggregations.clone();

    let mut tmp_inserts: Vec<BillableAggregate> = Vec::new();

    billings.into_iter().for_each(|bill| {
        // we get the hour by rounding down the timestamp
        let timestamp = DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(
                bill.timestamp.timestamp() - bill.timestamp.timestamp() % 3600,
                0,
            )
            .unwrap_or(bill.timestamp.naive_utc()),
        Utc);

        info!("Aggregating billable {} at {} with values: min: {}, max: {}, avg: {}, count: {}, sum: {}", bill.name, timestamp, bill.value, bill.value, bill.value, 1.0, bill.value);

        match aggregations.iter().position(
            |agg| agg.name == bill.name && agg.timestamp == timestamp,
        ) {
            Some(existing_aggregate_index) => {
                updates[existing_aggregate_index] = BillableAggregate {
                    name:  updates[existing_aggregate_index].name.clone(),
                    timestamp,
                    min: f64::min(bill.value,  updates[existing_aggregate_index].min),
                    max: f64::max(bill.value,  updates[existing_aggregate_index].max),
                    avg: (bill.value + updates[existing_aggregate_index].sum) / ( updates[existing_aggregate_index].count + 1.0),
                    count: updates[existing_aggregate_index].count + 1.0,
                    sum: updates[existing_aggregate_index].sum + bill.value, // sum of values
                };
            }
            None => {
                tmp_inserts.push(BillableAggregate {
                    name: bill.name,
                    timestamp,
                    min: bill.value,
                    max: bill.value,
                    avg: bill.value,
                    count: 1.0,
                    sum: bill.value,
                });
            }
        }

    });

    // we then do the same but in insert
    tmp_inserts.iter().for_each(|agg| {
        match inserts
            .iter()
            .position(|i| i.to_owned().name == agg.name && i.timestamp == agg.timestamp)
        {
            Some(existing_insert_index) => {
                inserts[existing_insert_index] = BillableAggregate {
                    name: inserts[existing_insert_index].name.clone(),
                    timestamp: inserts[existing_insert_index].timestamp,
                    min: f64::min(agg.min, inserts[existing_insert_index].min),
                    max: f64::max(agg.max, inserts[existing_insert_index].max),
                    avg: (agg.avg // because it was the first time it was inserted the average equals the value
                        + inserts[existing_insert_index].avg)
                        / (inserts[existing_insert_index].count + 1.0),
                    count: inserts[existing_insert_index].count + 1.0,
                    sum: inserts[existing_insert_index].sum + agg.sum,
                }
            }
            None => inserts.push(agg.clone()),
        }
    });

    AggregatesToBeWritten {
        _inserts: inserts,
        _updates: updates,
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_aggregation_without_prior_aggregates() {
        let billings = vec![
            BillableSQL {
                id: 0,
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 10, 9).unwrap(),
                value: 3.0,
                treated: false,
            },
            BillableSQL {
                id: 1,
                name: "test".to_string(),
                price: 10,
                timestamp: chrono::Utc
                    .with_ymd_and_hms(2023, 3, 2, 10, 10, 36)
                    .unwrap(),
                value: 1.0,
                treated: false,
            },
            BillableSQL {
                id: 2,
                name: "test2".to_string(),
                price: 10,
                timestamp: chrono::Utc
                    .with_ymd_and_hms(2025, 3, 2, 10, 10, 36)
                    .unwrap(),
                value: 11.0,
                treated: false,
            },
            BillableSQL {
                id: 3,
                name: "test2".to_string(),
                price: 1234,
                timestamp: chrono::Utc
                    .with_ymd_and_hms(2025, 3, 2, 11, 10, 36)
                    .unwrap(),
                value: 1234.0,
                treated: false,
            },
        ];

        let aggregated = aggregate(billings, vec![]);

        assert!(aggregated._updates.is_empty());
        assert_eq!(aggregated._inserts.len(), 3);

        assert_eq!(aggregated._inserts[0].name, "test");
        assert_eq!(aggregated._inserts[0].sum, 4.0);
        assert_eq!(aggregated._inserts[0].avg, 2.0);
        assert_eq!(aggregated._inserts[0].count, 2.0);
        assert_eq!(aggregated._inserts[0].min, 1.0);
        assert_eq!(aggregated._inserts[0].max, 3.0);
        assert_eq!(aggregated._inserts[1].name, "test2");
        assert_eq!(aggregated._inserts[1].sum, 11.0);
        assert_eq!(aggregated._inserts[2].name, "test2");
        assert_eq!(aggregated._inserts[2].sum, 1234.0);

        // we ensure that the hour is rounded down
        assert_eq!(
            aggregated._inserts[0].timestamp,
            chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 0, 0).unwrap()
        );
    }

    #[test]
    fn test_aggregation_with_prior_aggregates() {
        let billings = vec![BillableSQL {
            id: 0,
            name: "test".to_string(),
            price: 10,
            timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 10, 9).unwrap(),
            value: 1.0,
            treated: false,
        }];

        let previous_aggregates = vec![
            BillableAggregate {
                name: "test".to_string(),
                timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 0, 0).unwrap(),
                min: 2.0,
                max: 3.0,
                avg: 2.5,
                count: 2.0,
                sum: 5.0,
            },
            BillableAggregate {
                name: "test2".to_string(),
                timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 0, 0).unwrap(),
                min: 1.0,
                max: 3.0,
                avg: 2.0,
                count: 1.0,
                sum: 4.0,
            },
        ];

        let aggregated = aggregate(billings, previous_aggregates);

        assert!(aggregated._inserts.is_empty());
        assert_eq!(aggregated._updates.len(), 2);

        assert_eq!(
            aggregated._updates,
            vec![
                BillableAggregate {
                    name: "test".to_string(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 0, 0).unwrap(),
                    min: 1.0,
                    max: 3.0,
                    avg: 2.0,
                    count: 3.0,
                    sum: 6.0
                },
                BillableAggregate {
                    name: "test2".to_string(),
                    timestamp: chrono::Utc.with_ymd_and_hms(2023, 3, 2, 10, 0, 0).unwrap(),
                    min: 1.0,
                    max: 3.0,
                    avg: 2.0,
                    count: 1.0,
                    sum: 4.0
                }
            ]
        );
    }
}
