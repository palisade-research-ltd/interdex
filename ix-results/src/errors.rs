//! Custom Error Types
//! Provides the definition of error types that are custom made.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("HTTP request error: {0}")]
    HttpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Exchange API error: {0}")]
    ExchangeError(String),
}

impl From<reqwest::Error> for DatabaseError {
    fn from(err: reqwest::Error) -> Self {
        DatabaseError::HttpError(err.to_string())
    }
}

impl From<scylla::errors::BadQuery> for DatabaseError {
    fn from(err: scylla::errors::BadQuery) -> Self {
        DatabaseError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::SerializationError(err.to_string())
    }
}

#[derive(Error, Debug, Clone)]
pub enum DataError {
    #[error("Data not found")]
    DataNotFound,
    #[error("Data is incomplete")]
    DataIncomplete,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to open file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to open file: {0}")]
    Json(serde_json::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonError(String),

    #[error("Failed to parse JSON: {0}")]
    TypeMismatch(String),

    #[error("Failed to parse JSON: {0}")]
    InvalidInput(String),

    #[error("Failed to parse JSON: {0}")]
    MissingKey(String),
}

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
