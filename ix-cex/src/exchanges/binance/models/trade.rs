use serde::Deserialize;

// An enum to represent either a partial book depth snapshot or a diff update
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AggOrTrade {
    Trade(Trade),
}

// Represents the payload for <symbol>@depth<levels>
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_ts: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "q")]
    pub quantity: f64,
    #[serde(rename = "T")]
    pub trade_ts: u64,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "M")]
    pub is_ignore: bool,
}

