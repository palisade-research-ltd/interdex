pub mod binance;
pub mod coinbase;
pub mod kraken;

pub use binance::binance_rest::BinanceRestClient;
pub use coinbase::coinbase_client::CoinbaseRestClient;
pub use kraken::kraken_client::KrakenRestClient;

use crate::models::orderbook::{Orderbook, TradingPair};
use ix_results::errors::Result;

/// Trait for exchange clients
#[async_trait::async_trait]
pub trait ExchangeClient {
    /// Get order book snapshot for a trading pair
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<Orderbook>;

    /// Get the exchange name
    fn exchange_name(&self) -> &str;
}

#[async_trait::async_trait]
impl ExchangeClient for BinanceRestClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<Orderbook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Binance"
    }
}

#[async_trait::async_trait]
impl ExchangeClient for CoinbaseRestClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<Orderbook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Coinbase"
    }
}

#[async_trait::async_trait]
impl ExchangeClient for KrakenRestClient {
    async fn get_orderbook(
        &self,
        pair: TradingPair,
        limit: Option<u32>,
    ) -> Result<Orderbook> {
        self.get_orderbook(pair, limit).await
    }

    fn exchange_name(&self) -> &str {
        "Kraken"
    }
}
