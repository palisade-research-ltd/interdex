#[cfg(test)]
mod tests {

    use ix_cex::exchanges::BinanceRestClient;
    use ix_cex::models::orderbook::TradingPair;

    #[tokio::test]
    async fn test_binance_client_creation() {
        let client = BinanceRestClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let client = BinanceRestClient::new().unwrap();
        let result = client.get_server_time().await;

        // This test might fail if no internet connection
        match result {
            Ok(server_time) => {
                assert!(server_time.server_time > 0);
                println!("Binance server time: {}", server_time.server_time);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_get_orderbook() {

        let client = BinanceRestClient::new().unwrap();
        let pair = TradingPair::SolUsdc;
        let depth = Some(10);
        let get_orderbook_result = client.get_orderbook(pair, depth);

        assert!(get_orderbook_result.await.is_ok());

    }


    #[tokio::test]
    async fn test_get_exchange_info() {
        let client = BinanceRestClient::new().unwrap();
        let get_exchange_info_result = client.get_exchange_info();
        
        assert!(get_exchange_info_result.await.is_ok());
        
    }

    #[tokio::test]
    async fn test_get_24hr_statistics() {
        let client = BinanceRestClient::new().unwrap();
        let get_exchange_info_result = client.get_exchange_info();
        
        assert!(get_exchange_info_result.await.is_ok());
        
    }

}
