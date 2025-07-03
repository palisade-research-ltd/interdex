use clap::{Parser, ValueEnum};
use ix_cex::{
    exchanges::{BinanceClient, CoinbaseClient, ExchangeClient, KrakenClient},
    models::{OrderBookSummary, TradingPair},
    ExchangeError,
};
use tokio::time::{sleep, timeout, Duration};
use tracing::{error, warn};
//use serde_json;
//use tracing_subscriber;

#[derive(Parser)]
#[command(name = "ix-cex")]
#[command(about = "A CLI tool to fetch order book data from centralized exchanges")]
#[command(version = "0.0.1")]
struct Cli {
    /// Exchange to query
    #[arg(short, long, value_enum)]
    exchange: Option<Exchange>,

    /// Trading pair to query
    #[arg(short, long, value_enum)]
    pair: TradingPairArg,

    /// Maximum number of order book levels to fetch
    #[arg(short, long, default_value = "100")]
    limit: u32,

    /// Output format
    #[arg(short, long, value_enum, default_value = "summary")]
    format: OutputFormat,

    /// Request timeout in seconds
    #[arg(long, default_value = "10")]
    timeout: u64,

    /// Request timeout in seconds
    #[arg(long, default_value = "1000")]
    timewait: u64,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Query all exchanges for comparison
    #[arg(short, long)]
    all: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Exchange {
    Binance,
    Coinbase,
    Kraken,
}

#[derive(ValueEnum, Clone, Debug)]
enum TradingPairArg {
    BtcUsdc,
    SolUsdc,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Summary,
    Full,
    Json,
}

impl From<TradingPairArg> for TradingPair {
    fn from(pair: TradingPairArg) -> Self {
        match pair {
            TradingPairArg::BtcUsdc => TradingPair::BtcUsdc,
            TradingPairArg::SolUsdc => TradingPair::SolUsdc,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // println!("Starting crypto exchange client");

    let trading_pair = TradingPair::from(cli.pair);

    if cli.all {
        // Query all exchanges
        let results =
            query_all_exchanges(trading_pair, cli.limit, cli.timeout, cli.timewait).await;
        display_all_results(results, &cli.format).await;
    } else if let Some(exchange) = cli.exchange {
        // Query single exchange
        let result =
            query_exchange(exchange, trading_pair, cli.limit, cli.timeout, cli.timewait)
                .await;
        display_single_result(result, &cli.format).await;
    } else {
        eprintln!("You must specify either --exchange or --all");
        std::process::exit(1);
    }

    Ok(())
}

async fn query_exchange(
    exchange: Exchange,
    pair: TradingPair,
    limit: u32,
    timeout_secs: u64,
    timewait_millis: u64,
) -> Result<ix_cex::models::OrderBook, ExchangeError> {
    // println!("Querying {:?} for {} with limit {}", exchange, pair, limit);

    let client: Box<dyn ExchangeClient + Send + Sync> = match exchange {
        Exchange::Binance => Box::new(BinanceClient::new()?),
        Exchange::Coinbase => Box::new(CoinbaseClient::new()?),
        Exchange::Kraken => Box::new(KrakenClient::new()?),
    };

    sleep(Duration::from_millis(timewait_millis)).await;

    let request_future = client.get_orderbook(pair, Some(limit));

    timeout(Duration::from_secs(timeout_secs), request_future)
        .await
        .map_err(|_| {
            ExchangeError::Timeout(format!(
                "Request timed out after {} seconds",
                timeout_secs
            ))
        })?
}

async fn query_all_exchanges(
    pair: TradingPair,
    limit: u32,
    timeout_secs: u64,
    timewait_millis: u64,
) -> Vec<(Exchange, Result<ix_cex::models::OrderBook, ExchangeError>)> {
    println!("Querying all exchanges for {} with limit {}", pair, limit);

    let exchanges = vec![Exchange::Binance, Exchange::Coinbase, Exchange::Kraken];
    let mut results = Vec::new();

    // Query exchanges concurrently
    let futures: Vec<_> = exchanges
        .into_iter()
        .map(|exchange| {
            let pair = pair.clone();
            async move {
                let result = query_exchange(
                    exchange.clone(),
                    pair,
                    limit,
                    timeout_secs,
                    timewait_millis,
                )
                .await;
                (exchange, result)
            }
        })
        .collect();

    // Wait for all results
    for future in futures {
        results.push(future.await);
    }

    results
}

async fn display_single_result(
    result: Result<ix_cex::models::OrderBook, ExchangeError>,
    format: &OutputFormat,
) {
    match result {
        Ok(orderbook) => match format {
            OutputFormat::Summary => print_summary(&orderbook),
            OutputFormat::Full => print_full(&orderbook),
            OutputFormat::Json => print_json(&orderbook),
        },
        Err(e) => {
            error!("Failed to fetch order book: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn display_all_results(
    results: Vec<(Exchange, Result<ix_cex::models::OrderBook, ExchangeError>)>,
    format: &OutputFormat,
) {
    let mut successful_results = Vec::new();
    let mut failed_results = Vec::new();

    for (exchange, result) in results {
        match result {
            Ok(orderbook) => successful_results.push((exchange, orderbook)),
            Err(e) => {
                warn!("Failed to fetch from {:?}: {:?}", exchange, e);
                failed_results.push((exchange, e));
            }
        }
    }

    if successful_results.is_empty() {
        error!("All exchanges failed to respond");
        for (exchange, error) in failed_results {
            error!("{:?}: {:?}", exchange, error);
        }
        std::process::exit(1);
    }

    match format {
        OutputFormat::Summary => print_comparison_summary(&successful_results),
        OutputFormat::Full => print_comparison_full(&successful_results),
        OutputFormat::Json => print_comparison_json(&successful_results),
    }

    if !failed_results.is_empty() {
        println!("\nFailed exchanges:");
        for (exchange, error) in failed_results {
            println!("{:?}: {:?}", exchange, error);
        }
    }
}

fn print_summary(orderbook: &ix_cex::models::OrderBook) {
    let summary = OrderBookSummary::from(orderbook);

    println!("=== {} Order Book Summary ===", orderbook.exchange);
    println!("Symbol: {}", summary.symbol);
    println!("Timestamp: {}", summary.timestamp);

    if let Some(best_bid) = summary.best_bid {
        println!("Best Bid: {}", best_bid);
    }

    if let Some(best_ask) = summary.best_ask {
        println!("Best Ask: {}", best_ask);
    }

    if let Some(spread) = summary.spread {
        println!("Spread: {}", spread);
    }

    if let Some(mid_price) = summary.mid_price {
        println!("Mid Price: {}", mid_price);
    }

    println!("Bid Levels: {}", summary.bid_count);
    println!("Ask Levels: {}", summary.ask_count);
    println!("Total Bid Volume: {}", summary.total_bid_volume);
    println!("Total Ask Volume: {}", summary.total_ask_volume);
}

fn print_full(orderbook: &ix_cex::models::OrderBook) {
    println!("=== {} Full Order Book ===", orderbook.exchange);
    println!("Symbol: {}", orderbook.symbol);
    println!("Timestamp: {}", orderbook.timestamp);

    if let Some(update_id) = orderbook.last_update_id {
        println!("Last Update ID: {}", update_id);
    }

    println!("\nBids ({}):", orderbook.bids.len());
    println!("{:<15} {:<15}", "Price", "Quantity");
    println!("{}", "-".repeat(32));
    for (i, bid) in orderbook.bids.iter().take(10).enumerate() {
        println!("{:<15} {:<15}", bid.price, bid.quantity);
        if i >= 9 && orderbook.bids.len() > 10 {
            println!("... and {} more", orderbook.bids.len() - 10);
            break;
        }
    }

    println!("\nAsks ({}):", orderbook.asks.len());
    println!("{:<15} {:<15}", "Price", "Quantity");
    println!("{}", "-".repeat(32));
    for (i, ask) in orderbook.asks.iter().take(10).enumerate() {
        println!("{:<15} {:<15}", ask.price, ask.quantity);
        if i >= 9 && orderbook.asks.len() > 10 {
            println!("... and {} more", orderbook.asks.len() - 10);
            break;
        }
    }
}

fn print_json(orderbook: &ix_cex::models::OrderBook) {
    match serde_json::to_string_pretty(orderbook) {
        Ok(json) => println!("{}", json),
        Err(e) => error!("Failed to serialize to JSON: {:?}", e),
    }
}

fn print_comparison_summary(results: &[(Exchange, ix_cex::models::OrderBook)]) {
    println!("\n=== Exchange Comparison ===\n");
    println!(
        "{:<15} {:<15} {:<15} {:<15} {:<15}",
        "Exchange", "Best Bid", "Best Ask", "Spread", "Mid Price"
    );
    println!("{}", "-".repeat(76));

    for (exchange, orderbook) in results {
        let best_bid = orderbook
            .best_bid()
            .map(|b| b.price.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let best_ask = orderbook
            .best_ask()
            .map(|a| a.price.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let spread = orderbook
            .spread()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let mid_price = orderbook
            .mid_price()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        println!(
            "{:<15} {:<15} {:<15} {:<15} {:<15}",
            format!("{:?}", exchange),
            best_bid,
            best_ask,
            spread,
            mid_price
        );
    }

    println!("\n");
}

fn print_comparison_full(results: &[(Exchange, ix_cex::models::OrderBook)]) {
    for (exchange, orderbook) in results {
        println!("\n=== {:?} ===", exchange);
        print_full(orderbook);
    }
}

fn print_comparison_json(results: &[(Exchange, ix_cex::models::OrderBook)]) {
    let json_data: Vec<_> = results
        .iter()
        .map(|(exchange, orderbook)| {
            serde_json::json!({
                "exchange": format!("{:?}", exchange),
                "orderbook": orderbook
            })
        })
        .collect();

    match serde_json::to_string_pretty(&json_data) {
        Ok(json) => println!("{}", json),
        Err(e) => error!("Failed to serialize to JSON: {:?}", e),
    }
}
