//! Storage module for data persistence and partitioning
//!
//! This module provides functionality for storing orderbook data in various formats,
//! including Parquet files with efficient partitioning strategies.

pub mod parquet;
pub use parquet::*;
