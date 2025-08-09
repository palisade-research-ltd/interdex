use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}
