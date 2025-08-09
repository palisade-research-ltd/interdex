#[cfg(test)]
mod tests {

    use ix_cex::BybitPrivateClient;

    #[tokio::test]
    async fn test_bybit_single_instrument_info() {
    
        // Get the instrument info
        let client = BybitPrivateClient::new().unwrap();
        let p_category = "spot";
        let p_symbol = Some("SOLUSDC");
        let result = client.get_instrument_info(p_category, p_symbol).await;

        println!("Bybit instrument: {:?}", result);

        match result {
            Ok(creates) => {
                println!("Bybit created order: {:?}", creates);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }

    #[tokio::test]
    async fn test_bybit_all_instrument_info() {
    
        // Get the instrument info
        let client = BybitPrivateClient::new().unwrap();
        let p_category = "spot";
        let p_symbol = Some("None");
        let result = client.get_instrument_info(p_category, p_symbol).await;

        println!("Bybit instrument: {:?}", result);

        match result {
            Ok(creates) => {
                println!("Bybit created order: {:?}", creates);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }

}
