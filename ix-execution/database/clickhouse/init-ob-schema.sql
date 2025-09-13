
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the trading database
USE operations;

-- Orderbooks table
CREATE TABLE IF NOT EXISTS orderbooks (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    bids Array(Tuple(String, String)),
    asks Array(Tuple(String, String))
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192;

