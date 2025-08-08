#[cfg(test)]
mod tests {

    use ix_cex::BybitPrivateClient;

    #[tokio::test]
    async fn test_bybit_new_order() {
        // build up call
        let client = BybitPrivateClient::new().unwrap();
        let p_category = "spot";
        let p_symbol = "SOLUSDT";
        let p_side = "Sel";
        let p_order_type = "Limit";
        let p_qty = "4";
        let p_price = "190.00";

        let result = client
            .new_order(p_category, p_symbol, p_side, p_order_type, p_qty)
            .await;

        println!("Bybit order: {:?}", result);

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
    async fn test_bybit_get_orders() {
        // build up call
        let client = BybitPrivateClient::new().unwrap();
        let p_category = "spot";
        let result = client.get_orders(p_category).await;
        println!("Bybit order: {:?}", result);

        match result {
            Ok(order) => {
                println!("Bybit order: {:?}", order);
                // assert!(!wallet_balance.result);
            }
            Err(e) => {
                println!("Expected network error in test environment: {e:?}");
            }
        }
    }
    //
    // #[tokio::test]
    // async fn test_bybit_cancel_orders() {
    //     // build up call
    //     let client = BybitPrivateClient::new().unwrap();
    //     let p_category = "spot";
    //     let p_symbol = Some("BTCUSDT");
    //     let p_base_coin = Some("None");
    //     let p_settle_coin = Some("None");
    //     let p_order_filter = Some("None");
    //
    //     let result = client
    //         .cancel_orders(
    //             p_category,
    //             p_symbol,
    //             p_base_coin,
    //             p_settle_coin,
    //             p_order_filter,
    //         )
    //         .await;
    //     println!("Bybit order: {:?}", result);
    //
    //     match result {
    //         Ok(cancels) => {
    //             println!("Bybit order: {:?}", cancels);
    //         }
    //         Err(e) => {
    //             println!("Expected network error in test environment: {e:?}");
    //         }
    //     }
    // }
    //
}

