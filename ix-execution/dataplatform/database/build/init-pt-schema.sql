
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the trading database
USE operations;

-- Trades
CREATE TABLE IF NOT EXISTS publictrades (
    ts DateTime64(6, 'UTC'),
    symbol String,
    side String,
    amount Float64(6), 
    price Float64(6),
    exchange String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(ts)
ORDER BY (symbol, exchange, ts)
SETTINGS index_granularity = 8192;

