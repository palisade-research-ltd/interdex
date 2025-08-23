use atelier_dcml::features;
use chrono::Utc;
use ix_execution::{features::FeatureData, queries, ClickHouseClient};

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
        .create_table(&queries::features::create_tables::create_features_table_ddl())
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

        // --------------------------------------------------------- Public Trades --- //
        // --------------------------------------------------------- ------------- --- //

        let pt_query: String = queries::trades::read_tables::read_trades_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let _pt_data: Result<Vec<queries::trades::TradeNew>, _> =
            ch_rw_client.read_table(&pt_query).await;

        // ---------------------------------------------------------- Liquidations --- //
        // ---------------------------------------------------------- ------------ --- //

        let lq_query: String =
            queries::liquidations::read_tables::read_liquidations_table(
                p_exchange.clone(),
                p_symbol.clone(),
            )
            .await
            .unwrap();

        let _lq_data: Result<Vec<queries::liquidations::LiquidationNew>, _> =
            ch_rw_client.read_table(&lq_query).await;

        // ------------------------------------------------------------ Orderbooks --- //
        // ------------------------------------------------------------ ---------- --- //

        let ob_query: String = queries::orderbooks::read_tables::read_orderbooks_table(
            p_exchange.clone(),
            p_symbol.clone(),
        )
        .await
        .unwrap();

        let ob_data: Result<Vec<queries::orderbooks::OrderbookCH>, _> =
            ch_rw_client.read_table(&ob_query).await;

        let orderbook = ob_data.unwrap()[0].to_orderbook().unwrap();

        // -------------------------------------------------------------- Features --- //
        // -------------------------------------------------------------- -------- --- //

        let selected_features =
            ["spread", "midprice", "w_midprice", "vwap", "imb", "tav"];
        let depth: usize = 10;
        let bps: f64 = 1.0;
        let feature = features::compute_features(
            orderbook,
            &selected_features,
            depth,
            bps,
            features::FeaturesOutput::Values,
        )
        .unwrap();

        // println!("orderbook features: {:?}", features_vec);
        // output_format in features::compute_features is not working, so
        // transformation from Vec<Vec<f64>> into vec![FeatureData] is needed
        // previous to writing the data into th DB.

        let feature_ts = Utc::now().timestamp_millis() as u64;

        let ft_data = FeatureData::builder()
            .feature_ts(feature_ts)
            .symbol(p_symbol.to_string())
            .exchange(p_exchange.to_string())
            .spread(feature[0].to_string())
            .midprice(feature[1].to_string())
            .w_midprice(feature[2].to_string())
            .vwap(feature[3].to_string())
            .imb(feature[4].to_string())
            .tav(feature[5].to_string())
            .build()
            .unwrap();

        let feature_query =
            queries::features::write_tables::q_insert_features(&ft_data).unwrap();
        let _ = ch_rw_client.write_table(&feature_query).await;
    });

    // --- Spawn Write ask --- //
    // let write_data_task = tokio::spawn(async move {

        // into orderflow_ind
        // into vpin_ind
        // into liquidated_ind
    // });

    // Wait for tasks
    let _ = tokio::join!(read_compute_task);
}
