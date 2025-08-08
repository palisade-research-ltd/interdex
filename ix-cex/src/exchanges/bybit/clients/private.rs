// private
use crate::{
    client::http_client::{HttpClient, RequestType, RetryConfig, RetryableHttpClient},
    exchanges::bybit::configs::BybitConfig,
};

use config::{Config, ConfigError};
use hmac::{Hmac, Mac};
use ix_results::errors::{ExchangeError, Result};
use serde::Deserialize;
use sha2::Sha256;
use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info};
type HmacSha256 = Hmac<Sha256>;

/// Bybit REST API client with private endpoint support
#[derive(Clone)]
pub struct BybitPrivateClient {
    pub client: RetryableHttpClient,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub recv_window: u64,
    pub testnet: bool,
}

impl BybitPrivateClient {
    /// Create a new Bybit client with optional private credentials
    pub fn new() -> Result<Self> {
        Self::from_config(
            "/Users/franciscome/git/palisade/interdex/ix-cex/config/bybit.toml",
        )
    }

    /// Create Bybit client from configuration file
    pub fn from_config(config_path: &str) -> Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .map_err(|e: ConfigError| ExchangeError::Configuration {
                message: format!("Failed to load Bybit config: {}", e),
            })?;

        let config: BybitConfig =
            settings.try_deserialize().map_err(|e: ConfigError| {
                ExchangeError::Configuration {
                    message: format!("Failed to parse Bybit config: {}", e),
                }
            })?;

        let base_url = if config.api.as_ref().and_then(|a| a.testnet).unwrap_or(false) {
            config.exchange.testnet_url
        } else {
            config.exchange.base_url
        };
        let http_client = HttpClient::new(
            "Bybit".to_string(),
            base_url,
            10, // 10 requests per second to stay under limits
            config.collection.timeout_seconds,
        )?;

        let retry_client = RetryableHttpClient::new(http_client, RetryConfig::default());

        Ok(Self {
            client: retry_client,
            api_key: config.api.as_ref().map(|a| a.api_key.clone()),
            api_secret: config.api.as_ref().map(|a| a.api_secret.clone()),
            recv_window: config
                .api
                .as_ref()
                .and_then(|a| a.recv_window)
                .unwrap_or(5000),
            testnet: config.api.as_ref().and_then(|a| a.testnet).unwrap_or(false),
        })
    }

    /// Create Bybit client with explicit credentials
    pub fn with_credentials(
        api_key: String,
        api_secret: String,
        testnet: bool,
    ) -> Result<Self> {
        let base_url = if testnet {
            "https://api-testnet.bybit.com"
        } else {
            "https://api.bybit.com"
        };

        let http_client =
            HttpClient::new("Bybit".to_string(), base_url.to_string(), 10, 30)?;

        let retry_client = RetryableHttpClient::new(http_client, RetryConfig::default());

        Ok(Self {
            client: retry_client,
            api_key: Some(api_key),
            api_secret: Some(api_secret),
            recv_window: 5000,
            testnet,
        })
    }

    /// Generate timestamp for API requests
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    /// Generate HMAC-SHA256 signature for authenticated requests
    fn create_signature(
        &self,
        timestamp: u64,
        api_key: &str,
        recv_window: u64,
        param_str: &str, // This is the string to be signed
    ) -> Result<String> {
        // Get API_SECRET
        let api_secret = self.api_secret.as_ref().ok_or_else(|| {
            ExchangeError::Authentication(
                "API secret is required for private endpoints".to_string(),
            )
        })?;

        // The string to sign is: timestamp + api_key + recv_window + param_str
        let full_param_str =
            format!("{}{}{}{}", timestamp, api_key, recv_window, param_str);

        debug!("String to sign: {}", full_param_str); // Essential for debugging

        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).map_err(|e| {
            ExchangeError::Authentication(format!("Invalid API secret: {}", e))
        })?;

        mac.update(full_param_str.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        Ok(hex::encode(code_bytes))
    }

    /// Make authenticated GET request to private endpoint
    pub async fn request_private<T>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        request_type: RequestType,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let timestamp = Self::get_timestamp();
        let api_key = self.api_key.as_ref().ok_or_else(|| {
            ExchangeError::Authentication(
                "API key is required for private endpoints".to_string(),
            )
        })?;

        let param_str_for_signing = match request_type {
            RequestType::Get => {
                if params.is_empty() {
                    String::new()
                } else {
                    let mut sorted_params: Vec<_> = params.to_vec();
                    sorted_params.sort_by_key(|&(k, _)| k);
                    sorted_params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join("&")
                }
            }
            RequestType::Post => {
                if params.is_empty() {
                    "{}".to_string()
                } else {
                    let body_map: BTreeMap<_, _> = params.iter().cloned().collect();
                    serde_json::to_string(&body_map).unwrap_or_default()
                }
            }
        };

        println!("param_str_for_signing: {}", param_str_for_signing);

        let signature = self.create_signature(
            timestamp,
            api_key,
            self.recv_window,
            &param_str_for_signing,
        )?;

        println!("signature: {:?}", signature);

        // ... rest of the function remains the same ...
        let str_timestamp = timestamp.to_string();
        let str_recv = &self.recv_window.to_string();

        let mut headers = HashMap::new();
        headers.insert("X-BAPI-API-KEY", api_key.as_str());
        headers.insert("X-BAPI-TIMESTAMP", &str_timestamp);
        headers.insert("X-BAPI-SIGN", &signature);
        headers.insert("X-BAPI-RECV-WINDOW", str_recv);

        // CRITICAL: Always include Content-Type for POST requests
        if let RequestType::Post = request_type {
            headers.insert("Content-Type", "application/json");
        }

        info!("Making authenticated POST request to: {}", endpoint);
        debug!("Query string for signing: {}", param_str_for_signing);

        self.client_with_headers(endpoint, params, headers, request_type)
            .await
    }

    /// Make HTTP request with custom headers (for private endpoints)
    async fn client_with_headers<T>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        headers: HashMap<&str, &str>,
        request: RequestType,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        match request {
            RequestType::Get => {
                // GET
                self.client
                    .get_with_headers_retry(endpoint, params, headers)
                    .await
            }
            // POST
            RequestType::Post => {
                self.client
                    .post_with_headers_retry(endpoint, params, headers)
                    .await
            }
        }
    }
}

impl Default for BybitPrivateClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Bybit client")
    }
}
