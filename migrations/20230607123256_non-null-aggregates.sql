-- Add migration script here

CREATE TABLE IF NOT EXISTS billables.billable (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    treated BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS billables.billable_aggregate (
    name VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP with time zone NOT NULL,
    min DOUBLE PRECISION NOT NULL DEFAULT 0,
    max DOUBLE PRECISION NOT NULL DEFAULT 0,
    avg DOUBLE PRECISION NOT NULL DEFAULT 0,
    count DOUBLE PRECISION NOT NULL DEFAULT 0,
    sum DOUBLE PRECISION NOT NULL DEFAULT 0,

    CONSTRAINT composite_key PRIMARY KEY (name, timestamp)
);

CREATE OR REPLACE VIEW billables.billable_aggregate_by_day AS
SELECT
    name,
    timestamp::date as timestamp,
    min(min) as min,
    max(max) as max,
    avg(avg) as avg,
    sum(count) as count,
    sum(sum) as sum
FROM billables.billable_aggregate
GROUP BY 1, 2
ORDER BY 2 DESC;

CREATE OR REPLACE VIEW billables.billable_aggregate_oat AS
SELECT
    name,
    min(min) as min,
    max(max) as max,
    avg(avg) as avg,
    sum(count) as count,
    sum(sum) as sum
FROM billables.billable_aggregate
GROUP BY name;
