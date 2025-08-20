use atelier_base::orderbooks::Orderbook;
use atelier_dcml::features;
use ix_execution::{queries, ClickHouseClient};

#[tokio::main]
async fn main() {
    // --- Database Config --- //
    let ch_url = "http://localhost:8123".to_string();
    let ch_db = "default".to_string();

    let ch_admin_client = ClickHouseClient::builder()
        .url(ch_url.clone())
        .database(ch_db.clone())
        .build()
        .await
        .unwrap();

    let _ = ch_admin_client
        .create_table(&queries::features::create_tables::create_order_features_table_ddl())
        .await;

    // --- Database Client --- //
    let ch_rw_client = ClickHouseClient::builder()
        .url(&ch_url)
        .database(&ch_db)
        .build()
        .await
        .unwrap();

    // --- Spawn Read & Compute Task --- //
    let read_compute_task = tokio::spawn(async move {
        let p_symbol = "SOLUSDT".to_string();
        let p_exchange = "Bybit".to_string();

        // --- Public Trades --- //
        let pt_query: String = queries::trades::read_tables::read_trades_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let pt_data: Result<Vec<queries::trades::TradeNew>, _> =
            ch_rw_client.read_table(&pt_query).await;
        println!("public trades data: {:?}", pt_data);

        // --- Orderbook --- //
        let ob_query: String = queries::orderbooks::read_tables::read_orderbooks_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let ob_data: Result<Vec<queries::orderbooks::OrderbookCH>, _> =
            ch_rw_client.read_table(&ob_query).await;
        println!("\norderbook data: {:?}", ob_data);

        let orderbook: Vec<Orderbook> = ob_data
            .unwrap()
            .iter()
            .map(|ob| ob.to_orderbook().unwrap())
            .collect();

        // --- Liquidations --- //
        let lq_query: String =
            queries::liquidations::read_tables::read_liquidations_table(
                p_exchange.clone(),
                p_symbol.clone(),
            )
            .await
            .unwrap();

        let lq_data: Result<Vec<queries::liquidations::LiquidationNew>, _> =
            ch_rw_client.read_table(&lq_query).await;
        println!("\nliquidations data: {:?}", lq_data);

        let selected_features =
            ["spread", "midprice", "w_midprice", "vwap", "imb", "tav"];
        let depth: usize = 10;
        let bps: f64 = 1.0;
        let features_vec = features::compute_features(
            &orderbook.as_ref(),
            &selected_features,
            depth,
            bps,
            features::FeaturesOutput::Values,
        ).unwrap();

        println!("orderbook features: {:?}", features_vec);

        let trade_query =
            queries::features::write_tables::q_insert_features(&features_vec).unwrap();
        let _ = ch_rw_client.write_table(&trade_query).await;



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
