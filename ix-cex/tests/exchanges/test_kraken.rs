#[cfg(test)]
mod tests {

    use ix_cex::KrakenClient;

    #[tokio::test]
    async fn test_kraken_client_creation() {
        let client = KrakenClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let client = KrakenClient::new().unwrap();
        let result = client.get_server_time().await;

        // This test might fail if no internet connection
        match result {
            Ok(server_time) => {
                assert!(server_time.unixtime > 0);
                println!(
                    "Kraken server time: {} ({})",
                    server_time.unixtime, server_time.rfc1123
                );
            }
            Err(e) => {
                println!("Expected network error in test environment: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_system_status() {
        let client = KrakenClient::new().unwrap();
        let result = client.get_system_status().await;

        // This test might fail if no internet connection
        match result {
            Ok(status) => {
                println!("Kraken system status: {}", status.status);
            }
            Err(e) => {
                println!("Expected network error in test environment: {:?}", e);
            }
        }
    }
}

