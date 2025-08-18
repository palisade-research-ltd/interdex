use serde::Deserialize;

/// Bybit Orderbook response structure
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitOrderbookResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: BybitOrderbook,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

/// Bybit Orderbook result structure
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitOrderbook {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub asks: Vec<PriceLevel>,
    #[serde(rename = "b")]
    pub bids: Vec<PriceLevel>,
    #[serde(rename = "ts")]
    pub timestamp: String,
    #[serde(rename = "update_id")]
    pub u: String,
    #[serde(rename = "sequence_id")]
    pub seq: String,
    #[serde(rename = "executed_ts")]
    pub cts: String,
}

/// Bybit PriceLevel result structure
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevel {
    pub price: String,
    pub qty: String,
}

