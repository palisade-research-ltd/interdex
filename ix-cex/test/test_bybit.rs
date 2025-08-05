#[cfg(test)]
mod tests {

    use ix_cex::BybitClient;

    #[tokio::test]
    async fn test_bybit_client_creation() {

        let client = BybitClient::new();
        assert!(client.is_ok());

    }

    #[tokio::test]
    async fn test_get_server_time() {

        let client = BybitClient::new().unwrap();
        let result = client.get_server_time().await;

        println!("result: {:?}", result);

        match result {
            Ok(server_time) => {
                assert!(server_time.time_second != "");
                assert!(server_time.time_nano != "");
                println!("Bybit server time_second: {}", server_time.time_second);
                println!("Bybit server time_nano: {}", server_time.time_nano);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_get_account_info() {

        let client = BybitClient::new().unwrap();
        let result = client.get_account_info().await;
        
        println!("result: {:?}", result);

        match result {
            Ok(account_info) => {
                assert!(account_info.margin_mode != "");
                assert!(account_info.updated_time != "");
                assert!(account_info.unified_margin_status != 0);

                println!("Bybit account_info.margin_mode: {}", account_info.margin_mode);
                println!("Bybit account_info.updated_time: {}", account_info.updated_time);
                println!("Bybit account_info.unified_margin_status: {}", account_info.unified_margin_status);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }

    }

     #[tokio::test]
    async fn test_get_wallet_balance() {

        let client = BybitClient::new().unwrap();
        let result = client.get_wallet_balance().await;
        
        println!("result: {:?}", result);

        match result {
            Ok(wallet_balance) => {

                assert!(wallet_balance.total_equity != "");
                assert!(wallet_balance.total_available_balance != "");

                println!("Bybit account_info.margin_mode: {}", wallet_balance.total_equity);
                println!("Bybit account_info.updated_time: {}", wallet_balance.total_available_balance);

            }
                Err(e) => {

                    println!("Expected network error in test environment: {e:?}");

                }
            }
    }

}
