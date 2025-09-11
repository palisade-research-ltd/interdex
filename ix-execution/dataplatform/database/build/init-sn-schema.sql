
-- Create the database if does not exist
CREATE DATABASE IF NOT EXISTS operations;

-- Use the trading database
USE operations;

-- Signals table (from the signals service)
CREATE TABLE IF NOT EXISTS signals (
    ts DateTime64(6, 'UTC'),
    signal_id String,
    symbol String,
    signal_type Enum8('buy' = 1, 'sell' = 2),
    strength Float64,
    confidence Float64,
    source String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(ts)
ORDER BY (symbol, signal_type, ts)
SETTINGS index_granularity = 8192;

