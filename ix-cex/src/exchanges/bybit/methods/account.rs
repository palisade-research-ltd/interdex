// private
use crate::exchanges::bybit::methods::{
    AccountInfoResponse, ServerTimeResponse, WalletBalanceResponse,
};
use crate::{client::http_client::RequestType, BybitPrivateClient};
use ix_results::errors::Result;
use tracing::info;

impl BybitPrivateClient {
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

        let request_type = RequestType::Get;

        info!("Fetching wallet balance for account type: {}", account_type);
        self.request_private("/v5/account/wallet-balance", &params, request_type)
            .await
    }

    /// Get account information
    pub async fn get_account_info(&self) -> Result<AccountInfoResponse> {
        info!("Fetching account information");
        let request_type = RequestType::Get;

        let acc_priv = self
            .request_private("/v5/account/info", &[], request_type)
            .await;
        println!("\nasync get_account_info() {:?}\n", acc_priv);
        acc_priv
    }

    /// Get server time (public endpoint)
    pub async fn get_server_time(&self) -> Result<ServerTimeResponse> {
        info!("Fetching Bybit server time");
        self.client.get_with_retry("/v5/market/time").await
    }
}
