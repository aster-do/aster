-- Add migration script here
CREATE TABLE AlertingRule (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    rule_type VARCHAR(20) CHECK (rule_type IN ('value_based', 'price_based')) NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    threshold DECIMAL(10, 2) NOT NULL,
    trigger VARCHAR(20) CHECK (trigger IN ('greater_than', 'less_than', 'equal', 'not_equal')) NOT NULL,
    duration INT NOT NULL,
    notification_channel_ids TEXT
);

