use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a single price level in the order book
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Complete order book snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: DateTime<Utc>,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    /// Last update ID from the exchange (if available)
    pub last_update_id: Option<u64>,
    /// Sequence number for ordering updates
    pub sequence: Option<u64>,
}

impl OrderBook {
    /// Create a new empty order book
    pub fn new(symbol: String, exchange: String) -> Self {
        Self {
            symbol,
            exchange,
            timestamp: Utc::now(),
            bids: Vec::new(),
            asks: Vec::new(),
            last_update_id: None,
            sequence: None,
        }
    }

    /// Get the best bid (highest buy price)
    pub fn best_bid(&self) -> Option<&PriceLevel> {
        self.bids.first()
    }

    /// Get the best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<&PriceLevel> {
        self.asks.first()
    }

    /// Calculate the bid-ask spread
    pub fn spread(&self) -> Option<Decimal> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// Calculate the mid price
    pub fn mid_price(&self) -> Option<Decimal> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid.price + ask.price) / Decimal::from(2)),
            _ => None,
        }
    }

    /// Get total liquidity within a certain percentage of the mid price
    pub fn liquidity_within_percentage(&self, percentage: Decimal) -> (Decimal, Decimal) {
        let mid = match self.mid_price() {
            Some(mid) => mid,
            None => return (Decimal::ZERO, Decimal::ZERO),
        };

        let threshold = mid * percentage / Decimal::from(100);
        let bid_threshold = mid - threshold;
        let ask_threshold = mid + threshold;

        let bid_liquidity = self
            .bids
            .iter()
            .filter(|level| level.price >= bid_threshold)
            .map(|level| level.quantity)
            .sum();

        let ask_liquidity = self
            .asks
            .iter()
            .filter(|level| level.price <= ask_threshold)
            .map(|level| level.quantity)
            .sum();

        (bid_liquidity, ask_liquidity)
    }

    /// Sort the order book (bids descending, asks ascending)
    pub fn sort(&mut self) {
        self.bids.sort_by(|a, b| b.price.cmp(&a.price)); // Descending
        self.asks.sort_by(|a, b| a.price.cmp(&b.price)); // Ascending
    }

    /// Validate that the order book is properly sorted and has no crossed spread
    pub fn is_valid(&self) -> bool {
        // Check if bids are sorted in descending order
        for i in 1..self.bids.len() {
            if self.bids[i].price > self.bids[i - 1].price {
                return false;
            }
        }

        // Check if asks are sorted in ascending order
        for i in 1..self.asks.len() {
            if self.asks[i].price < self.asks[i - 1].price {
                return false;
            }
        }

        // Check that best bid < best ask (no crossed spread)
        if let (Some(best_bid), Some(best_ask)) = (self.best_bid(), self.best_ask()) {
            if best_bid.price >= best_ask.price {
                return false;
            }
        }

        true
    }
}

/// Supported trading pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingPair {
    BtcUsdc,
    SolUsdc,
}

impl TradingPair {
    /// Convert to exchange-specific symbol format
    pub fn to_exchange_symbol(&self, exchange: &str) -> String {
        match (self, exchange.to_lowercase().as_str()) {
            (TradingPair::BtcUsdc, "binance") => "BTCUSDC".to_string(),
            (TradingPair::SolUsdc, "binance") => "SOLUSDC".to_string(),
            (TradingPair::BtcUsdc, "coinbase") => "BTC-USDC".to_string(),
            (TradingPair::SolUsdc, "coinbase") => "SOL-USDC".to_string(),
            (TradingPair::BtcUsdc, "kraken") => "BTC/USDC".to_string(),
            (TradingPair::SolUsdc, "kraken") => "SOL/USDC".to_string(),
            _ => format!("{:?}", self), // Fallback
        }
    }

    /// Parse from string
    pub fn parse_from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BTCUSDC" | "BTC-USDC" | "BTC/USDC" | "XBTUSDC" => Some(TradingPair::BtcUsdc),
            "SOLUSDC" | "SOL-USDC" | "SOL/USDC" => Some(TradingPair::SolUsdc),
            _ => None,
        }
    }
}

impl std::fmt::Display for TradingPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TradingPair::BtcUsdc => "BTC/USDC",
            TradingPair::SolUsdc => "SOL/USDC",
        };
        write!(f, "{}", s)
    }
}

/// Summary statistics for an order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSummary {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: DateTime<Utc>,
    pub best_bid: Option<Decimal>,
    pub best_ask: Option<Decimal>,
    pub spread: Option<Decimal>,
    pub mid_price: Option<Decimal>,
    pub bid_count: usize,
    pub ask_count: usize,
    pub total_bid_volume: Decimal,
    pub total_ask_volume: Decimal,
}

impl From<&OrderBook> for OrderBookSummary {
    fn from(orderbook: &OrderBook) -> Self {
        Self {
            symbol: orderbook.symbol.clone(),
            exchange: orderbook.exchange.clone(),
            timestamp: orderbook.timestamp,
            best_bid: orderbook.best_bid().map(|b| b.price),
            best_ask: orderbook.best_ask().map(|a| a.price),
            spread: orderbook.spread(),
            mid_price: orderbook.mid_price(),
            bid_count: orderbook.bids.len(),
            ask_count: orderbook.asks.len(),
            total_bid_volume: orderbook.bids.iter().map(|b| b.quantity).sum(),
            total_ask_volume: orderbook.asks.iter().map(|a| a.quantity).sum(),
        }
    }
}
