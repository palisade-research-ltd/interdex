use serde::Deserialize;

/// Bybit executed order (trade) response structure
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    //pub result: TradeResult,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

/// Bybit executed order (trade) result structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResult {
    pub list: Vec<TradeBybit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeBybit {
    pub order_id: String,
    pub order_link: String,
}

