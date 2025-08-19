use serde::{Serialize, Deserialize};
use clickhouse::Row;

pub mod create_tables;
pub mod read_tables;
pub mod write_tables;

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct TradeNew {
    pub trade_ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub exchange: String,
}

