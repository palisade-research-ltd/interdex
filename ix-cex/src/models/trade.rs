
use serde::Deserialize;

// An enum to represent either a partial book depth snapshot or a diff update
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AggOrTrade {
    Trade(Trade),
}


// The outer wrapper for combined streams
#[derive(Deserialize, Debug)]
pub struct TradeEvent {
    pub stream: String,
    pub data: AggOrTrade,
}


