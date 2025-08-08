use serde::Deserialize;

/// Bybit server time response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: ServerTime,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub time_second: String,
    pub time_nano: String,
}

