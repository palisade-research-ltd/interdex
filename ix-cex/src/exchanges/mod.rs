pub mod binance;
pub mod coinbase;
pub mod kraken;

pub use binance::binance_client::BinanceClient;
pub use coinbase::coinbase_client::CoinbaseClient;
pub use kraken::kraken_client::KrakenClient;

use crate::models::orderbook::{OrderBook, TradingPair};
use ix_results::errors::Result;

/// Trait for exchange clients
#[async_trait::async_trait]
pub trait ExchangeClient {
    /// Get order book snapshot for a trading pair
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<OrderBook>;

    /// Get the exchange name
    fn exchange_name(&self) -> &str;
}

#[async_trait::async_trait]
impl ExchangeClient for BinanceClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<OrderBook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Binance"
    }
}

#[async_trait::async_trait]
impl ExchangeClient for CoinbaseClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<OrderBook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Coinbase"
    }
}

#[async_trait::async_trait]
impl ExchangeClient for KrakenClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<OrderBook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Kraken"
    }
}
