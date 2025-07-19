use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("WebSocket connection error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("URL parsing error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("JSON deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] clickhouse::error::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Channel send error")]
    ChannelSendError,

    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),
}
