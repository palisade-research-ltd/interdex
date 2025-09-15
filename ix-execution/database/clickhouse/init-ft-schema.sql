
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the operations database
USE operations;

-- Liquidations table
CREATE TABLE IF NOT EXISTS features_ob (
    timestamp DateTime64(6, 'UTC'),
    amount String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (timestamp)
SETTINGS index_granularity = 8192;

