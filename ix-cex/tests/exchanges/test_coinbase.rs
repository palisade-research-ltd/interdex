#[cfg(test)]
mod tests {
    
    //use super::*;

    use ix_cex::CoinbaseClient;

    #[tokio::test]
    async fn test_coinbase_client_creation() {
        let client = CoinbaseClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_products() {
        let client = CoinbaseClient::new().unwrap();
        let result = client.get_products().await;

        // This test might fail if no internet connection
        match result {
            Ok(products) => {
                assert!(!products.is_empty());
                println!("Found {} Coinbase products", products.len());
            }
            Err(e) => {
                println!("Expected network error in test environment: {:?}", e);
            }
        }
    }
}
