use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub enum Exchange {
    Binance,
    Coinbase,
    Kraken,
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
            _ => format!("{self:?}"), // Fallback
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
        write!(f, "{s}")
    }
}

