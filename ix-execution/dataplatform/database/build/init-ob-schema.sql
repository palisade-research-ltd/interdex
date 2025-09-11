
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the trading database
USE operations;

-- Orderbooks table
CREATE TABLE IF NOT EXISTS orderbooks (
    ts DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    bids Array(Tuple(Float64, Float64)),
    asks Array(Tuple(Float64, Float64))
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(ts)
ORDER BY (symbol, exchange, ts)
SETTINGS index_granularity = 8192;

