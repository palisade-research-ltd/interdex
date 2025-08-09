use serde::Deserialize;

/// Bybit order response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: OrderResult,
    #[serde(default)]
    pub next_page_cursor: String,
    #[serde(default)]
    pub category: String, // "linear"
    pub ret_ext_info: serde_json::Value,
    pub time: u64, // 1684765770483
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResult {
    pub list: Vec<OrderBybit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBybit {
    pub order_id: String,
    pub order_link_id: String,
    pub block_trade_id: String,
    pub symbol: String,
    pub price: String, // f64,
    pub qty: String, // f64,
    pub side: String,
    #[serde(default)]
    pub is_leverage: String,
    pub position_idx: String, // i32,
    pub order_status: String,
    pub cancel_type: String,
    pub reject_reason: String,
    pub avg_price: String, // f64,
    pub leaves_qty: String, // f64,
    pub leaves_value: String, // i32,
    pub cum_exec_qty: String, // f64,
    pub cum_exec_value: String, // i32,
    pub cum_exec_fee: String, // i32,
    pub time_in_force: String,
    pub order_type: String, // "Limit",
    pub stop_order_type: String, // "UNKNOWN",
    #[serde(default)] 
    pub order_iv: String, // "",
    pub trigger_price: String, // "0.00",
    pub take_profit: String, // f64, // "2500.00",
    pub stop_loss: String, // f64, // "1500.00",
    pub tp_trigger_by: String, // "LastPrice",
    pub sl_trigger_by: String, // "LastPrice",
    pub trigger_direction: String, // i32, // 0,
    pub trigger_by: String, // "UNKNOWN",
    #[serde(default)]
    pub last_price_on_created: String, //  "",
    pub reduce_only: bool, // false,
    pub close_on_trigger: bool, // false,
    pub smp_type: String, // "None",
    pub smp_group: String, // i32, // 0,
    #[serde(default)]
    pub smp_order_id: String, // "",
    pub tpsl_mode: String, // "Full",
    #[serde(default)]
    pub tp_limit_price: String, // "",
    #[serde(default)]
    pub sl_limit_price: String, // "",
    #[serde(default)]
    pub place_type: String, // "",
    pub created_time: String, // u64, // "1684738540559",
    pub updated_time: String, // u64, // "1684738540561"
}

/// Bybit cancel all order response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: CancelResult,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

/// Bybit cancel all order result structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResult {
    pub list: Vec<Cancel>,
}

/// Bybit cancel all order content
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancel {
    pub order_id: String,
    pub order_link_id: String,
}

