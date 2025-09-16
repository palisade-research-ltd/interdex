// Create the trades table DDL
pub fn create_trades_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS publictrades (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    side String,
    amount String, 
    price String,
    exchange String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, timestamp, exchange)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}
