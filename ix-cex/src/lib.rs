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

pub mod client;
pub mod exchanges;
pub mod models;
pub mod results;

// Re-export commonly used types
pub use exchanges::{BinanceClient, BybitClient, CoinbaseClient, ExchangeClient, KrakenClient};
pub use ix_results::errors::{ExchangeError, Result};
//pub use models::{Orderbook, OrderbookSummary, PriceLevel};
