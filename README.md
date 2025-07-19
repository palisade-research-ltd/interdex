# interdex

Off-Chain data pipeline for composable trading indicators, on-chain Solana programs, and real-time centralized-exchange (CEX) order-book capture.

`interdex` is a Rust mono-repo that already contains two key crates you can wire together today:

|  Crate             |  Purpose                                                         |  Status  |
|--------------------|------------------------------------------------------------------|----------|
|  **ix-cex**        | Async client + CLI to use REST endpoints to get Order books from **Binance**, **Coinbase**, and, **Kraken** | ✅  Working |
|  **ix-database**   | Wrapper around **ClickHouse** and Docker assets (server, collector) | ✅  Working |

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

```bash
git clone https://github.com/palisade-research-ltd/interdex
cd interdex/ix-database
docker compose --profile cex up --build
```

*The profile `cex` (added below) spins up:*

1. `clickhouse-server`
2. `orderbook-collector` → runs `ix-cex` in cron-like loop, writes to `clickhouse`

Stop everything with `docker compose down`.

## Detailed Setup

### 1 — Build the workspace

```bash
rustup override set stable
cargo build --workspace --release
```

### 2 — Review database schema

`ix-database/src/queries` contains helper SQL.  
For order books we'll create:

```sql
CREATE TABLE IF NOT EXISTS orderbooks
(
    ts          DateTime64(3, 'UTC'),
    exchange    LowCardinality(String),
    symbol      LowCardinality(String),
    side        Enum8('bid' = 0, 'ask' = 1),
    price       Decimal(32, 8),
    amount      Decimal(32, 8),
    level       UInt16
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(ts)
ORDER BY (symbol, exchange, side, price, ts);
```

### 3 — Docker Compose file

Add the following services (minimal excerpt):

```yaml
version: "3.9"

services:
  clickhouse:
    build:
      context: .
      dockerfile: clickhouse.Dockerfile
    ports: ["8123:8123", "9000:9000"]
    volumes: ["./data:/var/lib/clickhouse"]

  orderbook-collector:
    build:
      context: ../ix-cex
      dockerfile: ../ix-database/collector.Dockerfile
    environment:
      - CH_HOST=clickhouse
      - CH_PORT=9000
      - CH_DB=default
    depends_on: [clickhouse]
    command: |
      /usr/local/bin/ix-cex
      --all
      --pair btc-usdc
      --pair sol-usdc
      --depth 100
      --output clickhouse
      --interval 30
```

### 4 — Collector logic (already in ix-cex)

When the flag `--output clickhouse` is set, the program:

1. Fetches JSON snapshots from each exchange.  
2. Flattens bids/asks into rows with `level` index.  
3. Streams rows via HTTP `INSERT INTO orderbooks FORMAT TSV`.  
4. Sleeps for `--interval` seconds.

### 5 — Run & verify

```bash
docker compose up
# wait a few seconds ...
docker compose exec clickhouse clickhouse-client --query \
  "SELECT exchange, symbol, count() rows FROM orderbooks GROUP BY exchange, symbol;"
```

You should see non-zero row counts.

## Linking ix-cex ↔ ix-database – Feasibility Assessment

| Requirement                                               | Supported? | Notes |
|-----------------------------------------------------------|------------|-------|
| Build ix-cex into a slim image                            | ✅          | `collector.Dockerfile` uses Rust `--release`, strips symbols (<15 MB). |
| ClickHouse server in same compose network                 | ✅          | Container `clickhouse` already defined. |
| Write directly from ix-cex without Kafka/ETL              | ✅          | Uses ClickHouse HTTP streaming API. |
| Multiple exchanges / pairs                                | ✅          | Pass `--all` or repeated `--exchange`. |
| Incremental scheduling / cron                             | ✅          | Built-in `--interval` loop or use `cron` in container. |
| Schema evolution                                          | ✅          | MergeTree allows `ALTER TABLE ADD COLUMN` online. |
| High-frequency (≤1 s) ingestion                           | ⚠️          | REST snapshots are limited by exchange APIs – Binance fastest 100 ms, Kraken pushes real-time; adjust `--interval` accordingly. |
| Historical back-fill                                      | 🔜         | not implemented yet (TODO). |
| WebSocket real-time diffs                                 | 🔜         | roadmap in ix-cex README. |

**Conclusion:** nothing prevents you from running both crates together; only minimal glue (shown above) is needed.

## Roadmap

* Add WebSocket diff-stream support for micro-second latency.  
* Expose Grafana dashboards via ClickHouse plugin.  
* Support more symbols & exchanges (Bitstamp, KuCoin, OKX).  
* Continuous integration with `ix-dex` on-chain programs for hybrid CEX/DEX analytics.

## Development Tips

* Use `docker compose exec clickhouse clickhouse-client` for ad-hoc SQL.  
* Run `cargo test --workspace` to execute all unit tests.  
* Enable verbose tracing: `RUST_LOG=info cargo run -- --verbose ...`.

## License

Apache 2.0

### References

* Docker + ClickHouse best practices   
* Real-time ClickPipes ingestion patterns   
* Order-book recorder examples storing to ClickHouse
