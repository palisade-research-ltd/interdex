// src/bin/collector.rs

use ix_execution::ClickHouseClient;

#[tokio::main]
async fn main() {

    // -- Database Config
    let url = "http://localhost:8123".to_string();
    let database = "default".to_string();

    // -- Database Client
    let _ch_read_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // -- Database Client
    let _ch_write_client = ClickHouseClient::builder()
        .url(&url)
        .database(&database)
        .build()
        .await
        .unwrap();

    // --- Spawn Read ask --- //
    let read_data_task = tokio::spawn(async move {
    });

    // --- Spawn Compute ask --- //
    let compute_data_task = tokio::spawn(async move {
    });

    // --- Spawn Write ask --- //
    let write_data_task = tokio::spawn(async move {
    });

    // Wait for tasks
    let _ = tokio::join!(read_data_task, compute_data_task, write_data_task);

}
