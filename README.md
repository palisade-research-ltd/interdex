# interdex

Off-Chain data pipeline for composable trading indicators, on-chain Solana programs, and real-time centralized-exchange (CEX) order-book capture.

`interdex` is a Rust mono-repo that already contains two key crates you can wire together today:

| Crate            | Purpose                                                         | Status |
|------------------|-----------------------------------------------------------------|--------|
| **ix-cex**       | Async client + CLI to fetch depth snapshots from **Binance, Coinbase (Advanced Trade)** and **Kraken** for the pairs `BTC/USDC` and `SOL/USDC`.        | ✅  Working (REST) |
| **ix-database**  | Thin wrapper around **ClickHouse** plus Docker assets (server, collector) to create tables, ingest Parquet, query, and monitor system tables.        | ✅  Working |

Below you will find quick-start instructions, an architectural overview, and a step-by-step guide to run everything in one Docker Compose stack so that each new order-book snapshot is persisted to ClickHouse.

## Features

### ix-cex

* Unified `ExchangeClient` trait with implementations for Binance, Coinbase, Kraken  
* Snapshot depth limits up to 1000 levels  
* Rate-limit handling, exponential back-off, structured tracing logs  
* Library **and** CLI (`cargo run -- ...`)  

### ix-database

* Async ClickHouse client with connection pool  
* Helpers for DDL, Parquet export/import, system monitoring  
* Two ready-to-use Dockerfiles:  
  * `clickhouse.Dockerfile` – vanilla server  
  * `collector.Dockerfile` – thin Rust image to run ingestion tasks  
* `docker-compose.yml` wires both containers together

### Why ClickHouse for order books?

ClickHouse is columnar, compression-friendly, and easily sustains 100k+ rows/s inserts while supporting millisecond aggregations on billions of rows – perfect for tick-level order-book analytics.

## Quick-start (one-line)

