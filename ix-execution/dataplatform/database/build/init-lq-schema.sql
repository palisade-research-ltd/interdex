
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the operations database
USE operations;

-- Liquidations table (CORRECT)
CREATE TABLE IF NOT EXISTS liquidations (
    ts DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    side String,
    amount Float64,
    price Float64
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(ts)
ORDER BY (symbol, exchange, ts)
SETTINGS index_granularity = 8192;

