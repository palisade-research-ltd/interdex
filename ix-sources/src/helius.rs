use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeliusRpc {
    pub url: String,
    pub tkn: String,
    pub client: Option<RpcClient>,
}

pub struct HeliusRpcBuilder {
    url: Option<String>,
    tkn: Option<String>,
    client: Option<RpcClient>,
}

impl HeliusRpcBuilder {
    pub fn new() -> Self {
        HeliusRpcBuilder {
            url: None,
            tkn: None,
            client: None,
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

    pub fn client(mut self, client: RpcClient) -> Self {
        self.client = Some(client);
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

    pub fn get_client(&mut self) -> Self {
        
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );

        self.client = rpc_client;
        self

    }

}

