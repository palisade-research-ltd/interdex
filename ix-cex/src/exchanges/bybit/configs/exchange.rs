use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExchangeConfig {
    pub name: String,
    pub base_url: String,
    pub testnet_url: String,
    pub api_version: String,
}

