use serde::Deserialize;
use crate::exchanges::bybit::responses::balances::CoinBalance;


/// Bybit wallet balance response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: WalletBalanceResult,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    pub list: Vec<WalletBalance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub account_type: String,
    #[serde(default)]
    pub account_ltv: String,
    #[serde(default)]
    pub account_im_rate: String,
    #[serde(default)]
    pub account_im_rate_by_mp: String,
    #[serde(default)]
    pub account_mm_rate: String,
    #[serde(default)]
    pub account_mm_rate_by_mp: String,

    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,

    #[serde(default)]
    pub total_perp_upl: String,
    pub total_initial_margin: String,
    pub total_initial_margin_by_mp: String,
    pub total_maintenance_margin: String,
    pub total_maintenance_margin_by_mp: String,
    pub coin: Vec<CoinBalance>,
}

