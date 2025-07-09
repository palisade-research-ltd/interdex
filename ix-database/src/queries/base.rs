// Create the orderbooks table DDL
pub fn create_orderbooks_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS orderbooks (
    symbol String,
    exchange String,
    timestamp DateTime64(3),
    bids Array(Tuple(String, String)),
    asks Array(Tuple(String, String)),
    last_update_id UInt64,
    sequence Nullable(UInt64)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}

/// Create the data_reports table DDL
pub fn create_data_reports_table_ddl() -> String {
    r#"
CREATE TABLE IF NOT EXISTS data_reports (
    id UUID,
    report_type String,
    symbol String,
    exchange String,
    timestamp DateTime64(3),
    data String,
    metrics Map(String, Float64)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (report_type, symbol, exchange, timestamp)
SETTINGS index_granularity = 8192
"#
    .trim()
    .to_string()
}
