// src/bin/signals.rs

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
    let signal_task = tokio::spawn(async move {
    });

    // Wait for tasks
    let _ = tokio::join!(signal_task);

}
