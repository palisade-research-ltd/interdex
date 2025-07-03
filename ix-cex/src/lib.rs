//! # Centralized Exchange Client
//!
//! Query Centralized Exchange APIs to get order book data.
//! Currently supports Binance, Coinbase, and Kraken exchanges for BTC/USDC and SOL/USDC pairs.
//!
//! ## Features
//!
//! - Async/await support using Tokio
//! - HTTP client with automatic retries and rate limiting
//! - Structured error handling
//! - Support for multiple exchanges with unified interface
//! - Order book data validation and analysis
//!
//! ## Quick Start
//!
//! ```rust
//! use ix_cex::exchanges::{BinanceClient, ExchangeClient};
//! use ix_cex::models::TradingPair;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BinanceClient::new()?;
//!     let orderbook = client.get_orderbook(TradingPair::BtcUsdc, Some(100)).await?;
//!     
//!     println!("Best bid: {:?}", orderbook.best_bid());
//!     println!("Best ask: {:?}", orderbook.best_ask());
//!     println!("Spread: {:?}", orderbook.spread());
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod exchanges;
pub mod models;

pub use error::{ExchangeError, Result};

// Re-export commonly used types
pub use exchanges::{BinanceClient, CoinbaseClient, ExchangeClient, KrakenClient};
pub use models::{OrderBook, OrderBookSummary, PriceLevel, TradingPair};
