use serde::Deserialize;

/// Bybit account info response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AccountInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub unified_margin_status: i32,
    pub margin_mode: String,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
    pub updated_time: String,
    pub dcp_status: String,
    pub time_window: u32,
    pub smp_group: u32,
}

