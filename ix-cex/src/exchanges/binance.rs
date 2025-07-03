use crate::client::http_client::{HttpClient, RetryConfig, RetryableHttpClient};
use crate::error::{ExchangeError, Result};
use crate::models::orderbook::{OrderBook, PriceLevel, TradingPair};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;
use tracing::{debug, info, warn};

/// Binance REST API client
#[derive(Clone)]
pub struct BinanceClient {
    client: RetryableHttpClient,
}

impl BinanceClient {
    /// Create a new Binance client
    pub fn new() -> Result<Self> {
        let http_client = HttpClient::new(
            "Binance".to_string(),
            "https://api.binance.com".to_string(),
            10, // 10 requests per second to stay under limits
            30, // 30 second timeout
        )?;

        let retry_client = RetryableHttpClient::new(http_client, RetryConfig::default());

        Ok(Self {
            client: retry_client,
        })
    }

    /// Get order book snapshot for a trading pair
    pub async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<OrderBook> {
        let symbol = pair.to_exchange_symbol("binance");
        let limit_str = limit.unwrap_or(1000).to_string();

        info!(
            "Fetching Binance orderbook for {} with limit {}",
            symbol, limit_str
        );

        let params = vec![("symbol", symbol.as_str()), ("limit", limit_str.as_str())];

        let response: BinanceDepthResponse = self
            .client
            .get_with_params_retry("/api/v3/depth", &params)
            .await?;

        debug!(
            "Received Binance orderbook with {} bids, {} asks",
            response.bids.len(),
            response.asks.len()
        );

        self.convert_to_orderbook(response, symbol)
    }

    /// Convert Binance response to our OrderBook format
    fn convert_to_orderbook(
        &self,
        response: BinanceDepthResponse,
        symbol: String,
    ) -> Result<OrderBook> {
        let mut orderbook = OrderBook::new(symbol, "Binance".to_string());
        orderbook.timestamp = Utc::now();
        orderbook.last_update_id = Some(response.last_update_id);

        // Convert bids (should already be sorted from highest to lowest)
        for bid_array in response.bids {
            if bid_array.len() != 2 {
                warn!(
                    "Invalid bid format from Binance: expected 2 elements, got {}",
                    bid_array.len()
                );
                continue;
            }

            let price = Decimal::from_str(&bid_array[0]).map_err(|e| {
                ExchangeError::ApiError {
                    exchange: "Binance".to_string(),
                    message: format!("Invalid bid price '{}': {}", bid_array[0], e),
                }
            })?;

            let quantity = Decimal::from_str(&bid_array[1]).map_err(|e| {
                ExchangeError::ApiError {
                    exchange: "Binance".to_string(),
                    message: format!("Invalid bid quantity '{}': {}", bid_array[1], e),
                }
            })?;

            orderbook.bids.push(PriceLevel { price, quantity });
        }

        // Convert asks (should already be sorted from lowest to highest)
        for ask_array in response.asks {
            if ask_array.len() != 2 {
                warn!(
                    "Invalid ask format from Binance: expected 2 elements, got {}",
                    ask_array.len()
                );
                continue;
            }

            let price = Decimal::from_str(&ask_array[0]).map_err(|e| {
                ExchangeError::ApiError {
                    exchange: "Binance".to_string(),
                    message: format!("Invalid ask price '{}': {}", ask_array[0], e),
                }
            })?;

            let quantity = Decimal::from_str(&ask_array[1]).map_err(|e| {
                ExchangeError::ApiError {
                    exchange: "Binance".to_string(),
                    message: format!("Invalid ask quantity '{}': {}", ask_array[1], e),
                }
            })?;

            orderbook.asks.push(PriceLevel { price, quantity });
        }

        // Validate the orderbook
        if !orderbook.is_valid() {
            return Err(ExchangeError::ApiError {
                exchange: "Binance".to_string(),
                message: "Received invalid orderbook data".to_string(),
            });
        }

        info!(
            "Successfully converted Binance orderbook: {} bids, {} asks, spread: {:?}",
            orderbook.bids.len(),
            orderbook.asks.len(),
            orderbook.spread()
        );

        Ok(orderbook)
    }

    /// Get exchange information (available symbols, etc.)
    pub async fn get_exchange_info(&self) -> Result<BinanceExchangeInfo> {
        info!("Fetching Binance exchange information");

        self.client.get_with_retry("/api/v3/exchangeInfo").await
    }

    /// Get server time (useful for synchronization)
    pub async fn get_server_time(&self) -> Result<BinanceServerTime> {
        self.client.get_with_retry("/api/v3/time").await
    }

    /// Get 24hr ticker statistics
    pub async fn get_24hr_ticker(&self, pair: TradingPair) -> Result<Binance24hrTicker> {
        let symbol = pair.to_exchange_symbol("binance");
        let params = vec![("symbol", symbol.as_str())];

        self.client
            .get_with_params_retry("/api/v3/ticker/24hr", &params)
            .await
    }
}

/// Binance depth/orderbook response format
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BinanceDepthResponse {
    last_update_id: u64,
    bids: Vec<Vec<String>>, // [price, quantity] pairs
    asks: Vec<Vec<String>>, // [price, quantity] pairs
}

/// Binance exchange info response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub symbols: Vec<BinanceSymbol>,
}

/// Binance symbol information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceSymbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub base_asset_precision: u32,
    pub quote_precision: u32,
}

/// Binance server time response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceServerTime {
    pub server_time: u64,
}

/// Binance 24hr ticker response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binance24hrTicker {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

impl Default for BinanceClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Binance client")
    }
}

