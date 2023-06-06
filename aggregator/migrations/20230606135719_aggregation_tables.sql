-- Add migration script here

CREATE TABLE IF NOT EXISTS billable (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price BIGINT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    treated BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS billable_aggregate (
    name VARCHAR(255) NOT NULL,
    timestamp TIMESTAMP with time zone NOT NULL,
    min DOUBLE PRECISION,
    max DOUBLE PRECISION,
    avg DOUBLE PRECISION,
    count DOUBLE PRECISION,
    sum DOUBLE PRECISION,

    CONSTRAINT composite_key PRIMARY KEY (name, timestamp)
);

CREATE OR REPLACE VIEW billable_aggregate_by_day AS
SELECT
    name,
    timestamp::date as timestamp,
    min(min) as min,
    max(max) as max,
    avg(avg) as avg,
    sum(count) as count,
    sum(sum) as sum
FROM billable_aggregate
GROUP BY 1, 2
ORDER BY 2 DESC;

CREATE OR REPLACE VIEW billable_aggregate_oat AS
SELECT
    name,
    min(min) as min,
    max(max) as max,
    avg(avg) as avg,
    sum(count) as count,
    sum(sum) as sum
FROM billable_aggregate
GROUP BY name;
