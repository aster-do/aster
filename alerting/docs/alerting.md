# This file describes the alerting API

## Alerting rule structure

```rust
struct AlertingRule {
    name: String,
    rule_type: RuleType,
    metric_name: String,
    threshold: f64,
    trigger: RuleTrigger,
    duration: u64,
    notification_channel_ids: Vec<String>,
}

enum RuleType {
    ValueBased,
    PriceBased,
}

enum RuleTrigger {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
```

## Alerts structure

```rust
struct Alert {
    alerting_rule_id: String,
    value: f64,
    status: AlertStatus,
    notification_id: String,
}

enum AlertStatus {
    Triggered(DateTime<Utc>), // timestamp
    Resolved(DateTime<Utc>),  // timestamp
}
```
