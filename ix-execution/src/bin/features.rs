use ix_execution::{queries, ClickHouseClient};

#[tokio::main]
async fn main() {
    // --- Database Config --- //
    let url = "http://localhost:8123".to_string();
    let database = "default".to_string();

    // --- Database Client --- //
    let ch_read_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // --- Spawn Read & Compute Task --- //
    let read_compute_task = tokio::spawn(async move {
        // --- Public Trades --- //
        let p_symbol = "SOLUSDT".to_string();
        let p_exchange = "bybit".to_string();
        let pt_query: String = queries::trades::read_tables::read_trades_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let pt_data: Result<Vec<queries::trades::TradeNew>, _> =
            ch_read_client.read_table(&pt_query).await;

        println!("public trades data: {:?}", pt_data);

        // --- Orderbook --- //
        let p_symbol = "SOLUSDT".to_string();
        let p_exchange = "Binance".to_string();
        let ob_query: String = queries::orderbooks::read_tables::read_orderbooks_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let ob_data: Result<Vec<queries::orderbooks::OrderbookCH>, _> =
            ch_read_client.read_table(&ob_query).await;

        println!("\norderbook data: {:?}", ob_data);

        // match ob_data {
        //     Ok(data) => {
        //         println!("orderbook data count: {}", data.len());
        //         for orderbook in &data {
        //             println!(
        //                 "Exchange: {}, Symbol: {}",
        //                 orderbook.exchange, orderbook.symbol
        //             );
        //         }
        //     }
        //     Err(e) => println!("Error: {:?}", e),
        // }

        // Liquidations data (Past 10 values)
        //
        // Publictrades data (Past 10 values)

        // let features: Vec<
        //     Box<dyn Feature<Input = Orderbook, Output = f64, Config = OrderbookConfig>>,
        // > = vec![
        //     Box::new(SpreadFeature),
        //     Box::new(MidpriceFeature),
        //     Box::new(WeightedMidpriceFeature),
        //     Box::new(ImbalanceFeature),
        //     Box::new(VWAPFeature),
        //     Box::new(TAVFeature),
        // ];

        // vpin indicator (Based in Publictrades)
        // liquidated gaps indicator (Based in Liquidations)
    });

    // --- Spawn Write ask --- //
    let write_data_task = tokio::spawn(async move {

        // into orderflow_ind
        // into vpin_ind
        // into liquidated_ind
    });

    // Wait for tasks
    let _ = tokio::join!(read_compute_task, write_data_task);
}
