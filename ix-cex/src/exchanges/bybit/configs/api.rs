use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub api_key: String,
    pub api_secret: String,
    pub recv_window: Option<u64>,
    pub testnet: Option<bool>,
}
