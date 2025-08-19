use serde::{Deserialize, Serialize};
use clickhouse::Row;
use chrono::{DateTime, Utc};

pub mod create_tables;
pub mod read_tables;
pub mod write_tables;

// This struct matches your ClickHouse schema exactly
#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct OrderbookCH {
    pub timestamp: String,
    pub symbol: String,
    pub exchange: String,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

