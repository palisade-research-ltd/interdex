
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the operations database
USE operations;

-- Liquidations table
CREATE TABLE IF NOT EXISTS liquidations (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    side String,
    amount String, -- Float64,
    price String -- Float64
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192;

