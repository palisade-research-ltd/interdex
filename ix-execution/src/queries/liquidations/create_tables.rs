// Create the liquidations table DDL
pub fn create_liquidations_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS liquidations (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    side String,
    amount String,
    price String,
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}
