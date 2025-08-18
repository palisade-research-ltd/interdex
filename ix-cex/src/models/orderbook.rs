use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents a single price level in the order book
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceLevel {
    pub price: f64,
    pub quantity: f64,
}

impl PriceLevel {
    pub fn new(price: f64, quantity: f64) -> Self {
        Self { price, quantity }
    }
}

impl Default for PriceLevel {
    fn default() -> Self {
        Self {
            price: 0.0,
            quantity: 0.01,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceLevelInput {
    pub price: String,
    pub quantity: String,
}

/// Input structure for JSON parsing (matches the provided format)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookInput {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: String,
    pub bids: Vec<PriceLevelInput>,
    pub asks: Vec<PriceLevelInput>,
    pub last_update_id: u64,
    pub sequence: Option<u64>,
}

/// Complete order book snapshot
#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct Orderbook {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: DateTime<Utc>,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub last_update_id: Option<u64>,
    pub sequence: Option<u64>,
}

impl TryFrom<OrderbookInput> for Orderbook {
    type Error = chrono::ParseError;

    fn try_from(input: OrderbookInput) -> Result<Self, Self::Error> {
        let timestamp =
            DateTime::parse_from_rfc3339(&input.timestamp)?.with_timezone(&Utc);

        let bids = input
            .bids
            .into_iter()
            .map(|level| {
                PriceLevel::new(
                    f64::from_str(&level.price).unwrap(),
                    f64::from_str(&level.quantity).unwrap(),
                )
            })
            .collect();

        let asks = input
            .asks
            .into_iter()
            .map(|level| {
                PriceLevel::new(
                    f64::from_str(&level.price).unwrap(),
                    f64::from_str(&level.quantity).unwrap(),
                )
            })
            .collect();

        Ok(Orderbook::new(
            input.symbol,
            input.exchange,
            timestamp,
            bids,
            asks,
            Some(input.last_update_id),
            input.sequence,
        ))
    }
}

impl Default for Orderbook {
    fn default() -> Self {
        Self {
            symbol: "default_symbol".to_string(),
            exchange: "default_exchange".to_string(),
            timestamp: DateTime::default(),
            bids: vec![PriceLevel::default()],
            asks: vec![PriceLevel::default()],
            last_update_id: Some(1234),
            sequence: Some(4321),
        }
    }
}

impl Orderbook {
    /// Create a new empty order book
    pub fn new(
        symbol: String,
        exchange: String,
        timestamp: DateTime<Utc>,
        bids: Vec<PriceLevel>,
        asks: Vec<PriceLevel>,
        last_update_id: Option<u64>,
        sequence: Option<u64>,
    ) -> Self {
        Self {
            symbol,
            exchange,
            timestamp,
            bids,
            asks,
            last_update_id,
            sequence,
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
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask.price - bid.price),
            _ => None,
        }
    }

    /// Calculate the mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid.price + ask.price) / 2.0),
            _ => None,
        }
    }

    /// Get total liquidity within a certain percentage of the mid price
    pub fn liquidity_within_percentage(&self, percentage: f64) -> (f64, f64) {
        let mid = match self.mid_price() {
            Some(mid) => mid,
            None => return (0.0, 0.0),
        };

        let threshold = mid * percentage / 100.0;
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

    /// Get total volume on bid side
    pub fn bid_volume(&self) -> f64 {
        self.bids.iter().map(|level| level.quantity).sum()
    }

    /// Get total volume on ask side
    pub fn ask_volume(&self) -> f64 {
        self.asks.iter().map(|level| level.quantity).sum()
    }

    /// Create partitioned file path for parquet storage
    pub fn parquet_path(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}.parquet",
            self.exchange,
            self.timestamp.format("%Y%m%d"),
            self.timestamp.format("%H"),
            self.timestamp.format("%M"),
            self.symbol
        )
    }

    /// Validate Orderbook data
    pub fn validate(&self) -> Result<(), String> {
        if self.symbol.is_empty() {
            return Err("Symbol cannot be empty".to_string());
        }

        if self.exchange.is_empty() {
            return Err("Exchange cannot be empty".to_string());
        }

        if self.bids.is_empty() && self.asks.is_empty() {
            return Err("Orderbook must have at least one bid or ask".to_string());
        }

        Ok(())
    }
}

/// Supported trading pairs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TradingPair {
    BtcUsdt,
    BtcUsdc,
    EthUsdt,
    EthUsdc,
    SolUsdt,
    SolUsdc,
    LinkUsdc,
    LinkUsdt,
    UniUsdt,
    UniUsdc,
}

impl TradingPair {
    /// Convert to exchange-specific symbol format
    pub fn to_exchange_symbol(&self, exchange: &str) -> String {
        match (self, exchange.to_lowercase().as_str()) {
            (TradingPair::BtcUsdt, "binance") => "BTCUSDT".to_string(),
            (TradingPair::SolUsdt, "binance") => "SOLUSDT".to_string(),
            (TradingPair::EthUsdt, "binance") => "ETHUSDT".to_string(),
            (TradingPair::UniUsdt, "binance") => "UNIUSDT".to_string(),
            (TradingPair::LinkUsdt, "binance") => "LINKUSDT".to_string(),
            (TradingPair::BtcUsdc, "binance") => "BTCUSDC".to_string(),
            (TradingPair::SolUsdc, "binance") => "SOLUSDC".to_string(),
            (TradingPair::EthUsdc, "binance") => "ETHUSDC".to_string(),
            (TradingPair::UniUsdc, "binance") => "UNIUSDC".to_string(),
            (TradingPair::LinkUsdc, "binance") => "LINKUSDC".to_string(),

            (TradingPair::BtcUsdt, "bybit") => "BTCUSDT".to_string(),
            (TradingPair::SolUsdt, "bybit") => "SOLUSDT".to_string(),
            (TradingPair::EthUsdt, "bybit") => "ETHUSDT".to_string(),
            (TradingPair::UniUsdt, "bybit") => "UNIUSDT".to_string(),
            (TradingPair::LinkUsdt, "bybit") => "LINKUSDT".to_string(),
            (TradingPair::BtcUsdc, "bybit") => "BTCUSDC".to_string(),
            (TradingPair::SolUsdc, "bybit") => "SOLUSDC".to_string(),
            (TradingPair::EthUsdc, "bybit") => "ETHUSDC".to_string(),
            (TradingPair::UniUsdc, "bybit") => "UNIUSDC".to_string(),
            (TradingPair::LinkUsdc, "bybit") => "LINKUSDC".to_string(),

            (TradingPair::BtcUsdt, "coinbase") => "BTC-USDT".to_string(),
            (TradingPair::SolUsdt, "coinbase") => "SOL-USDT".to_string(),
            (TradingPair::EthUsdt, "coinbase") => "ETH-USDT".to_string(),
            (TradingPair::UniUsdt, "coinbase") => "UNI-USDT".to_string(),
            (TradingPair::LinkUsdt, "coinbase") => "LINK-USDT".to_string(),
            (TradingPair::BtcUsdc, "coinbase") => "BTC-USDC".to_string(),
            (TradingPair::SolUsdc, "coinbase") => "SOL-USDC".to_string(),
            (TradingPair::EthUsdc, "coinbase") => "ETH-USDC".to_string(),
            (TradingPair::UniUsdc, "coinbase") => "UNI-USDC".to_string(),
            (TradingPair::LinkUsdc, "coinbase") => "LINK-USDC".to_string(),

            (TradingPair::BtcUsdt, "kraken") => "BTCUSDT".to_string(),
            (TradingPair::SolUsdt, "kraken") => "SOLUSDT".to_string(),
            (TradingPair::EthUsdt, "kraken") => "ETHUSDT".to_string(),
            (TradingPair::UniUsdt, "kraken") => "UNIUSDT".to_string(),
            (TradingPair::LinkUsdt, "kraken") => "LINKUSDT".to_string(),
            (TradingPair::BtcUsdc, "kraken") => "BTCUSDC".to_string(),
            (TradingPair::SolUsdc, "kraken") => "SOLUSDC".to_string(),
            (TradingPair::EthUsdc, "kraken") => "ETHUSDC".to_string(),
            (TradingPair::UniUsdc, "kraken") => "UNIUSDC".to_string(),
            (TradingPair::LinkUsdc, "kraken") => "LINKUSDC".to_string(),

            _ => format!("{self:?}"), // Fallback
        }
    }

    /// Parse from string
    pub fn parse_from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "BTCUSDT" | "BTC-USDT" | "BTC/USDT" => Some(TradingPair::BtcUsdt),
            "SOLUSDT" | "SOL-USDT" | "SOL/USDT" => Some(TradingPair::SolUsdt),
            "ETHUSDT" | "ETH-USDT" | "ETH/USDT" => Some(TradingPair::EthUsdt),
            "UNIUSDT" | "UNI-USDT" | "UNI/USDT" => Some(TradingPair::UniUsdt),
            "LINKUSDT" | "LINK-USDT" | "LINK/USDT" => Some(TradingPair::LinkUsdt),

            "BTCUSDC" | "BTC-USDC" | "BTC/USDC" => Some(TradingPair::BtcUsdc),
            "SOLUSDC" | "SOL-USDC" | "SOL/USDC" => Some(TradingPair::SolUsdc),
            "ETHUSDC" | "ETH-USDC" | "ETH/USDC" => Some(TradingPair::EthUsdc),
            "UNIUSDC" | "UNI-USDC" | "UNI/USDC" => Some(TradingPair::UniUsdc),
            "LINKUSDC" | "LINK-USDC" | "LINK/USDC" => Some(TradingPair::LinkUsdc),

            _ => None,
        }
    }
}

impl std::fmt::Display for TradingPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TradingPair::BtcUsdt => "BTC/USDT",
            TradingPair::SolUsdt => "SOL/USDT",
            TradingPair::EthUsdt => "ETH/USDT",
            TradingPair::UniUsdt => "UNI/USDT",
            TradingPair::LinkUsdt => "LINK/USDT",

            TradingPair::BtcUsdc => "BTC/USDC",
            TradingPair::SolUsdc => "SOL/USDC",
            TradingPair::EthUsdc => "ETH/USDC",
            TradingPair::UniUsdc => "UNI/USDC",
            TradingPair::LinkUsdc => "LINK/USDC",
        };
        write!(f, "{s}")
    }
}

/// Summary statistics for an order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookSummary {
    pub symbol: String,
    pub exchange: String,
    pub timestamp: DateTime<Utc>,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub spread: Option<f64>,
    pub mid_price: Option<f64>,
    pub bid_count: usize,
    pub ask_count: usize,
    pub total_bid_volume: f64,
    pub total_ask_volume: f64,
}

impl From<&Orderbook> for OrderbookSummary {
    fn from(orderbook: &Orderbook) -> Self {
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
