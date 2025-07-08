use async_rate_limiter::RateLimiter;
use ix_results::errors::{ExchangeError, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use url::Url;

/// HTTP client wrapper with rate limiting and error handling
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    rate_limiter: RateLimiter,
    exchange_name: String,
    base_url: String,
    timeout: Duration,
}

impl HttpClient {
    /// Create a new HTTP client for an exchange
    pub fn new(
        exchange_name: String,
        base_url: String,
        requests_per_second: u32,
        timeout_seconds: u64,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .user_agent("ix_cex/0.0.1")
            .build()
            .map_err(ExchangeError::Network)?;

        let rate_limiter = RateLimiter::new(requests_per_second as usize);

        Ok(Self {
            client,
            rate_limiter,
            exchange_name,
            base_url,
            timeout: Duration::from_secs(timeout_seconds),
        })
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    /// Make a GET request with rate limiting
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_params(endpoint, &[]).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params<T>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Wait for rate limiter
        self.rate_limiter.acquire().await;

        let url = self.build_url(endpoint, params)?;

        debug!("Making GET request to: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(ExchangeError::Network)?;

        self.handle_response(response).await
    }

    /// Build full URL with query parameters
    fn build_url(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<String> {
        let base = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!(
                "{}/{}",
                self.base_url.trim_end_matches('/'),
                endpoint.trim_start_matches('/')
            )
        };

        if params.is_empty() {
            Ok(base)
        } else {
            let mut url =
                Url::parse(&base).map_err(|e| ExchangeError::Configuration {
                    message: format!("Invalid URL: {e}"),
                })?;

            for (key, value) in params {
                url.query_pairs_mut().append_pair(key, value);
            }

            Ok(url.to_string())
        }
    }

    /// Handle HTTP response and deserialize JSON
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let url = response.url().to_string();

        debug!("Response status: {} for URL: {}", status, url);

        if status.is_success() {
            let text = response.text().await.map_err(ExchangeError::Network)?;

            debug!("Response body length: {} bytes", text.len());
            debug!("\nResponse content: {:?}\n", text);

            serde_json::from_str(&text).map_err(|e| {
                error!("JSON parsing error for {}: {}", url, e);
                error!("Response body: {}", text);
                ExchangeError::JsonParsing(e)
            })
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            let error = match status.as_u16() {
                429 => {
                    warn!("Rate limit exceeded for {}", self.exchange_name);
                    ExchangeError::RateLimit {
                        exchange: self.exchange_name.clone(),
                    }
                }
                400..=499 => ExchangeError::ApiError {
                    exchange: self.exchange_name.clone(),
                    message: format!("Client error ({status}): {error_text}"),
                },
                500..=599 => ExchangeError::ApiError {
                    exchange: self.exchange_name.clone(),
                    message: format!("Server error ({status}): {error_text}"),
                },
                _ => ExchangeError::ApiError {
                    exchange: self.exchange_name.clone(),
                    message: format!("HTTP error ({status}): {error_text}"),
                },
            };

            error!("HTTP error for {}: {:?}", url, error);
            Err(error)
        }
    }

    /// Get the exchange name
    pub fn exchange_name(&self) -> &str {
        &self.exchange_name
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Manually trigger rate limiting (useful for retries)
    pub async fn wait_for_rate_limit(&self) {
        self.rate_limiter.acquire().await;
    }

    /// Check if rate limiter allows immediate request
    pub fn can_make_request(&self) -> bool {
        self.rate_limiter.try_acquire().is_ok()
    }
}

/// Retry configuration for HTTP requests
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
        }
    }
}

/// HTTP client with retry capabilities
#[derive(Clone)]
pub struct RetryableHttpClient {
    client: HttpClient,
    retry_config: RetryConfig,
}

impl RetryableHttpClient {
    pub fn new(client: HttpClient, retry_config: RetryConfig) -> Self {
        Self {
            client,
            retry_config,
        }
    }

    /// Make a GET request with automatic retries
    pub async fn get_with_retry<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_params_retry(endpoint, &[]).await
    }

    /// Make a GET request with parameters and automatic retries
    pub async fn get_with_params_retry<T>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut last_error = None;
        let mut delay = self.retry_config.initial_delay;

        for attempt in 0..=self.retry_config.max_retries {
            match self.client.get_with_params(endpoint, params).await {
                Ok(result) => return Ok(result),

                Err(error) => {
                    last_error = Some(error);

                    if attempt < self.retry_config.max_retries {
                        info!(
                            "Request failed (attempt {}/{}), retrying in {:?}",
                            attempt + 1,
                            self.retry_config.max_retries + 1,
                            delay,
                        );

                        tokio::time::sleep(delay).await;

                        // Exponential backoff
                        delay = std::cmp::min(
                            Duration::from_millis(
                                (delay.as_millis() as f64
                                    * self.retry_config.backoff_factor)
                                    as u64,
                            ),
                            self.retry_config.max_delay,
                        );
                    } else {
                        break;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| ExchangeError::Unknown("No attempts made".to_string())))
    }

    /// Access the underlying client
    pub fn client(&self) -> &HttpClient {
        &self.client
    }
}
