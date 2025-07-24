// ix-cex/src/binance_ws/client.rs
use crate::exchanges::binance::models::{DepthOrDiff, StreamEvent};
use crate::results::errors::ExchangeError;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{error, info, warn};
use url::Url;

pub async fn run_websocket_client(
    tx: mpsc::Sender<DepthOrDiff>,
    streams: Vec<String>,
) -> Result<(), ExchangeError> {

    const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/stream";

    // let streams = [
    //     "btcusdt@depth5@100ms", // Partial book depth
    //     "btcusdt@depth@100ms",  // Diff. depth stream
    // ];

    let stream_names = streams.join("/");
    let url_str = format!("{BINANCE_WS_URL}?streams={stream_names}");
    let url = Url::parse(&url_str)?;

    info!("Connecting to WebSocket URL: {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket connection established.");

    let (mut write, mut read) = ws_stream.split();

    // Main message processing loop
    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<StreamEvent>(&text) {
                            Ok(event) => {
                                if tx.send(event.data).await.is_err() {
                                    error!("Receiver dropped. Shutting down websocket client.");
                                    break;
                                }
                            }
                            Err(e) => warn!("Failed to deserialize message: {}. Text: {}", e, text),
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        // Respond to pings to keep the connection alive
                        if let Err(e) = write.send(Message::Pong(ping)).await {
                            error!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed by server.");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket read error: {}", e);
                        break;
                    }
                    _ => {} // Ignore other message types
                }
            },
            else => break,
        }
    }

    warn!("WebSocket client loop terminated.");
    Ok(())
}
