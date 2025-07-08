use anyhow::{Context, Result};
use ix_core::data::TransactionResponse;
use reqwest::Client;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GenericRpc {
    pub url: String,
    pub tkn: String,
}

pub struct GenericRpcBuilder {
    url: Option<String>,
    tkn: Option<String>,
}

impl Default for GenericRpcBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GenericRpcBuilder {
    pub fn new() -> Self {
        GenericRpcBuilder {
            url: None,
            tkn: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn tkn(mut self, tkn: String) -> Self {
        self.tkn = Some(tkn);
        self
    }

    pub fn build(self) -> Result<GenericRpc, String> {
        match (self.url, self.tkn) {
            (Some(url), Some(tkn)) => Ok(GenericRpc { url, tkn }),
            _ => Err("Both URL and token must be provided".to_string()),
        }
    }
}

impl GenericRpc {
    pub async fn get_tx(&self, tx_signature: &str) -> Result<TransactionResponse> {
        let generic_client = Client::new();
        let url = format!("{}{}", self.url, self.tkn);

        let tx_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransaction",
            "params": [
                tx_signature,
                {
                    "maxSupportedTransactionVersion": 0,
                }
            ]
        });

        let generic_response = generic_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&tx_request)
            .send()
            .await
            .context("Failed to send RPC request to Generic")?;

        let tx_response: TransactionResponse = generic_response
            .json()
            .await
            .context("Failed to parse Generic response JSON data")?;

        Ok(tx_response)
    }
}
