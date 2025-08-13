use chrono::{DateTime, Utc};
use ix_cex::models::orderbook::{Orderbook, PriceLevel};
use std::error;

fn format_symbol_for_clickhouse(symbol: &str) -> String {
    symbol
        .replace("-", "")
        .replace("/", "")
        .to_uppercase()
        .to_owned()
}

/// Format DateTime<Utc> for ClickHouse DateTime64(3, 'UTC')
fn format_datetime_for_clickhouse(dt: &DateTime<Utc>) -> String {
    // Format with millisecond precision (3 decimal places)
    dt.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// Convert PriceLevel vector to ClickHouse Array(Tuple(String, String)) format
fn format_price_levels_for_clickhouse(levels: &[PriceLevel]) -> String {
    let tuples: Vec<String> = levels
        .iter()
        .map(|level| format!("('{}', '{}')", level.price, level.quantity))
        .collect();

    format!("[{}]", tuples.join(", "))
}

pub fn q_insert_orderbook(
    orderbook: &Orderbook,
) -> Result<String, Box<dyn error::Error>> {
    let bids_json = format_price_levels_for_clickhouse(&orderbook.bids);
    let asks_json = format_price_levels_for_clickhouse(&orderbook.asks);
    let timestamp = format_datetime_for_clickhouse(&orderbook.timestamp);

    let query = format!(
        r#"INSERT INTO 
                orderbooks 
                    (symbol, exchange, timestamp, bids, asks)
                VALUES 
                    ('{}', '{}', '{}', '{}', '{}')
            "#,
        format_symbol_for_clickhouse(&orderbook.symbol),
        orderbook.exchange,
        timestamp,
        bids_json.replace("'", "''"), // Escape single quotes
        asks_json.replace("'", "''"), // Escape single quotes
    );

    Ok(query)
}

