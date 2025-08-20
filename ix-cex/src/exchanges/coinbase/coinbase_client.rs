use crate::client::http_client::{HttpClient, RetryConfig, RetryableHttpClient};
use crate::models::orderbook::{Orderbook, PriceLevel, TradingPair};
use crate::exchanges::coinbase::responses::{orderbook, trades};

use chrono::Utc;
use ix_results::errors::{ExchangeError, Result};
use serde::Deserialize;
use std::str::FromStr;
use tracing::{debug, info};

/// Coinbase Advanced Trade API client
#[derive(Clone)]
pub struct CoinbaseClient {
    client: RetryableHttpClient,
}

impl CoinbaseClient {
    /// Create a new Coinbase client
    pub fn new() -> Result<Self> {
        let http_client = HttpClient::new(
            "Coinbase".to_string(),
            "https://api.coinbase.com".to_string(),
            10, // 10 requests per second
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
        depth: Option<u32>,
    ) -> Result<Orderbook> {
        let product_id = pair.to_exchange_symbol("coinbase");
        info!("Fetching Coinbase orderbook for {}", product_id);

        let mut params: Vec<(&str, String)> = vec![("product_id", product_id.clone())];

        if let Some(depth) = depth {
            params.push(("limit", depth.to_string()));
        }

        // Convert params to the expected type for get_with_params_retry
        let params_ref: Vec<(&str, &str)> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response: orderbook::CoinbaseProductBookResponse = self
            .client
            .get_with_params_retry("/api/v3/brokerage/market/product_book", &params_ref)
            .await?;

        debug!(
            "Received Coinbase orderbook with {} bids, {} asks",
            response.pricebook.bids.len(),
            response.pricebook.asks.len()
        );

        self.convert_to_orderbook(response, product_id)
    }

    /// Convert Coinbase response to our OrderBook format
    fn convert_to_orderbook(
        &self,
        response: orderbook::CoinbaseProductBookResponse,
        symbol: String,
    ) -> Result<Orderbook> {
        let mut v_bids = Vec::new();
        let mut v_asks = Vec::new();

        // Convert bids
        for bid in response.pricebook.bids {
            let price =
                f64::from_str(&bid.price).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid price '{}': {}", bid.price, e),
                })?;

            let quantity =
                f64::from_str(&bid.size).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid bid size '{}': {}", bid.size, e),
                })?;

            // orderbook.bids.push(PriceLevel { price, quantity });
            v_bids.push(PriceLevel { price, quantity });
        }

        // Convert asks
        for ask in response.pricebook.asks {
            let price =
                f64::from_str(&ask.price).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid ask price '{}': {}", ask.price, e),
                })?;

            let quantity =
                f64::from_str(&ask.size).map_err(|e| ExchangeError::ApiError {
                    exchange: "Coinbase".to_string(),
                    message: format!("Invalid ask size '{}': {}", ask.size, e),
                })?;

            // orderbook.asks.push(PriceLevel { price, quantity });
            v_asks.push(PriceLevel { price, quantity });
        }

        // Final value
        let orderbook = Orderbook::new(
            symbol,
            "Coinbase".to_string(),
            Utc::now(),
            v_bids,
            v_asks,
            None,
            None,
        );

        // Validate the orderbook
        if !orderbook.is_valid() {
            return Err(ExchangeError::ApiError {
                exchange: "Coinbase".to_string(),
                message: "Received invalid orderbook data".to_string(),
            });
        }

        info!(
            "Successfully converted Coinbase orderbook: {} bids, {} asks, spread: {:?}",
            orderbook.bids.len(),
            orderbook.asks.len(),
            orderbook.spread()
        );

        Ok(orderbook)
    }

    /// Get all products (trading pairs)
    pub async fn get_products(&self) -> Result<Vec<CoinbaseProduct>> {
        info!("Fetching Coinbase products");

        let response: CoinbaseProductsResponse = self
            .client
            .get_with_retry("/api/v3/brokerage/products")
            .await?;

        Ok(response.products)
    }

    /// Get specific product information
    pub async fn get_product(&self, product_id: &str) -> Result<CoinbaseProduct> {
        info!("Fetching Coinbase product info for {}", product_id);

        let endpoint = format!("/api/v3/brokerage/products/{product_id}");
        self.client.get_with_retry(&endpoint).await
    }

    /// Get market trades
    pub async fn get_market_trades(
        &self,
        product_id: &str,
        depth: Option<u32>,
    ) -> Result<trades::CoinbaseTradesResponse> {
        let endpoint = format!("/api/v3/brokerage/products/{product_id}/ticker");
        let mut params: Vec<(&str, String)> =
            vec![("product_id", product_id.to_string())];

        if let Some(depth) = depth {
            params.push(("depth", depth.to_string()));
        }

        // Convert params to the expected type for get_with_params_retry
        let params_ref: Vec<(&str, &str)> =
            params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.client
            .get_with_params_retry(&endpoint, &params_ref)
            .await
    }
}

// /// Coinbase product book response
// #[derive(Debug, Deserialize)]
// pub struct CoinbaseProductBookResponse {
//     pub pricebook: CoinbasePricebook,
// }
//
// /// Coinbase pricebook
// #[derive(Debug, Deserialize)]
// pub struct CoinbasePricebook {
//     pub product_id: String,
//     pub bids: Vec<CoinbasePriceLevel>,
//     pub asks: Vec<CoinbasePriceLevel>,
//     pub time: String,
// }
//
// /// Coinbase price level
// #[derive(Debug, Deserialize)]
// pub struct CoinbasePriceLevel {
//     pub price: String,
//     pub size: String,
// }

/// Coinbase products response
#[derive(Debug, Deserialize)]
pub struct CoinbaseProductsResponse {
    pub products: Vec<CoinbaseProduct>,
}

/// Coinbase product information
#[derive(Debug, Deserialize)]
pub struct CoinbaseProduct {
    pub product_id: String,
    pub price: Option<String>,
    pub price_percentage_change_24h: Option<String>,
    pub volume_24h: Option<String>,
    pub volume_percentage_change_24h: Option<String>,
    pub base_increment: String,
    pub quote_increment: String,
    pub quote_min_size: String,
    pub quote_max_size: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub base_name: String,
    pub quote_name: String,
    pub watched: bool,
    pub is_disabled: bool,
    pub new: bool,
    pub status: String,
    pub cancel_only: bool,
    pub depth_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
    pub auction_mode: bool,
    pub product_type: String,
    pub quote_currency_id: String,
    pub base_currency_id: String,
}

// /// Coinbase trades response
// #[derive(Debug, Deserialize)]
// pub struct CoinbaseTradesResponse {
//     pub trades: Vec<CoinbaseTrade>,
//     pub best_bid: String,
//     pub best_ask: String,
// }

/// Coinbase trade
// #[derive(Debug, Deserialize)]
// pub struct CoinbaseTrade {
//     pub trade_id: String,
//     pub product_id: String,
//     pub price: String,
//     pub size: String,
//     pub time: String,
//     pub side: String,
//     pub bid: String,
//     pub ask: String,
// }

impl Default for CoinbaseClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Coinbase client")
    }
}
