use thiserror::Error;

/// Main error type for the crypto exchange client
#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),

    #[error("Rate limit exceeded for exchange: {exchange}")]
    RateLimit { exchange: String },

    #[error("API error from {exchange}: {message}")]
    ApiError { exchange: String, message: String },

    #[error("Invalid trading pair: {pair}")]
    InvalidTradingPair { pair: String },

    #[error("Exchange not supported: {exchange}")]
    UnsupportedExchange { exchange: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias using our custom error
pub type Result<T> = std::result::Result<T, ExchangeError>;

impl ExchangeError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ExchangeError::Network(_)
                | ExchangeError::RateLimit { .. }
                | ExchangeError::Timeout(_)
        )
    }

    /// Get the exchange name if applicable
    pub fn exchange(&self) -> Option<&str> {
        match self {
            ExchangeError::RateLimit { exchange, .. }
            | ExchangeError::ApiError { exchange, .. } => Some(exchange),
            _ => None,
        }
    }
}
