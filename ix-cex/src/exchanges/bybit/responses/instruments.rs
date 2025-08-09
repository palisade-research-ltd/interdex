use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: InstrumentResult,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentResult {
    pub category: String,
    pub list: Vec<InstrumentInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub innovation: String,
    pub status: String,
    pub margin_trading: String,
    pub st_tag: String,
    pub lot_size_filter: LotSize,
    pub price_filter: PriceFilter,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSize { 
    pub base_precision: String, 
    pub quote_precision: String,
    pub min_order_qty: String,
    pub max_order_qty: String,
    pub min_order_amt: String,
    pub max_order_amt: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub tick_size: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub price_limit_ratio_x: String,
    pub price_limit_ratio_y: String,
}

