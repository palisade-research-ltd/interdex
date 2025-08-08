use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CollectionConfig {
    pub interval_seconds: u64,
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
}

