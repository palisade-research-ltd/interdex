#[cfg(test)]
mod tests {

    use ix_cex::exchanges::BinanceRestClient;

    #[tokio::test]
    async fn test_binance_client_creation() {
        let client = BinanceRestClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_dynamic_stream_creation() {
        let stream = "";
        println!("stream: {stream:?}");
    }
}
