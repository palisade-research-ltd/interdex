// Configs

use serde::Deserialize;

pub mod api;
pub mod collection;
pub mod database;
pub mod exchange;
pub mod logging;
pub mod pairs;

/// Bybit configuration loaded from TOML
#[derive(Debug, Deserialize)]
pub struct BybitConfig {
    pub exchange: exchange::ExchangeConfig,
    pub api: Option<api::ApiConfig>,
    pub pairs: pairs::PairsConfig,
    pub collection: collection::CollectionConfig,
    pub database: database::DatabaseConfig,
    pub logging: logging::LoggingConfig,
}

