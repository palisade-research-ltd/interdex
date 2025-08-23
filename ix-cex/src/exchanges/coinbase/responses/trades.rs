use serde::Deserialize;

/// Coinbase trades response
#[derive(Debug, Deserialize)]
pub struct CoinbaseTradesResponse {
    pub trades: Vec<CoinbaseTrade>,
    pub best_bid: String,
    pub best_ask: String,
}

/// Coinbase trade
#[derive(Debug, Deserialize)]
pub struct CoinbaseTrade {
    pub trade_id: String,
    pub product_id: String,
    pub price: String,
    pub size: String,
    pub time: String,
    pub side: String,
    pub bid: String,
    pub ask: String,
}

