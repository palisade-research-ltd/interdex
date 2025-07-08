use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HeliusRpc {
    pub url: String,
    pub tkn: String,
}

pub struct HeliusRpcBuilder {
    url: Option<String>,
    tkn: Option<String>,
}

impl Default for HeliusRpcBuilder {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn builder() -> HeliusRpcBuilder {
        HeliusRpcBuilder::new()
    }

    pub fn get_client(&mut self, rpc_url: &str) -> RpcClient {
        RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed())
    }
}
