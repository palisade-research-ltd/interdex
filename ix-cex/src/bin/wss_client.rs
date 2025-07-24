use ix_cex::exchanges::binance::{binance_wss, models::DepthOrDiff};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // The channel for sending data from the websocket client to the DB writer
    let (tx_0, _rx_0) = mpsc::channel::<DepthOrDiff>(10_000);
    let (tx_1, _rx_1) = mpsc::channel::<DepthOrDiff>(10_000);

    // Spawn the database writer task
    // let db_handle = tokio::spawn(db::database_writer(rx));
    // info!("Database writer task started.");
    
    let streams_0 = vec![
        "btcusdt@depth5@100ms".to_string(), // Partial book depth
        "btcusdt@depth@100ms".to_string(),  // Diff. depth stream
    ];

    let streams_1 = vec![
        "btcusdt@depth5@100ms".to_string(), // Partial book depth
        "btcusdt@depth@100ms".to_string(),  // Diff. depth stream
    ];

    // Spawn the websocket client task
    let client_handle_0 = tokio::spawn(binance_wss::run_websocket_client(tx_0, streams_0));
    let client_handle_1 = tokio::spawn(binance_wss::run_websocket_client(tx_1, streams_1));

    // Await both tasks to complete
    let (db_res, client_res) =
        tokio::try_join!(client_handle_0, client_handle_1).unwrap();

    db_res?;
    client_res?;

    Ok(())
}
