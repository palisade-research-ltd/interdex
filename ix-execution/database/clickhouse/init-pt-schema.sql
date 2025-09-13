
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the trading database
USE operations;

-- Trades
CREATE TABLE IF NOT EXISTS publictrades (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    side String,
    amount String, -- Float64(6), 
    price String, -- Float64(6),
    exchange String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192;

