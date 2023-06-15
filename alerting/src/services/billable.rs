use common::models::Billable;
use log::{debug, info};
use notification::{
    models::notification::Notification, services::notification::Notificationservice,
};

use crate::models::alerting_rule::{RuleTrigger, RuleType};

use super::rule::RuleService;

const RETENTION_DURATION_HOUR: u32 = 1;

#[derive(Debug)]
struct InMemoryBillable {
    billable: Billable,
    added_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct BillableService {
    //Config & stateful info
    notification_service: Notificationservice,
    rule_service: RuleService,
    billables: Vec<InMemoryBillable>,
}

impl BillableService {
    pub async fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            //Config & stateful info
            notification_service: Notificationservice::new()?,
            rule_service: RuleService::new().await?,
            billables: Vec::new(),
        })
    }

    pub async fn handle_billable(&mut self, billable: Billable) -> Result<(), anyhow::Error> {
        debug!("Adding billable to memory: {:?}", billable);
        // Add billable to in-memory list
        self.billables.push(InMemoryBillable {
            billable: billable.clone(),
            added_at: chrono::Utc::now(),
        });

        debug!(
            "Removing old billables, retention duration: {} hours",
            RETENTION_DURATION_HOUR
        );
        // Remove old billables
        self.billables.retain(|b| {
            let now = chrono::Utc::now();
            let duration = chrono::Duration::hours(RETENTION_DURATION_HOUR as i64);
            b.added_at + duration > now
        });

        debug!("Checking rules for billable: {:?}", billable);
        // Check rules
        for rule in self
            .rule_service
            .get_rules()
            .await?
            .iter()
            .filter(|rule| rule.metric_name == billable.clone().name)
        {
            if self.check_trigger(&billable, &rule.trigger, rule.threshold, &rule.rule_type)? {
                info!("Rule triggered: {:?}", rule);

                let alert_message = format!(
                    "Rule triggered: {:?}, billable: {:?}",
                    rule,
                    billable.clone()
                );

                self.notification_service
                    .handle_notification(Notification::new("test".to_string(), alert_message))?;
            }
        }

        Ok(())
    }

    fn check_trigger(
        &self,
        billable: &Billable,
        trigger: &RuleTrigger,
        threshold: f64,
        rule_type: &RuleType,
    ) -> Result<bool, anyhow::Error> {
        let sum = self
            .billables
            .iter()
            .filter(|b| b.billable.name == billable.name)
            .map(|b| match rule_type {
                RuleType::PriceBased => b.billable.price as f64,
                RuleType::ValueBased => b.billable.value,
            })
            .reduce(|a, b| a + b)
            .unwrap_or_default();

        match trigger {
            RuleTrigger::GreaterThan => Ok(sum > threshold),
            RuleTrigger::LessThan => Ok(sum < threshold),
            RuleTrigger::Equal => Ok(sum == threshold),
            RuleTrigger::NotEqual => Ok(sum != threshold),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::models::Billable;

    use super::InMemoryBillable;

    use crate::{
        models::alerting_rule::{RuleTrigger, RuleType},
        services::billable::BillableService,
    };

    #[tokio::test]
    async fn test_check_trigger_greater_than() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::GreaterThan;
        let threshold = 12.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 10.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_not_trigger_greater_than() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::GreaterThan;
        let threshold = 12.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 3.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 2.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 4.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(!result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_trigger_less_than() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::LessThan;
        let threshold = 32.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 10.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_trigger_equal() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::Equal;
        let threshold = 22.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 12.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_trigger_not_equal() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::NotEqual;
        let threshold = 12.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 10.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_trigger_different_names() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::GreaterThan;
        let threshold = 12.0;
        let rule_type = RuleType::ValueBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 10.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test2".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test2".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(!result);

        Ok(())
    }

    #[tokio::test]
    async fn test_check_trigger_price_based() -> Result<(), anyhow::Error> {
        let trigger = RuleTrigger::GreaterThan;
        let threshold = 12.0;
        let rule_type = RuleType::PriceBased;

        let incoming_billable = Billable {
            name: "test".to_string(),
            value: 10.0,
            price: 10,
            timestamp: Utc::now(),
        };

        let in_memory_billables = vec![
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
            Billable {
                name: "test".to_string(),
                value: 5.0,
                price: 5,
                timestamp: Utc::now(),
            },
        ];

        let mut billable_service = BillableService::new().await?;

        billable_service.billables = in_memory_billables
            .iter()
            .map(|b| InMemoryBillable {
                billable: b.clone(),
                added_at: Utc::now(),
            })
            .collect();

        billable_service.billables.push(InMemoryBillable {
            billable: incoming_billable.clone(),
            added_at: Utc::now(),
        });

        let result =
            billable_service.check_trigger(&incoming_billable, &trigger, threshold, &rule_type)?;

        assert!(result);

        Ok(())
    }
}
