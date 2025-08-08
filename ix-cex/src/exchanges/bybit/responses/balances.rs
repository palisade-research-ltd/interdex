use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    pub wallet_balance: String,
    pub free: Option<String>,
    pub locked: Option<String>,
    pub spot_hedging_qty: String,
    pub borrow_amount: String,
    #[serde(default)]
    pub available_to_withdraw: String,
    pub accrued_interest: String,
    #[serde(default)]
    pub total_order_im: String,
    #[serde(default)]
    pub total_position_im: String,
    #[serde(default)]
    pub total_position_mm: String,
    pub unrealised_pnl: String,
    pub cum_realised_pnl: String,
    pub bonus: String,
    pub margin_collateral: bool,
    pub collateral_switch: bool,
    #[serde(default)]
    pub available_to_borrow: String,
}

