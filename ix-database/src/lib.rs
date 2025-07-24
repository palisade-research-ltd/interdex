//! # ix-database
//!
//! A ClickHouse database client library for the interdex workspace with orderbook data handling.
//!
//! This library provides:
//! - Async ClickHouse client with connection pooling
//! - Orderbook data structures and operations
//! - Parquet file storage and partitioning
//! - System table monitoring and reporting
//!

use std::collections::HashMap;
use std::sync::Arc;

use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod client;
pub mod queries;
pub use client::*;
pub use queries::*;

/// Main errors for the ix-database library
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("ClickHouse client error: {0}")]
    ClickHouseError(#[from] clickhouse::error::Error),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parquet error: {0}")]
    ParquetError(String),

    #[error("Database operation failed: {0}")]
    OperationFailed(String),
}

/// Result type for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: Uuid,
    pub url: String,
    pub database: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_ping: Option<chrono::DateTime<chrono::Utc>>,
}

/// ClickHouse client with connection management
pub struct ClickHouseClient {
    client: Client,
    url: String,
    database: String,
    connections: Arc<RwLock<HashMap<Uuid, ConnectionInfo>>>,
}

impl ClickHouseClient {
    /// Create a new client builder
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Create a new connection to ClickHouse
    pub async fn create_connection(&self) -> DatabaseResult<Uuid> {
        let connection_id = Uuid::new_v4();

        // Test the connection by running a simple query
        let _: Vec<SystemTable> = self
            .client
            .query("SELECT name FROM system.tables LIMIT 1")
            .fetch_all()
            .await?;

        let connection_info = ConnectionInfo {
            id: connection_id,
            url: self.url.clone(),
            database: self.database.clone(),
            connected_at: chrono::Utc::now(),
            last_ping: None,
        };

        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection_info);

        Ok(connection_id)
    }

    /// Destroy a connection
    pub async fn destroy_connection(&self, connection_id: Uuid) -> DatabaseResult<()> {
        let mut connections = self.connections.write().await;
        if connections.remove(&connection_id).is_some() {
            Ok(())
        } else {
            Err(DatabaseError::ConnectionError(format!(
                "Connection {connection_id} not found"
            )))
        }
    }

    /// Get information about active connections
    pub async fn get_connections(&self) -> DatabaseResult<Vec<ConnectionInfo>> {
        let connections = self.connections.read().await;
        Ok(connections.values().cloned().collect())
    }

    /// Get all system table names
    pub async fn get_system_tables(&self) -> DatabaseResult<Vec<String>> {
        let tables: Vec<SystemTable> = self
            .client
            .query(
                "SELECT name FROM system.tables WHERE database = 'system' ORDER BY name",
            )
            .fetch_all()
            .await?;

        Ok(tables.into_iter().map(|t| t.name).collect())
    }

    /// Create a table from a SQL query string
    pub async fn create_table(&self, query: &str) -> DatabaseResult<()> {
        self.client.query(query).execute().await?;
        Ok(())
    }

    pub async fn write_table(&self, query: &str) -> DatabaseResult<()> {
        self.client.query(query).execute().await?;
        Ok(())
    }

    /// Get raw client for custom queries
    pub fn client(&self) -> &Client {
        &self.client
    }
}

/// System table structure for queries
#[derive(Debug, Row, Deserialize, Serialize)]
pub struct SystemTable {
    pub name: String,
}

/// Builder for ClickHouse client
pub struct ClientBuilder {
    url: Option<String>,
    database: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            url: None,
            database: None,
            username: None,
            password: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn database(mut self, database: impl Into<String>) -> Self {
        self.database = Some(database.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub async fn build(self) -> DatabaseResult<ClickHouseClient> {
        let url = self
            .url
            .unwrap_or_else(|| "http://localhost:8123".to_string());
        let database = self.database.unwrap_or_else(|| "default".to_string());

        let mut client = Client::default().with_url(&url).with_database(&database);

        if let Some(username) = self.username {
            client = client.with_user(username);
        }

        if let Some(password) = self.password {
            client = client.with_password(password);
        }

        Ok(ClickHouseClient {
            client,
            url,
            database,
            connections: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}
