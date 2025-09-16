// In y
// our ix-execution/src/queries/trades/mod.rs
use serde::{Serialize, Deserialize};
use clickhouse::Row;
use atelier_quant::data::HasTradeFields; // Import the correct trait

pub mod create_tables;
pub mod read_tables;
pub mod write_tables;

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct ClickhouseTradeData {
    pub timestamp: u64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub exchange: String,
}

// Remove your local trait definition - use the one from atelier_quant instead
// pub trait HasTradeFields { ... } // DELETE THIS

// Implement the trait from atelier_quant
impl HasTradeFields for ClickhouseTradeData {
    fn price(&self) -> f64 {
        // Convert string to f64 if needed
        self.price.parse().expect("failed to parse price")
    }
    fn amount(&self) -> f64 {
        // Convert string to f64 if needed
        self.amount.parse().expect("failed to parse amount")
    }
    
    fn trade_ts(&self) -> u64 {
        self.timestamp
    }
    
}
