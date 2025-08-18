
use atelier_base::orderbooks::Orderbook;
use ix_execution::ClickHouseClient;
use atelier_dcml::{
    features::{
        ImbalanceFeature, MidpriceFeature, SpreadFeature, TAVFeature, VWAPFeature,
        WeightedMidpriceFeature,
    },
    Feature, OrderbookConfig,
};

#[tokio::main]
async fn main() {
    // --- Database Config --- //
    let url = "http://localhost:8123".to_string();
    let database = "default".to_string();

    // --- Database Client --- //
    let _ch_read_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // --- Database Client --- //
    let _ch_write_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // --- Spawn Read ask --- //
    let read_data_task = tokio::spawn(async move {

        // Orderbooks data
        //
        // Liquidations data
        //
        // Publictrades data
    
    });

    // --- Spawn Compute ask --- //
    let compute_data_task = tokio::spawn(async move {
        // order flow indicator (Based in Orderbooks)

        let features: Vec<
            Box<dyn Feature<Input = Orderbook, Output = f64, Config = OrderbookConfig>>,
        > = vec![
            Box::new(SpreadFeature),
            Box::new(MidpriceFeature),
            Box::new(WeightedMidpriceFeature),
            Box::new(ImbalanceFeature),
            Box::new(VWAPFeature),
            Box::new(TAVFeature),
        ];

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
    let _ = tokio::join!(read_data_task, compute_data_task, write_data_task);
}

