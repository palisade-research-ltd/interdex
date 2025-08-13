pub mod create_tables;
pub mod write_tables;

// In your LiquidationData struct
#[derive(Debug, Clone)]
pub struct LiquidationNew {
    pub ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub exchange: String,
}

