// ix-cex/src/main.rs

// Adjust the use path according to your project structure
use ix_cex::exchanges::binance::{
    binance_wss,
    models::{orderbook::DepthOrDiff, trade::AggOrTrade},
};

use chrono::{TimeZone, Utc};
use tokio::sync::mpsc;
use tracing::{info, instrument};

/// Process Public Trade update and formats it for logging.
///
///
fn _print_trade_update(data: AggOrTrade) {
    let (trade_ts, price, quantity, update_type) = match data {
        AggOrTrade::Trade(trade) => (
            trade.trade_ts,
            trade.price,
            trade.quantity,
            trade.event_type,
        ),
    };

    // Convert Milliseconds Unix Timestamp to a readable DateTime<Utc>
    let trade_timestamp = Utc.timestamp_millis_opt(trade_ts as i64).unwrap();

    // Log the formatted output
    info!(
        "\n\n| Update Type: {} \n| Trade TS: {} \n| Price: {} \n| Quantity: {}\n",
        update_type,
        trade_timestamp.to_rfc3339(),
        price,
        quantity,
    );
}

/// Processes an order book update and formats it for logging.
///
/// This function extracts the best bid and ask from either a differential
/// update or a partial book snapshot and prints it to the console.
#[instrument(skip_all, name = "process_orderbook_update")]
fn print_orderbook_update(data: DepthOrDiff) {
    let (timestamp_ms, bids, asks, update_type) = match data {
        // Differential update provides an event time
        DepthOrDiff::Diff(diff) => (diff.event_time, diff.bids, diff.asks, "Diff"),

        // Partial book snapshot does not; we use the current time
        DepthOrDiff::PartialBook(partial) => (
            Utc::now().timestamp_millis() as u64,
            partial.bids,
            partial.asks,
            "Snapshot",
        ),
    };

    // Convert Milliseconds Unix Timestamp to a readable DateTime<Utc>
    let timestamp = Utc.timestamp_millis_opt(timestamp_ms as i64).unwrap();

    // Binance sends bids sorted high-to-low and asks sorted low-to-high.
    // The "best" is always the first element in the list.
    let best_bid_str = if let Some(level) = bids.first() {
        format!("Price: {:<12} Amount: {:<12}", level.price, level.qty)
    } else {
        "N/A".to_string()
    };

    let best_ask_str = if let Some(level) = asks.first() {
        format!("Price: {:<12} Amount: {:<12}", level.price, level.qty)
    } else {
        "N/A".to_string()
    };

    // Log the formatted output
    info!(
        "\n\n| Update Type: {} \n| Timestamp: {} \n| Bid: {} \n| Ask: {}\n",
        update_type,
        timestamp.to_rfc3339(),
        best_bid_str,
        best_ask_str
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ob_streams = vec![String::from("solusdc@depth20@100ms")];

    // Initialize logging so we can see the output from `info!`
    tracing_subscriber::fmt::init();

    // We only need one channel for one client
    let (tx, mut rx) = mpsc::channel::<DepthOrDiff>(10_000);

    // Spawn the websocket client task. It will send data into `tx`.
    let client_handle = tokio::spawn(binance_wss::run_websocket_client(tx, ob_streams));
    
    info!("Binance WebSocket client started.");

    // This is the consumer task. It receives data from `rx` and prints it.
    let printer_handle = tokio::spawn(async move {
        // Loop forever, waiting for messages from the client
        while let Some(data) = rx.recv().await {
            print_orderbook_update(data);
        }
        info!("Printer task finished.");
    });

    info!("Order book printer started.");

    // Await both tasks. The program will exit if either one fails or finishes.
    let (client_res, _printer_res) = tokio::try_join!(
        client_handle,
        printer_handle
    )?;

    client_res?;
    Ok(())

}

