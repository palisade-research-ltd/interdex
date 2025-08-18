pub mod create_tables;
pub mod write_tables;

#[derive(Debug, Clone)]
pub struct TradeNew {
    pub trade_ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: String,
    pub price: String,
    pub exchange: String,
    pub id: String,
}

