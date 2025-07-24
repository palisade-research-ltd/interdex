// ix-cex/src/binance_ws/models.rs
use rust_decimal::Decimal;
use serde::Deserialize;

// A simple struct to hold a price level. No serde attributes are needed here.
#[derive(Deserialize, Debug, Clone)]
pub struct Level {
    pub price: Decimal,
    pub qty: Decimal,
}

// The outer wrapper for combined streams
#[derive(Deserialize, Debug)]
pub struct StreamEvent {
    pub stream: String,
    pub data: DepthOrDiff,
}

// An enum to represent either a partial book depth snapshot or a diff update
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DepthOrDiff {
    PartialBook(PartialBookDepth),
    Diff(DiffDepth),
}

// Represents the payload for <symbol>@depth<levels>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartialBookDepth {
    pub last_update_id: u64,
    #[serde(with = "price_level_format")] // Use the custom deserializer here
    pub bids: Vec<Level>,
    #[serde(with = "price_level_format")] // And here
    pub asks: Vec<Level>,
}

// Represents the payload for <symbol>@depth
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiffDepth {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b", with = "price_level_format")] // And here
    pub bids: Vec<Level>,
    #[serde(rename = "a", with = "price_level_format")] // And here
    pub asks: Vec<Level>,
}

// Custom deserializer for Binance's ["price", "qty"] array format
mod price_level_format {
    use super::Level;
    use rust_decimal::Decimal;
    use serde::{self, de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Level>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize into a vector of string pairs first
        let pairs: Vec<(String, String)> = Vec::deserialize(deserializer)?;
        // Manually parse each string pair into a Level struct
        pairs
            .into_iter()
            .map(|(price_str, qty_str)| {
                let price = price_str.parse::<Decimal>().map_err(Error::custom)?;
                let qty = qty_str.parse::<Decimal>().map_err(Error::custom)?;
                Ok(Level { price, qty })
            })
            .collect()
    }
}


