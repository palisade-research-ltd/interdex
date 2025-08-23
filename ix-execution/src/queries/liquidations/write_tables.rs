use crate::liquidations::LiquidationNew;
use chrono::{DateTime, TimeZone, Utc};

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

pub fn q_insert_liquidations(
    liquidations: &LiquidationNew,
) -> Result<String, Box<dyn error::Error>> {
    let timestamp_dt: DateTime<Utc> =
        Utc.timestamp_millis_opt(liquidations.ts as i64).unwrap();
    let timestamp = format_datetime_for_clickhouse(&timestamp_dt);

    let query = format!(
        r#"INSERT INTO 
                liquidations
                    (timestamp, symbol, exchange, side, amount, price)
                VALUES 
                    ('{}', '{}', '{}', '{}', '{}', '{}')
            "#,
        timestamp,
        format_symbol_for_clickhouse(&liquidations.symbol),
        liquidations.exchange,
        liquidations.side,
        liquidations.amount,
        liquidations.price,
    );

    Ok(query)
}
