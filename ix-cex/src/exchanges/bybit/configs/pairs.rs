use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct PairsConfig {
    pub symbols: Vec<String>,
}
