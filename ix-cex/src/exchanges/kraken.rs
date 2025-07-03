use crate::client::http_client::{HttpClient, RetryConfig, RetryableHttpClient};
use crate::error::{ExchangeError, Result};
use crate::models::orderbook::{OrderBook, PriceLevel, TradingPair};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{debug, info};

/// Kraken REST API client
#[derive(Clone)]
pub struct KrakenClient {
    client: RetryableHttpClient,
}

impl KrakenClient {
    /// Create a new Kraken client
    pub fn new() -> Result<Self> {
        let http_client = HttpClient::new(
            "Kraken".to_string(),
            "https://api.kraken.com".to_string(),
            1,  // Conservative rate limit for Kraken (1 request per second)
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
        count: Option<u32>,
    ) -> Result<OrderBook> {
        let product_id = pair.to_exchange_symbol("kraken");
        info!("Fetching Kraken orderbook for {}", product_id);

        let mut params: Vec<(&str, String)> = vec![("pair", product_id.clone())];

        if let Some(count) = count {
            params.push(("count", count.to_string()));
        }

        // Convert params to the expected type for get_with_params_retry
        let params_ref: Vec<(&str, &str)> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        // debug!("Params: {:?}", params_ref);

        let response: KrakenDepthResponse = self
            .client
            .get_with_params_retry("/0/public/Depth", &params_ref)
            .await?;

        // Check for API errors
        if !response.error.is_empty() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: format!("Kraken API errors: {:?}", response.error),
            });
        }

        debug!(
            "Received Kraken orderbook response with {} pairs",
            response.result.len()
        );

        // Find the orderbook data for our pair
        let orderbook_data =
            response
                .result
                .values()
                .next()
                .ok_or_else(|| ExchangeError::ApiError {
                    exchange: "Kraken".to_string(),
                    message: "No orderbook data found in response".to_string(),
                })?;

        self.convert_to_orderbook(orderbook_data.clone(), product_id)
    }

    /// Convert Kraken response to our OrderBook format
    fn convert_to_orderbook(
        &self,
        data: KrakenOrderbookData,
        symbol: String,
    ) -> Result<OrderBook> {
        let mut orderbook = OrderBook::new(symbol, "Kraken".to_string());
        let mut ob_ts: u64 = 0;

        for bid in data.bids {
            // Update orderbook timestamp to the newst timestamp found in the levels
            // exclusive from Kraken response.
            if bid.2 > ob_ts {
                let delta_ts = bid.2 - ob_ts;
                ob_ts += delta_ts;
            }

            let price =
                Decimal::from_str(&bid.0).map_err(|e| ExchangeError::ApiError {
                    exchange: "Kraken".to_string(),
                    message: format!("Invalid bid price '{}': {}", bid.0, e),
                })?;
            let quantity =
                Decimal::from_str(&bid.1).map_err(|e| ExchangeError::ApiError {
                    exchange: "Kraken".to_string(),
                    message: format!("Invalid bid volume '{}': {}", bid.1, e),
                })?;
            orderbook.bids.push(PriceLevel { price, quantity });
        }

        for ask in data.asks {
            if ask.2 > ob_ts {
                let delta_ts = ask.2 - ob_ts;
                ob_ts += delta_ts;
            }

            let price =
                Decimal::from_str(&ask.0).map_err(|e| ExchangeError::ApiError {
                    exchange: "Kraken".to_string(),
                    message: format!("Invalid ask price '{}': {}", ask.0, e),
                })?;
            let quantity =
                Decimal::from_str(&ask.1).map_err(|e| ExchangeError::ApiError {
                    exchange: "Kraken".to_string(),
                    message: format!("Invalid ask volume '{}': {}", ask.1, e),
                })?;
            orderbook.asks.push(PriceLevel { price, quantity });
        }

        orderbook.sort();

        if !orderbook.is_valid() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: "Received invalid orderbook data".to_string(),
            });
        }

        info!(
            "Successfully converted Kraken orderbook: {} bids, {} asks, spread: {:?}",
            orderbook.bids.len(),
            orderbook.asks.len(),
            orderbook.spread()
        );

        orderbook.timestamp = DateTime::<Utc>::from_timestamp(ob_ts as i64, 0).unwrap();

        Ok(orderbook)
    }

    /// Get server time
    pub async fn get_server_time(&self) -> Result<KrakenServerTime> {
        debug!("Fetching Kraken server time");

        let response: KrakenServerTimeResponse =
            self.client.get_with_retry("/0/public/Time").await?;

        if !response.error.is_empty() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: format!("Kraken API errors: {:?}", response.error),
            });
        }

        Ok(response.result)
    }

    /// Get system status
    pub async fn get_system_status(&self) -> Result<KrakenSystemStatus> {
        info!("Fetching Kraken system status");

        let response: KrakenSystemStatusResponse =
            self.client.get_with_retry("/0/public/SystemStatus").await?;

        if !response.error.is_empty() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: format!("Kraken API errors: {:?}", response.error),
            });
        }

        Ok(response.result)
    }

    /// Get asset pairs information
    pub async fn get_asset_pairs(&self) -> Result<HashMap<String, KrakenAssetPair>> {
        println!("Fetching Kraken asset pairs");

        let response: KrakenAssetPairsResponse =
            self.client.get_with_retry("/0/public/AssetPairs").await?;

        if !response.error.is_empty() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: format!("Kraken API errors: {:?}", response.error),
            });
        }

        Ok(response.result)
    }

    /// Get ticker information
    pub async fn get_ticker(
        &self,
        pair: TradingPair,
    ) -> Result<HashMap<String, KrakenTicker>> {
        let pair_name = pair.to_exchange_symbol("kraken");
        info!("Fetching Kraken ticker for {}", pair_name);

        let params = vec![("pair", pair_name.as_str())];
        let response: KrakenTickerResponse = self
            .client
            .get_with_params_retry("/0/public/Ticker", &params)
            .await?;

        if !response.error.is_empty() {
            return Err(ExchangeError::ApiError {
                exchange: "Kraken".to_string(),
                message: format!("Kraken API errors: {:?}", response.error),
            });
        }

        Ok(response.result)
    }
}

/// Kraken depth response
#[derive(Debug, Deserialize)]
struct KrakenDepthResponse {
    error: Vec<String>,
    result: HashMap<String, KrakenOrderbookData>,
}

#[derive(Debug, Clone, Deserialize)]
struct KrakenPriceLevel(
    String, // price
    String, // volume
    u64,    // timestamp
);

#[derive(Debug, Clone, Deserialize)]
struct KrakenOrderbookData {
    bids: Vec<KrakenPriceLevel>,
    asks: Vec<KrakenPriceLevel>,
}

/// Kraken server time response
#[derive(Debug, Deserialize)]
struct KrakenServerTimeResponse {
    error: Vec<String>,
    result: KrakenServerTime,
}

/// Kraken server time
#[derive(Debug, Deserialize)]
pub struct KrakenServerTime {
    pub unixtime: u64,
    pub rfc1123: String,
}

/// Kraken system status response
#[derive(Debug, Deserialize)]
struct KrakenSystemStatusResponse {
    error: Vec<String>,
    result: KrakenSystemStatus,
}

/// Kraken system status
#[derive(Debug, Deserialize)]
pub struct KrakenSystemStatus {
    pub status: String,
    pub timestamp: String,
}

/// Kraken asset pairs response
#[derive(Debug, Deserialize)]
struct KrakenAssetPairsResponse {
    error: Vec<String>,
    result: HashMap<String, KrakenAssetPair>,
}

/// Kraken asset pair information
#[derive(Debug, Deserialize)]
pub struct KrakenAssetPair {
    pub altname: String,
    pub wsname: Option<String>,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub pair_decimals: u32,
    pub cost_decimals: u32,
    pub lot_decimals: u32,
    pub lot_multiplier: u32,
    pub leverage_buy: Vec<u32>,
    pub leverage_sell: Vec<u32>,
    pub fees: Vec<Vec<f64>>,
    pub fees_maker: Vec<Vec<f64>>,
    pub fee_volume_currency: String,
    pub margin_call: u32,
    pub margin_stop: u32,
    pub ordermin: String,
    pub costmin: Option<String>,
    pub tick_size: Option<String>,
    pub status: String,
    pub long_position_limit: Option<u32>,
    pub short_position_limit: Option<u32>,
}

/// Kraken ticker response
#[derive(Debug, Deserialize)]
struct KrakenTickerResponse {
    error: Vec<String>,
    result: HashMap<String, KrakenTicker>,
}

/// Kraken ticker data
#[derive(Debug, Deserialize)]
pub struct KrakenTicker {
    pub a: Vec<String>, // ask [price, whole_lot_volume, lot_volume]
    pub b: Vec<String>, // bid [price, whole_lot_volume, lot_volume]
    pub c: Vec<String>, // last trade closed [price, lot_volume]
    pub v: Vec<String>, // volume [today, last_24_hours]
    pub p: Vec<String>, // volume weighted average price [today, last_24_hours]
    pub t: Vec<u32>,    // number of trades [today, last_24_hours]
    pub l: Vec<String>, // low [today, last_24_hours]
    pub h: Vec<String>, // high [today, last_24_hours]
    pub o: String,      // today's opening price
}

impl Default for KrakenClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Kraken client")
    }
}

