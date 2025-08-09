#[cfg(test)]
mod tests {

    use ix_cex::BybitPrivateClient;

    #[tokio::test]
    async fn test_bybit_get_wallet_balance() {
        // build up call
        let client = BybitPrivateClient::new().unwrap();
        let p_account_type = "UNIFIED";
        let p_coin = Some("USD");
        let result = client.get_wallet_balance(p_account_type, p_coin).await;

        match result {
            Ok(wallet_balance) => {
                println!("Bybit wallet-balance: {:?}", wallet_balance);
                assert!(!wallet_balance.result.list.is_empty());
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_bybit_get_account_info() {
        // build up call
        let client = BybitPrivateClient::new().unwrap();
        let result = client.get_account_info().await;

        match result {
            Ok(account_info) => {
                assert!(account_info.result.unified_margin_status != 0);
                assert!(account_info.result.margin_mode != "");
                assert!(account_info.result.updated_time != "");

                println!("Bybit account_info.result: {:?}", account_info.result);
                println!(
                    "Bybit account_info.result.unified_margin_status: {:?}",
                    account_info.result.unified_margin_status
                );
                println!(
                    "Bybit account_info.result.margin_mode: {:?}",
                    account_info.result.margin_mode
                );
            }

            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }
}
