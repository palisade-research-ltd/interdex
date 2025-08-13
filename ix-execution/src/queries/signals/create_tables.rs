// Create the liquidations table DDL
pub fn create_signals_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS signals (
    timestamp DateTime64(6, 'UTC'),
    exchange String,
    symbol String,
    side String,
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}
