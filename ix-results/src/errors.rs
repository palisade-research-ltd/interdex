//! Custom Error Types
//! Provides the definition of error types that are custom made.

use thiserror::Error;

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

