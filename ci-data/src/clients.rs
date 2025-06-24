use crate::data::{
    priorityFeeEstimateResponse,
    priorityFeeRecentResponse,
    SolanaResponse,
    SolanaResponse2,
    TransactionResponse,
};

use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

// https://api.devnet.solana.com

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SolanaRpc {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SolanaRpcBuilder {
    url: Option<String>,
}

impl SolanaRpcBuilder {
    pub fn new() -> Self {
        SolanaRpcBuilder { url: None }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn build(self) -> Result<SolanaRpc, String> {
        match self.url {
            Some(url) => Ok(SolanaRpc { url }),
            _ => Err("Both URL and token must be provided".to_string()),
        }
    }
}

impl SolanaRpc {

    pub async fn get_block(&self, slot: u64) -> Result<SolanaResponse2> {
        
        let solana_client = Client::new();
        let url = format!("{}", self.url);

        let solana_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [
                slot,
                { 
                    "encoding": "json",
                    "maxSupportedTransactionVersion": 0,
                    "transactionDetails": "full",
                    "rewards": false
                }
            ]
        });

        let solana_post = solana_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&solana_request)
            .send()
            .await
            .context("Failed to send getBlock RPC request")?;
        

        let solana_response: SolanaResponse2 = solana_post
            .json()
            .await
            .context("Failed to parse getBlock response data")?;

        // println!("solana_response: {:?}", &solana_transactions);
        
        Ok(solana_response)
    }

    pub async fn get_priority_fee_recent(
        &self,
        v_accounts: Vec<String>,
    ) -> Result<priorityFeeRecentResponse> {
        let solana_client = Client::new();
        let url = format!("{}", self.url);

        let solana_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getRecentPrioritizationFees",
            "params": [v_accounts],
        });

        let solana_response = solana_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&solana_request)
            .send()
            .await
            .context("Failed to send RPC request to Solana")?;

        // When solana_response fails, parse this as String with all the contents

        let solana_rpf_response: SolanaResponse = solana_response
            .json()
            .await
            .context("Failed to parse Solana response JSON data")?;

        let fees = solana_rpf_response.result.as_ref().map(|results| {
            results
                .iter()
                .filter_map(|r| r.prioritization_fee)
                .collect()
        });

        let slots = solana_rpf_response
            .result
            .as_ref()
            .map(|results| results.iter().filter_map(|r| r.slot).collect());

        Ok(priorityFeeRecentResponse { slots, fees })
    }
}

// -------------------------------------------------------------------------------------------- //
// -------------------------------------------------------------------------------------------- //

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeliusRpc {
    pub url: String,
    pub tkn: String,
}

pub struct HeliusRpcBuilder {
    url: Option<String>,
    tkn: Option<String>,
}

impl HeliusRpcBuilder {
    pub fn new() -> Self {
        HeliusRpcBuilder {
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

    pub fn build(self) -> Result<HeliusRpc, String> {
        match (self.url, self.tkn) {
            (Some(url), Some(tkn)) => Ok(HeliusRpc { url, tkn }),
            _ => Err("Both URL and token must be provided".to_string()),
        }
    }
}

impl HeliusRpc {
    pub async fn get_tx(&self, tx_signature: &str) -> Result<TransactionResponse> {
        let helius_client = Client::new();
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

        let helius_response = helius_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&tx_request)
            .send()
            .await
            .context("Failed to send RPC request to Helius")?;

        let tx_response: TransactionResponse = helius_response
            .json()
            .await
            .context("Failed to parse Helius response JSON data")?;

        Ok(tx_response)
    }

    pub async fn get_priority_fee_estimate(
        &self,
        account_keys: Vec<String>,
    ) -> Result<priorityFeeEstimateResponse> {
        let helius_endpoint = "getPriorityFeeEstimate".to_string();
        let helius_client = Client::new();
        let url = format!("{}{}", self.url, self.tkn);

        let pfe_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": &helius_endpoint,
            "params": [
                {
                    "accountKeys": account_keys,
                    "options": { "includeAllPriorityFeeLevels": true },
                },
            ],
        });

        let context_msg = "Failed request to: ".to_string() + &helius_endpoint.to_string();

        let response = helius_client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&pfe_request)
            .send()
            .await
            .context(context_msg)?;

        let priority_fee_response: priorityFeeEstimateResponse = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        Ok(priority_fee_response)
    }
}
