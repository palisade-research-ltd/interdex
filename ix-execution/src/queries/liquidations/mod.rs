use serde::{Deserialize, Serialize};
use clickhouse::Row;

pub mod create_tables;
pub mod write_tables;
pub mod read_tables;

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct LiquidationNew {
    pub ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub exchange: String,
}

