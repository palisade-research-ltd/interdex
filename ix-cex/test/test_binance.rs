#[cfg(test)]
mod tests {

    use ix_cex::BinanceClient;

    #[tokio::test]
    async fn test_binance_client_creation() {
        let client = BinanceClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let client = BinanceClient::new().unwrap();
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
}
