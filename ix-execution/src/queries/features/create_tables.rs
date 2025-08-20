// Create the order features table
pub fn create_order_features_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS order_features (
    timestamp DateTime64(6, 'UTC'),
    symbol String,
    exchange String,
    spread String,
    midprice String,
    w_midprice String,
    vwap String,
    imb String,
    tav String,
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}
