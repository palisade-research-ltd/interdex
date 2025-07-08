# Centralized Exchanges Client

Library and CLI tool for querying centralized exchange APIs to retrieve order book snapshots. Currently supports **Binance**, **Coinbase**, and **Kraken** exchanges for **BTC/USDC** and **SOL/USDC** trading pairs.

## Features

- 🚀 **Async/await support** using Tokio runtime
- 🔄 **Automatic retries** with exponential backoff
- ⚡ **Rate limiting** to respect exchange API limits
- 🛡️ **Robust error handling** with detailed error types
- 📊 **Order book validation** and analysis tools
- 🔌 **Unified interface** across multiple exchanges
- 🖥️ **CLI tool** for easy command-line usage
- 📦 **Library crate** for integration into other projects

## Quick Start

### As a CLI Tool

```bash
# Fetch BTC/USDC order book from Binance
cargo run -- --exchange binance --pair btc-usdc --depth 50

# Get full order book data in JSON format
cargo run -- --exchange coinbase --pair btc-usdc --format json --depth 100
```

# Compare exchanges 

Select the pair `sol-usdc`, exchanges `--all`, number of levels per orderbook `--limit 10`, a summary as the format of the output `--format summary`.

```
cargo run -- --pair sol-usdc --all --limit 10 --format summary
```

with a result: 

```
Querying all exchanges for SOL/USDC with limit 10
2025-07-08T02:00:09.190581Z  INFO ix_cex::exchanges::binance::binance: Fetching Binance orderbook for SOLUSDC with limit 10
2025-07-08T02:00:09.420868Z  INFO ix_cex::exchanges::binance::binance: Successfully converted Binance orderbook: 10 bids, 10 asks, spread: Some(0.01000000)
2025-07-08T02:00:10.422446Z  INFO ix_cex::exchanges::coinbase::coinbase: Fetching Coinbase orderbook for SOL-USDC
2025-07-08T02:00:10.787503Z  INFO ix_cex::exchanges::coinbase::coinbase: Successfully converted Coinbase orderbook: 10 bids, 10 asks, spread: Some(0.01)
2025-07-08T02:00:11.791522Z  INFO ix_cex::exchanges::kraken::kraken: Fetching Kraken orderbook for SOL/USDC
2025-07-08T02:00:12.510733Z  INFO ix_cex::exchanges::kraken::kraken: Successfully converted Kraken orderbook: 10 bids, 10 asks, spread: Some(0.010000)

=== Exchange Comparison ===

Exchange        Best Bid        Best Ask        Spread          Mid Price      
----------------------------------------------------------------------------
Binance         148.68000000    148.69000000    0.01000000      148.68500000   
Coinbase        148.67          148.68          0.01            148.6750       
Kraken          148.760000      148.770000      0.010000        148.765000     
```

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
ix_cex = { path = "." }
tokio = { version = "1.0", features = ["full"] }
```

```rust
use ix_cex::exchanges::{BinanceClient, ExchangeClient};
use ix_cex::models::TradingPair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Binance client
    let client = BinanceClient::new()?;
    
    // Fetch order book snapshot
    let orderbook = client.get_orderbook(TradingPair::BtcUsdc, Some(100)).await?;
    
    // Analyze the data
    println!("Best bid: {:?}", orderbook.best_bid());
    println!("Best ask: {:?}", orderbook.best_ask());
    println!("Spread: {:?}", orderbook.spread());
    println!("Mid price: {:?}", orderbook.mid_price());
    
    // Calculate liquidity within 1% of mid price
    let (bid_liq, ask_liq) = orderbook.liquidity_within_percentage(rust_decimal::Decimal::from(1));
    println!("Bid liquidity within 1%: {}", bid_liq);
    println!("Ask liquidity within 1%: {}", ask_liq);
    
    Ok(())
}
```

## Project Structure

```
ix-cex/
├── src/
│   ├── main.rs              # CLI application
│   ├── lib.rs               # Library root
│   ├── error.rs             # Error handling
│   ├── models/
│   │   ├── mod.rs
│   │   └── orderbook.rs     # Order book data structures
│   ├── client/
│   │   ├── mod.rs
│   │   └── http_client.rs   # HTTP client with rate limiting
│   └── exchanges/
│       ├── mod.rs
│       ├── binance.rs       # Binance API implementation
│       ├── coinbase.rs      # Coinbase API implementation
│       └── kraken.rs        # Kraken API implementation
├── Cargo.toml
└── README.md
```

## API Endpoints Used

### Binance
- **Endpoint**: `GET /api/v3/depth`
- **Documentation**: https://developers.binance.com/docs/binance-spot-api-docs/rest-api/market-data-endpoints
- **Rate Limit**: 5-250 weight depending on limit
- **Symbols**: 
  - BTC/USDC: `BTCUSDC`
  - SOL/USDC: `SOLUSDC`

### Coinbase (Advanced Trade API)
- **Endpoint**: `GET /api/v3/brokerage/product_book`
- **Documentation**: https://docs.cdp.coinbase.com/coinbase-app/advanced-trade-apis/
- **Rate Limit**: 10 requests/second
- **Symbols**: 
  - BTC/USDC: `BTC-USDC`
  - SOL/USDC: `SOL-USDC`

### Kraken
- **Endpoint**: `GET /0/public/Depth`
- **Documentation**: https://docs.kraken.com/api/docs/rest-api/get-order-book/
- **Rate Limit**: 1 request/second (conservative)
- **Symbols**: 
  - BTC/USDC: `XBTUSDC`
  - SOL/USDC: `SOLUSDC`

## Usage Examples

### CLI Usage

```bash
# Basic usage
cargo run -- --exchange binance --pair btc-usdc

# Compare all exchanges
cargo run -- --pair sol-usdc --all

# Get detailed output with verbose logging
cargo run -- --exchange kraken --pair btc-usdc --format full --verbose

# Set custom timeout and limit
cargo run -- --exchange coinbase --pair sol-usdc --limit 200 --timeout 60

# JSON output for processing
cargo run -- --pair btc-usdc --all --format json > orderbooks.json
```

### Library Usage

#### Query Multiple Exchanges

```rust
use ix_cex::exchanges::*;
use ix_cex::models::TradingPair;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pair = TradingPair::BtcUsdc;
    
    // Create clients
    let binance = BinanceClient::new()?;
    let coinbase = CoinbaseClient::new()?;
    let kraken = KrakenClient::new()?;
    
    // Query all exchanges concurrently
    let (binance_result, coinbase_result, kraken_result) = tokio::try_join!(
        binance.get_orderbook(pair.clone(), Some(100)),
        coinbase.get_orderbook(pair.clone(), Some(100)),
        kraken.get_orderbook(pair.clone(), Some(100))
    )?;
    
    // Compare spreads
    println!("Binance spread: {:?}", binance_result.spread());
    println!("Coinbase spread: {:?}", coinbase_result.spread());
    println!("Kraken spread: {:?}", kraken_result.spread());
    
    Ok(())
}
```

#### Custom Error Handling

```rust
use ix_cex::{ExchangeError, exchanges::BinanceClient, models::TradingPair};

#[tokio::main]
async fn main() {
    let client = BinanceClient::new().unwrap();
    
    match client.get_orderbook(TradingPair::BtcUsdc, Some(100)).await {
        Ok(orderbook) => {
            println!("Success: {} levels", orderbook.bids.len() + orderbook.asks.len());
        }
        Err(ExchangeError::RateLimit { exchange }) => {
            println!("Rate limited by {}, retrying later...", exchange);
            // Implement retry logic
        }
        Err(ExchangeError::Network(e)) => {
            println!("Network error: {}", e);
            // Handle network issues
        }
        Err(e) => {
            println!("Other error: {:?}", e);
        }
    }
}
```

#### Order Book Analysis

```rust
use ix_cex::exchanges::BinanceClient;
use ix_cex::models::{TradingPair, OrderBookSummary};
use rust_decimal::Decimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BinanceClient::new()?;
    let orderbook = client.get_orderbook(TradingPair::BtcUsdc, Some(500)).await?;
    
    // Basic metrics
    let summary = OrderBookSummary::from(&orderbook);
    println!("Summary: {:#?}", summary);
    
    // Liquidity analysis
    let percentages = vec![
        Decimal::from_str("0.1")?,  // 0.1%
        Decimal::from_str("0.5")?,  // 0.5%
        Decimal::from_str("1.0")?,  // 1.0%
        Decimal::from_str("2.0")?,  // 2.0%
    ];
    
    for pct in percentages {
        let (bid_liq, ask_liq) = orderbook.liquidity_within_percentage(pct);
        println!("Liquidity within {}%: Bids={}, Asks={}", pct, bid_liq, ask_liq);
    }
    
    // Order book validation
    if orderbook.is_valid() {
        println!("Order book is valid ✓");
    } else {
        println!("Order book validation failed ✗");
    }
    
    Ok(())
}
```

## Configuration

### Rate Limiting

Each exchange client is configured with conservative rate limits:

- **Binance**: 10 requests/second
- **Coinbase**: 10 requests/second  
- **Kraken**: 1 request/second

These can be adjusted in the client constructors if needed.

### Timeouts

Default timeouts are set to 10 seconds per request, with automatic retries for transient failures.

### Retry Logic

The client implements exponential backoff with the following defaults:
- **Max retries**: 3
- **Initial delay**: 500ms
- **Max delay**: 30 seconds
- **Backoff factor**: 2.0

## Error Handling

The library provides comprehensive error types:

- `ExchangeError::Network` - HTTP/network issues
- `ExchangeError::RateLimit` - API rate limit exceeded
- `ExchangeError::ApiError` - Exchange-specific API errors
- `ExchangeError::JsonParsing` - Response parsing failures
- `ExchangeError::InvalidTradingPair` - Unsupported trading pair
- `ExchangeError::Timeout` - Request timeout
- `ExchangeError::Authentication` - API key issues (future)

All errors include context about which exchange and operation failed.

## Dependencies

Key dependencies used in this project:

- **tokio** - Async runtime
- **reqwest** - HTTP client
- **serde** - Serialization/deserialization
- **anyhow** & **thiserror** - Error handling
- **async-rate-limiter** - API rate limiting
- **rust_decimal** - Precise decimal arithmetic
- **chrono** - Date/time handling
- **clap** - CLI argument parsing
- **tracing** - Structured logging

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_binance_client_creation
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Linting

```bash
# Run clippy lints
cargo clippy

# Run formatter
cargo fmt

# Check formatting
cargo fmt --check
```

### Documentation

```bash
# Generate documentation
cargo doc --open
```

## Future Enhancements

- [ ] WebSocket support for real-time order book updates
- [ ] Additional exchanges (Binance.US, Huobi, etc.)
- [ ] More trading pairs
- [ ] Order book diff calculations
- [ ] Historical data support
- [ ] Authentication for private endpoints
- [ ] Benchmarking and performance optimization
- [ ] Configuration file support
- [ ] Docker containerization

## API Rate Limits

Rate limits of each exchange:

- **Binance**: https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-api-information#limits
- **Coinbase**: https://docs.cdp.coinbase.com/advanced-trade-api/docs/rate-limits
- **Kraken**: https://docs.kraken.com/api/docs/rest-api/usage-limits

