use crate::client::http_client::{HttpClient, RetryConfig, RetryableHttpClient};
use crate::exchanges::bybit::responses;
use crate::models::orderbook::{Orderbook, TradingPair, PriceLevel};
use chrono::Utc;

use ix_results::errors::{ExchangeError, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, info};

/// Bybit API Client
#[derive(Clone)]
pub struct BybitClient {
    pub client: RetryableHttpClient,
}

impl BybitClient {

    /// Create a new Bybit client
    pub fn new() -> Result<Self> {
        debug!("Fetching Bybit Get New Client");

        let exchange_name = "Bybit".to_string();
        let base_url = "https://api.bybit.com".to_string();
        let timeout_secs = 30;
        let req_per_sec = 10;

        let http_client =
            HttpClient::new(exchange_name, base_url, req_per_sec, timeout_secs)?;

        let retry_client = RetryableHttpClient::new(http_client, RetryConfig::default());

        Ok(Self {
            client: retry_client,
        })
    }

    /// Get order book snapshot for a trading pair
    pub async fn get_orderbook(
        &self,
        pair: TradingPair,
        depth: Option<u32>,
    ) -> Result<Orderbook> {
        let symbol_id = pair.to_exchange_symbol("bybit");
        info!("Fetching Bybit orderbook for {}", symbol_id);

        let mut params: Vec<(&str, String)> = vec![("product_id", symbol_id.clone())];

        if let Some(depth) = depth {
            params.push(("limit", depth.to_string()));
        }

        // Convert params to the expected type for get_with_params_retry
        let params_ref: Vec<(&str, &str)> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response: responses::BybitOrderbookResponse = self
            .client
            .get_with_params_retry("/v5/market/orderbook", &params_ref)
            .await?;

        debug!(
            "Received Bybit orderbook with {} bids, {} asks",
            response.result.bids.len(),
            response.result.asks.len()
        );

        self.convert_to_orderbook(response, symbol_id)
    }

    fn convert_to_orderbook(
        &self,
        response: responses::BybitOrderbookResponse,
        symbol: String,
    ) -> Result<Orderbook> {

        let mut v_bids = Vec::new();
        let mut v_asks = Vec::new();

        // --- 
        for bid in response.result.bids {
            
            let price =
                f64::from_str(&bid.price).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid price '{}': {}", bid.price, e),
                })?;

            let quantity =
                f64::from_str(&bid.qty).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid size '{}': {}", bid.qty, e),
                })?;

            v_bids.push(PriceLevel {
                price: price,
                quantity: quantity,
            });

        }

        // ---  
        for ask in response.result.asks {

            let price =
                f64::from_str(&ask.price).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid price '{}': {}", ask.price, e),
                })?;

            let quantity =
                f64::from_str(&ask.qty).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid size '{}': {}", ask.qty, e),
                })?;

            v_asks.push(PriceLevel {
                price: price,
                quantity: quantity,
            });

        }

        let mut orderbook = Orderbook::new(
            symbol, 
            "Bybit".to_string(),
            Utc::now(),
            v_bids,
            v_asks,
            None,
            None,
        );

        if !orderbook.is_valid() {
            return Err(ExchangeError::ApiError {
                exchange: "Bybit".to_string(),
                message: "Received invalidad orderbook data".to_string(),
            });
        }

        info!(
            "Succesfully converted Bybit orderbok: {} bids, {} asks, spread: {:?}", 
            orderbook.bids.len(),
            orderbook.asks.len(),
            orderbook.spread()
        );

        orderbook.timestamp = Utc::now();

        Ok(orderbook)

    }

    /// Get Bybit server time
    pub async fn get_server_time(&self) -> Result<BybitServerTime> {
        debug!("Fetching Bybit Server Time");

        let response: BybitServerTimeResponse =
            self.client.get_with_retry("/v5/market/time").await?;

        println!("get_server_time.response: {:?}", response);

        if response.ret_code != 0 {
            return Err(ExchangeError::ApiError {
                exchange: "Bybit".to_string(),
                message: format!(
                    "Bybit API Error\n Code: {:?} Message: {:?} \n
                        Time {:?} ExtInof {:?}",
                    response.ret_code,
                    response.ret_msg,
                    response.ret_ext_info,
                    response.time,
                ),
            });
        }

        Ok(response.result)

    }

    /// Get Account Info
    pub async fn get_account_info(&self) -> Result<BybitAccountInfo> {

        debug!("Fetching Bybit Get Account Info");

        let response: BybitAccountInfoResponse =
            self.client.get_with_retry("/v5/account/info").await?;

        println!("get_account_info.response: {:?}", response);

        if response.ret_code != 0 {
            return Err(ExchangeError::ApiError {
                exchange: "Bybit".to_string(),
                message: format!(
                    "Bybit API Error\n Code: {:?} Message: {:?}",
                    response.ret_code, response.ret_msg,
                ),
            });
        }
        Ok(response.result)
    }

    /// Get Wallet Balence
    pub async fn get_wallet_balance(&self) -> Result<BybitWalletBalance> {
        println!("Fetching Bybit Get Wallet Balance");

        let p_endpoint = "/v5/wallet-balance".to_string();
        let p_params = [("accountType", "UNIFIED")];

        let response: BybitWalletBalanceResponse = self
            .client
            .get_with_params_retry(&p_endpoint, &p_params)
            .await?;

        if response.ret_code != 0 {
            return Err(ExchangeError::ApiError {
                exchange: "Bybit".to_string(),
                message: format!(
                    "Bybit API Error\n Code: {:?} Message: {:?}",
                    response.ret_code, response.ret_msg,
                ),
            });
        }
        Ok(response.result)
    }
}

/// Bybit server time
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitServerTime {
    pub time_second: String,
    pub time_nano: String,
}

/// Bybit server time response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitServerTimeResponse {
    ret_code: u64,
    ret_msg: String,
    ret_ext_info: HashMap<String, String>,
    time: u64,
    result: BybitServerTime,
}

/// Bybit account info
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitAccountInfo {
    pub margin_mode: String,
    pub updated_time: String,
    pub unified_margin_status: u64,
    pub dcp_status: String,
    pub time_window: u64,
    pub smp_group: u64,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
}

/// Bybit account info response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitAccountInfoResponse {
    ret_code: u64,
    ret_msg: String,
    result: BybitAccountInfo,
}

/// Bybit wallet balance response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitWalletBalance {
    pub total_equity: String,                  // "3.31216591",
    pub account_im_rate: String,               // "0",
    pub account_im_rate_bymp: String,          // "0",
    pub total_margin_balance: String,          // "3.00326056",
    pub total_initial_margin: String,          // "0",
    pub total_initial_margin_bymp: String,     // "0",
    pub account_type: String,                  // "UNIFIED",
    pub total_available_balance: String,       // "3.00326056",
    pub account_mm_rate: String,               // "0",
    pub account_mm_rate_bymp: String,          // "0",
    pub total_perp_upl: String,                // "0",
    pub total_wallet_balance: String,          // "3.00326056",
    pub account_ltv: String,                   // "0",
    pub total_maintenance_margin: String,      // "0",
    pub total_maintenance_margin_bymp: String, // "0",
    pub coin: BybitWalletBalanceCoin,
}

/// Bybit wallet balance response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitWalletBalanceCoin {
    pub available_to_borrow: String,   // "3",
    pub bonus: String,                 // "0",
    pub accrued_interest: String,      // "0",
    pub available_to_withdraw: String, // "0",
    pub total_order_im: String,        // "0",
    pub equity: String,                // "0",
    pub total_position_mm: String,     // "0",
    pub usd_alue: String,              // "0",
    pub spot_hedging_qty: String,      // "0.01592413",
    pub unrealised_pnl: String,        // "0",
    pub collateral_switch: bool,       // true,
    pub borrow_amount: String,         // "0.0",
    pub total_position_im: String,     // "0",
    pub wallet_balance: String,        // "0",
    pub cum_realised_pnl: String,      // "0",
    pub locked: String,                // "0",
    pub margin_collateral: bool,       // true,
    pub coin: String,                  // "BTC"
}

/// Bybit wallet balance
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitWalletBalanceResponse {
    ret_code: u64,
    ret_msg: String,
    result: BybitWalletBalance,
}
