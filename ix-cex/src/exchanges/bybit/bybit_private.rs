use crate::client::http_client::{HttpClient, RetryConfig, RetryableHttpClient};
use config::{Config, ConfigError};
use hmac::{Hmac, Mac};
use ix_results::errors::{ExchangeError, Result};
use serde::Deserialize;
use sha2::Sha256;
use std::collections::HashMap;
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

/// Bybit configuration loaded from TOML
#[derive(Debug, Deserialize)]
pub struct BybitConfig {
    pub exchange: ExchangeConfig,
    pub api: Option<ApiConfig>,
    pub pairs: PairsConfig,
    pub collection: CollectionConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeConfig {
    pub name: String,
    pub base_url: String,
    pub testnet_url: String,
    pub api_version: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub api_key: String,
    pub api_secret: String,
    pub recv_window: Option<u64>,
    pub testnet: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PairsConfig {
    pub symbols: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CollectionConfig {
    pub interval_seconds: u64,
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub hosts: Vec<String>,
    pub database: String,
    pub table: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
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
        recv_window: u64,
        query_string: &str,
    ) -> Result<String> {
        let api_secret = self.api_secret.as_ref().ok_or_else(|| {
            ExchangeError::Authentication(
                "API secret is required for private endpoints".to_string(),
            )
        })?;

        let api_key = self.api_key.as_ref().ok_or_else(|| {
            ExchangeError::Authentication(
                "API key is required for private endpoints".to_string(),
            )
        })?;

        // For GET requests: timestamp + api_key + recv_window + queryString
        // For POST requests: timestamp + api_key + recv_window + jsonBodyString
        let param_str =
            format!("{}{}{}{}", timestamp, api_key, recv_window, query_string);

        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).map_err(|e| {
            ExchangeError::Authentication(format!("Invalid API secret: {}", e))
        })?;

        mac.update(param_str.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        Ok(hex::encode(code_bytes))
    }

    /// Make authenticated GET request to private endpoint
    async fn get_private<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let timestamp = Self::get_timestamp();
        let query_string = if params.is_empty() {
            String::new()
        } else {
            params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&")
        };

        let signature =
            self.create_signature(timestamp, self.recv_window, &query_string)?;

        let api_key = self.api_key.as_ref().ok_or_else(|| {
            ExchangeError::Authentication(
                "API key is required for private endpoints".to_string(),
            )
        })?;

        let str_timestamp = timestamp.clone().to_string();
        let str_recv = &self.recv_window.clone().to_string();

        // Build headers for authentication
        let mut headers = HashMap::new();
        headers.insert("X-BAPI-API-KEY", api_key.as_str());
        headers.insert("X-BAPI-TIMESTAMP", &str_timestamp);
        headers.insert("X-BAPI-SIGN", &signature);
        headers.insert("X-BAPI-RECV-WINDOW", str_recv);

        info!("Making authenticated GET request to: {}", endpoint);
        debug!("Query string: {}", query_string);

        self.client_with_headers(endpoint, params, headers).await
    }

    /// Make HTTP request with custom headers (for private endpoints)
    async fn client_with_headers<T>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
        headers: HashMap<&str, &str>,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.client
            .get_with_headers_retry(endpoint, params, headers)
            .await
    }

    //  TODO:
    // Get master (all accounts and subaccounts) balance for account
    // pub async fn get_master_balance(&self) {
    //     println!("placeholder");
    // }

    /// Get wallet balance for account
    pub async fn get_wallet_balance(
        &self,
        account_type: &str,
        coin: Option<&str>,
    ) -> Result<WalletBalanceResponse> {
        let mut params = vec![("accountType", account_type)];
        if let Some(c) = coin {
            params.push(("coin", c));
        }

        info!("Fetching wallet balance for account type: {}", account_type);
        self.get_private("/v5/account/wallet-balance", &params)
            .await
    }

    /// Get account information
    pub async fn get_account_info(&self) -> Result<AccountInfoResponse> {
        info!("Fetching account information");
        let acc_priv = self.get_private("/v5/account/info", &[]).await;
        println!("\nasync get_account_info() {:?}\n", acc_priv);
        acc_priv
    }

    /// Get server time (public endpoint)
    pub async fn get_server_time(&self) -> Result<ServerTimeResponse> {
        info!("Fetching Bybit server time");
        self.client.get_with_retry("/v5/market/time").await
    }
}

/// Bybit wallet balance response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: WalletBalanceResult,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    pub list: Vec<WalletBalance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub account_type: String,
    #[serde(default)]
    pub account_ltv: String,
    #[serde(default)]
    pub account_im_rate: String,
    #[serde(default)]
    pub account_im_rate_by_mp: String,
    #[serde(default)]
    pub account_mm_rate: String,
    #[serde(default)]
    pub account_mm_rate_by_mp: String,

    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,

    #[serde(default)]
    pub total_perp_upl: String,
    pub total_initial_margin: String,
    pub total_initial_margin_by_mp: String,
    pub total_maintenance_margin: String,
    pub total_maintenance_margin_by_mp: String,
    pub coin: Vec<CoinBalance>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    pub wallet_balance: String,
    pub free: Option<String>,
    pub locked: Option<String>,
    pub spot_hedging_qty: String,
    pub borrow_amount: String,
    #[serde(default)]
    pub available_to_withdraw: String,
    pub accrued_interest: String,
    #[serde(default)]
    pub total_order_im: String,
    #[serde(default)]
    pub total_position_im: String,
    #[serde(default)]
    pub total_position_mm: String,
    pub unrealised_pnl: String,
    pub cum_realised_pnl: String,
    pub bonus: String,
    pub margin_collateral: bool,
    pub collateral_switch: bool,
    #[serde(default)]
    pub available_to_borrow: String,
}

/// Bybit account info response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: AccountInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub unified_margin_status: i32,
    pub margin_mode: String,
    pub is_master_trader: bool,
    pub spot_hedging_status: String,
    pub updated_time: String,
    pub dcp_status: String,
    pub time_window: u32,
    pub smp_group: u32,
}

/// Bybit server time response structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: ServerTime,
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub time_second: String,
    pub time_nano: String,
}

impl Default for BybitPrivateClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default Bybit client")
    }
}
