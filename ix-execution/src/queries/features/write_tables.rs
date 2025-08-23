use crate::features::FeatureData;
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

pub fn q_insert_features(
    features: &FeatureData,
) -> Result<String, Box<dyn error::Error>> {
    let timestamp_dt: DateTime<Utc> = Utc
        .timestamp_millis_opt(features.feature_ts as i64)
        .unwrap();

    let timestamp = format_datetime_for_clickhouse(&timestamp_dt);

    let query = format!(
        r#"INSERT INTO 
                features
                    (timestamp, symbol, exchange, spread, midprice, w_midprice, vwap, imb, tav)
                VALUES
                    ('{}','{}','{}','{}','{}','{}','{}','{}','{}')
            "#,
        timestamp,
        format_symbol_for_clickhouse(&features.symbol),
        features.exchange,
        features.spread,
        features.midprice,
        features.w_midprice,
        features.vwap,
        features.imb,
        features.tav
    );

    Ok(query)
}
