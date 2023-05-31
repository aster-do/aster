# This file describes the alerting API

## Alerting rule structure

```rust
struct AlertingRule {
    name: String,
    ruleType: RuleType,
    metricName: String,
    threshold: f64,
    trigger: Trigger,
    duration: u64,
    notificationChannelIds: Vec<String>,
}

enum RuleType {
    ValueBased,
    PriceBased,
}

enum Trigger {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}
```

## Alerts structure

```rust
struct Alert {
    alertingRuleId: String,
    value: f64,
    status: Status,
    notificationId: String,
}

enum Status {
    Triggered(u64), // timestamp
    Resolved(u64), // timestamp
}
```
