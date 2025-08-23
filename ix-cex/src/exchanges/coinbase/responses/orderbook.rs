use serde::Deserialize;

/// Coinbase product book response
#[derive(Debug, Deserialize)]
pub struct CoinbaseProductBookResponse {
    pub pricebook: CoinbasePricebook,
}

/// Coinbase pricebook
#[derive(Debug, Deserialize)]
pub struct CoinbasePricebook {
    pub product_id: String,
    pub bids: Vec<CoinbasePriceLevel>,
    pub asks: Vec<CoinbasePriceLevel>,
    pub time: String,
}

/// Coinbase price level
#[derive(Debug, Deserialize)]
pub struct CoinbasePriceLevel {
    pub price: String,
    pub size: String,
}

