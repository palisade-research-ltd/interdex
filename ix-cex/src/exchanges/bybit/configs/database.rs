use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub hosts: Vec<String>,
    pub database: String,
    pub table: String,
}
