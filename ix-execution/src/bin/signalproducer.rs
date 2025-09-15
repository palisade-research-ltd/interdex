// src/bin/signals.rs

use atelier_quant::VpinCalculator;
use ix_execution::ClickHouseClient;

#[tokio::main]
async fn main() {

    // -- Database Config
    let url = "http://localhost:8123".to_string();
    let database = "default".to_string();

    // -- Database Client
    let _ch_signals_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // --- Spawn Read/Compute/Write Ask --- //
    let data_task = tokio::spawn(async move {
                // Test that the example code runs without panicking
        let _trades: Vec<f64> = vec![];

    });

    // --- Spawn Read/Compute/Write Ask --- //
    let signal_task = tokio::spawn(async move {
        
        // Test that the example code runs without panicking
        let trades = vec![];

        // Test basic VPIN calculation
        let mut calculator = VpinCalculator::new(500.0, 10, 0.25).unwrap();
        let _result = calculator.process_trades(&trades);

    });

    // Wait for tasks
    let _ = tokio::join!(data_task, signal_task);

}
