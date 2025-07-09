//! Data structures for orderbook entries and related types
//!
//! This module defines the core data structures used for parsing and storing
//! orderbook data from various exchanges.

use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use ix_cex::models::orderbook::{PriceLevel, OrderBook};

/// Data report structure for analytics
#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct DataReport {
    pub id: Uuid,
    pub report_type: String,
    pub symbol: String,
    pub exchange: String,
    pub timestamp: DateTime<Utc>,
    pub data: String, // JSON string
    pub metrics: std::collections::HashMap<String, f64>,
}

impl DataReport {
    /// Create a new data report
    pub fn new(
        report_type: impl Into<String>,
        symbol: impl Into<String>,
        exchange: impl Into<String>,
        data: impl Serialize,
        metrics: std::collections::HashMap<String, f64>,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            report_type: report_type.into(),
            symbol: symbol.into(),
            exchange: exchange.into(),
            timestamp: Utc::now(),
            data: serde_json::to_string(&data)?,
            metrics,
        })
    }

    /// Parse data field as JSON
    pub fn parse_data<T: for<'de> Deserialize<'de>>(
        &self,
    ) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.data)
    }
}

/// Input structure for JSON parsing (matches the provided format)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookInput {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: String,
    pub bids: Vec<PriceLevelInput>,
    pub asks: Vec<PriceLevelInput>,
    pub last_update_id: u64,
    pub sequence: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceLevelInput {
    pub price: Decimal,
    pub quantity: Decimal,
}

impl TryFrom<OrderBookInput> for OrderBook {

    type Error = chrono::ParseError;

    fn try_from(input: OrderBookInput) -> Result<Self, Self::Error> {

        let timestamp =
            DateTime::parse_from_rfc3339(&input.timestamp)?.with_timezone(&Utc);

        let bids = input
            .bids
            .into_iter()
            .map(|level| PriceLevel::new(level.price, level.quantity))
            .collect();

        let asks = input
            .asks
            .into_iter()
            .map(|level| PriceLevel::new(level.price, level.quantity))
            .collect();

        Ok(OrderBook::new(
            input.symbol,
            input.exchange,
            timestamp,
            bids,
            asks,
            None,
            None,
        ))

    }
}
