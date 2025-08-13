// Create the orderbooks table DDL
pub fn create_orderbooks_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS orderbooks (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    bids Array(Tuple(String, String)),
    asks Array(Tuple(String, String)),
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}

